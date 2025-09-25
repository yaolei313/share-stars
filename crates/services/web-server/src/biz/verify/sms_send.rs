use crate::biz::security::SmsStatistic;
use crate::biz::verify::code::CodeManager;
use crate::config::{Env, SmsSetting};
use crate::http::vo::error::AppError;
use crate::http::vo::sms::SmsType;
use crate::http::vo::{AppResult, RequestInfo};
use std::sync::Arc;
use twilio::OutboundMessage;

pub struct SmsService {
    env: Env,
    code_manager: Arc<CodeManager>,
    sms_statistic: Arc<SmsStatistic>,
    twilio_client: twilio::Client,
    from: String,
    status_callback_url: String,
}

impl SmsService {
    pub fn new(
        env: Env,
        code_manager: Arc<CodeManager>,
        sms_statistic: Arc<SmsStatistic>,
        sms_setting: &SmsSetting,
    ) -> Self {
        Self {
            env,
            code_manager,
            sms_statistic,
            twilio_client: twilio::Client::new(&sms_setting.account_sid, &sms_setting.auth_token),
            from: sms_setting.from_phone.clone(),
            status_callback_url: sms_setting.status_callback_url.clone(),
        }
    }

    pub async fn send_sms_code(
        &self,
        e164_phone: &str,
        sms_type: &SmsType,
        req_info: &RequestInfo,
    ) -> AppResult<()> {
        self.sms_statistic
            .check_and_incr_send_sms_count(&e164_phone, &sms_type, &req_info)
            .await?;

        let key = format!("str:sms-code:{}.{}", sms_type.code(), e164_phone);
        let otp = self.code_manager.gen_code(&key, 60).await?;
        let message = format!("您的验证码为：{}, 请勿告知他人。", otp);

        if Env::DEV == self.env || Env::TEST == self.env {
            log::info!(
                "offline env do not send verification sms. {} {}",
                e164_phone,
                message
            );
            return Ok(());
        }

        self.twilio_client
            .send_message(OutboundMessage::new(&self.from, e164_phone, &message))
            .await?;

        Ok(())
    }

    pub async fn validate_sms_code(
        &self,
        e164_phone: &str,
        sms_type: &SmsType,
        sms_code: &str,
    ) -> AppResult<()> {
        if lib_utils::is_test_phone_number(e164_phone) {
            validate_test_code(e164_phone, sms_type, sms_code).await
        } else {
            let key = format!("str:sms-code:{}.{}", sms_type.code(), e164_phone);
            self.code_manager.validate_code(&key, sms_code).await
        }
    }
}

async fn validate_test_code(
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

// 测试手机号，短信验证码登录时使用固定的验证码
const TEST_LOGIN_CODE: &'static str = "151488";

//测试手机号，短信验证码绑定时使用固定的验证码
const TEST_BIND_PHONE_CODE: &'static str = "151489";

//测试手机号，短信验证码重置密码时使用固定的验证码
const TEST_RESET_PWD_CODE: &'static str = "151490";
