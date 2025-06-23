use crate::biz::security;
use crate::config::AppState;
use crate::http::common;
use crate::http::mw::ExtractDeviceInfo;
use crate::http::vo::error::AppError;
use crate::http::vo::sms::{SmsSendReq, SmsSendResult};
use crate::http::vo::{AppResult, RespVo};
use axum::extract::State;
use axum::Json;
use validator::Validate;

pub async fn send_sms(
    State(state): State<AppState>,
    ExtractDeviceInfo(device_info): ExtractDeviceInfo,
    Json(payload): Json<SmsSendReq>,
) -> AppResult<Json<RespVo<SmsSendResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        return Err(AppError::InvalidArgument(err.to_string()));
    }
    let e164phone = common::validate_then_format_phone_number(&payload.phone)?;
    log::info!("send sms. {}", e164phone);

    // 业务校验
    security::check_send_sms_limit(&state, &e164phone, &device_info).await?;
    todo!()
}
