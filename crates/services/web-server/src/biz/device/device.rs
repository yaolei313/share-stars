use crate::biz::dto::AuthnMethodEnum;
use crate::http::AppState;
use crate::http::vo::{AppResult, DeviceInfo};

pub async fn save_new_device(
    state: &AppState,
    user_id: i64,
    device: &DeviceInfo,
    auth_type: &AuthnMethodEnum,
) -> AppResult<()> {
    Ok(())
}
