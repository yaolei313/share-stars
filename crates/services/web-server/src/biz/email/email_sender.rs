use crate::config::{EmailSettings, Env};
use crate::http::vo::AppResult;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

pub struct EmailSender {
    env: Env,
    email_setting: Arc<EmailSettings>,
}

impl EmailSender {
    pub fn new(env: Env, email_setting: Arc<EmailSettings>) -> Result<Self> {
        Ok(Self { env, email_setting })
    }

    pub async fn send_by_template(
        &self,
        to: &str,
        template_key: i64,
        template_params: &HashMap<&'static str, String>,
    ) -> AppResult<()> {
        todo!()
    }
}
