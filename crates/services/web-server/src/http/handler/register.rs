use crate::http::mw::ExtractAccessContext;
use crate::http::vo::error::AppError;
use crate::http::vo::register::{EmailRegisterResponseData, RegisterByEmailReq};
use crate::http::vo::{ApiResponse, AppResult};
use crate::http::AppState;
use axum::extract::State;
use axum::Json;
use std::borrow::Cow;
use validator::Validate;

pub async fn register_by_email(
    State(state): State<AppState>,
    ExtractAccessContext(req_info): ExtractAccessContext,
    Json(payload): Json<RegisterByEmailReq>,
) -> AppResult<ApiResponse<EmailRegisterResponseData>> {
    if let Err(err) = payload.validate() {
        tracing::warn!("register validate error: {}", err);
        return Err(AppError::InvalidArgument(Cow::Owned(err.to_string())));
    }

    state
        .service_state
        .register_service
        .register_by_email(&payload.email, &payload.password, &req_info)
        .await
        .map(|r| ApiResponse::success(r))
}
