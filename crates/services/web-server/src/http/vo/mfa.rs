use super::validators::validate_mfa_challenge_chosen_method;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MfaMethod {
    Totp, // Time-based One-time Password，Google Authenticator
    Sms,
    Email,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MfaVerifyReq {
    pub mfa_session_id: String,

    pub code: String,
    pub remember_device: bool,
}
