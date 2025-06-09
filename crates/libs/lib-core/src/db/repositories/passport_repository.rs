use crate::db::models::Passport;
use std::fmt::Debug;

pub trait PassportRepository: Send + Sync + Debug {
    async fn by_user_id(&self, user_id: i64) -> Result<Option<Passport>, sqlx::Error>;
}
