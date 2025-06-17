use anyhow::Context;
use anyhow::Result;
use axum::extract::FromRef;
use config::{Config, File};
use jsonwebtoken::{DecodingKey, EncodingKey};
use lib_core::db::repositories::{PgPassportRepository, PgPhoneMappingRepository};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::Arc;
use std::{env, fs};

#[derive(Debug, Deserialize)]
struct ServerSetting {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct DatabaseSetting {
    database_url: String,
    min_connections: u32,
    max_connections: u32,
    idle_timeout_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct KeySetting {
    kid: String,
    public_key_path: String,
    private_key_path: String,
}
#[derive(Debug, Deserialize)]
struct JwtSetting {
    keys: Vec<KeySetting>,
    issuer: String,
    expire_seconds: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Env {
    #[serde(rename = "dev")]
    DEV,
    #[serde(rename = "test")]
    TEST,
    #[serde(rename = "staging")]
    STAGING,
    #[serde(rename = "production")]
    PRODUCTION,
}

impl FromStr for Env {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Env::DEV),
            "test" => Ok(Env::TEST),
            "staging" => Ok(Env::STAGING),
            "prod" => Ok(Env::PRODUCTION),
            _ => Err(Error::new(ErrorKind::InvalidData, "unknown err")),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub env: Env,
    pub server: ServerSetting,
    pub database: DatabaseSetting,
    pub jwt: JwtSetting,
}

impl AppSettings {
    pub fn init() -> Result<Self> {
        dotenv::from_filename(".env").ok();

        let run_mode = env::var("RUN_MODE")
            .unwrap_or_else(|_| "dev".into())
            .to_lowercase();
        let database_url = env::var("DATABASE_URL")?;

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
            .set_default("database.database_url", database_url)?
            .set_default("env", run_mode)?
            .build()?;
        let app_settings: AppSettings = settings.try_deserialize()?;

        Ok(app_settings)
    }

    pub fn get_bind_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
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
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}
#[derive(Clone)]
pub struct JwtManager {
    pub keys: Vec<JwtKey>,
    pub issuer: String,
    pub expire_seconds: u64,
}

impl JwtManager {
    pub fn new(settings: &JwtSetting) -> Result<Self> {
        let tmp_keys = settings
            .keys
            .iter()
            .map(|k| {
                let rsa_private_pem = fs::read_to_string(&k.private_key_path)?;
                let rsa_public_pem = fs::read_to_string(&k.public_key_path)?;

                let encoding_key = EncodingKey::from_rsa_pem(rsa_private_pem.as_bytes())?;
                let decoding_key = DecodingKey::from_rsa_pem(rsa_public_pem.as_bytes())?;

                Ok(JwtKey {
                    kid: k.kid.clone(),
                    encoding_key,
                    decoding_key,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(JwtManager {
            keys: tmp_keys,
            issuer: settings.issuer.clone(),
            expire_seconds: settings.expire_seconds,
        })
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
