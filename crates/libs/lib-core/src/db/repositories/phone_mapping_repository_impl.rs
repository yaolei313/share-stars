use crate::db::models::PhoneMapping;
use crate::db::repositories::phone_mapping_repository::PhoneMappingRepository;
use sqlx::PgPool;

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
}
