use crate::biz::email::EmailSender;
use crate::biz::sms::SmsSender;
use crate::biz::statistic::VerifyStatistic;
use crate::biz::verify::{CodeManager, VerifyScenario};
use crate::http::vo::error::AppError;
use crate::http::vo::{AppResult, RequestInfo};
use std::collections::HashMap;
use std::sync::Arc;

pub struct VerifyManager {
    verify_statistic: VerifyStatistic,
    code_manager: Arc<CodeManager>,
    sms_sender: Arc<SmsSender>,
    email_sender: Arc<EmailSender>,
}

impl VerifyManager {
    pub fn new(
        redis_client: Arc<redis::Client>,
        code_manager: Arc<CodeManager>,
        sms_sender: Arc<SmsSender>,
        email_sender: Arc<EmailSender>,
    ) -> Self {
        Self {
            verify_statistic: VerifyStatistic::new(redis_client.clone()),
            code_manager,
            sms_sender,
            email_sender,
        }
    }

    pub async fn send_sms_code(
        &self,
        e164_phone: &str,
        scenario: VerifyScenario,
        req_info: &RequestInfo,
    ) -> AppResult<()> {
        self.verify_statistic
            .check_and_incr_count(e164_phone, scenario, &req_info)
            .await?;

        let key = self.gen_key(e164_phone, scenario);
        let otp = self.code_manager.gen_code(&key, 60).await?;

        let map = HashMap::from([("code".to_string(), otp)]);
        self.sms_sender
            .send_by_template(e164_phone, SMS_TEMPLATE_ID, &map)
            .await
    }

    pub async fn send_email_code(
        &self,
        email: &str,
        scenario: VerifyScenario,
        req_info: &RequestInfo,
    ) -> AppResult<()> {
        self.verify_statistic
            .check_and_incr_count(email, scenario, &req_info)
            .await?;

        let key = self.gen_key(email, scenario);
        let otp = self.code_manager.gen_code(&key, 60).await?;

        let map = HashMap::from([("code", otp)]);
        self.email_sender
            .send_by_template(email, EMAIL_TEMPLATE_ID, &map)
            .await
    }

    pub async fn verify_sms_code(
        &self,
        e164_phone: &str,
        scenario: VerifyScenario,
        code: &str,
        req_info: &RequestInfo,
    ) -> AppResult<()> {
        if lib_utils::is_test_phone_number(e164_phone) {
            log::debug!("Using test phone number. Skipping Redis verification.");
            validate_test_code(scenario, code).await
        } else {
            let key = self.gen_key(e164_phone, scenario);
            self.code_manager.validate_code(&key, code).await
        }
    }

    pub async fn verify_email_code(
        &self,
        email: &str,
        scenario: VerifyScenario,
        code: &str,
        req_info: &RequestInfo,
    ) -> AppResult<()> {
        if lib_utils::is_test_email(email) {
            validate_test_code(scenario, code).await
        } else {
            let key = self.gen_key(email, scenario);
            self.code_manager.validate_code(&key, code).await
        }
    }

    fn gen_key(&self, identifier: &str, scenario: VerifyScenario) -> String {
        format!("str:code:{}.{}", scenario.code(), identifier)
    }
}

const SMS_TEMPLATE_ID: i64 = 1;

const EMAIL_TEMPLATE_ID: i64 = 1;

async fn validate_test_code(scenario: VerifyScenario, input_code: &str) -> AppResult<()> {
    let target_code = match scenario {
        VerifyScenario::Login => TEST_LOGIN_CODE,
        VerifyScenario::BindPhone => TEST_BIND_PHONE_CODE,
        VerifyScenario::ResetPwd => TEST_RESET_PWD_CODE,
        VerifyScenario::Mfa => TEST_MFA_CHALLENGE_CODE,
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

const TEST_MFA_CHALLENGE_CODE: &'static str = "151491";
