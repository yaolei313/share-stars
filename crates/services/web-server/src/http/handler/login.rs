use crate::biz::authn;
use crate::config::AppState;
use crate::http::mw::ExtractDeviceInfo;
use crate::http::vo;
use crate::http::vo::error::AppError;
use crate::http::vo::login::*;
use crate::http::vo::*;
use axum::extract::Json;
use axum::extract::State;
use lib_core::db::models::LoginPrincipal;
use phonenumber::Mode;
use std::borrow::Borrow;
use validator::Validate;

///
///
pub async fn login_by_password(
    State(state): State<AppState>,
    ExtractDeviceInfo(device_info): ExtractDeviceInfo,
    Json(payload): Json<LoginByPasswordReq>,
) -> AppResult<Json<RespVo<LoginResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        return Err(AppError::InvalidArgument(err.to_string()));
    }

    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|e| AppError::InvalidPhoneNumber(payload.phone.to_string()))?;
    log::info!("login by password. {}", e164_phone);
    let principal = LoginPrincipal::Phone(&e164_phone);
    authn::login_by_password(state, &principal, &payload.password, &device_info)
        .await
        .map(|r| Json(success_resp(r)))
}

pub async fn login_by_sms(
    State(state): State<AppState>,
    ExtractDeviceInfo(device_info): ExtractDeviceInfo,
    Json(payload): Json<LoginBySmsReq>,
) -> AppResult<Json<RespVo<LoginResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        return Err(AppError::InvalidArgument(err.to_string()));
    }

    let e164_phone = lib_utils::validate_then_format_phone_number(&payload.phone)
        .map_err(|e| AppError::InvalidPhoneNumber(payload.phone.to_string()))?;
    log::info!("login by sms. {}", e164_phone);
    authn::login_by_sms(state, &e164_phone, &payload.sms_code, &device_info)
        .await
        .map(|r| Json(success_resp(r)))
}
