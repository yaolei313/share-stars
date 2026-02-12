use crate::db::models::{EmailTemplate, SmsTemplate};
use crate::db::repositories::{
    EmailTemplateRepository, PgEmailTemplateRepository, PgSmsTemplateRepository,
    SmsTemplateRepository,
};
use crate::db::RepositoryState;
use crate::db::SqlxResult;
use std::sync::Arc;

pub struct TemplateDbService {
    sms_template_repo: Arc<PgSmsTemplateRepository>,
    email_template_repo: Arc<PgEmailTemplateRepository>,
}
impl TemplateDbService {
    pub fn new(repository_state: Arc<RepositoryState>) -> Self {
        Self {
            sms_template_repo: repository_state.sms_template_repo.clone(),
            email_template_repo: repository_state.email_template_repo.clone(),
        }
    }

    pub async fn query_sms_template_by_id(&self, id: i64) -> SqlxResult<Option<SmsTemplate>> {
        self.sms_template_repo.find_by_id(id).await
    }

    pub async fn query_email_template_by_id(&self, id: i64) -> SqlxResult<Option<EmailTemplate>> {
        self.email_template_repo.find_by_id(id).await
    }
}
