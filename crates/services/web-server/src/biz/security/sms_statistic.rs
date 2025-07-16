use crate::http::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::sms::SmsType;
use crate::http::vo::{AppResult, DeviceInfo};
use chrono::{Datelike, Utc};
use redis::Script;
use std::sync::Arc;

pub struct SmsStatistic {
    redis_client: Arc<redis::Client>,
}

impl SmsStatistic {
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Self { redis_client }
    }

    pub async fn check_and_incr_send_sms_count(
        &self,
        e164_phone: &str,
        device_info: &DeviceInfo,
        sms_type: &SmsType,
    ) -> AppResult<()> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let day_of_month = Utc::now().naive_local().day();

        // Prepare keys for MGET
        let cool_down_key = gen_cool_down_duration_key(e164_phone, sms_type);
        let phone_daily_count_key = gen_today_phone_send_count_key(e164_phone, day_of_month);
        let device_daily_count_key = device_info
            .device_fp
            .as_ref()
            .map(|df| gen_today_device_send_count_key(df, day_of_month))
            .unwrap_or_else(|| "".to_string());
        let keys = vec![
            &cool_down_key,
            &phone_daily_count_key,
            &device_daily_count_key,
        ];
        log::info!("keys: {:?}", keys);

        let script = Script::new(SMS_LUA_SCRIPT);

        // The invoke_async method automatically handles SCRIPT LOAD and EVALSHA for efficiency.
        let (cooldown_status, phone_count, device_count): (i64, i64, i64) = script
            .key(keys)
            .arg(SMS_COOLDOWN_SECONDS)
            .arg(24 * 60 * 60)
            .arg(SMS_PHONE_DAILY_LIMIT)
            .arg(SMS_DEVICE_DAILY_LIMIT)
            .invoke_async(&mut conn)
            .await?;

        log::info!(
            "sms status: {} {} {}",
            cooldown_status,
            phone_count,
            device_count
        );

        if cooldown_status == 0 {
            return Ok(());
        } else if cooldown_status == 1 {
            return Err(AppError::SmsFrequencyExceed(SMS_COOLDOWN_SECONDS));
        } else if cooldown_status == 2 {
            return Err(AppError::SmsPhoneDailyQuotaExceed);
        } else if cooldown_status == 3 {
            return Err(AppError::SmsDeviceDailyQuotaExceed);
        }

        Ok(())
    }
}

// 某段时间内(key的失效时间)内某种类型的短信只能发送一次
fn gen_cool_down_duration_key(e164_phone: &str, sms_type: &SmsType) -> String {
    format!("{{int:sms}}-cool-down:{}.{}", e164_phone, sms_type.code())
}

fn gen_today_phone_send_count_key(e164_phone: &str, day_of_mouth: u32) -> String {
    format!("{{int:sms}}-today-phone:{}.{}", e164_phone, day_of_mouth)
}

fn gen_today_device_send_count_key(device_id: &str, day_of_mouth: u32) -> String {
    format!("{{int:sms}}-today-device:{}.{}", device_id, day_of_mouth)
}

const SMS_COOLDOWN_SECONDS: i64 = 60;
const SMS_PHONE_DAILY_LIMIT: i32 = 10;
const SMS_DEVICE_DAILY_LIMIT: i32 = 20;

const SMS_LUA_SCRIPT: &str = r#"
local cool_down_key = KEYS[1]
local phone_daily_count_key = KEYS[2]
local device_daily_count_key = KEYS[3]

local cool_down_expiry_seconds = tonumber(ARGV[1])
local daily_count_expiry_seconds = tonumber(ARGV[2])
local phone_daily_limit = tonumber(ARGV[3])
local device_daily_limit = tonumber(ARGV[4])

local current_phone_count = 0
local current_device_count = 0

local set_cooldown_result = redis.call('SET', cool_down_key, '1', 'NX', 'EX', cool_down_expiry_seconds)
if not set_cooldown_result then
    return {1, current_phone_count, current_device_count}
end

current_phone_count = tonumber(redis.call('GET', phone_daily_count_key)) or 0
if current_phone_count >= phone_daily_limit then
    return {2, current_phone_count, current_device_count}
end

if device_daily_count_key ~= '' then
    current_device_count = tonumber(redis.call('GET', device_daily_count_key)) or 0
    if current_device_count >= device_daily_limit then
        return {3, current_phone_count, current_device_count}
    end
end

current_phone_count = redis.call('INCR', phone_daily_count_key)
if current_phone_count == 1 then
    redis.call('EXPIRE', phone_daily_count_key, daily_count_expiry_seconds)
end

if device_daily_count_key ~= '' then
    current_device_count = redis.call('INCR', device_daily_count_key)
    if current_device_count == 1 then
        redis.call('EXPIRE', device_daily_count_key, daily_count_expiry_seconds)
    end
end

return {0, current_phone_count, current_device_count}
"#;
