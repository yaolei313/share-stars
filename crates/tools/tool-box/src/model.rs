
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserCredential {
    pub id: i64,
    pub user_id: i64,
    pub salt: String,
    pub password_hash: String,
    pub closed: bool,
    pub closed_at: Option<chrono::DateTime<Utc>>,
    pub disabled: bool,
    pub disabled_at: Option<chrono::DateTime<Utc>>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserIdentity {
    pub id: i64,
    pub user_id: i64,
    pub provider: i32,
    pub identifier: String,
    pub is_verified: bool,
    pub verified_at: Option<chrono::DateTime<Utc>>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UserIdentityMap {
    pub id: i64,
    pub identifier: String,
    pub provider: i32,
    pub user_id: i64,
}
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct TrustedDevice {
    pub id: i64,
    pub user_id: i64,
    pub device_fp_hash: String,
    pub device_name: Option<String>,
    pub device_platform: Option<i32>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub browser_family: Option<String>,
    pub browser_version: Option<String>,
    pub app_family: Option<String>,
    pub app_version: Option<String>,
    pub last_login_ip: Option<std::net::IpAddr>,
    pub last_login_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub is_active: bool,
}