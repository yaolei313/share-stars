use std::sync::Arc;
use axum::http::HeaderMap;
use axum::{Extension, Json};
use lib_core::AppState;
use crate::http::vo::register::{RegisterByEmailReq, RegisterResult};
use crate::http::vo::RespVo;

pub async fn register_by_email(
    ctx: Extension<Arc<AppState>>,
    Json(payload): Json<RegisterByEmailReq>,
) -> Json<RespVo<RegisterResult>> {
    todo!()
}