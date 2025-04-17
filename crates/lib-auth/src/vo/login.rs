use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginByPasswordReq {
    pub phone: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginBySmsReq {
    pub phone: String,
    pub sms_code: String,
}

#[derive(Serialize)]
pub struct LoginResult {
    pub user_id: u64,
    pub new_register: bool,
    pub access_token: Option<String>,
    pub expire_seconds: u64,
    pub refresh_token: Option<String>,
}
