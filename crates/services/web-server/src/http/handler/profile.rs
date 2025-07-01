use crate::http::mw::CurrentUser;
use crate::http::vo::{success_resp, RespVo};
use crate::http::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use std::collections::HashMap;

pub async fn profile_me(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Json<RespVo<String>> {
    Json(success_resp("hello world".into()))
}

pub async fn profile(
    State(state): State<AppState>,
    Path(user_id): Path<u64>,
    current_user: CurrentUser,
    Query(param): Query<HashMap<String, String>>,
) -> String {
    format!("user id:{}", user_id)
}
