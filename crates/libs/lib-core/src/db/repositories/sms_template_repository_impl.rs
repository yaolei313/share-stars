use crate::db::models::SmsTemplate;
use crate::db::repositories::SmsTemplateRepository;
use crate::db::SqlxResult;
use sqlx::{PgExecutor, PgPool};
use std::fmt::Debug;

#[derive(Debug)]
pub struct PgSmsTemplateRepository {
    pool: PgPool,
}

impl PgSmsTemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SmsTemplateRepository for PgSmsTemplateRepository {
    async fn find_by_id(&self, id: i64) -> SqlxResult<Option<SmsTemplate>> {
        sqlx::query_as::<_, SmsTemplate>("SELECT * FROM sms_template WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn insert<'c, E>(&self, executor: E, template: &SmsTemplate) -> SqlxResult<()>
    where
        E: PgExecutor<'c>,
    {
        sqlx::query("insert into sms_template (template_key, language_code, content, template_type, enabled) values ($1,$2,$3,$4,$5)")
            .bind(&template.template_key)
            .bind(&template.language_code)
            .bind(&template.content)
            .bind(&template.template_type)
            .bind(&template.enabled)
            .execute(executor)
            .await?;
        Ok(())
    }
}
