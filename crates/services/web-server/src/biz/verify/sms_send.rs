use crate::biz::verify::code::CodeManager;
use crate::config::{Env, SmsSetting};
use crate::http::vo::AppResult;
use crate::http::vo::sms::SmsType;
use std::sync::Arc;
use twilio::OutboundMessage;

pub struct SmsService {
    env: Env,
    code_manager: Arc<CodeManager>,
    twilio_client: twilio::Client,
    from: String,
    status_callback_url: String,
}

impl SmsService {
    pub fn new(env: Env, code_manager: Arc<CodeManager>, sms_setting: &SmsSetting) -> Self {
        Self {
            env,
            code_manager,
            twilio_client: twilio::Client::new(&sms_setting.account_sid, &sms_setting.auth_token),
            from: sms_setting.from_phone.clone(),
            status_callback_url: sms_setting.status_callback_url.clone(),
        }
    }

    pub async fn send_verification_sms(
        &self,
        e164_phone: &str,
        sms_type: &SmsType,
    ) -> AppResult<()> {
        let otp = self.code_manager.gen_code(e164_phone, sms_type).await?;
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
}
