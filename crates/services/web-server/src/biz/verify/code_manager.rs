use crate::http::vo::error::AppError;
use crate::http::vo::AppResult;
use redis::{AsyncCommands, SetExpiry, SetOptions};
use std::sync::Arc;

pub struct CodeManager {
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

impl CodeManager {
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Self { redis_client }
    }

    pub async fn gen_code(&self, key: &str, expiration_seconds: u64) -> AppResult<String> {
        let val = lib_utils::rand_verify_code();

        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        //
        let options = SetOptions::default().with_expiration(SetExpiry::EX(expiration_seconds));
        let _: () = conn.set_options(key, &val, options).await?;

        Ok(val)
    }

    pub async fn validate_code(&self, key: &str, input_code: &str) -> AppResult<()> {
        if input_code.len() != 6 {
            tracing::info!("invalid verification code length: {}", input_code);
            return Err(AppError::InvalidSmsCode);
        }

        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;

        let script = redis::Script::new(VALIDATE_AND_DELETE_SCRIPT);
        let result: i32 = script
            .key(key)
            .arg(input_code)
            .invoke_async(&mut conn)
            .await?;

        if result == 1 {
            Ok(())
        } else {
            tracing::warn!("invalid verification code: {}", input_code);
            Err(AppError::InvalidSmsCode)
        }
    }
}
