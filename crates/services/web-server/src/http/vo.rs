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
use std::borrow::Cow;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::net::IpAddr;
use std::str::FromStr;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, BindCode)]
pub enum PlatformEnum {
    #[code(1)]
    Web,
    #[code(2)]
    Ios,
    #[code(3)]
    Android,
    #[code(4)]
    Pc,
}

impl FromStr for PlatformEnum {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "web" => Ok(PlatformEnum::Web),
            "ios" => Ok(PlatformEnum::Ios),
            "android" => Ok(PlatformEnum::Android),
            "pc" => Ok(PlatformEnum::Pc),
            _ => Err(AppError::InvalidArgument(Cow::Borrowed(
                "invalid platform string",
            ))),
        }
    }
}

//
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessContext {
    pub request_id: Option<String>,
    pub client_ip: Option<IpAddr>,
    pub device_id: String,
    pub locale: String,
    pub env: SystemEnvironment,
    pub source: AccessSource,
}

impl AccessContext {
    pub fn is_same_device(&self, other: &AccessContext) -> bool {
        // 核心判断：Device ID 必须一致
        if self.device_id != other.device_id {
            return false;
        }

        // 硬件特征冲突校验
        if !self.env.is_consistent_with(&other.env) {
            return false;
        }

        // 平台一致性校验 (Platform Consistency Check)
        if !self.is_platform_compatible(other) {
            return false;
        }

        true
    }

    fn is_platform_compatible(&self, other: &Self) -> bool {
        use AccessSource::*;

        match (&self.source, &other.source) {
            // 互斥场景：同一个 DeviceID 不可能既是 iPhone 又是 Android 手机
            (Ios(_), Android(_)) | (Android(_), Ios(_)) => false,
            (Pc(_), Ios(_)) | (Ios(_), Pc(_)) => false,
            (Pc(_), Android(_)) | (Android(_), Pc(_)) => false,

            _ => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemEnvironment {
    /// 操作系统家族: "iOS", "Android", "Windows", "MacOS" (原 os_name)
    pub os_family: Option<String>,

    /// 系统版本号: "15.2", "12" (原 os_version)
    pub os_version: Option<String>,

    /// 设备品牌: "Apple", "Xiaomi", "Huawei" (原 device_name)
    pub device_brand: Option<String>,

    /// 设备具体型号: "iPhone13,2", "Mi 11" (原 device_model)
    pub device_model: Option<String>,
}

impl SystemEnvironment {
    pub fn is_consistent_with(&self, other: &Self) -> bool {
        let conflict = |a: &Option<String>, b: &Option<String>| matches!((a, b), (Some(v1), Some(v2)) if v1 != v2);

        if conflict(&self.os_family, &other.os_family) {
            return false;
        }
        if conflict(&self.device_model, &other.device_model) {
            return false;
        }

        true
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, BindCode)]
pub enum AccessSource {
    #[code(1)]
    Web(WebMetadata),
    #[code(2)]
    Ios(AppMetadata),
    #[code(3)]
    Android(AppMetadata),
    #[code(4)]
    Pc(DesktopMetadata),
}

impl Display for AccessSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessSource::Web(_) => write!(f, "web"),
            AccessSource::Ios(_) => write!(f, "ios"),
            AccessSource::Android(_) => write!(f, "android"),
            AccessSource::Pc(_) => write!(f, "pc"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebMetadata {
    pub user_agent: String,             // 原始 UA，用于兜底
    pub browser_family: Option<String>, // "Chrome", "Safari"
    pub browser_version: Option<String>,
    pub referrer: Option<String>, // Web 端特有的流量来源
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppMetadata {
    pub bundle_id: String, // App 场景下通常是必填
    pub app_version: String,
    pub build_number: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesktopMetadata {
    pub app_name: String,
    pub app_version: String,
}

fn is_option_match(a: &Option<&String>, b: &Option<&String>) -> bool {
    match (a, b) {
        (Some(v1), Some(v2)) => v1 == v2,
        _ => true, // 其中一个为空，认为暂时没有冲突证据
    }
}
