pub mod error;
pub mod login;
pub mod register;

use serde::Serialize;

pub enum RespCode {
    Success = 200,
    BadRequest = 400,
}

impl Into<i32> for RespCode {
    fn into(self) -> i32 {
        self as i32
    }
}

#[derive(Serialize)]
pub struct RespVo<T> {
    pub code: i32,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVo<T> {
    pub fn success(data: T) -> Self {
        RespVo {
            code: RespCode::Success.into(),
            message: Some(String::from("success")),
            data: Some(data),
        }
    }

    pub fn bad_request(message: String) -> Self {
        RespVo {
            code: RespCode::BadRequest.into(),
            message: Some(message),
            data: None,
        }
    }

    pub fn bad_request_info(message: &'static str) -> Self {
        Self::bad_request(String::from(message))
    }
}
