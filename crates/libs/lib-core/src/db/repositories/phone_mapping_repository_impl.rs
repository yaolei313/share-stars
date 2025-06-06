use crate::db::models::PhoneMapping;
use crate::db::repositories::phone_mapping_repository::PhoneMappingRepository;
use sqlx::PgPool;

pub struct PgPhoneMappingRepository {
    pool: PgPool,
}

impl PhoneMappingRepository for PgPhoneMappingRepository {
    async fn by_phone(&self, phone: &str) -> anyhow::Result<Option<PhoneMapping>> {
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
