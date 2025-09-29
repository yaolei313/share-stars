use crate::http::mw::ExtractRequestInfo;
use crate::http::vo::error::AppError;
use crate::http::vo::sms::{SmsSendReq, SmsSendResult, SmsType};
use crate::http::vo::{success_resp_none_data, AppResult, RespVo};
use crate::http::AppState;
use axum::extract::State;
use axum::Json;
use std::borrow::Cow;
use validator::Validate;

#[axum::debug_handler]
pub async fn send_sms(
    State(state): State<AppState>,
    ExtractRequestInfo(req_info): ExtractRequestInfo,
    Json(payload): Json<SmsSendReq>,
) -> AppResult<Json<RespVo<SmsSendResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        return Err(AppError::InvalidArgument(Cow::Owned(err.to_string())));
    }
    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|_e| AppError::InvalidPhoneNumber(payload.phone))?;
    log::info!("send sms. {}", e164_phone);
    let sms_type = SmsType::Login;

    // 业务
    state
        .service_state
        .sms_service
        .send_sms_code(&e164_phone, sms_type, &req_info)
        .await?;

    let rsp: RespVo<SmsSendResult> = success_resp_none_data();
    Ok(Json::from(rsp))
}
