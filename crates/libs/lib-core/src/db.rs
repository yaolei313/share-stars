use crate::db::repositories::{
    PgAccountDeviceRepository, PgAccountIdentityRepository, PgAccountRepository,
    PgDeviceRepository, PgEmailTemplateRepository, PgLookupAccountRepository,
    PgSmsTemplateRepository,
};
use crate::db::services::{AccountDbService, DeviceDbService, TemplateDbService};
use sqlx::PgPool;
use std::sync::Arc;

pub mod models;
mod repositories;
pub mod services;

pub type SqlxResult<T> = sqlx::Result<T>;

#[derive(Clone)]
pub struct RepositoryState {
    pub pool: PgPool,
    pub account_repo: Arc<PgAccountRepository>,
    pub account_identity_repo: Arc<PgAccountIdentityRepository>,
    pub lookup_account_repo: Arc<PgLookupAccountRepository>,
    pub device_repo: Arc<PgDeviceRepository>,
    pub account_device_repo: Arc<PgAccountDeviceRepository>,
    pub sms_template_repo: Arc<PgSmsTemplateRepository>,
    pub email_template_repo: Arc<PgEmailTemplateRepository>,
}

impl RepositoryState {
    pub fn new(db_pool: PgPool) -> Self {
        RepositoryState {
            pool: db_pool.clone(),
            account_repo: Arc::new(PgAccountRepository::new(db_pool.clone())),
            account_identity_repo: Arc::new(PgAccountIdentityRepository::new(db_pool.clone())),
            lookup_account_repo: Arc::new(PgLookupAccountRepository::new(db_pool.clone())),
            device_repo: Arc::new(PgDeviceRepository::new(db_pool.clone())),
            account_device_repo: Arc::new(PgAccountDeviceRepository::new(db_pool.clone())),
            sms_template_repo: Arc::new(PgSmsTemplateRepository::new(db_pool.clone())),
            email_template_repo: Arc::new(PgEmailTemplateRepository::new(db_pool.clone())),
        }
    }
}

#[derive(Clone)]
pub struct DbServiceState {
    pub account_db_service: Arc<AccountDbService>,
    pub device_db_service: Arc<DeviceDbService>,
    pub template_db_service: Arc<TemplateDbService>,
}

impl DbServiceState {
    pub fn new(db_pool: PgPool) -> Self {
        let repository_state = Arc::new(RepositoryState::new(db_pool.clone()));

        let account_db_service = Arc::new(AccountDbService::new(repository_state.clone()));
        let device_db_service = Arc::new(DeviceDbService::new(repository_state.clone()));
        let template_db_service = Arc::new(TemplateDbService::new(repository_state.clone()));
        Self {
            account_db_service,
            device_db_service,
            template_db_service,
        }
    }
}
