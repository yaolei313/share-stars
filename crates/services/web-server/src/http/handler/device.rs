use crate::http::vo::device::{RegisterDeviceReq, RegisterDeviceResult};
use crate::http::vo::error::AppError;
use crate::http::vo::{ApiResponse, AppResult};
use crate::http::AppState;
use axum::extract::State;
use axum::Json;
use std::borrow::Cow;
use validator::Validate;

pub async fn register_device(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDeviceReq>,
) -> AppResult<ApiResponse<RegisterDeviceResult>> {
    // 校验参数
    if let Err(err) = payload.validate() {
        tracing::warn!("validation error: {}", err);
        return Err(AppError::InvalidArgument(Cow::Owned(err.to_string())));
    }
    todo!()
}
