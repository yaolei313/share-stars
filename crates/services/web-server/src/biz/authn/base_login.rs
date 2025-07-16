use crate::biz::dto::{AuthnMethodEnum, Identity};
use crate::biz::session::TokenService;
use crate::biz::{device, security, session};
use crate::http::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};
use lib_core::db::models::{Account, AccountIdentity};
use lib_core::db::services::AccountService;
use lib_utils::IdGenerator;
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub async fn query_then_check_status(
    state: &AppState,
    idt: &Identity<'_>,
) -> AppResult<Option<Account>> {
    let account = state
        .service_state
        .account_service
        .query_by_identity(idt.provider(), idt.identifier())
        .await?;
    if let Some(ref accountRef) = account {
        check_status(accountRef)?;
    }
    Ok(account)
}

pub async fn check_password(
    state: &AppState,
    input_password: &str,
    account: &Account,
) -> AppResult<()> {
    let user_id = account.user_id;
    // 密码错误次数检查
    state
        .service_state
        .password_statistic
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
        state
            .service_state
            .password_statistic
            .add_password_error_count(user_id)
            .await?;
        return Err(AppError::InvalidUserOrPassword);
    }

    Ok(())
}

pub async fn register(state: &AppState, idt: &Identity<'_>) -> AppResult<i64> {
    let new_user_id = state.service_state.id_generator.next_id()?;
    state
        .service_state
        .account_service
        .create_account(idt.provider(), idt.identifier(), new_user_id)
        .await?;
    Ok(new_user_id)
}

pub async fn do_login(
    state: &AppState,
    user_id: i64,
    new_register: bool,
    authn_method: &AuthnMethodEnum,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    // 1.保存设备或可信设备校验
    if new_register {
        device::save_new_device(state, user_id, device_info, authn_method).await?;
    } else {
        device::check_trusted_device(state, user_id, device_info, authn_method).await?;
    }

    // 2.token生成
    let token =
        state
            .service_state
            .token_service
            .create_token(user_id, authn_method, device_info)?;
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
