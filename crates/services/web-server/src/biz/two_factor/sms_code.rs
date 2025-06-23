use crate::http::vo::sms::SmsType;
use crate::http::vo::AppResult;

pub fn get_sms_code(e164phone: &str, sms_type: &SmsType) -> AppResult<String> {
    todo!()
}

pub fn validate_sms_code(e164phone: &str, sms_type: &SmsType, input_code: &str) -> AppResult<()> {
    todo!()
}
