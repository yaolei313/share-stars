use crate::db::models::AccountIdentity;
use sqlx::{PgExecutor, Result as SqlxResult};
use std::fmt::Debug;

pub trait AccountIdentityRepository: Send + Sync + Debug {
    fn list_by_user_id(
        &self,
        user_id: &str,
    ) -> impl Future<Output = SqlxResult<Vec<AccountIdentity>>> + Send;

    fn find_by_user_id_provider(
        &self,
        user_id: &str,
        provider: i32,
    ) -> impl Future<Output = SqlxResult<Option<AccountIdentity>>> + Send;

    fn insert<'c, E>(
        &self,
        executor: E,
        identity: AccountIdentity,
    ) -> impl Future<Output = SqlxResult<()>> + Send
    where
        E: PgExecutor<'c>;
}
