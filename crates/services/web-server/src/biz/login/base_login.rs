use crate::biz::dto::AuthnTypeEnum;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};

fn common_login(
    login_method: AuthnTypeEnum,
    user_id: i64,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    let result = LoginResult {
        user_id: 123,
        new_register: false,
        access_token: Some(String::from("123")),
        expire_seconds: 0,
        refresh_token: Some(String::from("refresh token")),
    };
    return Ok(result);
}

fn common_login_auto_register() {}
