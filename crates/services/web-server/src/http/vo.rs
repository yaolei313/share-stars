pub mod error;
pub mod login;
pub mod register;

use crate::http::vo::error::AppError;
use axum::response::IntoResponse;
use lib_macro_derive::BindCode;
use serde::Serialize;
use std::fmt::Display;
use validator::ValidationErrors;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize, BindCode)]
pub enum PlatformEnum {
    #[code(1)]
    Web,
    #[code(2)]
    AppIos,
    #[code(3)]
    AppAndroid,
    #[code(4)]
    Pc,
}

impl Display for PlatformEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformEnum::Web => write!(f, "web"),
            PlatformEnum::AppIos => write!(f, "AppIos"),
            PlatformEnum::AppAndroid => write!(f, "AppAndroid"),
            PlatformEnum::Pc => write!(f, "Pc"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DeviceInfo {
    pub platform: PlatformEnum,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub device_fp: Option<String>,
    pub request_id: Option<String>,
}
