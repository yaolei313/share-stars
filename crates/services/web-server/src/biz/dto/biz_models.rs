use lib_macro_derive::BindCode;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, BindCode)]
pub enum AuthnTypeEnum {
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

impl Display for AuthnTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthnTypeEnum::SmsCode => write!(f, "sms-code"),
            AuthnTypeEnum::Password => write!(f, "password"),
            AuthnTypeEnum::OidcFacebook => write!(f, "oidc-facebook"),
            AuthnTypeEnum::OidcGoogle => write!(f, "oidc-google"),
            AuthnTypeEnum::OidcApple => write!(f, "oidc-apple"),
            AuthnTypeEnum::QrCode => write!(f, "qr-code"),
        }
    }
}

pub struct TokenInfo {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}
