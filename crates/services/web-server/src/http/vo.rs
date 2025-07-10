pub mod error;
pub mod login;
pub mod register;
pub mod sms;

use crate::http::vo::error::AppError;
use lib_macro_derive::BindCode;
use serde::Serialize;
use std::fmt::Display;

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

pub fn success_resp<T>(data: T) -> RespVo<T>
where
    T: Serialize,
{
    let rsp: RespVo<T> = AppError::Success.into();
    RespVo {
        data: Some(data),
        ..rsp
    }
}

pub fn success_resp_none_data<T>() -> RespVo<T>
where
    T: Serialize,
{
    AppError::Success.into()
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
