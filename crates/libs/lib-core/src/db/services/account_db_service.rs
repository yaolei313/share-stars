use crate::db::models::{Account, AccountDevice, AccountIdentity, LookupAccount};
use crate::db::repositories::{
    AccountDeviceRepository, AccountIdentityRepository, AccountRepository, LookupAccountRepository,
    PgAccountDeviceRepository, PgAccountIdentityRepository, PgAccountRepository,
    PgLookupAccountRepository,
};
use crate::db::RepositoryState;
use crate::db::SqlxResult;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::log;

pub struct AccountDbService {
    pg_pool: PgPool,
    account_repo: Arc<PgAccountRepository>,
    account_identity_repo: Arc<PgAccountIdentityRepository>,
    account_device_repo: Arc<PgAccountDeviceRepository>,
    lookup_account_repo: Arc<PgLookupAccountRepository>,
}

impl AccountDbService {
    pub fn new(repository_state: Arc<RepositoryState>) -> Self {
        Self {
            pg_pool: repository_state.pool.clone(),
            account_repo: repository_state.account_repo.clone(),
            account_identity_repo: repository_state.account_identity_repo.clone(),
            account_device_repo: repository_state.account_device_repo.clone(),
            lookup_account_repo: repository_state.lookup_account_repo.clone(),
        }
    }

    pub async fn query_by_identity(
        &self,
        provider: i32,
        identifier: &str,
    ) -> SqlxResult<Option<Account>> {
        let user_id = self.lookup_user_id(provider, identifier).await?;

        let passport = if let Some(user_id) = user_id {
            self.account_repo.find_by_user_id(user_id).await?
        } else {
            None
        };

        Ok(passport)
    }

    pub async fn create_account(
        &self,
        provider: i32,
        identifier: &str,
        user_id: i64,
    ) -> SqlxResult<()> {
        let now = Utc::now();
        let account = Account {
            user_id,
            salt: "".to_string(),
            password_hash: "".to_string(),
            closed: false,
            closed_at: None,
            disabled: false,
            disabled_at: None,
            created_at: now,
            updated_at: now,
        };
        let identity = AccountIdentity {
            id: 0,
            user_id,
            provider,
            identifier: identifier.to_string(),
            is_verified: false,
            verified_at: None,
            created_at: now,
            updated_at: now,
        };

        // sharding by user_id
        let mut tx = self.pg_pool.begin().await?;
        self.account_repo.insert(&mut *tx, &account).await?;
        self.account_identity_repo
            .insert(&mut *tx, &identity)
            .await?;
        tx.commit().await?;
        log::info!("Account inserted successful. {}", identity.id);

        // sharding by identifier
        let lookup = LookupAccount {
            id: 0,
            identifier: identifier.to_string(),
            provider,
            user_id,
        };
        self.lookup_account_repo
            .insert(&self.pg_pool, &lookup)
            .await?;
        log::info!("Account-Lookup inserted successful. {}", identity.id);

        Ok(())
    }

    async fn lookup_user_id(&self, provider: i32, identifier: &str) -> SqlxResult<Option<i64>> {
        let mapping = self
            .lookup_account_repo
            .find_by_provider_identifier(provider, identifier)
            .await?;
        let user_id = mapping.map(|m| m.user_id);
        Ok(user_id)
    }

    pub async fn query_account_device(
        &self,
        user_id: i64,
        device_id: &str,
    ) -> SqlxResult<Option<AccountDevice>> {
        self.account_device_repo
            .find_by_user_id_device_id(user_id, device_id)
            .await
    }

    pub async fn add_account_device(&self, device: &AccountDevice) -> SqlxResult<()> {
        self.account_device_repo.insert(&self.pg_pool, device).await
    }

    pub async fn query_identities(
        &self,
        user_id: i64,
        providers: Vec<i32>,
    ) -> SqlxResult<Vec<AccountIdentity>> {
        self.account_identity_repo
            .find_by_user_id_providers(user_id, providers)
            .await
    }
}
