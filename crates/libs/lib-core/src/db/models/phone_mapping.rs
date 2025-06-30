use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PhoneMapping {
    pub id: i64,
    pub phone: String,
    pub user_id: i64,
}
