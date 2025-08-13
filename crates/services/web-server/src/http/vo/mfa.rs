use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum MfaMethodType {
    Sms,
    Totp, // Time-based One-time Password，Google Authenticator
    Email,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MfaMethod {
    #[serde(rename = "type")]
    pub method_type: MfaMethodType,
    pub contact: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MfaVerificationChallenge {
    pub mfa_token: String,
    pub mfa_methods: Vec<MfaMethod>,
    pub recommended_method: MfaMethodType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MfaChallengeReq {
    pub method_type: MfaMethodType,
    pub code: String,
    pub remember_device: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MfaChallengeResult {}
