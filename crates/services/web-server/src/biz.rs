use crate::biz::session::JwtManager;
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
    pub jwt_manager: Arc<JwtManager>,
    pub code_manager: Arc<CodeManager>,
    pub id_generator: Arc<IdGenerator>,
    pub sms_service: Arc<SmsService>,
    pub account_service: Arc<AccountService>,
}

impl ServiceState {
    pub fn new(
        env: Env,
        repository_state: Arc<RepositoryState>,
        redis_client: Arc<redis::Client>,
        config: Arc<AppSettings>,
    ) -> Result<Self> {
        let jwt_manager = Arc::new(JwtManager::new(&config.jwt)?);
        let code_manager = Arc::new(CodeManager::new(redis_client.clone()));
        let id_generator = Arc::new(IdGenerator::new(config.server.worker_id)?);
        let sms_service = Arc::new(SmsService::new(
            env.clone(),
            code_manager.clone(),
            &config.sms,
        ));
        let passport_service = Arc::new(AccountService::new(repository_state));
        Ok(Self {
            jwt_manager,
            code_manager,
            id_generator,
            sms_service,
            account_service: passport_service,
        })
    }
}
