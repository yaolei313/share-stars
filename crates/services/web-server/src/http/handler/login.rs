use crate::http::vo::login::*;
use crate::http::vo::*;
use axum::{
    extract::{Json, Path, Query},
    http::HeaderMap,
};
use std::{borrow::Borrow, collections::HashMap};
use crate::http::middleware::CurrentUser;

///
///
pub async fn login_by_password(
    headers: HeaderMap,
    Json(payload): Json<LoginByPasswordReq>,
) -> Json<RespVo<LoginResult>> {
    for (key, value) in &headers {
        println!("{} {}", key, value.to_str().ok().unwrap());
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
    let vo = RespVo::bad_request(String::from("forbidden"));
    Json(vo)
}

pub async fn login_by_sms(Json(payload): Json<LoginBySmsReq>) -> Json<LoginResult> {
    todo!()
}


