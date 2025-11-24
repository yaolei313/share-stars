use crate::biz::account::AccountDeviceService;
pub(crate) use crate::biz::authn::mfa::MultiFactorAuthService;
use crate::biz::dto::{AuthnMethod, Identity};
use crate::biz::statistic::PasswordStatisticService;
use crate::biz::token::AccessTokenService;
use crate::biz::verify::VerifyManager;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, RequestInfo};
use lib_core::db::models::Account;
use lib_core::db::services::AccountDbService;
use lib_utils::IdGenerator;
use sha2::{Digest, Sha256};
use std::sync::Arc;

mod login_by_password;
mod login_by_sms;

mod mfa;

pub struct LoginService {
    id_generator: Arc<IdGenerator>,
    token_service: Arc<AccessTokenService>,
    password_statistic_service: Arc<PasswordStatisticService>,
    account_db_service: Arc<AccountDbService>,
    account_device_service: Arc<AccountDeviceService>,
    verify_manager: Arc<VerifyManager>,
    mfa_service: Arc<MultiFactorAuthService>,
}

impl LoginService {
    pub fn new(
        id_generator: Arc<IdGenerator>,
        account_db_service: Arc<AccountDbService>,
        password_statistic_service: Arc<PasswordStatisticService>,
        account_device_service: Arc<AccountDeviceService>,
        token_service: Arc<AccessTokenService>,
        verify_manager: Arc<VerifyManager>,
        mfa_service: Arc<MultiFactorAuthService>,
    ) -> Self {
        Self {
            id_generator,
            token_service,
            password_statistic_service,
            account_db_service,
            account_device_service,
            verify_manager,
            mfa_service,
        }
    }

    pub async fn query_then_check_status(&self, idt: &Identity<'_>) -> AppResult<Option<Account>> {
        let account = self
            .account_db_service
            .query_by_identity(idt.provider(), idt.identifier())
            .await?;
        if let Some(ref account_ref) = account {
            check_status(account_ref)?;
        }
        Ok(account)
    }

    pub async fn check_password(&self, input_password: &str, account: &Account) -> AppResult<()> {
        let user_id = account.user_id;
        // 密码错误次数检查
        self.password_statistic_service
            .is_exceed_password_error_limit(user_id)
            .await?;

        // 密码是否一致
        let mut hasher = Sha256::new();
        // 将密码和盐值连接起来，然后进行散列
        hasher.update(input_password.as_bytes());
        hasher.update(&account.salt.as_bytes());
        let input_password_sha256 = hex::encode(hasher.finalize());

        if &input_password_sha256 != &account.password_hash {
            log::warn!("password not matched. {}", user_id);
            self.password_statistic_service
                .add_password_error_count(user_id)
                .await?;
            return Err(AppError::InvalidUserOrPassword);
        }

        Ok(())
    }

    pub async fn register(&self, idt: &Identity<'_>) -> AppResult<i64> {
        log::info!("Initiating new user registration. {}", idt);
        let new_user_id = self.id_generator.next_id()?;
        self.account_db_service
            .create_account(idt.provider(), idt.identifier(), new_user_id)
            .await?;
        log::info!("User registration successful. {}", new_user_id);
        Ok(new_user_id)
    }

    pub async fn do_login(
        &self,
        user_id: i64,
        new_register: bool,
        authn_method: AuthnMethod,
        req_info: &RequestInfo,
    ) -> AppResult<LoginResult> {
        // 1.保存设备或可信设备校验
        if new_register {
            self.account_device_service
                .save_new_account_device(user_id, req_info, authn_method)
                .await?;
            log::info!("new device.{} {}", user_id, &req_info.device_id);
        } else {
            let trusted = self
                .account_device_service
                .check_trusted_device(user_id, req_info)
                .await?;
            if !trusted {
                let challenge = self
                    .mfa_service
                    .generate_challenge(user_id, authn_method, req_info)
                    .await?;
                return Err(AppError::UpgradedMFA(challenge));
            }
            log::info!(
                "login from trusted device. {} {}",
                user_id,
                &req_info.device_id
            );
        }

        // 2.token生成
        let token = self
            .token_service
            .create_access_token(user_id, authn_method, req_info)?;
        let result = LoginResult {
            user_id,
            new_register,
            access_token: token.access_token,
            expires_in: token.expires_in,
            refresh_token: token.refresh_token,
        };
        log::info!("gen token finished. {}", user_id);
        Ok(result)
    }
}

fn check_status(account: &Account) -> AppResult<()> {
    let user_id = account.user_id;
    if account.disabled {
        log::warn!("account temporarily disabled. {}", user_id);
        return Err(AppError::AccountTemporarilyDisabled);
    }
    if account.closed {
        log::warn!("account closed. {}", user_id);
        return Err(AppError::AccountClosed);
    }
    Ok(())
}
