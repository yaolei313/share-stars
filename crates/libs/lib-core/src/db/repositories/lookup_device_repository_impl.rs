use crate::db::models::LookupDevice;
use crate::db::repositories::lookup_device_repository::LookupDeviceRepository;
use sqlx::{PgExecutor, PgPool, Result as SqlxResult};

#[derive(Debug)]
pub struct PgLookupDeviceRepository {
    pool: PgPool,
}

impl PgLookupDeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl LookupDeviceRepository for PgLookupDeviceRepository {
    async fn find_by_fingerprint(&self, stable_fingerprint_hash: &str) -> SqlxResult<LookupDevice> {
        sqlx::query_as::<_, LookupDevice>(
            "SELECT * FROM lookup_device WHERE stable_fingerprint_hash = $1 ",
        )
        .bind(stable_fingerprint_hash)
        .fetch_one(&self.pool)
        .await
    }

    async fn insert<'c, E>(&self, executor: E, lookup_device: &LookupDevice) -> SqlxResult<()>
    where
        E: PgExecutor<'c>,
    {
        sqlx::query(
            "insert into lookup_device (stable_fingerprint_hash, device_id) values ($1,$2)",
        )
        .bind(&lookup_device.stable_fingerprint_hash)
        .bind(&lookup_device.device_id)
        .execute(executor)
        .await?;
        Ok(())
    }
}
