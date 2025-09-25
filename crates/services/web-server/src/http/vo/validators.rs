use crate::http::vo::mfa::MfaMethod;
use validator::ValidationError;

pub fn validate_mfa_challenge_chosen_method(method: &MfaMethod) -> Result<(), ValidationError> {
    match method {
        MfaMethod::Sms => Ok(()),
        MfaMethod::Email => Ok(()),
        MfaMethod::Totp => Err(ValidationError::new("invalid challenge method")),
    }
}
