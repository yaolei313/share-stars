use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct KeySetting {
    #[validate(length(min = 1))]
    pub kid: String,
    #[validate(length(min = 1))]
    pub public_key_path: String,
    #[validate(length(min = 1))]
    pub private_key_path: String,
}

#[derive(Clone)]
struct JwtKey {
    kid: String,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

#[derive(Clone)]
pub struct JwtDelegate {
    keys: HashMap<String, JwtKey>,
    default_kid: String,
}

impl JwtDelegate {
    pub fn new(keys: &Vec<KeySetting>) -> anyhow::Result<Self> {
        let default_kid = keys.first().unwrap().kid.clone();

        let tmp_keys = keys
            .iter()
            .map(|k| {
                let rsa_private_pem = fs::read_to_string(&k.private_key_path)?;
                let rsa_public_pem = fs::read_to_string(&k.public_key_path)?;

                let encoding_key = EncodingKey::from_rsa_pem(rsa_private_pem.as_bytes())?;
                let decoding_key = DecodingKey::from_rsa_pem(rsa_public_pem.as_bytes())?;

                Ok((
                    k.kid.clone(),
                    JwtKey {
                        kid: k.kid.clone(),
                        encoding_key,
                        decoding_key,
                    },
                ))
            })
            .collect::<anyhow::Result<HashMap<String, JwtKey>>>()?;

        Ok(JwtDelegate {
            keys: tmp_keys,
            default_kid,
        })
    }

    pub fn generate_jwt_token<T: Serialize>(
        &self,
        claims: &T,
        alg: Algorithm,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let key = self.keys.get(&self.default_kid).expect("always found");
        let mut header = Header::new(alg);
        header.kid = Some(key.kid.clone());
        let token = jsonwebtoken::encode(&header, claims, &key.encoding_key)?;
        Ok(token)
    }

    pub fn validate_jwt_token<T: DeserializeOwned>(
        &self,
        token: &str,
        validation: &Validation,
    ) -> Option<T> {
        let header = jsonwebtoken::decode_header(token).ok()?;
        let Some(ref kid) = header.kid else {
            tracing::warn!("invalid token header. {}", token);
            return None;
        };
        let Some(key) = self.keys.get(kid) else {
            tracing::warn!("invalid token header kid. {} {}", token, kid);
            return None;
        };
        jsonwebtoken::decode::<T>(token, &key.decoding_key, validation)
            .ok()
            .map(|d| d.claims)
    }
}
