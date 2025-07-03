mod passport;
mod phone_mapping;
mod trusted_device;

use lib_macro_derive::BindCode;
pub use passport::*;
pub use phone_mapping::*;
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
