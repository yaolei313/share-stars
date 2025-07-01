use crate::http::vo::{AppResult, DeviceInfo};
use crate::http::AppState;

pub async fn check_send_sms_limit(
    state: &AppState,
    e64phone: &str,
    device_info: &DeviceInfo,
) -> AppResult<()> {
    // 检查该手机号短信发送频率,60s冷却期

    // 检查该手机号今日发送次数，手机号每个自然日所有类型短信发送次数限制=10

    // 检查该设备今日发送次数，设备每个自然日所有类型短信发送次数限制=20

    Ok(())
}

pub fn add_send_sms_count(
    state: &AppState,
    e64phone: &str,
    device_info: &DeviceInfo,
) -> AppResult<()> {
    Ok(())
}
