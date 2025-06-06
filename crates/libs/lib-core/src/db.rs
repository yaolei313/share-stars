pub mod models;
pub mod repositories;

use crate::db::models::User;
use sqlx::{Executor, PgPool};

pub struct DbClient {
    pool: PgPool,
}

impl DbClient {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

type SqlxResult<T> = Result<T, sqlx::Error>;

pub trait UserExt {
    async fn add_user(&self, user: User) -> SqlxResult<User>;

    async fn update_user(&self, user: User) -> SqlxResult<()>;

    async fn update_user_password(&self, user_id: i64, password: String) -> SqlxResult<()>;

    async fn get_user(&self, user_id: i64) -> SqlxResult<Option<User>>;
}

impl UserExt for DbClient {
    async fn add_user(&self, user: User) -> SqlxResult<User> {
        let mut conn = self.pool.acquire().await?;
        todo!()
    }

    async fn update_user(&self, user: User) -> SqlxResult<()> {
        todo!()
    }

    async fn update_user_password(&self, user_id: i64, password: String) -> SqlxResult<()> {
        todo!()
    }

    async fn get_user(&self, user_id: i64) -> SqlxResult<Option<User>> {
        let mut conn = self.pool.acquire().await?;
        let record = sqlx::query_as::<_, User>(r#"select * from users where user_id = ?"#)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(record)
    }
}
