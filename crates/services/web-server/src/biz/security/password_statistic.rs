use crate::config::JwtSetting;
use crate::http::vo::error::AppError;
use crate::http::vo::AppResult;
use chrono::{Datelike, Utc};
use lib_utils::{JwtDelegate, ONE_DAY_SECONDS};
use redis::AsyncTypedCommands;
use std::sync::Arc;

pub struct PasswordStatisticService {
    redis_client: Arc<redis::Client>,
    jwt_delegate: JwtDelegate,
}

impl PasswordStatisticService {
    pub fn new(redis_client: Arc<redis::Client>, settings: &JwtSetting) -> anyhow::Result<Self> {
        let jwt_delegate = JwtDelegate::new(&settings.keys)?;
        Ok(Self {
            redis_client,
            jwt_delegate,
        })
    }

    pub async fn is_exceed_password_error_limit(&self, user_id: i64) -> AppResult<()> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = gen_key(user_id);
        let val = conn.get(key).await?;
        if let Some(val) = val {
            let count = val.parse::<i32>().unwrap_or_else(|_e| {
                log::warn!("invalid val.{}", val);
                0
            });
            if count >= MAX_FAIL_COUNT_ONE_DAY {
                log::warn!("too many incorrect password attempts. {}", user_id);
                return Err(AppError::TooManyIncorrectPasswordAttempts);
            }
        }
        Ok(())
    }

    pub async fn add_password_error_count(&self, user_id: i64) -> AppResult<()> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = gen_key(user_id);

        let _ = conn.incr(&key, 1).await?;
        let _ = conn.expire(&key, ONE_DAY_SECONDS).await?;
        Ok(())
    }
}

fn gen_key(user_id: i64) -> String {
    let day = Utc::now().naive_local().day();
    format!("int:pwd-fail-count:{}.{}", user_id, day)
}

const MAX_FAIL_COUNT_ONE_DAY: i32 = 4;
