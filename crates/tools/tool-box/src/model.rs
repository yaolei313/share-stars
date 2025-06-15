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
    pub register_at: chrono::DateTime<Utc>,
    pub closed: bool,
    pub closed_at: Option<chrono::DateTime<Utc>>,
    pub disabled: bool,
    pub disabled_at: Option<chrono::DateTime<Utc>>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
