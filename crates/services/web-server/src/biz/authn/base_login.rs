use crate::biz::dto::AuthnTypeEnum;
use crate::biz::session;
use crate::config::AppState;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};

pub(crate) async fn common_login(
    state: AppState,
    login_method: AuthnTypeEnum,
    user_id: i64,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    let token = session::create_token(state, login_method, user_id, device_info).await?;
    let result = LoginResult {
        user_id,
        new_register: false,
        access_token: token.access_token,
        expires_in: token.expires_in,
        refresh_token: token.refresh_token,
    };
    Ok(result)
}

fn common_login_with_auto_registration_support() {}
