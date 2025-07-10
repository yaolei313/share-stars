use crate::db::repositories::{
    PgAccountIdentityRepository, PgAccountRepository, PgLookupAccountRepository,
};
use sqlx::PgPool;
use std::sync::Arc;

pub mod db;

#[derive(Clone)]
pub struct RepositoryState {
    pub pool: PgPool,
    pub account_repo: Arc<PgAccountRepository>,
    pub account_identity_repo: Arc<PgAccountIdentityRepository>,
    pub lookup_account_repo: Arc<PgLookupAccountRepository>,
}

impl RepositoryState {
    pub fn new(db_pool: PgPool) -> Self {
        RepositoryState {
            pool: db_pool.clone(),
            account_repo: Arc::new(PgAccountRepository::new(db_pool.clone())),
            account_identity_repo: Arc::new(PgAccountIdentityRepository::new(db_pool.clone())),
            lookup_account_repo: Arc::new(PgLookupAccountRepository::new(db_pool.clone())),
        }
    }
}
