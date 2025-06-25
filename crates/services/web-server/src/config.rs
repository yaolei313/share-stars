use crate::biz::ServiceState;
use anyhow::Context;
use anyhow::Result;
use axum::extract::FromRef;
use config::{Config, File};
use jsonwebtoken::{DecodingKey, EncodingKey};
use lib_core::RepositoryState;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::iter::Map;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
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
struct RedisSetting {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct KeySetting {
    pub kid: String,
    pub public_key_path: String,
    pub private_key_path: String,
}
#[derive(Debug, Deserialize)]
pub struct JwtSetting {
    pub keys: Vec<KeySetting>,
    pub issuer: String,
    pub expire_seconds: u32,
}

#[derive(Debug, Deserialize)]
pub struct SmsSetting {
    pub account_sid: String,
    pub auth_token: String,
    pub from_phone: String,
    pub status_callback_url: String,
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
    pub redis: RedisSetting,
    pub jwt: JwtSetting,
    pub sms: SmsSetting,
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
    pub repository_state: Arc<RepositoryState>,
    pub redis_client: Arc<redis::Client>,
    pub service_state: Arc<ServiceState>,
}

impl AppState {
    pub async fn from(config: Arc<AppSettings>) -> Result<AppState> {
        log::info!("creating app state");
        let db_pool = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
            // Since we're using the default superuser we don't have to worry about this too much,
            // although we should leave some connections available for manual access.
            .min_connections(config.database.min_connections)
            .max_connections(config.database.max_connections)
            .acquire_timeout(Duration::from_secs(2))
            .connect(&config.database.database_url)
            .await
            .context("could not connect to database_url")?;
        let repository_state = Arc::new(RepositoryState::new(db_pool));
        let redis_client = Arc::new(redis::Client::open(config.redis.url.as_str())?);
        let service_state = Arc::new(ServiceState::new(
            repository_state.clone(),
            redis_client.clone(),
            config.clone(),
        )?);
        let state = AppState {
            env: config.env.clone(),
            redis_client,
            repository_state,
            service_state,
        };
        Ok(state)
    }
}
