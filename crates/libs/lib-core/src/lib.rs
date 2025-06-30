use crate::db::repositories::{PgPassportRepository, PgPhoneMappingRepository};
use sqlx::PgPool;
use std::sync::Arc;

pub mod db;

#[derive(Clone)]
pub struct RepositoryState {
    pub passport_repo: Arc<PgPassportRepository>,
    pub phone_mapping_repo: Arc<PgPhoneMappingRepository>,
}

impl RepositoryState {
    pub fn new(db_pool: PgPool) -> Self {
        RepositoryState {
            passport_repo: Arc::new(PgPassportRepository::new(db_pool.clone())),
            phone_mapping_repo: Arc::new(PgPhoneMappingRepository::new(db_pool.clone())),
        }
    }
}
