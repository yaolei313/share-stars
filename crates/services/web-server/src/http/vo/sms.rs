use lib_macro_derive::BindCode;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize, BindCode)]
pub enum SmsType {
    #[code(1)]
    Login,
    #[code(2)]
    BindPhone,
    #[code(3)]
    ResetPwd,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SmsSendReq {
    #[validate(custom(function = "validate_sms_type"))]
    pub sms_type: i32,

    #[validate(length(
        min = 11,
        max = 15,
        message = "phone must at least be 11 characters and at most 15"
    ))]
    pub phone: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SmsSendResult {}

pub fn validate_sms_type(sms_type: i32) -> Result<(), validator::ValidationError> {
    let Some(_) = SmsType::from_code(sms_type) else {
        return Err(ValidationError::new("invalid sms_type"));
    };
    Ok(())
}
