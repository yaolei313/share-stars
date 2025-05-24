pub mod login;
pub mod register;
pub mod result;

use crate::http::vo::result::BizResultCode;
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Serialize)]
pub struct RespVo<T>
where
    T: Serialize,
{
    pub code: i32,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVo<T>
where
    T: Serialize,
{
    pub fn new_with_data(result: BizResultCode, data: Option<T>) -> Self {
        Self {
            code: result.code(),
            message: Some(result.message().to_string()),
            data,
        }
    }

    pub fn new_with_message(result: BizResultCode, message: Option<String>) -> Self {
        let message = message.or(Some(result.message().to_string()));
        Self {
            code: result.code(),
            message,
            data: None,
        }
    }

    pub fn success(data: T) -> Self {
        Self::new_with_data(BizResultCode::SUCCESS, Some(data))
    }

    pub fn invalid_request(error: ValidationErrors) -> Self {
        Self::new_with_message(BizResultCode::INVALID_ARGUMENT, Some(error.to_string()))
    }

    pub fn bad_request_with_message(message: String) -> Self {
        Self::new_with_message(BizResultCode::BAD_REQUEST, Some(message))
    }

    pub fn bad_request() -> Self {
        Self::new_with_message(BizResultCode::BAD_REQUEST, None)
    }
}
