use crate::http::vo::register::{RegisterByEmailReq, RegisterResult};
use crate::http::vo::RespVo;
use crate::http::AppState;
use axum::extract::State;
use axum::Json;

pub async fn register_by_email(
    State(state): State<AppState>,
    Json(payload): Json<RegisterByEmailReq>,
) -> Json<RespVo<RegisterResult>> {
    todo!()
}
