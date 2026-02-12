use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterDeviceReq {
    #[validate(length(min = 1, max = 20, message = "device model must be not empty"))]
    pub device_model: String,

    #[validate(length(min = 1, max = 20, message = "platform name must be not empty"))]
    pub platform_name: String,

    #[validate(length(min = 1, max = 20, message = "os name must be not empty"))]
    pub os_name: String,

    pub screen_width: i32,

    pub screen_height: i32,

    pub device_memory_gb: i32,

    pub cpu_cores: i32,

    #[validate(length(min = 1, max = 50, message = "renderer vendor must be not empty"))]
    pub renderer_vendor: String,

    #[validate(length(min = 1, max = 50, message = "renderer model must be not empty"))]
    pub renderer_model: String,

    #[validate(length(min = 1, max = 20))]
    pub browser_major_version: Option<String>,

    #[validate(length(min = 1, max = 20))]
    pub app_major_version: Option<String>,

    #[validate(length(min = 1, max = 10))]
    pub timezone_id: Option<String>,

    #[validate(length(min = 1, max = 20))]
    pub language_code: Option<String>,

    #[validate(ip)]
    pub ip_address_segment: Option<IpAddr>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterDeviceResult {
    pub device_id: String,
    pub device_token: String,
}
