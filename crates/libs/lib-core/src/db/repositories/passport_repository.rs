use crate::db::models::Passport;
use std::fmt::Debug;

pub trait PassportRepository: Send + Sync + Debug {
    fn by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Option<Passport>, sqlx::Error>> + Send;
}
