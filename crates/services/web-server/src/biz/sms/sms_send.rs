use crate::config::SmsSetting;

pub struct SmsSendService {
    url: String,
    account_sid: String,
    auth_token: String,
    status_callback_url: String,
}

impl SmsSendService {
    pub fn new(sms_setting: &SmsSetting) -> Self {
        Self {
            url: sms_setting.url.clone(),
            account_sid: sms_setting.account_sid.clone(),
            auth_token: sms_setting.auth_token.clone(),
            status_callback_url: sms_setting.status_callback_url.clone(),
        }
    }
}
