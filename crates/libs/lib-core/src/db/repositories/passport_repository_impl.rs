use crate::db::models::{NewPassport, Passport};
use crate::db::repositories::PassportRepository;
use sqlx::{Error, PgPool};
use std::fmt::Debug;
use tracing::log;

#[derive(Debug)]
pub struct PgPassportRepository {
    pool: PgPool,
}

impl PgPassportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PassportRepository for PgPassportRepository {
    async fn by_user_id(&self, user_id: i64) -> Result<Option<Passport>, sqlx::Error> {
        let passport = sqlx::query_as!(
            Passport,
            "SELECT * FROM passport WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(passport)
    }

    async fn insert(&self, passport: NewPassport) -> Result<(), Error> {
        let id = sqlx::query_as::<_, (i64,)>(r#"insert into passport (user_id, phone, email, created_at, updated_at) values ($1, $2, $3, $4, $5) returning id"#, )
            .bind(passport.user_id)
            .bind(passport.phone)
            .bind(passport.email)
            .bind(passport.created_at)
            .bind(passport.updated_at)
            .fetch_one(&self.pool)
            .await?;
        log::info!("inserted passport (id: {:?})", id);
        Ok(())
    }
}
