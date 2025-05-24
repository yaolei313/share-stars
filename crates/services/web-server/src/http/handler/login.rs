use crate::config::AppState;
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
) -> Json<RespVo<LoginResult>> {
    if let Err(err) = payload.validate() {
        return Json(RespVo::invalid_request(err));
    }
    for (key, value) in &headers {
        log::info!("header: {} {}", key, value.to_str().ok().unwrap());
    }
    if &payload.phone == "18866668888" && &payload.password == "abc123" {
        let result = LoginResult {
            user_id: 123,
            new_register: false,
            access_token: Some(String::from("123")),
            expire_seconds: 0,
            refresh_token: Some(String::from("refresh token")),
        };
        let vo = RespVo::success(result);
        return Json(vo);
    }
    let vo = RespVo::bad_request_with_message(String::from("forbidden"));
    Json(vo)
}

pub async fn login_by_sms(
    State(state): State<AppState>,
    Json(payload): Json<LoginBySmsReq>,
) -> Json<LoginResult> {
    todo!()
}
