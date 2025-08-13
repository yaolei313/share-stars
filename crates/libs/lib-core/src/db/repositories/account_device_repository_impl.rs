use crate::db::models::AccountDevice;
use crate::db::repositories::account_device_repository::AccountDeviceRepository;
use sqlx::{PgExecutor, PgPool, Result as SqlxResult};

#[derive(Debug)]
pub struct PgAccountDeviceRepository {
    pool: PgPool,
}

impl PgAccountDeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AccountDeviceRepository for PgAccountDeviceRepository {
    async fn find_by_user_id_device_id(
        &self,
        user_id: i64,
        device_id: &str,
    ) -> SqlxResult<Option<AccountDevice>> {
        sqlx::query_as::<_, AccountDevice>(
            "SELECT * FROM account_device WHERE user_id = $1 AND device_id = $2",
        )
        .bind(user_id)
        .bind(device_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_user_id(&self, user_id: i64) -> SqlxResult<Vec<AccountDevice>> {
        sqlx::query_as::<_, AccountDevice>(
            "SELECT * FROM account_device WHERE user_id = $1 order by last_login_at desc",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn insert<'c, E>(&self, executor: E, account_device: &AccountDevice) -> SqlxResult<()>
    where
        E: PgExecutor<'c>,
    {
        sqlx::query("insert into account_device (user_id, device_id, last_login_ip, last_login_method, last_login_at, nickname, trusted_at, expires_at, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(account_device.user_id)
            .bind(&account_device.device_id)
            .bind(account_device.last_login_ip)
            .bind(account_device.last_login_method)
            .bind(account_device.last_login_at)
            .bind(&account_device.nickname)
            .bind(account_device.trusted_at)
            .bind(account_device.expires_at)
            .bind(account_device.created_at)
            .bind(account_device.updated_at)
            .fetch_one(executor).await?;
        Ok(())
    }
}
