use crate::biz::authn::base_login;
use crate::biz::dto::AuthnTypeEnum;
use crate::biz::security::{
    add_password_error_count, is_exceed_password_error_limit, is_trusted_device,
};
use crate::config::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};
use lib_core::db::models::PhoneMapping;
use lib_core::db::repositories::{PassportRepository, PhoneMappingRepository};
use sha2::{Digest, Sha256};

pub async fn login_by_password(
    state: AppState,
    e64phone: &str,
    password: &str,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    let phone_mapping = state
        .repository_state
        .phone_mapping_repo
        .by_phone(e64phone)
        .await?;
    let Some(PhoneMapping { user_id, .. }) = phone_mapping else {
        log::warn!("no phone mapping found. {}", e64phone);
        return Err(AppError::UnregisterPhone);
    };
    let passport = state
        .repository_state
        .passport_repo
        .by_user_id(user_id)
        .await?;
    let Some(passport) = passport else {
        log::warn!("no passport found. {} {}", e64phone, user_id);
        return Err(AppError::UnregisterPhone);
    };

    // 状态检查
    if passport.disabled {
        log::warn!("passport temporarily disabled. {}", user_id);
        return Err(AppError::AccountTemporarilyDisabled);
    }
    if passport.closed {
        log::warn!("passport closed. {}", user_id);
        return Err(AppError::AccountClosed);
    }

    // 密码校验
    if is_exceed_password_error_limit(user_id) {
        log::warn!("too many incorrect password attempts. {}", user_id);
        return Err(AppError::TooManyIncorrectPasswordAttempts);
    }
    if !check_pwd(password, &passport.salt, &passport.password_sha256) {
        log::warn!("password not matched. {}", user_id);
        add_password_error_count(user_id);
        return Err(AppError::InvalidUserOrPassword);
    }

    // 可信设备校验
    if !is_trusted_device(user_id, device_info) {
        return Err(AppError::Upgraded2FASms);
    }

    // 公共登陆逻辑
    base_login::common_login(state, AuthnTypeEnum::Password, user_id, device_info).await
}

fn check_pwd(input_password: &str, salt: &str, password_sha256: &str) -> bool {
    let mut hasher = Sha256::new();
    // 将密码和盐值连接起来，然后进行散列
    hasher.update(input_password.as_bytes());
    hasher.update(salt.as_bytes());

    hex::encode(hasher.finalize()) == password_sha256
}
