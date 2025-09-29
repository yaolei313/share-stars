use crate::biz::dto::{AuthnMethod, TokenInfo};
use crate::config::JwtSetting;
use crate::http::vo::{AppResult, RequestInfo};
use chrono::Utc;
use jsonwebtoken::{Algorithm, Validation};
use lib_utils::JwtDelegate;
use serde::{Deserialize, Serialize};

pub struct AccessTokenService {
    jwt_delegate: JwtDelegate,
    algorithm: Algorithm,
    issuer: String,
    audience: String,
    expire_seconds: u32,
    validation: Validation,
}

impl AccessTokenService {
    pub fn new(settings: &JwtSetting) -> anyhow::Result<Self> {
        let jwt_delegate = JwtDelegate::new(&settings.keys)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[settings.audience.clone()]);
        validation.set_issuer(&[settings.issuer.clone()]);
        Ok(Self {
            jwt_delegate,
            algorithm: Algorithm::RS256,
            issuer: settings.issuer.clone(),
            audience: settings.audience.clone(),
            expire_seconds: settings.expire_seconds,
            validation,
        })
    }

    pub fn create_access_token(
        &self,
        user_id: i64,
        authn_method: AuthnMethod,
        req_info: &RequestInfo,
    ) -> AppResult<TokenInfo> {
        let iat = Utc::now().timestamp();
        let exp = iat + self.expire_seconds as i64;
        let claims = AccessTokenClaims {
            aud: self.audience.clone(),
            exp,
            iat,
            iss: self.issuer.clone(),
            sub: user_id,
            dvf: req_info.device_id.clone(),
            aum: authn_method.code(),
        };

        let token = self
            .jwt_delegate
            .generate_jwt_token(&claims, self.algorithm)?;
        Ok(TokenInfo {
            access_token: token,
            refresh_token: None,
            expires_in: self.expire_seconds as i64,
        })
    }

    pub fn validate_access_token(&self, token: &str) -> Option<AccessTokenClaims> {
        let claims = self
            .jwt_delegate
            .validate_jwt_token::<AccessTokenClaims>(token, &self.validation)?;
        if !self.verify_validity_of_claims(&claims) {
            return None;
        };

        Some(claims)
    }

    fn verify_validity_of_claims(&self, claims: &AccessTokenClaims) -> bool {
        let now = Utc::now().timestamp();
        let exp = claims.exp;

        todo!()
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
