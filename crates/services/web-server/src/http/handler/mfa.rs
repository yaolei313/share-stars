use crate::http::vo::login::LoginResult;
use crate::http::vo::mfa::MfaChallengeResult;
use crate::http::vo::{AppResult, RespVo};
use axum::Json;

pub async fn mfa_challenge() -> AppResult<Json<RespVo<MfaChallengeResult>>> {
    todo!()
}

pub async fn mfa_verify() -> AppResult<Json<RespVo<LoginResult>>> {
    todo!()
}
