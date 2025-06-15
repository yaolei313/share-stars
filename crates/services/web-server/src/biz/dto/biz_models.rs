pub enum AuthnTypeEnum {
    SmsCode = 1,
    Password = 2,
    OidcFacebook = 4,
    OidcGoogle = 5,
    OidcApple = 7,

    QrCode = 10,
}

pub struct TokenInfo {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}
