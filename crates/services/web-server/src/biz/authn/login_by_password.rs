use crate::biz::authn::base_login;
use crate::biz::authn::base_login::common_login;
use crate::biz::dto::AuthnMethodEnum;
use crate::biz::security::{add_password_error_count, is_exceed_password_error_limit};
use crate::http::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};
use lib_core::db::models::{PhoneMapping, Principal};
use lib_core::db::repositories::{PassportRepository, PhoneMappingRepository};
use sha2::{Digest, Sha256};

pub async fn login_by_password(
    state: AppState,
    principal: &Principal<'_>,
    password: &str,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    common_login(
        &state,
        principal,
        AuthnMethodEnum::Password,
        device_info,
        |p| {
            // 1.校验密码和可信设备
            base_login::check_password(p, password)
        },
        || {
            log::warn!("no passport found. {}", principal);
            Err(AppError::UnregisterPhone)
        },
    )
    .await
}
