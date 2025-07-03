use crate::db::models::{NewPassport, Passport};
use std::fmt::Debug;

pub trait PassportRepository: Send + Sync + Debug {
    // fn by_user_id(
    //     &self,
    //     user_id: i64,
    // ) -> impl Future<Output = Result<Option<Passport>, sqlx::Error>> + Send;

    async fn by_user_id(&self, user_id: i64) -> Result<Option<Passport>, sqlx::Error>;

    async fn insert(&self, passport: NewPassport) -> Result<(), sqlx::Error>;
}
