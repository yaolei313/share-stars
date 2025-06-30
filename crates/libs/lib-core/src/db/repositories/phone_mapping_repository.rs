use crate::db::models::PhoneMapping;
use std::fmt::Debug;

pub trait PhoneMappingRepository: Send + Sync + Debug {
    fn by_phone(
        &self,
        phone: &str,
    ) -> impl Future<Output = Result<Option<PhoneMapping>, sqlx::Error>> + Send;
}
