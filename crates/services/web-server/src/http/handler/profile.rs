use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::Json;
use crate::http::middleware::CurrentUser;
use crate::http::vo::RespVo;

pub async fn profile_me(current_user: CurrentUser) -> Json<RespVo<String>> {
    Json("hello world".into())
}

pub async fn profile(
    Path(user_id): Path<u64>,
    current_user: CurrentUser,
    Query(param): Query<HashMap<String, String>>,
) -> String {
    format!("user id:{}", user_id)
}