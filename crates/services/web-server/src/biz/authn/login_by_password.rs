use crate::biz::authn::base_login;
use crate::biz::dto::{AuthnMethodEnum, Identity};
use crate::biz::security::PasswordStatistic;
use crate::biz::{device, security};
use crate::http::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};
use lib_core::db::models::Account;
use rsa::signature::digest::{Digest, Update};
use sha2::Sha256;
use std::sync::Arc;

pub async fn login_by_password(
    state: AppState,
    e164_phone: &str,
    password: &str,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    log::info!("login by. {}", e164_phone);
    let identity = Identity::PhoneNumber(e164_phone);
    let account = base_login::query_then_check_status(&state, &identity).await?;
    let Some(account) = account else {
        return Err(AppError::UnregisterAccount);
    };
    base_login::check_password(&state, password, &account).await?;
    base_login::do_login(
        &state,
        account.user_id,
        false,
        &AuthnMethodEnum::Password,
        device_info,
    )
    .await
}
