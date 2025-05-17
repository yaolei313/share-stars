use sqlx::PgPool;

mod db;
pub mod dtos;


#[derive(Debug)]
pub struct AppState {
    pub db_pool: PgPool,
    pub jwt_secret: String,
    pub jwt_max_age: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
