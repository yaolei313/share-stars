use crate::biz::account::AccountDeviceService;
use crate::biz::authn::{LoginService, MultiFactorAuthService};
use crate::biz::email::EmailSender;
use crate::biz::sms::{SmsSender, SmsTemplateManager};
use crate::biz::statistic::PasswordStatisticService;
use crate::biz::token::AccessTokenService;
use crate::biz::verify::{CodeManager, VerifyManager};
use crate::config::AppSettings;
use anyhow::{Context, Result};
use lib_core::db::DbServiceState;
use lib_utils::IdGenerator;
use std::sync::Arc;

pub mod account;
pub mod authn;
pub mod device;
pub mod dto;
mod email;
pub mod sms;
pub mod statistic;
pub mod token;
pub mod verify;

#[derive(Clone)]
pub struct ServiceState {
    pub code_manager: Arc<CodeManager>,
    pub id_generator: Arc<IdGenerator>,
    pub sms_sender: Arc<SmsSender>,
    pub token_service: Arc<AccessTokenService>,
    pub mfa_service: Arc<MultiFactorAuthService>,
    pub password_statistic: Arc<PasswordStatisticService>,
    pub account_device_service: Arc<AccountDeviceService>,
    pub login_service: Arc<LoginService>,
    pub verify_manager: Arc<VerifyManager>,
}

impl ServiceState {
    pub fn new(
        db_service_state: Arc<DbServiceState>,
        redis_client: Arc<redis::Client>,
        config: Arc<AppSettings>,
    ) -> Result<Self> {
        let code_manager = Arc::new(CodeManager::new(redis_client.clone()));
        let id_generator = Arc::new(IdGenerator::new(config.server.worker_id)?);
        let sms_template_manager = Arc::new(SmsTemplateManager::new());
        let sms_sender = Arc::new(SmsSender::new(
            config.env,
            sms_template_manager.clone(),
            &config.sms,
        ));
        // let email_template_manager = Arc::new(EmailTemplateManager::new());
        let email_config = Arc::new(config.email.clone());
        let email_sender = Arc::new(
            EmailSender::new(
                config.env,
                // email_template_manager.clone(),
                email_config,
            )
            .context("creating email sender failed")?,
        );

        let token_service = Arc::new(AccessTokenService::new(&config.login)?);
        let verify_manager = Arc::new(VerifyManager::new(
            redis_client.clone(),
            code_manager.clone(),
            sms_sender.clone(),
            email_sender.clone(),
        ));
        let mfa_service = Arc::new(MultiFactorAuthService::new(
            redis_client.clone(),
            db_service_state.account_db_service.clone(),
            verify_manager.clone(),
        )?);

        let password_statistic = Arc::new(PasswordStatisticService::new(redis_client.clone())?);

        let account_device_service = Arc::new(AccountDeviceService::new(
            db_service_state.account_db_service.clone(),
        ));
        let login_service = Arc::new(LoginService::new(
            id_generator.clone(),
            db_service_state.account_db_service.clone(),
            password_statistic.clone(),
            account_device_service.clone(),
            token_service.clone(),
            verify_manager.clone(),
            mfa_service.clone(),
        ));
        Ok(Self {
            code_manager,
            id_generator,
            sms_sender,
            token_service,
            mfa_service,
            password_statistic,
            account_device_service,
            login_service,
            verify_manager,
        })
    }
}
