use crate::db::models::EmailTemplate;
use crate::db::SqlxResult;
use sqlx::PgExecutor;
use std::fmt::Debug;

pub trait EmailTemplateRepository: Send + Sync + Debug {
    fn find_by_id(&self, id: i64)
    -> impl Future<Output = SqlxResult<Option<EmailTemplate>>> + Send;

    fn insert<'c, E>(
        &self,
        executor: E,
        template: &EmailTemplate,
    ) -> impl Future<Output = SqlxResult<()>> + Send
    where
        E: PgExecutor<'c>;
}
