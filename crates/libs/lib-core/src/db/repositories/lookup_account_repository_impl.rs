use crate::db::models::LookupAccount;
use crate::db::repositories::lookup_account_repository::LookupAccountRepository;
use sqlx::{PgExecutor, PgPool, Result as SqlxResult};
use std::fmt::Debug;
use tracing::log;

#[derive(Debug)]
pub struct PgLookupAccountRepository {
    pool: PgPool,
}

impl PgLookupAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl LookupAccountRepository for PgLookupAccountRepository {
    async fn find_by_provider_identifier(
        &self,
        provider: i32,
        identifier: &str,
    ) -> SqlxResult<Option<LookupAccount>> {
        sqlx::query_as::<_, LookupAccount>(
            "SELECT * FROM lookup_account WHERE provider = $1 and identifier = $2",
        )
        .bind(provider)
        .bind(identifier)
        .fetch_optional(&self.pool)
        .await
    }

    async fn insert<'c, E>(&self, executor: E, lookup: LookupAccount) -> SqlxResult<()>
    where
        E: PgExecutor<'c>,
    {
        let id = sqlx::query_as::<_, (i64,)>("insert into lookup_account (provider, identifier, user_id) values ($1, $2, $3) returning id")
            .bind(lookup.provider)
            .bind(lookup.identifier)
            .bind(lookup.user_id)
            .fetch_one(executor)
            .await?;
        log::info!("inserted lookup_account (id: {:?})", id);
        Ok(())
    }
}
