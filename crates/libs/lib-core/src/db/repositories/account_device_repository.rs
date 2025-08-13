use crate::db::models::AccountDevice;
use sqlx::PgExecutor;
use sqlx::Result as SqlxResult;
use std::fmt::Debug;

pub trait AccountDeviceRepository: Send + Sync + Debug {
    fn find_by_user_id_device_id(
        &self,
        user_id: i64,
        device_id: &str,
    ) -> impl Future<Output = SqlxResult<Option<AccountDevice>>> + Send;

    fn find_by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = SqlxResult<Vec<AccountDevice>>> + Send;

    fn insert<'c, E>(
        &self,
        executor: E,
        device: &AccountDevice,
    ) -> impl Future<Output = SqlxResult<()>> + Send
    where
        E: PgExecutor<'c>;
}
