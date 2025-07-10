use crate::db::models::AccountIdentity;
use crate::db::repositories::account_identity_repository::AccountIdentityRepository;
use sqlx::{PgExecutor, PgPool, Result as SqlxResult};
use tracing::log;

#[derive(Debug)]
pub struct PgAccountIdentityRepository {
    pool: PgPool,
}

impl PgAccountIdentityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AccountIdentityRepository for PgAccountIdentityRepository {
    async fn list_by_user_id(&self, user_id: &str) -> SqlxResult<Vec<AccountIdentity>> {
        sqlx::query_as::<_, AccountIdentity>("select * from account_identity where user_id = $1")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_user_id_provider(
        &self,
        user_id: &str,
        provider: i32,
    ) -> SqlxResult<Option<AccountIdentity>> {
        sqlx::query_as::<_, AccountIdentity>(
            "select * from account_identity where user_id = $1 and provider = $2",
        )
        .bind(user_id)
        .bind(provider)
        .fetch_optional(&self.pool)
        .await
    }

    async fn insert<'c, E>(&self, executor: E, identity: AccountIdentity) -> SqlxResult<()>
    where
        E: PgExecutor<'c>,
    {
        let id = sqlx::query_as::<_, (i64,)>("insert into account_identity (user_id, provider, identifier, is_verified, verified_at, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7) returning id")
        .bind(identity.user_id)
        .bind(identity.provider)
        .bind(identity.identifier)
        .bind(identity.is_verified)
        .bind(identity.verified_at)
        .bind(identity.created_at)
        .bind(identity.updated_at)
            .fetch_one(executor)
            .await?;
        log::info!("inserted account_identity (id: {:?})", id);
        Ok(())
    }
}
