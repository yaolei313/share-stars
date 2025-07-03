use crate::biz::dto::AuthnMethodEnum;
use crate::http::vo::{AppResult, DeviceInfo};

pub async fn save_new_device(
    user_id: i64,
    device: &DeviceInfo,
    auth_type: &AuthnMethodEnum,
) -> AppResult<()> {
    Ok(())
}
