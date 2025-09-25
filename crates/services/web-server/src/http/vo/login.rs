use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginByPasswordReq {
    #[validate(length(
        min = 11,
        max = 15,
        message = "phone must be between 11 and 15 digits long."
    ))]
    pub phone: String,

    #[validate(length(
        min = 8,
        max = 16,
        message = "password must be between 8 and 16 digits long."
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
