use crate::db::models::Account;
use sqlx::{PgExecutor, Result as SqlxResult};
use std::fmt::Debug;

pub trait AccountRepository: Send + Sync + Debug {
    fn find_by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = SqlxResult<Option<Account>>> + Send;

    fn insert<'c, E>(
        &self,
        executor: E,
        account: Account,
    ) -> impl Future<Output = SqlxResult<()>> + Send
    where
        E: PgExecutor<'c>;
}
