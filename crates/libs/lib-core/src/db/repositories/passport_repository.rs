use crate::db::models::Passport;
use anyhow::Result;

pub trait PassportRepository {
    async fn by_user_id(&self, user_id: i64) -> Result<Option<Passport>>;
}
