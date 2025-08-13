use crate::biz::dto::{AuthnMethodEnum, TokenInfo};
use crate::biz::token::JwtManager;
use crate::http::vo::error::AppError;
use crate::http::vo::{AppResult, RequestInfo};
use chrono::Utc;
use jsonwebtoken::{decode, decode_header, encode, Algorithm, Header};
use serde::{Deserialize, Serialize};

pub struct TokenService {
    jwt_manager: JwtManager,
}

impl TokenService {
    pub fn new(jwt_manager: JwtManager) -> Self {
        Self { jwt_manager }
    }

    pub fn create_access_token(
        &self,
        user_id: i64,
        authn_method: &AuthnMethodEnum,
        req_info: &RequestInfo,
    ) -> AppResult<TokenInfo> {
        let manager = &self.jwt_manager;
        let Some(key) = manager.get_default_jwt_key() else {
            return Err(AppError::ComponentInvalidConfig(
                "not found default jwt key config",
            ));
        };
        let expires_in = manager.expire_seconds as i64;
        let iat = Utc::now().timestamp();
        let exp = iat + expires_in;
        let dvf = format!("{}-{}", &req_info.platform.code(), &req_info.device_id);
        let claims = AccessTokenClaims {
            aud: manager.audience.clone(),
            exp,
            iat,
            iss: manager.issuer.to_string(),
            sub: user_id,
            dvf,
            aum: authn_method.code(),
        };
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(key.kid.clone());
        let token = encode(&header, &claims, &key.encoding_key)?;
        Ok(TokenInfo {
            access_token: token,
            expires_in,
            refresh_token: None,
        })
    }

    pub fn validate_access_token(&self, token: &str) -> Option<AccessTokenClaims> {
        let manager = &self.jwt_manager;
        let header = decode_header(token).ok()?;
        let Some(ref kid) = header.kid else {
            log::warn!("invalid token header. {}", token);
            return None;
        };
        let Some(key) = manager.keys.get(kid) else {
            log::warn!("invalid token header kid. {} {}", token, kid);
            return None;
        };
        let Ok(data) = decode::<AccessTokenClaims>(token, &key.decoding_key, &manager.validation)
        else {
            log::warn!("invalid token kid. {} {}", token, kid);
            return None;
        };

        Some(data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub aud: String, // Optional. Audience 令牌是发给谁
    pub exp: i64,    // Required. Expiration time (as UTC timestamp)
    pub iat: i64,    // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer   令牌颁发人，比如接入google oidc，那这里就是google
    pub sub: i64,    // Required. Subject (user id)
    pub dvf: String,
    pub aum: i32,
}
