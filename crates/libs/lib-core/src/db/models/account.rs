use chrono::prelude::*;
use sqlx::FromRow;
use std::net::IpAddr;

#[derive(Debug, FromRow)]
pub struct Account {
    pub user_id: i64,
    pub salt: String,
    pub password_hash: String,
    pub closed: bool,
    pub closed_at: Option<DateTime<Utc>>,
    pub disabled: bool,
    pub disabled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AccountIdentity {
    pub id: i64,
    pub user_id: i64,
    pub provider: i32,
    pub identifier: String,
    pub is_verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AccountDevice {
    pub id: i64,
    pub user_id: i64,
    pub device_id: String,
    pub last_login_ip: Option<IpAddr>,
    pub last_login_method: i32,
    pub last_login_at: DateTime<Utc>,
    pub nickname: Option<String>,
    pub trusted_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct LookupAccount {
    pub id: i64,
    pub identifier: String,
    pub provider: i32,
    pub user_id: i64,
}
