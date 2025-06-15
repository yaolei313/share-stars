use crate::biz::dto::{AuthnTypeEnum, TokenInfo};
use crate::http::vo::{AppResult, DeviceInfo};
use serde::{Deserialize, Serialize};

pub async fn create_token(
    login_method: AuthnTypeEnum,
    user_id: i64,
    device_info: &DeviceInfo,
) -> AppResult<TokenInfo> {
    let claims = Claims {
        aud: "".to_string(),
        exp: 0,
        iat: 0,
        iss: "".to_string(),
        nbf: 0,
        sub: "".to_string(),
    };
    // let token = encode(
    //     &Header::new(Algorithm::RS256),
    //     &claims,
    //     &EncodingKey::from_rsa_pem(include_bytes!("privkey.pem"))?,
    // )?;

    todo!()
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (user id)
}
