use crate::biz::sms::code::CodeGenerator;
use crate::config::{AppState, SmsSetting};
use crate::http::vo::sms::SmsType;
use crate::http::vo::AppResult;
use std::sync::Arc;
use twilio::OutboundMessage;

pub struct SmsService {
    code_generator: Arc<CodeGenerator>,
    twilio_client: twilio::Client,
    from: String,
    status_callback_url: String,
}

impl SmsService {
    pub fn new(code_generator: Arc<CodeGenerator>, sms_setting: &SmsSetting) -> Self {
        Self {
            code_generator,
            twilio_client: twilio::Client::new(&sms_setting.account_sid, &sms_setting.auth_token),
            from: sms_setting.from_phone.clone(),
            status_callback_url: sms_setting.status_callback_url.clone(),
        }
    }

    pub async fn send_verification_code(
        &self,
        e164_phone: &str,
        sms_type: &SmsType,
    ) -> AppResult<()> {
        let otp = self.code_generator.gen_code(e164_phone, sms_type).await?;
        let message = format!("您的验证码为：{}, 请勿告知他人。", otp);

        let msg = self
            .twilio_client
            .send_message(OutboundMessage::new(&self.from, e164_phone, &message))
            .await?;
        log::info!("send sms {:?}", msg);
        Ok(())
    }
}
