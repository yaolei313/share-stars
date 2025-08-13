use crate::db::models::LookupDevice;
use sqlx::{PgExecutor, Result as SqlxResult};
use std::fmt::Debug;

pub trait LookupDeviceRepository: Send + Sync + Debug {
    fn find_by_fingerprint(
        &self,
        stable_fingerprint_hash: &str,
    ) -> impl Future<Output = SqlxResult<LookupDevice>> + Send;

    fn insert<'c, E>(
        &self,
        executor: E,
        lookup_device: &LookupDevice,
    ) -> impl Future<Output = SqlxResult<()>> + Send
    where
        E: PgExecutor<'c>;
}
