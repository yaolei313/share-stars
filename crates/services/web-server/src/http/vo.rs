pub mod error;
pub mod login;
pub mod register;

use crate::http::vo::error::AppError;
use axum::response::IntoResponse;
use serde::Serialize;
use std::fmt::Display;
use validator::ValidationErrors;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
pub struct RespVo<T>
where
    T: Serialize,
{
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> From<AppError> for RespVo<T>
where
    T: Serialize,
{
    fn from(value: AppError) -> Self {
        let message = match value {
            AppError::Success => "success".to_owned(),
            _ => format!("{}", value),
        };
        RespVo {
            code: value.code(),
            message,
            data: None,
        }
    }
}

impl<T> RespVo<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        let rsp: RespVo<T> = AppError::Success.into();
        Self {
            data: Some(data),
            ..rsp
        }
    }

    pub fn fail(msg: String) -> Self {
        AppError::Fail(msg).into()
    }

    pub fn invalid_argument(error: ValidationErrors) -> Self {
        AppError::InvalidArgument(error.to_string()).into()
    }
}
