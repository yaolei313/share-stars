use chrono::prelude::*;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Account {
    pub id: i64,
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

#[derive(Debug, Clone, FromRow)]
pub struct NewAccount {
    pub user_id: i64,
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
pub struct LookupAccount {
    pub id: i64,
    pub identifier: String,
    pub provider: i32,
    pub user_id: i64,
}
