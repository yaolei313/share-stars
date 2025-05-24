use crate::config::AppState;
use crate::http::vo::RespVo;
use crate::http::vo::register::{RegisterByEmailReq, RegisterResult};
use axum::Json;
use axum::extract::State;

pub async fn register_by_email(
    State(state): State<AppState>,
    Json(payload): Json<RegisterByEmailReq>,
) -> Json<RespVo<RegisterResult>> {
    todo!()
}
