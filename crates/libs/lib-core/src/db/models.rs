mod account;
mod device;
pub use account::*;
use lib_macro_derive::BindCode;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, BindCode)]
pub enum OidcProviderEnum {
    #[code(11)]
    Facebook,
    #[code(12)]
    Google,
    #[code(13)]
    Apple,
}

#[derive(Debug)]
pub enum Principal<'a> {
    Phone(&'a str),
    Email(&'a str),
    OpenId {
        provider: OidcProviderEnum,
        open_id: &'a str,
    },
}

impl<'a> Principal<'a> {
    pub fn provider(&self) -> i32 {
        match self {
            Principal::Phone(_) => 1,
            Principal::Email(_) => 2,
            Principal::OpenId { provider, .. } => provider.code(),
        }
    }

    pub fn identity(&self) -> &str {
        match self {
            Principal::Phone(phone) => phone,
            Principal::Email(email) => email,
            Principal::OpenId { open_id, .. } => open_id,
        }
    }
}

pub enum Credential {
    Password(String),
    AuthorizationCode(String),
    SmsCode(String),
}

impl<'a> Display for Principal<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Principal::Phone(principal) => write!(f, "{}", principal),
            Principal::Email(principal) => write!(f, "{}", principal),
            Principal::OpenId { provider, open_id } => {
                write!(f, "{} {}", provider.code(), open_id)
            }
        }
    }
}
