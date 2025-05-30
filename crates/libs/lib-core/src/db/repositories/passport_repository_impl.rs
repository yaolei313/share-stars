use crate::db::models::Passport;
use crate::db::repositories::PassportRepository;
use anyhow::Result;
use sqlx::PgPool;

pub struct PgPassportRepository {
    pool: PgPool,
}

impl PgPassportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PassportRepository for PgPassportRepository {
    async fn by_user_id(&self, user_id: i64) -> Result<Option<Passport>> {
        let passport = sqlx::query_as!(
            Passport,
            "SELECT * FROM passport WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(passport)
    }
}
