use crate::http::vo::device::{RegisterDeviceReq, RegisterDeviceResult};
use crate::http::vo::error::AppError;
use crate::http::vo::{AppResult, RespVo};
use crate::http::AppState;
use axum::extract::State;
use axum::Json;
use validator::Validate;

pub async fn register_device(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDeviceReq>,
) -> AppResult<Json<RespVo<RegisterDeviceResult>>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        log::warn!("validation error: {}", err);
        return Err(AppError::InvalidArgument(err.to_string()));
    }
    todo!()
}
