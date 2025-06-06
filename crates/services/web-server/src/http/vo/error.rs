use crate::http::vo::RespVo;
use axum::Json;
use axum::response::{IntoResponse, Response};
use lib_macro_derive::BindCode;
use thiserror::Error;

#[derive(Error, Debug, BindCode)]
pub enum AppError {
    #[code(0)]
    #[error("success")]
    Success,

    #[code(1)]
    #[error("fail:{0}")]
    Fail(String),

    #[code(2)]
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[code(401)]
    #[error("authentication required")]
    Unauthorized,

    #[code(403)]
    #[error("user may not perform that action")]
    Forbidden,

    #[code(500)]
    #[error("user may not exists or password error")]
    InvalidUserOrPassword,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let vo: RespVo<()> = RespVo::from(self);
        Json(vo).into_response()
    }
}
