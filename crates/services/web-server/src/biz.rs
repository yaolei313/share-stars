use crate::biz::session::JwtManager;
use crate::biz::sms::{CodeGenerator, SmsService};
use crate::config::AppSettings;
use anyhow::Result;
use lib_core::RepositoryState;
use std::sync::Arc;

pub mod authn;
pub mod dto;
pub mod security;
mod session;
mod sms;

#[derive(Clone)]
pub struct ServiceState {
    pub jwt_manager: Arc<JwtManager>,
    pub sms_service: Arc<SmsService>,
}

impl ServiceState {
    pub fn new(
        repository_state: Arc<RepositoryState>,
        redis_client: Arc<redis::Client>,
        config: Arc<AppSettings>,
    ) -> Result<Self> {
        let jwt_manager = Arc::new(JwtManager::new(&config.jwt)?);
        let code_generator = Arc::new(CodeGenerator::new(redis_client.clone()));
        let sms_service = Arc::new(SmsService::new(code_generator.clone(), &config.sms));
        Ok(Self {
            jwt_manager,
            sms_service,
        })
    }
}
