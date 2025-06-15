use anyhow::Context;
use anyhow::Result;
use axum::extract::FromRef;
use config::{Config, File};
use lib_core::db::repositories::{PgPassportRepository, PgPhoneMappingRepository};
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct Server {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct Database {
    database_url: String,
    min_connections: u32,
    max_connections: u32,
    idle_timeout_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct Key {
    kid: String,
    public_key_path: String,
    private_key_path: String,
}
#[derive(Debug, Deserialize)]
struct Jwt {
    keys: Vec<Key>,
    issuer: String,
    expire_seconds: u64,
}

#[derive(Clone, Debug)]
pub enum Env {
    DEV,
    TEST,
    STAGING,
    PRODUCTION,
}

impl FromStr for Env {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Env::DEV),
            "test" => Ok(Env::TEST),
            "staging" => Ok(Env::STAGING),
            "prod" => Ok(Env::PRODUCTION),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub env: Env,
    pub server: Server,
    pub database: Database,
    pub jwt: Jwt,
}

impl AppSettings {
    pub fn init() -> Result<Self> {
        dotenv::from_filename(".env").ok();

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        // let jwt_max_age: u64 = std::env::var("JWT_MAX_AGE")
        //     .map(|v| v.parse().expect("JWT_MAX_AGE must be u64"))
        //     .expect("JWT_MAX_AGE must be set");
        let settings = Config::builder()
            .add_source(File::with_name("config/application.toml"))
            .add_source(File::with_name(&format!("config/application_{run_mode}")).required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("_")
                    .list_separator(",")
                    .ignore_empty(true),
            )
            .build()?;
        let mut app_settings: AppSettings = settings.try_deserialize()?;

        app_settings.env = run_mode.into();
        app_settings.database.database_url = env::var("DATABASE_URL")?;
        Ok(app_settings)
    }
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub env: Env,
    pub jwt_manager: Arc<JwtManager>,
    pub repository: Repository,
}

#[derive(Clone)]
pub struct JwtKey {
    kid: String,
    public_key: String,
    private_key: String,
}
#[derive(Clone)]
pub struct JwtManager {
    pub keys: Vec<JwtKey>,
    pub issuer: String,
    pub expire_seconds: u64,
}

impl JwtManager {
    pub fn new(settings: &Jwt) -> Result<Self> {
        todo!()
    }
}

#[derive(Clone)]
pub struct Repository {
    pub passport_repo: Arc<PgPassportRepository>,
    pub phone_mapping_repo: Arc<PgPhoneMappingRepository>,
}

impl Repository {
    pub fn new(db_pool: PgPool) -> Self {
        Repository {
            passport_repo: Arc::new(PgPassportRepository::new(db_pool.clone())),
            phone_mapping_repo: Arc::new(PgPhoneMappingRepository::new(db_pool.clone())),
        }
    }
}

impl AppState {
    pub async fn from(config: Arc<AppSettings>) -> Result<AppState> {
        let db_pool = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
            // Since we're using the default superuser we don't have to worry about this too much,
            // although we should leave some connections available for manual access.
            .min_connections(config.database.min_connections)
            .max_connections(config.database.max_connections)
            .connect(&config.database.database_url)
            .await
            .context("could not connect to database_url")?;
        let repository = Repository::new(db_pool);
        let state = AppState {
            env: config.env.clone(),
            jwt_manager: Arc::new(JwtManager::new(&config.jwt)?),
            repository,
        };
        Ok(state)
    }
}
