use crate::biz::dto::Identity;
use crate::biz::verify::VerifyScenario;
use crate::http::vo::error::AppError;
use crate::http::vo::{AppResult, RequestInfo};
use chrono::{Datelike, Utc};
use redis::Script;
use std::sync::Arc;

pub struct VerifyStatistic {
    redis_client: Arc<redis::Client>,
}

impl VerifyStatistic {
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Self { redis_client }
    }

    pub async fn check_and_incr_count(
        &self,
        identifier: &str,
        scenario: VerifyScenario,
        req_info: &RequestInfo,
    ) -> AppResult<()> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let day_of_month = Utc::now().naive_local().day();

        // Prepare keys for MGET
        let id_cool_down_key = gen_id_cool_down_duration_key(identifier, scenario);
        let id_daily_count_key = gen_today_id_send_count_key(identifier, scenario, day_of_month);
        let device_daily_count_key =
            gen_today_device_send_count_key(&req_info.device_id, scenario, day_of_month);
        let keys = vec![
            &id_cool_down_key,
            &id_daily_count_key,
            &device_daily_count_key,
        ];
        tracing::info!("keys: {:?}", keys);

        let script = Script::new(LUA_SCRIPT);

        // The invoke_async method automatically handles SCRIPT LOAD and EVALSHA for efficiency.
        let (id_cooldown_status, id_daily_count, device_daily_count): (i64, i64, i64) = script
            .key(keys)
            .arg(COOLDOWN_SECONDS)
            .arg(ID_DAILY_LIMIT)
            .arg(DEVICE_DAILY_LIMIT)
            .invoke_async(&mut conn)
            .await?;

        tracing::info!(
            "status: {} {} {}",
            id_cooldown_status,
            id_daily_count,
            device_daily_count
        );

        if id_cooldown_status == 0 {
            return Ok(());
        } else if id_cooldown_status == 1 {
            return Err(AppError::FrequencyExceed(scenario, COOLDOWN_SECONDS));
        } else if id_cooldown_status == 2 {
            return Err(AppError::DailyQuotaExceed(scenario, ID_DAILY_LIMIT));
        } else if id_cooldown_status == 3 {
            return Err(AppError::DeviceDailyQuotaExceed(
                scenario,
                DEVICE_DAILY_LIMIT,
            ));
        }

        Ok(())
    }
}

// 某段时间内(key的失效时间)内某种类型的短信只能发送一次
fn gen_id_cool_down_duration_key(identifier: &str, verify_type: VerifyScenario) -> String {
    format!(
        "{{verify:statistic:}}cool-down:{}-{}",
        identifier,
        verify_type.code()
    )
}

fn gen_today_id_send_count_key(
    identifier: &str,
    verify_type: VerifyScenario,
    day_of_mouth: u32,
) -> String {
    format!(
        "{{verify:statistic:}}today-count:{}-{}-{}",
        identifier,
        verify_type.code(),
        day_of_mouth
    )
}

fn gen_today_device_send_count_key(
    device_id: &str,
    verify_type: VerifyScenario,
    day_of_mouth: u32,
) -> String {
    format!(
        "{{verify:statistic:}}today-device:{}-{}-{}",
        device_id,
        verify_type.code(),
        day_of_mouth
    )
}

const COOLDOWN_SECONDS: i64 = 60;
const ID_DAILY_LIMIT: i32 = 10;
const DEVICE_DAILY_LIMIT: i32 = 20;

const LUA_SCRIPT: &str = r#"
local id_cool_down_key = KEYS[1]
local id_daily_count_key = KEYS[2]
local device_daily_count_key = KEYS[3]

local cool_down_expiry_seconds = tonumber(ARGV[1])
local id_daily_limit = tonumber(ARGV[2])
local device_daily_limit = tonumber(ARGV[3])

local current_id_daily_count = 0
local current_device_daily_count = 0

local set_cooldown_result = redis.call('SET', id_cool_down_key, '1', 'NX', 'EX', cool_down_expiry_seconds)
if not set_cooldown_result then
    return {1, current_id_daily_count, current_device_daily_count}
end

current_id_daily_count = tonumber(redis.call('GET', id_daily_count_key)) or 0
if current_id_daily_count >= id_daily_limit then
    return {2, current_id_daily_count, current_device_daily_count}
end

current_device_daily_count = tonumber(redis.call('GET', device_daily_count_key)) or 0
if current_device_daily_count >= device_daily_limit then
    return {3, current_id_daily_count, current_device_daily_count}
end

current_id_daily_count = redis.call('INCR', id_daily_count_key)
if current_id_daily_count == 1 then
    redis.call('EXPIRE', id_daily_count_key, 24 * 60 * 60)
end

current_device_daily_count = redis.call('INCR', device_daily_count_key)
if current_device_daily_count == 1 then
    redis.call('EXPIRE', device_daily_count_key, 24 * 60 * 60)
end

return {0, current_id_daily_count, current_device_daily_count}
"#;
