use crate::http::vo::RespVo;
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Json};
use lib_macro_derive::BindCode;
use redis::RedisError;
use std::io;
use thiserror::Error;
use twilio::TwilioError;

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
    #[error("invalid phone number: {0}")]
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

    #[code(109)]
    #[error("invalid sms code")]
    InvalidSmsCode,

    #[code(401)]
    #[error("authentication required")]
    Unauthorized,

    #[code(403)]
    #[error("operation not allowed")]
    Forbidden,

    #[code(408)]
    #[error("request timeout")]
    RequestTimeout,

    #[code(500)]
    #[error("internal server error: {0}")]
    InternalServerError(#[from] BoxError),

    #[code(503)]
    #[error("service is overloaded, try again later")]
    ServiceUnavailable,

    // --below system error--
    #[code(1000)]
    #[error("database operation failed: {0}")]
    ComponentDatabase(#[from] sqlx::Error),

    #[code(1001)]
    #[error("I/O operation failed: {0}")]
    ComponentIo(#[from] io::Error),

    #[code(1002)]
    #[error("jsonwebtoken operation failed: {0}")]
    ComponentJwt(#[from] jsonwebtoken::errors::Error),

    #[code(1003)]
    #[error("invalid config: {0}")]
    ComponentInvalidConfig(&'static str),

    #[code(1004)]
    #[error("twilio error: {0}")]
    ComponentTwilioError(#[from] TwilioError),

    #[code(1005)]
    #[error("redis error")]
    ComponentRedisError(#[from] RedisError),

    #[code(1006)]
    #[error("sonyflake error: {0}")]
    IdGeneratorError(#[from] sonyflake::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let vo: RespVo<()> = RespVo::from(self);
        Json(vo).into_response()
    }
}
