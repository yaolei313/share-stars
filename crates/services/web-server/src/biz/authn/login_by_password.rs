use crate::biz::authn::base_login;
use crate::biz::dto::AuthnMethodEnum;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};
use crate::http::AppState;
use lib_core::db::models::Principal;

pub async fn login_by_password(
    state: AppState,
    principal: &Principal<'_>,
    password: &str,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    base_login::common_login(
        &state,
        principal,
        AuthnMethodEnum::Password,
        device_info,
        |a| {
            // 1.校验密码和可信设备
            base_login::check_password(a, password)
        },
        || {
            log::warn!("no passport found. {}", principal);
            Err(AppError::UnregisterPhone)
        },
    )
    .await
}
