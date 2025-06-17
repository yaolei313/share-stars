use crate::biz::authn;
use crate::config::AppState;
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
    let Ok(number) = phonenumber::parse(None, &payload.phone) else {
        return Err(AppError::InvalidPhoneNumber(payload.phone.to_string()));
    };
    let std_phone = number.format().mode(Mode::E164).to_string();
    log::info!("phone number is {}", number);
    authn::login_by_password(state, &std_phone, &payload.password, &device_info)
        .await
        .map(|r| Json(RespVo::success(r)))
}

pub async fn login_by_sms(
    State(state): State<AppState>,
    Json(payload): Json<LoginBySmsReq>,
) -> Json<LoginResult> {
    todo!()
}
