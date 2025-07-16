use crate::biz::security::{PasswordStatistic, SmsStatistic};
use crate::biz::session::{JwtManager, TokenService};
use crate::biz::verify::{CodeManager, SmsService};
use crate::config::{AppSettings, Env};
use anyhow::Result;
use lib_core::RepositoryState;
use lib_core::db::services::AccountService;
use lib_utils::IdGenerator;
use std::sync::Arc;

pub mod authn;
mod device;
pub mod dto;
pub mod security;
pub mod session;
pub mod verify;

#[derive(Clone)]
pub struct ServiceState {
    pub code_manager: Arc<CodeManager>,
    pub id_generator: Arc<IdGenerator>,
    pub sms_service: Arc<SmsService>,
    pub token_service: Arc<TokenService>,
    pub account_service: Arc<AccountService>,
    pub password_statistic: Arc<PasswordStatistic>,
    pub sms_statistic: Arc<SmsStatistic>,
}

impl ServiceState {
    pub fn new(
        env: Env,
        repository_state: Arc<RepositoryState>,
        redis_client: Arc<redis::Client>,
        config: Arc<AppSettings>,
    ) -> Result<Self> {
        let code_manager = Arc::new(CodeManager::new(redis_client.clone()));
        let id_generator = Arc::new(IdGenerator::new(config.server.worker_id)?);
        let sms_service = Arc::new(SmsService::new(
            env.clone(),
            code_manager.clone(),
            &config.sms,
        ));
        let account_service = Arc::new(AccountService::new(repository_state));
        let token_service = Arc::new(TokenService::new(JwtManager::new(&config.jwt)?));
        let password_statistic = Arc::new(PasswordStatistic::new(redis_client.clone()));
        let sms_statistic = Arc::new(SmsStatistic::new(redis_client.clone()));
        Ok(Self {
            code_manager,
            id_generator,
            sms_service,
            account_service,
            token_service,
            password_statistic,
            sms_statistic,
        })
    }
}
