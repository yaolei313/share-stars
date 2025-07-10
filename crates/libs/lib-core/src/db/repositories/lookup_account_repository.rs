use crate::db::models::LookupAccount;
use sqlx::{PgExecutor, Result as SqlxResult};
use std::fmt::Debug;

pub trait LookupAccountRepository: Send + Sync + Debug {
    fn find_by_provider_identifier(
        &self,
        provider: i32,
        identifier: &str,
    ) -> impl Future<Output = SqlxResult<Option<LookupAccount>>> + Send;

    fn insert<'c, E>(
        &self,
        executor: E,
        lookup: LookupAccount,
    ) -> impl Future<Output = SqlxResult<()>> + Send
    where
        E: PgExecutor<'c>;
}
