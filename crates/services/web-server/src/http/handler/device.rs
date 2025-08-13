use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, RespVo};
use crate::http::AppState;
use axum::extract::State;
use axum::Json;

pub async fn register_device(
    State(state): State<AppState>,
) -> AppResult<Json<RespVo<LoginResult>>> {
    todo!()
}
