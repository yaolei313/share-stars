use chrono::DateTime;
use chrono::Utc;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct SmsTemplate {
    pub id: i64,
    pub template_key: String,
    pub language_code: String,
    pub content: String,
    pub template_type: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct EmailTemplate {
    pub id: i64,
    pub template_key: String,
    pub language_code: String,
    pub subject: String,
    pub content: String,
    pub template_type: String,
    pub sender_name: Option<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
