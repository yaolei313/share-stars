use crate::biz::account::AccountDeviceService;
use crate::biz::authn::LoginService;
use crate::biz::security::{MultiFactorAuthService, PasswordStatisticService, SmsStatistic};
use crate::biz::token::{JwtManager, TokenService};
use crate::biz::verify::{CodeManager, SmsService};
use crate::config::{AppSettings, Env};
use anyhow::Result;
use lib_core::db::services::{AccountDbService, DeviceDbService};
use lib_core::RepositoryState;
use lib_utils::IdGenerator;
use std::sync::Arc;

mod account;
pub mod authn;
mod device;
pub mod dto;
pub mod security;
pub mod token;
pub mod verify;

#[derive(Clone)]
pub struct ServiceState {
    pub account_db_service: Arc<AccountDbService>,
    pub device_db_service: Arc<DeviceDbService>,
    pub code_manager: Arc<CodeManager>,
    pub id_generator: Arc<IdGenerator>,
    pub sms_service: Arc<SmsService>,
    pub token_service: Arc<TokenService>,
    pub mfa_service: Arc<MultiFactorAuthService>,
    pub password_statistic: Arc<PasswordStatisticService>,
    pub sms_statistic: Arc<SmsStatistic>,
    pub account_device_service: Arc<AccountDeviceService>,
    pub login_service: Arc<LoginService>,
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
        let account_db_service = Arc::new(AccountDbService::new(repository_state.clone()));
        let device_db_service = Arc::new(DeviceDbService::new(repository_state.clone()));
        let token_service = Arc::new(TokenService::new(JwtManager::new(&config.login)?));
        let password_statistic = Arc::new(PasswordStatisticService::new(redis_client.clone()));
        let sms_statistic = Arc::new(SmsStatistic::new(redis_client.clone()));
        let mfa_service = Arc::new(MultiFactorAuthService::new(JwtManager::new(&config.mfa)?));
        let account_device_service = Arc::new(AccountDeviceService::new(
            account_db_service.clone(),
            mfa_service.clone(),
        ));
        let login_service = Arc::new(LoginService::new(
            id_generator.clone(),
            account_db_service.clone(),
            password_statistic.clone(),
            account_device_service.clone(),
            token_service.clone(),
            sms_service.clone(),
        ));
        Ok(Self {
            account_db_service,
            device_db_service,
            code_manager,
            id_generator,
            sms_service,
            token_service,
            mfa_service,
            password_statistic,
            sms_statistic,
            account_device_service,
            login_service,
        })
    }
}
