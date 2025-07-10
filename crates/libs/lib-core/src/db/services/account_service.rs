use crate::db::models::{Account, AccountIdentity, LookupAccount, Principal};
use crate::db::repositories::{
    AccountIdentityRepository, AccountRepository, LookupAccountRepository,
    PgAccountIdentityRepository, PgAccountRepository, PgLookupAccountRepository,
};
use crate::RepositoryState;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AccountService {
    pg_pool: PgPool,
    account_repo: Arc<PgAccountRepository>,
    account_identity_repo: Arc<PgAccountIdentityRepository>,
    lookup_account_repo: Arc<PgLookupAccountRepository>,
}

impl AccountService {
    pub fn new(repository_state: Arc<RepositoryState>) -> Self {
        Self {
            pg_pool: repository_state.pool.clone(),
            account_repo: repository_state.account_repo.clone(),
            account_identity_repo: repository_state.account_identity_repo.clone(),
            lookup_account_repo: repository_state.lookup_account_repo.clone(),
        }
    }

    pub async fn query_by_principal(
        &self,
        principal: &Principal<'_>,
    ) -> Result<Option<Account>, sqlx::Error> {
        let user_id = self.lookup_user_id(principal).await?;

        let passport = if let Some(user_id) = user_id {
            self.account_repo.find_by_user_id(user_id).await?
        } else {
            None
        };

        Ok(passport)
    }

    pub async fn create_account(
        &self,
        principal: &Principal<'_>,
        user_id: i64,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        let provider = principal.provider();
        let identifier = principal.identity();
        let account = Account {
            id: 0,
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
        self.account_repo.insert(&mut *tx, account).await?;
        self.account_identity_repo
            .insert(&mut *tx, identity)
            .await?;
        tx.commit().await?;

        let lookup = LookupAccount {
            id: 0,
            identifier: identifier.to_string(),
            provider,
            user_id,
        };
        // sharding by identifier
        self.lookup_account_repo
            .insert(&self.pg_pool, lookup)
            .await?;

        Ok(())
    }

    async fn lookup_user_id(&self, principal: &Principal<'_>) -> Result<Option<i64>, sqlx::Error> {
        let mapping = self
            .lookup_account_repo
            .find_by_provider_identifier(principal.provider(), principal.identity())
            .await?;
        let user_id = mapping.map(|m| m.user_id);
        Ok(user_id)
    }
}
