use crate::config::SmsSetting;
use crate::http::vo::AppResult;
use twilio::{Client, OutboundMessage};

pub struct SmsService {
    client: Client,
    from: String,
    status_callback_url: String,
}

impl SmsService {
    pub fn new(sms_setting: &SmsSetting) -> Self {
        Self {
            client: Client::new(&sms_setting.account_sid, &sms_setting.auth_token),
            from: sms_setting.from_phone.clone(),
            status_callback_url: sms_setting.status_callback_url.clone(),
        }
    }

    pub async fn send(&self, to: &str, message: &str) -> AppResult<()> {
        let msg = self
            .client
            .send_message(OutboundMessage::new(&self.from, to, message))
            .await?;
        log::info!("send sms {:?}", msg);
        Ok(())
    }
}
