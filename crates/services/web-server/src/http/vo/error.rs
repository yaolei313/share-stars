use crate::http::vo::RespVo;
use axum::Json;
use axum::response::{IntoResponse, Response};
use lib_macro_derive::BindCode;
use std::io;
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

    #[code(100)]
    #[error("invalid phone number {0}")]
    InvalidPhoneNumber(String),

    #[code(101)]
    #[error("unregister phone number")]
    UnregisterPhone,

    #[code(102)]
    #[error("user may not exists or password error")]
    InvalidUserOrPassword,

    #[code(103)]
    #[error("account has been temporarily disabled")]
    AccountTemporarilyDisabled,

    #[code(104)]
    #[error("account has been closed")]
    AccountClosed,

    #[code(105)]
    #[error("too many incorrect password attempts")]
    TooManyIncorrectPasswordAttempts,

    #[code(106)]
    #[error("login attempt from an unrecognized device")]
    Upgraded2FASms,

    #[code(107)]
    #[error("login attempt from an unrecognized device")]
    Upgraded2FAHardToken,

    #[code(108)]
    #[error("login attempt from an unrecognized device")]
    Upgraded2FAPushCode,

    #[code(401)]
    #[error("authentication required")]
    Unauthorized,

    #[code(403)]
    #[error("operation not allowed")]
    Forbidden,

    #[code(1000)]
    #[error("database operation failed: {0}")]
    Database(#[from] sqlx::Error),

    #[code(1001)]
    #[error("I/O operation failed: {0}")]
    Io(#[from] io::Error),

    #[code(1002)]
    #[error("jsonwebtoken operation failed: {0}")]
    Token(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let vo: RespVo<()> = RespVo::from(self);
        Json(vo).into_response()
    }
}
