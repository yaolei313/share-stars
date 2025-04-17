#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_max_age: u64,
    pub port: u16,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenv::from_filename(".env").ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_max_age: u64 = std::env::var("JWT_MAX_AGE")
            .map(|v| v.parse().expect("JWT_MAX_AGE must be u64"))
            .expect("JWT_MAX_AGE must be set");
        let port: u16 = std::env::var("PORT").map_or_else(
            |_| 8080,
            |port| port.parse::<u16>().expect("PORT must be u16"),
        );
        AppConfig {
            database_url,
            jwt_secret,
            jwt_max_age,
            port,
        }
    }
}
