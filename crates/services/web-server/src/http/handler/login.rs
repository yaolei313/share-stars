use crate::biz::authn;
use crate::config::AppState;
use crate::http::common;
use crate::http::mw::ExtractDeviceInfo;
use crate::http::vo::error::AppError;
use crate::http::vo::login::*;
use crate::http::vo::*;
use axum::extract::Json;
use axum::extract::State;
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

    let e64phone = common::validate_then_format_phone_number(&payload.phone)?;
    log::info!("login by password. {}", e64phone);
    authn::login_by_password(state, &e64phone, &payload.password, &device_info)
        .await
        .map(|r| Json(RespVo::success(r)))
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
    let e64phone = common::validate_then_format_phone_number(&payload.phone)?;
    log::info!("login by sms. {}", e64phone);
    authn::login_by_sms(state, &e64phone, &payload.sms_code, &device_info)
        .await
        .map(|r| Json(RespVo::success(r)))
}
