use crate::http::vo::mfa::MfaMethod;
use validator::ValidationError;

pub fn validate_mfa_challenge_chosen_method(method: &MfaMethod) -> Result<(), ValidationError> {
    match method {
        MfaMethod::SmsCode => Ok(()),
        MfaMethod::EmailCode => Ok(()),
        MfaMethod::Totp => Err(ValidationError::new("invalid challenge method")),
    }
}
