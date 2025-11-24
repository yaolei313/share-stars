use crate::biz::verify::VerifyScenario;
use crate::http::vo::mfa::MfaVerificationChallenge;
use crate::http::vo::RespVo;
use axum::response::{IntoResponse, Response};
use axum::Json;
use lib_macro_derive::BindCode;
use redis::RedisError;
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::io;
use thiserror::Error;
use twilio::TwilioError;

#[derive(Error, Debug, BindCode)]
pub enum AppError {
    #[code(1)]
    #[error("fail:{0}")]
    Fail(Cow<'static, str>),

    #[code(10)]
    #[error("invalid argument: {0}")]
    InvalidArgument(Cow<'static, str>),

    #[code(400)]
    #[error("invalid request")]
    InvalidRequest(Cow<'static, str>),

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
    InternalServerError(Cow<'static, str>),

    #[code(503)]
    #[error("service unavailable, try again later")]
    ServiceUnavailable,

    // --below system error--
    #[code(1000)]
    #[error("database operation failed")]
    ComponentDatabase(#[from] sqlx::Error),

    #[code(1001)]
    #[error("I/O operation failed: {0}")]
    ComponentIo(#[from] io::Error),

    #[code(1002)]
    #[error("jsonwebtoken operation failed: {0}")]
    ComponentJwt(#[from] jsonwebtoken::errors::Error),

    #[code(1003)]
    #[error("invalid config: {0}")]
    ComponentInvalidConfig(Cow<'static, str>),

    #[code(1004)]
    #[error("twilio error: {0}")]
    ComponentTwilioError(#[from] TwilioError),

    #[code(1005)]
    #[error("redis error")]
    ComponentRedisError(#[from] RedisError),

    #[code(1006)]
    #[error("sonyflake error: {0}")]
    IdGeneratorError(#[from] sonyflake::Error),

    // below is biz error
    #[code(2000)]
    #[error("invalid phone number: {0}")]
    InvalidPhoneNumber(String),

    #[code(2001)]
    #[error("unregister account")]
    UnregisterAccount,

    #[code(2002)]
    #[error("user may not exists or password error")]
    InvalidUserOrPassword,

    #[code(2003)]
    #[error("account has been temporarily disabled")]
    AccountTemporarilyDisabled,

    #[code(2004)]
    #[error("account has been closed")]
    AccountClosed,

    #[code(2005)]
    #[error("too many incorrect password attempts")]
    TooManyIncorrectPasswordAttempts,

    #[code(2006)]
    #[error("MFA verification required")]
    UpgradedMFA(MfaVerificationChallenge),

    #[code(2007)]
    #[error("invalid mfa token")]
    InvalidMfaToken,

    #[code(2009)]
    #[error("invalid sms code")]
    InvalidSmsCode,

    #[code(2010)]
    #[error("please wait {1} seconds before requesting another code")]
    FrequencyExceed(VerifyScenario, i64),

    #[code(2011)]
    #[error("quota exceeded")]
    DailyQuotaExceed(VerifyScenario, i32),

    #[code(2012)]
    #[error("quota exceeded")]
    DeviceDailyQuotaExceed(VerifyScenario, i32),

    #[code(2013)]
    #[error("please wait {0} seconds before requesting another code")]
    EmailFrequencyExceed(i64),

    #[code(2014)]
    #[error("email quota exceeded")]
    EmailDailyQuotaExceed,

    #[code(2015)]
    #[error("email quota exceeded")]
    EmailDeviceDailyQuotaExceed,

    #[code(2016)]
    #[error("template error: {0}")]
    TemplateError(Cow<'static, str>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        log::error!("{:?}", self);
        let vo: RespVo<()> = RespVo::from(self);
        Json(vo).into_response()
    }
}
