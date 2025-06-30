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
    #[code(1)]
    Facebook,
    #[code(2)]
    Google,
    #[code(3)]
    Apple,
}

#[derive(Debug)]
pub enum LoginPrincipal<'a> {
    Phone(&'a str),
    Email(&'a str),
    OpenId {
        provider: OidcProviderEnum,
        open_id: &'a str,
    },
}

impl<'a> Display for LoginPrincipal<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginPrincipal::Phone(principal) => write!(f, "{}", principal),
            LoginPrincipal::Email(principal) => write!(f, "{}", principal),
            LoginPrincipal::OpenId { provider, open_id } => {
                write!(f, "{} {}", provider.code(), open_id)
            }
        }
    }
}

// use chrono::prelude::*;
// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;
// use uuid::Uuid;
//
// #[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, PartialEq)]
// pub struct User {
//     pub id: Uuid,
//     pub name: String,
//     pub email: String,
//     pub password: String,
//     pub verified: bool,
//     pub verification_token: Option<String>,
//     pub token_expires_at: Option<DateTime<Utc>>,
//     #[serde(rename = "createdAt")]
//     pub created_at: DateTime<Utc>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: DateTime<Utc>,
// }
