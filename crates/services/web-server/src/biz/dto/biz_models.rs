use crate::http::vo::mfa::{MfaInfo, MfaMethod};
use crate::http::vo::AccessContext;
use chrono::Utc;
use lib_core::db::models::ProviderType;
use lib_macro_derive::BindCode;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, BindCode)]
pub enum AuthnMethod {
    #[code(1)]
    SmsCode,
    #[code(2)]
    PhonePassword,
    #[code(3)]
    OidcFacebook,
    #[code(4)]
    OidcGoogle,
    #[code(5)]
    OidcApple,
    #[code(10)]
    QrCode,
}

impl Display for AuthnMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthnMethod::SmsCode => write!(f, "sms-code"),
            AuthnMethod::PhonePassword => write!(f, "password"),
            AuthnMethod::OidcFacebook => write!(f, "oidc-facebook"),
            AuthnMethod::OidcGoogle => write!(f, "oidc-google"),
            AuthnMethod::OidcApple => write!(f, "oidc-apple"),
            AuthnMethod::QrCode => write!(f, "qr-code"),
        }
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub enum OidcProvider {
    Facebook,
    Google,
    Apple,
}

impl From<OidcProvider> for ProviderType {
    fn from(value: OidcProvider) -> Self {
        match value {
            OidcProvider::Facebook => ProviderType::Facebook,
            OidcProvider::Google => ProviderType::Google,
            OidcProvider::Apple => ProviderType::Apple,
        }
    }
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
        provider: OidcProvider,
        id_token: String,
    },
}

pub enum Identity<'a> {
    PhoneNumber(&'a str),
    Email(&'a str),
    OpenId {
        provider: OidcProvider,
        open_id: &'a str,
    },
}

impl Identity<'_> {
    pub fn provider(&self) -> i32 {
        let provider_type = match self {
            Identity::PhoneNumber(_) => ProviderType::PhoneNumber,
            Identity::Email(_) => ProviderType::Email,
            Identity::OpenId { provider, .. } => (*provider).into(),
        };
        provider_type.code()
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
                write!(f, "{:?} {}", ProviderType::from(*provider), open_id)
            }
        }
    }
}

pub struct TokenInfo {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MfaSession {
    pub user_id: i64,
    pub mfa_infos: Vec<MfaInfo>,
    pub create_time: chrono::DateTime<Utc>,
    pub authn_method: AuthnMethod,
    pub req_info: AccessContext,
    pub chosen_method: Option<MfaMethod>,
}
