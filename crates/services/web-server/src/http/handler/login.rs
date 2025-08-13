use crate::http::mw::ExtractRequestInfo;
use crate::http::vo::error::AppError;
use crate::http::vo::login::*;
use crate::http::vo::*;
use crate::http::AppState;
use axum::extract::Json;
use axum::extract::State;
use validator::Validate;

///
///
pub async fn login_by_password(
    State(state): State<AppState>,
    ExtractRequestInfo(req_info): ExtractRequestInfo,
    Json(payload): Json<LoginByPasswordReq>,
) -> AppResult<Json<RespVo<LoginResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        log::warn!("validation error: {}", err);
        return Err(AppError::InvalidArgument(err.to_string()));
    }

    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|_| AppError::InvalidPhoneNumber(payload.phone.to_string()))?;

    state
        .service_state
        .login_service
        .login_by_password(&e164_phone, &payload.password, &req_info)
        .await
        .map(|r| Json(success_resp(r)))
}

pub async fn login_by_sms(
    State(state): State<AppState>,
    ExtractRequestInfo(req_info): ExtractRequestInfo,
    Json(payload): Json<LoginBySmsReq>,
) -> AppResult<Json<RespVo<LoginResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        return Err(AppError::InvalidArgument(err.to_string()));
    }

    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|_e| AppError::InvalidPhoneNumber(payload.phone.to_string()))?;
    log::info!("login by sms. {}", e164_phone);
    state
        .service_state
        .login_service
        .login_by_sms(&e164_phone, &payload.sms_code, &req_info)
        .await
        .map(|r| Json(success_resp(r)))
}
