use chrono::DateTime;
use chrono::Utc;
use serde_json::Value;
use sqlx::FromRow;
use std::net::IpAddr;

#[derive(Debug, FromRow, Clone)]
pub struct Device {
    pub device_id: String,
    pub full_fingerprint: Value,
    pub platform_type: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct LookupDevice {
    pub stable_fingerprint_hash: String,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct PrefilterDevice {
    pub id: i64,
    pub prefilter_hash: Option<String>,
    pub device_model: String,
    pub platform_name: String,
    pub os_name: String,
    pub screen_width: i32,
    pub screen_height: i32,
    pub device_memory_gb: i32,
    pub cpu_cores: i32,
    pub renderer_vendor: String,
    pub renderer_model: String,
    pub browser_major_version: Option<String>,
    pub app_major_version: Option<String>,
    pub timezone_id: Option<String>,
    pub language_code: Option<String>,
    pub ip_address_segment: Option<IpAddr>,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
