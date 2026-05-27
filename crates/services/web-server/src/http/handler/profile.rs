use crate::http::mw::AccessTokenAuth;
use crate::http::vo::{ApiResponse, AppResult};
use crate::http::AppState;
use axum::extract::{Path, Query, State};
use std::collections::HashMap;

pub async fn profile_me(
    State(state): State<AppState>,
    AccessTokenAuth(current_user): AccessTokenAuth,
) -> AppResult<ApiResponse<String>> {
    Ok(ApiResponse::success(format!(
        "hello world {:?}",
        current_user
    )))
}

pub async fn profile(
    State(state): State<AppState>,
    Path(user_id): Path<u64>,
    AccessTokenAuth(current_user): AccessTokenAuth,
    Query(param): Query<HashMap<String, String>>,
) -> String {
    format!("user id:{}", user_id)
}
