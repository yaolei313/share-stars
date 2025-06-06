use crate::db::models::PhoneMapping;
use anyhow::Result;

pub trait PhoneMappingRepository {
    async fn by_phone(&self, phone: &str) -> Result<Option<PhoneMapping>>;
}
