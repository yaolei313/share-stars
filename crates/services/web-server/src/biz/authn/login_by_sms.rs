use crate::biz::authn::base_login;
use crate::biz::dto::AuthnMethodEnum;
use crate::http::vo::login::LoginResult;
use crate::http::vo::sms::SmsType;
use crate::http::vo::{AppResult, DeviceInfo};
use crate::http::AppState;
use lib_core::db::models::Principal;

pub async fn login_by_sms(
    state: AppState,
    e164_phone: &str,
    sms_code: &str,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    // 1.校验验证码
    validate_sms_code(&state, e164_phone, sms_code).await?;

    base_login::common_login(
        &state,
        &Principal::Phone(e164_phone),
        AuthnMethodEnum::SmsCode,
        device_info,
        |_| Ok(()),
        || Ok(()),
    )
    .await
}

async fn validate_sms_code(state: &AppState, e164_phone: &str, sms_code: &str) -> AppResult<()> {
    if lib_utils::is_test_phone_number(e164_phone) {
        state
            .service_state
            .code_manager
            .validate_test_code(e164_phone, &SmsType::Login, sms_code)
            .await
    } else {
        state
            .service_state
            .code_manager
            .validate_code(e164_phone, &SmsType::Login, sms_code)
            .await
    }
}
