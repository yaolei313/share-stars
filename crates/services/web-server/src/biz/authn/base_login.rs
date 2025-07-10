use crate::biz::dto::AuthnMethodEnum;
use crate::biz::{device, security, session};
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};
use crate::http::AppState;
use lib_core::db::models::{Account, Principal};
use sha2::{Digest, Sha256};

pub async fn common_login<T, P>(
    state: &AppState,
    principal: &Principal<'_>,
    auth_type: AuthnMethodEnum,
    device_info: &DeviceInfo,
    validate_login: T,
    validate_register: P,
) -> AppResult<LoginResult>
where
    T: Fn(&Account) -> AppResult<()>,
    P: Fn() -> AppResult<()>,
{
    log::info!("login by. {}", principal);
    let account = state
        .service_state
        .account_service
        .query_by_principal(principal)
        .await?;
    let mut new_register = false;
    let user_id = match account {
        Some(account) => {
            log::info!("account status checking. {}", principal);
            let user_id = account.user_id;
            // 1.校验状态
            check_status(&account)?;

            // 2.登陆前校验
            validate_login(&account)?;

            // 3.可信设备校验
            device::check_trusted_device(user_id, device_info, auth_type).await?;

            user_id
        }
        None => {
            log::info!("registering. {}", principal);
            new_register = true;

            // 1.注册前校验
            validate_register()?;
            let new_user_id = state.service_state.id_generator.next_id()?;

            // 2.注册
            state
                .service_state
                .account_service
                .create_account(principal, new_user_id)
                .await?;

            // 3.保存设备
            device::save_new_device(new_user_id, device_info, &auth_type).await?;

            new_user_id
        }
    };

    // token生成
    gen_token(
        state,
        AuthnMethodEnum::Password,
        user_id,
        new_register,
        device_info,
    )
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

pub(crate) fn check_password(account: &Account, input_password: &str) -> AppResult<()> {
    let user_id = account.user_id;
    // 密码错误次数检查
    if security::is_exceed_password_error_limit(user_id) {
        log::warn!("too many incorrect password attempts. {}", user_id);
        return Err(AppError::TooManyIncorrectPasswordAttempts);
    }

    // 密码是否一致
    let mut hasher = Sha256::new();
    // 将密码和盐值连接起来，然后进行散列
    hasher.update(input_password.as_bytes());
    hasher.update(&account.salt.as_bytes());
    let input_password_sha256 = hex::encode(hasher.finalize());

    if &input_password_sha256 != &account.password_hash {
        log::warn!("password not matched. {}", user_id);
        security::add_password_error_count(user_id);
        return Err(AppError::InvalidUserOrPassword);
    }

    Ok(())
}
fn gen_token(
    state: &AppState,
    authn_method: AuthnMethodEnum,
    user_id: i64,
    new_register: bool,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    let token = session::create_token(state, authn_method, user_id, device_info)?;
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
