use crate::db::models::Account;
use crate::db::repositories::AccountRepository;
use sqlx::{PgExecutor, PgPool, Result as SqlxResult};
use std::fmt::Debug;

#[derive(Debug)]
pub struct PgAccountRepository {
    pool: PgPool,
}

impl PgAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AccountRepository for PgAccountRepository {
    async fn find_by_user_id(&self, user_id: i64) -> SqlxResult<Option<Account>> {
        sqlx::query_as::<_, Account>("SELECT * FROM account WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn insert<'c, E>(&self, executor: E, account: &Account) -> SqlxResult<()>
    where
        E: PgExecutor<'c>,
    {
        sqlx::query(r#"insert into account (user_id, created_at, updated_at) values ($1, $2, $3)"#)
            .bind(account.user_id)
            .bind(account.created_at)
            .bind(account.updated_at)
            .fetch_one(executor)
            .await?;
        tracing::info!("inserted account. {:?}", account);
        Ok(())
    }
}
