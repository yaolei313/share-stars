use crate::db::models::Device;
use crate::db::repositories::device_repository::DeviceRepository;
use sqlx::{PgExecutor, PgPool, Result as SqlxResult};
use std::fmt::Debug;

#[derive(Debug)]
pub struct PgDeviceRepository {
    pool: PgPool,
}

impl PgDeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl DeviceRepository for PgDeviceRepository {
    async fn find_by_device_id(&self, device_id: &str) -> SqlxResult<Option<Device>> {
        sqlx::query_as::<_, Device>("SELECT * FROM device WHERE device_id = $1")
            .bind(device_id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn insert<'c, E>(&self, executor: E, device: &Device) -> SqlxResult<()>
    where
        E: PgExecutor<'c>,
    {
        sqlx::query("insert into device (device_id, full_fingerprint, platform_type, created_at, updated_at) values ($1, $2, $3, $4, $5)")
            .bind(&device.device_id)
            .bind(&device.full_fingerprint)
            .bind(device.platform_type)
            .bind(device.created_at)
            .bind(device.updated_at)
            .fetch_one(executor)
            .await?;
        Ok(())
    }
}
