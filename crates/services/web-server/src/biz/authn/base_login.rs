use crate::biz::authn::base_login;
use crate::biz::dto::AuthnMethodEnum;
use crate::biz::security::{add_password_error_count, is_exceed_password_error_limit};
use crate::biz::{device, security, session};
use crate::http::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};
use axum_extra::handler::HandlerCallWithExtractors;
use lib_core::db::models::{Passport, Principal};
use sha2::{Digest, Sha256};

pub async fn common_login<T, P>(
    state: &AppState,
    principal: &Principal<'_>,
    auth_type: AuthnMethodEnum,
    device_info: &DeviceInfo,
    validate_credentials: T,
    validate_register: P,
) -> AppResult<LoginResult>
where
    T: Fn(&Passport) -> AppResult<()>,
    P: Fn() -> AppResult<()>,
{
    log::info!("login by. {}", principal);
    let passport = state
        .service_state
        .passport_service
        .query_passport(principal)
        .await?;
    let mut new_register = false;
    let user_id = match passport {
        Some(passport) => {
            log::info!("passport status checking. {}", principal);
            let user_id = passport.user_id;
            // 1.校验状态
            check_passport_status(&passport)?;

            // 2.登陆前校验
            validate_credentials(&passport)?;

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
                .passport_service
                .create_passport(principal, new_user_id)
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

fn check_passport_status(passport: &Passport) -> AppResult<()> {
    let user_id = passport.user_id;
    if passport.disabled {
        log::warn!("passport temporarily disabled. {}", user_id);
        return Err(AppError::AccountTemporarilyDisabled);
    }
    if passport.closed {
        log::warn!("passport closed. {}", user_id);
        return Err(AppError::AccountClosed);
    }
    Ok(())
}

pub(crate) fn check_password(passport: &Passport, input_password: &str) -> AppResult<()> {
    let user_id = passport.user_id;
    // 密码错误次数检查
    if is_exceed_password_error_limit(user_id) {
        log::warn!("too many incorrect password attempts. {}", user_id);
        return Err(AppError::TooManyIncorrectPasswordAttempts);
    }

    // 密码是否一致
    let mut hasher = Sha256::new();
    // 将密码和盐值连接起来，然后进行散列
    hasher.update(input_password.as_bytes());
    hasher.update(&passport.salt.as_bytes());
    let input_password_sha256 = hex::encode(hasher.finalize());

    if &input_password_sha256 != &passport.password_sha256 {
        log::warn!("password not matched. {}", user_id);
        add_password_error_count(user_id);
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
