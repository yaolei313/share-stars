use chrono::Utc;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct TrustedDevice {
    pub id: i64,
    pub user_id: i64,
    pub device_fp_hash: String,
    pub device_name: Option<String>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub browser_family: Option<String>,
    pub browser_version: Option<String>,
    pub device_type: Option<String>,
    pub last_login_ip: Option<std::net::IpAddr>,
    pub last_login_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, FromRow)]
pub struct Device {}
