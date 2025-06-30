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

pub struct TokenInfo {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}
