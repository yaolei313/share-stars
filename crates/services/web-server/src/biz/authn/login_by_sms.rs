use crate::config::AppState;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AppResult, DeviceInfo};

pub async fn login_by_sms(
    state: AppState,
    e64phone: &str,
    sms_code: &str,
    device_info: &DeviceInfo,
) -> AppResult<LoginResult> {
    todo!()
}
