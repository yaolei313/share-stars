use crate::http::mw::ExtractRequestInfo;
use crate::http::vo::error::AppError;
use crate::http::vo::login::*;
use crate::http::vo::*;
use crate::http::AppState;
use axum::extract::Json;
use axum::extract::State;
use std::borrow::Cow;
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
        log::warn!("login by password validate error: {}", err);
        return Err(AppError::InvalidArgument(Cow::Owned(err.to_string())));
    }

    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|_| AppError::InvalidPhoneNumber(payload.phone))?;

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
        log::warn!("login by sms invalid argument: {:?} {}", payload, err);
        return Err(AppError::InvalidArgument(Cow::Owned(err.to_string())));
    }

    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|_e| AppError::InvalidPhoneNumber(payload.phone))?;
    log::info!("login by phone with sms. {}", e164_phone);
    state
        .service_state
        .login_service
        .login_by_sms(&e164_phone, &payload.sms_code, &req_info)
        .await
        .map(|r| Json(success_resp(r)))
}
