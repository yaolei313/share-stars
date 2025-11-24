use crate::http::vo::error::AppError;
use crate::http::vo::AppResult;
use std::borrow::Cow;
use std::collections::HashMap;
use strfmt::strfmt;

pub struct SmsTemplateManager {}

impl SmsTemplateManager {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn render_template(
        &self,
        template_id: i64,
        template_params: &HashMap<String, String>,
    ) -> AppResult<String> {
        if template_id == 1 {
            let body = "您的验证码为：{code}, 请勿告知他人。";
            let message = strfmt(body, template_params)
                .map_err(|e| AppError::TemplateError(Cow::Owned(e.to_string())))?;
            Ok(message)
        } else {
            Err(AppError::TemplateError(Cow::Borrowed(
                "template not found!",
            )))
        }
    }
}
