use crate::biz::dto::{AuthnTypeEnum, TokenInfo};
use crate::config::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::{AppResult, DeviceInfo};
use chrono::Utc;
use jsonwebtoken::{Algorithm, Header, encode};
use serde::{Deserialize, Serialize};

pub async fn create_token(
    state: AppState,
    login_method: AuthnTypeEnum,
    user_id: i64,
    device_info: &DeviceInfo,
) -> AppResult<TokenInfo> {
    let manager = &state.service_state.jwt_manager;
    let Some(key) = manager.get_default_jwt_key() else {
        return Err(AppError::InvalidConfig("not found default jwt key config"));
    };
    let expires_in = manager.expire_seconds as i64;
    let iat = Utc::now().timestamp();
    let exp = iat + expires_in;
    let dvf = format!(
        "{}-{}",
        &device_info.platform.code(),
        &device_info
            .device_fp
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("")
    );
    let claims = Claims {
        aud: login_method.to_string(),
        exp,
        iat,
        iss: "https://accounts.share-stars.com".to_string(),
        sub: user_id.to_string(),
        dvf,
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience 令牌是发给谁
    exp: i64,    // Required. Expiration time (as UTC timestamp)
    iat: i64,    // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer   令牌颁发人，比如接入google oidc，那这里就是google
    sub: String, // Required. Subject (user id)
    dvf: String,
}
