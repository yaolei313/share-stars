use crate::biz::verify::VerifyScenario;
use crate::http::mw::ExtractAccessContext;
use crate::http::vo::error::AppError;
use crate::http::vo::sms::{SmsSendReq, SmsSendResult};
use crate::http::vo::{ApiResponse, AppResult};
use crate::http::AppState;
use axum::extract::State;
use axum::Json;
use std::borrow::Cow;
use validator::Validate;

#[axum::debug_handler]
pub async fn send_sms(
    State(state): State<AppState>,
    ExtractAccessContext(req_info): ExtractAccessContext,
    Json(payload): Json<SmsSendReq>,
) -> AppResult<ApiResponse<SmsSendResult>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        tracing::warn!("failed to validate payload: {:?}", payload);
        return Err(AppError::InvalidArgument(Cow::Owned(err.to_string())));
    }
    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|_e| AppError::InvalidPhoneNumber(payload.phone))?;
    tracing::info!("send sms. {}", e164_phone);
    let verify_type = VerifyScenario::Login;

    // 业务
    state
        .service_state
        .verify_manager
        .send_sms_code(&e164_phone, verify_type, &req_info)
        .await?;

    Ok(ApiResponse::success_none())
}
