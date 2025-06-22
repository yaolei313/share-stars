use crate::http::vo::AppResult;
use crate::http::vo::error::AppError;
use phonenumber::Mode;

pub(crate) fn validate_then_format_phone_number(phone_number: &str) -> AppResult<String> {
    let Ok(number) = phonenumber::parse(None, phone_number) else {
        return Err(AppError::InvalidPhoneNumber(phone_number.to_string()));
    };
    let std_phone = number.format().mode(Mode::E164).to_string();
    Ok(std_phone)
}
