pub mod device;
pub mod error;
pub mod login;
pub mod mfa;
pub mod register;
pub mod sms;
pub mod validators;

use crate::http::vo::error::AppError;
use lib_macro_derive::BindCode;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::net::IpAddr;

pub type AppResult<T> = Result<T, AppError>;

const SUCCESS: i32 = 0;
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
        let message = format!("{}", value);
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
    let rsp: RespVo<T> = success_resp_none_data();
    RespVo {
        data: Some(data),
        ..rsp
    }
}

pub fn success_resp_none_data<T>() -> RespVo<T>
where
    T: Serialize,
{
    RespVo {
        code: SUCCESS,
        message: "OK".to_string(),
        data: None,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, BindCode)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestInfo {
    pub platform: PlatformEnum,
    pub device_id: String,
    pub ip: Option<IpAddr>,
    pub request_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewDeviceRequestInfo {
    pub platform: PlatformEnum,
    pub device_id: Option<String>,
    pub ip: Option<IpAddr>,
    pub request_id: Option<String>,
}

pub struct PlatformInfo {}
