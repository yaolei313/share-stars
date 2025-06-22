use jsonwebtoken::{DecodingKey, EncodingKey};
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Clone)]
pub struct JwtKey {
    pub kid: String,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}
#[derive(Clone)]
pub struct JwtManager {
    pub keys: HashMap<String, JwtKey>,
    pub issuer: String,
    pub expire_seconds: u32,
    pub default_kid: String,
}

impl JwtManager {
    pub fn new(settings: &crate::config::JwtSetting) -> anyhow::Result<Self> {
        if settings.keys.is_empty() || settings.issuer.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "invalid config"))?;
        }
        let default_kid = settings.keys.first().unwrap().kid.clone();

        let tmp_keys = settings
            .keys
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

        Ok(JwtManager {
            keys: tmp_keys,
            issuer: settings.issuer.clone(),
            expire_seconds: settings.expire_seconds,
            default_kid,
        })
    }

    pub fn get_default_jwt_key(&self) -> Option<&JwtKey> {
        self.keys.get(&self.default_kid)
    }
}
