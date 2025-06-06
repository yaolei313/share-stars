use crate::biz::login;
use crate::config::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::login::*;
use crate::http::vo::*;
use axum::extract::State;
use axum::{extract::Json, http::HeaderMap};
use std::borrow::Borrow;
use validator::{Validate, ValidationErrors};

///
///
pub async fn login_by_password(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginByPasswordReq>,
) -> AppResult<Json<RespVo<LoginResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        return Err(AppError::InvalidArgument(err.to_string()));
    }
    for (key, value) in &headers {
        log::info!("header: {} {}", key, value.to_str().ok().unwrap());
    }
    login::login_by_password(state, &payload.phone, &payload.password)
        .map(|r| Json(RespVo::success(r)))
}

pub async fn login_by_sms(
    State(state): State<AppState>,
    Json(payload): Json<LoginBySmsReq>,
) -> Json<LoginResult> {
    todo!()
}
