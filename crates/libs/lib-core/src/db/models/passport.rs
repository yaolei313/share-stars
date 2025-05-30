use chrono::prelude::*;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Passport {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub salt: String,
    pub password_sha256: String,
    pub register_type: i16,
    pub register_at: NaiveDateTime,
    pub closed: bool,
    pub closed_at: Option<NaiveDateTime>,
    pub disabled: bool,
    pub disabled_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
