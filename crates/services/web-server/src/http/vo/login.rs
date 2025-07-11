use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginByPasswordReq {
    #[validate(length(
        min = 11,
        max = 15,
        message = "phone must at least be 11 characters and at most 15"
    ))]
    pub phone: String,

    #[validate(length(
        min = 6,
        max = 16,
        message = "password must at least 6 characters and at most 16"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginBySmsReq {
    #[validate(length(min = 11, max = 15, message = "phone is required"))]
    pub phone: String,
    #[validate(length(min = 6, max = 6, message = "sms code is required"))]
    pub sms_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResult {
    pub user_id: i64,
    pub new_register: bool,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}
