use crate::db::models::EmailTemplate;
use crate::db::repositories::EmailTemplateRepository;
use crate::db::SqlxResult;
use sqlx::{PgExecutor, PgPool};
use std::fmt::Debug;

#[derive(Debug)]
pub struct PgEmailTemplateRepository {
    pool: PgPool,
}

impl PgEmailTemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl EmailTemplateRepository for PgEmailTemplateRepository {
    async fn find_by_id(&self, id: i64) -> SqlxResult<Option<EmailTemplate>> {
        sqlx::query_as::<_, EmailTemplate>("SELECT * FROM email_template WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn insert<'c, E>(&self, executor: E, template: &EmailTemplate) -> SqlxResult<()>
    where
        E: PgExecutor<'c>,
    {
        sqlx::query("insert into email_template (template_key, language_code, subject, content, template_type, sender_name, enabled) values ($1,$2,$3,$4,$5,$6,$7)")
            .bind(&template.template_key)
            .bind(&template.language_code)
            .bind(&template.subject)
            .bind(&template.content)
            .bind(&template.template_type)
            .bind(&template.sender_name)
            .bind(&template.enabled)
            .execute(executor)
            .await?;
        Ok(())
    }
}
