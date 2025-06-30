use crate::biz::{security, verify};
use crate::config::AppState;
use crate::http::handler::sms;
use crate::http::mw::ExtractDeviceInfo;
use crate::http::vo::error::AppError;
use crate::http::vo::sms::{SmsSendReq, SmsSendResult, SmsType};
use crate::http::vo::{AppResult, RespVo, success_resp_none_data};
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use validator::Validate;

#[axum::debug_handler]
pub async fn send_sms(
    State(state): State<AppState>,
    ExtractDeviceInfo(device_info): ExtractDeviceInfo,
    Json(payload): Json<SmsSendReq>,
) -> AppResult<Json<RespVo<SmsSendResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        return Err(AppError::InvalidArgument(err.to_string()));
    }
    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|e| AppError::InvalidPhoneNumber(payload.phone.to_string()))?;
    log::info!("send sms. {}", e164_phone);

    // 业务校验
    security::check_send_sms_limit(&state, &e164_phone, &device_info).await?;

    state
        .service_state
        .sms_service
        .send_verification_sms(&e164_phone, &SmsType::Login)
        .await?;

    let rsp: RespVo<SmsSendResult> = success_resp_none_data();
    Ok(Json::from(rsp))
}
