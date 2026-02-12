use crate::db::models::SmsTemplate;
use crate::db::SqlxResult;
use sqlx::PgExecutor;
use std::fmt::Debug;

pub trait SmsTemplateRepository: Send + Sync + Debug {
    fn find_by_id(&self, id: i64) -> impl Future<Output = SqlxResult<Option<SmsTemplate>>> + Send;

    fn insert<'c, E>(
        &self,
        executor: E,
        template: &SmsTemplate,
    ) -> impl Future<Output = SqlxResult<()>> + Send
    where
        E: PgExecutor<'c>;
}
