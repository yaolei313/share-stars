use super::validators::validate_mfa_challenge_chosen_method;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MfaMethod {
    Totp, // Time-based One-time Password，Google Authenticator
    SmsCode,
    EmailCode,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct MfaInfo {
    pub method: MfaMethod,
    pub detail: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MfaVerificationChallenge {
    pub user_id: i64,
    pub mfa_session_id: String,
    pub mfa_infos: Vec<MfaInfo>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MfaChallengeReq {
    #[validate(length(min = 1, message = "mfa_session_id is required"))]
    pub mfa_session_id: String,
    #[validate(custom(function = "validate_mfa_challenge_chosen_method"))]
    pub chosen_method: MfaMethod,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MfaVerifyReq {
    #[validate(length(min = 1, message = "mfa_session_id is required"))]
    pub mfa_session_id: String,
    #[validate(length(min = 6, max = 6, message = "verify_code is required"))]
    pub verify_code: String,
}

impl Display for MfaVerifyReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.mfa_session_id, self.verify_code)
    }
}
