use crate::db::models::{NewPhoneMapping, PhoneMapping};
use crate::db::repositories::phone_mapping_repository::PhoneMappingRepository;
use sqlx::PgPool;
use tracing::log;

#[derive(Debug)]
pub struct PgPhoneMappingRepository {
    pool: PgPool,
}

impl PgPhoneMappingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PhoneMappingRepository for PgPhoneMappingRepository {
    async fn by_phone(&self, phone: &str) -> Result<Option<PhoneMapping>, sqlx::Error> {
        let mapping = sqlx::query_as!(
            PhoneMapping,
            "select * from phone_mapping where phone = $1",
            phone
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(mapping)
    }

    async fn insert(&self, mapping: NewPhoneMapping) -> Result<(), sqlx::Error> {
        let id = sqlx::query_as::<_, (i64,)>(
            r#"insert into phone_mapping (user_id, phone) values ($1, $2) returning id"#,
        )
        .bind(mapping.user_id)
        .bind(mapping.phone)
        .fetch_one(&self.pool)
        .await?;
        log::info!("inserted phone_mapping (id: {:?})", id);
        Ok(())
    }
}
