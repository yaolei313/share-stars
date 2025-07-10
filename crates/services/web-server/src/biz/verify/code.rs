use crate::http::vo::error::AppError;
use crate::http::vo::sms::SmsType;
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

    pub async fn gen_code(&self, e164_phone: &str, sms_type: &SmsType) -> AppResult<String> {
        // 避免
        let val = lib_utils::rand_verify_code();

        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        //
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
            log::info!("Invalid code length: {}", input_code);
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
            log::info!("not matched code: {}", input_code);
            Ok(())
        } else {
            Err(AppError::InvalidSmsCode)
        }
    }

    pub async fn validate_test_code(
        &self,
        _e164_phone: &str,
        sms_type: &SmsType,
        input_code: &str,
    ) -> AppResult<()> {
        let target_code = match sms_type {
            SmsType::Login => TEST_LOGIN_CODE,
            SmsType::BindPhone => TEST_BIND_PHONE_CODE,
            SmsType::ResetPwd => TEST_RESET_PWD_CODE,
        };
        if target_code == input_code {
            Ok(())
        } else {
            Err(AppError::InvalidSmsCode)
        }
    }
}

// 测试手机号，短信验证码登录时使用固定的验证码
const TEST_LOGIN_CODE: &'static str = "151488";

//测试手机号，短信验证码绑定时使用固定的验证码
const TEST_BIND_PHONE_CODE: &'static str = "151489";

//测试手机号，短信验证码重置密码时使用固定的验证码
const TEST_RESET_PWD_CODE: &'static str = "151490";
