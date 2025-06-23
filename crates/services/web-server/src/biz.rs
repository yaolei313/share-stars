use crate::biz::session::JwtManager;
use crate::biz::sms::SmsService;
use crate::config::AppSettings;
use anyhow::Result;
use std::sync::Arc;

pub mod authn;
pub mod dto;
pub mod security;
mod session;
mod sms;
mod two_factor;

#[derive(Clone)]
pub struct ServiceState {
    pub jwt_manager: Arc<JwtManager>,
    pub sms_service: Arc<SmsService>,
}

impl ServiceState {
    pub fn new(config: Arc<AppSettings>) -> Result<Self> {
        let jwt_manager = Arc::new(JwtManager::new(&config.jwt)?);
        let sms_service = Arc::new(SmsService::new(&config.sms));
        Ok(Self {
            jwt_manager,
            sms_service,
        })
    }
}
