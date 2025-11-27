use crate::biz::sms::sms_template::SmsTemplateManager;
use crate::config::{Env, SmsSetting};
use crate::http::vo::AppResult;
use std::collections::HashMap;
use std::sync::Arc;
use twilio::OutboundMessage;

pub struct SmsSender {
    env: Env,
    sms_template_service: Arc<SmsTemplateManager>,
    twilio_client: twilio::Client,
    from: String,
    status_callback_url: String,
}

impl SmsSender {
    pub fn new(
        env: Env,
        sms_template_service: Arc<SmsTemplateManager>,
        sms_setting: &SmsSetting,
    ) -> Self {
        Self {
            env,
            sms_template_service,
            twilio_client: twilio::Client::new(&sms_setting.account_sid, &sms_setting.auth_token),
            from: sms_setting.from_phone.clone(),
            status_callback_url: sms_setting.status_callback_url.clone(),
        }
    }

    pub async fn send_by_template(
        &self,
        e164_phone: &str,
        template_key: i64,
        template_params: &HashMap<String, String>,
    ) -> AppResult<()> {
        let message = self
            .sms_template_service
            .render_template(template_key, template_params)
            .await?;

        if Env::DEV == self.env || Env::TEST == self.env {
            tracing::info!(
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
