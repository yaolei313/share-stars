use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Validate)]
pub struct LoginByPasswordReq {
    pub phone: String,
    pub password: String,
}

#[derive(Debug,Deserialize)]
pub struct LoginBySmsReq {
    pub phone: String,
    pub sms_code: String,
}

#[derive(Debug,Serialize)]
pub struct LoginResult {
    pub user_id: u64,
    pub new_register: bool,
    pub access_token: Option<String>,
    pub expire_seconds: u64,
    pub refresh_token: Option<String>,
}
