use crate::http::mw::ExtractAccessContext;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::mfa::{MfaChallengeReq, MfaVerifyReq};
use crate::http::vo::{ApiResponse, AppResult};
use crate::http::AppState;
use axum::extract::State;
use axum::Json;
use std::borrow::Cow;
use validator::Validate;

pub async fn mfa_challenge(
    State(state): State<AppState>,
    ExtractAccessContext(req_info): ExtractAccessContext,
    Json(payload): Json<MfaChallengeReq>,
) -> AppResult<ApiResponse<()>> {
    if let Err(e) = payload.validate() {
        tracing::warn!("validation error: {}", e);
        return Err(AppError::InvalidArgument(Cow::Owned(e.to_string())));
    }
    state
        .service_state
        .mfa_service
        .send_challenge(&payload.mfa_session_id, payload.chosen_method, &req_info)
        .await
        .map(|_| ApiResponse::success_none())
}

pub async fn mfa_verify(
    State(state): State<AppState>,
    ExtractAccessContext(req_info): ExtractAccessContext,
    Json(payload): Json<MfaVerifyReq>,
) -> AppResult<ApiResponse<LoginResult>> {
    if let Err(e) = payload.validate() {
        tracing::warn!("invalid mfa verify request: {} {}", payload, e);
        return Err(AppError::InvalidArgument(Cow::Owned(e.to_string())));
    }
    state
        .service_state
        .mfa_service
        .verify_challenge(&payload.mfa_session_id, &payload.verify_code, &req_info)
        .await?;
    todo!()
}
