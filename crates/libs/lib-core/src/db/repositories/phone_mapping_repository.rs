use crate::db::models::{NewPhoneMapping, PhoneMapping};
use std::fmt::Debug;

pub trait PhoneMappingRepository: Send + Sync + Debug {
    async fn by_phone(&self, phone: &str) -> Result<Option<PhoneMapping>, sqlx::Error>;

    async fn insert(&self, mapping: NewPhoneMapping) -> Result<(), sqlx::Error>;
}
