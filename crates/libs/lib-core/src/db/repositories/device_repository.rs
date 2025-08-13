use crate::db::models::Device;
use sqlx::{PgExecutor, Result as SqlxResult};
use std::fmt::Debug;

pub trait DeviceRepository: Send + Sync + Debug {
    fn find_by_device_id(
        &self,
        device_id: &str,
    ) -> impl Future<Output = SqlxResult<Option<Device>>> + Send;

    fn insert<'c, E>(
        &self,
        executor: E,
        device: &Device,
    ) -> impl Future<Output = SqlxResult<()>> + Send
    where
        E: PgExecutor<'c>;
}
