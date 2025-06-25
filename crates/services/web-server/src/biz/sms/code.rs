use crate::http::vo::error::AppError;
use crate::http::vo::sms::SmsType;
use crate::http::vo::AppResult;
use rand::Rng;
use redis::{AsyncCommands, SetExpiry, SetOptions};
use std::sync::Arc;

pub struct CodeGenerator {
    redis_client: Arc<redis::Client>,
}

const VALIDATE_AND_DELETE_SCRIPT: &str = r#"
local stored_code = redis.call('GET', KEYS[1])
if stored_code == ARGV[1] then
  redis.call('DEL', KEYS[1])
  return 1
else
  return 0
end
"#;

impl CodeGenerator {
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Self { redis_client }
    }

    pub async fn gen_code(&self, e164_phone: &str, sms_type: &SmsType) -> AppResult<String> {
        let mut rng = rand::rng();
        // let r = 100_000..=999_999;
        let random_number: u32 = rng.random_range(100_000..=999_999);
        let val = random_number.to_string();

        // 保存到redis中
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let key = format!("str:sms-code:{}.{}", sms_type.code(), e164_phone);
        let options = SetOptions::default().with_expiration(SetExpiry::EX(60));
        let _: () = conn.set_options(&key, &val, options).await?;

        Ok(val)
    }

    pub async fn validate_code(
        &self,
        e164_phone: &str,
        sms_type: &SmsType,
        input_code: &str,
    ) -> AppResult<()> {
        if input_code.len() != 6 {
            return Err(AppError::InvalidSmsCode);
        }
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = format!("str:sms-code:{}.{}", sms_type.code(), e164_phone);

        let script = redis::Script::new(VALIDATE_AND_DELETE_SCRIPT);
        let result: i32 = script
            .key(&key)
            .arg(input_code)
            .invoke_async(&mut conn)
            .await?;

        if result == 1 {
            Ok(())
        } else {
            Err(AppError::InvalidSmsCode)
        }
    }
}
