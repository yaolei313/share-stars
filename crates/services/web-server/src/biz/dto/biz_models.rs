use lib_macro_derive::BindCode;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Deserialize, BindCode)]
pub enum AuthnMethodEnum {
    #[code(1)]
    SmsCode,
    #[code(2)]
    Password,
    #[code(3)]
    OidcFacebook,
    #[code(4)]
    OidcGoogle,
    #[code(5)]
    OidcApple,
    #[code(10)]
    QrCode,
}

impl Display for AuthnMethodEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthnMethodEnum::SmsCode => write!(f, "sms-code"),
            AuthnMethodEnum::Password => write!(f, "password"),
            AuthnMethodEnum::OidcFacebook => write!(f, "oidc-facebook"),
            AuthnMethodEnum::OidcGoogle => write!(f, "oidc-google"),
            AuthnMethodEnum::OidcApple => write!(f, "oidc-apple"),
            AuthnMethodEnum::QrCode => write!(f, "qr-code"),
        }
    }
}

#[derive(Debug, Deserialize, BindCode)]
pub enum ProviderTypeEnum {
    #[code(1)]
    Phone,
    #[code(2)]
    Email,
}

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
pub enum Credential {
    PhoneNumberPassword {
        phone_number: String,
        password: String,
    },
    PhoneNumberSmsCode {
        phone_number: String,
        sms_code: String,
    },
    EmailPassword {
        email: String,
        password: String,
    },
    OpenIdConnect {
        provider: OidcProviderEnum,
        id_token: String,
    },
}

pub enum Identity<'a> {
    PhoneNumber(&'a str),
    Email(&'a str),
    OpenId {
        provider: OidcProviderEnum,
        open_id: &'a str,
    },
}

impl Identity<'_> {
    pub fn provider(&self) -> i32 {
        match self {
            Identity::PhoneNumber(_) => 1,
            Identity::Email(_) => 2,
            Identity::OpenId { provider, .. } => provider.code(),
        }
    }

    pub fn identifier(&self) -> &str {
        match self {
            Identity::PhoneNumber(phone_number) => phone_number,
            Identity::Email(email) => email,
            Identity::OpenId {
                provider: _,
                open_id,
            } => open_id,
        }
    }
}

impl Display for Identity<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Identity::PhoneNumber(phone_number) => write!(f, "{}", phone_number),
            Identity::Email(email) => write!(f, "{}", email),
            Identity::OpenId { provider, open_id } => {
                write!(f, "{} {}", provider.code(), open_id)
            }
        }
    }
}

pub struct TokenInfo {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}
