use anyhow::{Context, Result};
use config::{Config, File};
use lib_utils::KeySetting;
use serde::Deserialize;
use std::env;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ServerSetting {
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    #[validate(range(min = 1, max = 1024))]
    pub worker_id: u16,
}

impl ServerSetting {
    pub fn get_bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct DatabaseSetting {
    #[validate(length(min = 1))]
    pub database_url: String,
    #[validate(range(min = 1, max = 500))]
    pub min_connections: u32,
    #[validate(range(min = 1, max = 500))]
    pub max_connections: u32,
    pub idle_timeout_seconds: Option<u64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RedisSetting {
    #[validate(length(min = 1))]
    pub url: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct JwtSetting {
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub keys: Vec<KeySetting>,
    #[validate(length(min = 1))]
    pub audience: String,
    #[validate(length(min = 1))]
    pub issuer: String,
    pub expire_seconds: u32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SmsSetting {
    #[validate(length(min = 1))]
    pub account_sid: String,
    #[validate(length(min = 1))]
    pub auth_token: String,
    #[validate(length(min = 11, max = 15))]
    pub from_phone: String,
    pub status_callback_url: String,
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct EmailSettings {
    #[validate(email)]
    pub from_email: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Copy)]
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
            _ => Err(Error::new(ErrorKind::InvalidData, "unknown env str")),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct AppSettings {
    pub env: Env,
    #[validate(nested)]
    pub server: ServerSetting,
    #[validate(nested)]
    pub database: DatabaseSetting,
    #[validate(nested)]
    pub redis: RedisSetting,
    #[validate(nested)]
    pub login: JwtSetting,
    #[validate(nested)]
    pub sms: SmsSetting,
    #[validate(nested)]
    pub email: EmailSettings,
}

impl AppSettings {
    pub fn init() -> Result<Self> {
        dotenv::from_filename(".env").ok();

        let run_mode = env::var("RUN_MODE")
            .unwrap_or_else(|_| "dev".into())
            .to_lowercase();
        let database_url = env::var("DATABASE_URL").context("Failed to get DATABASE_URL")?;

        let settings = Config::builder()
            .add_source(File::with_name("config/application"))
            .add_source(File::with_name(&format!("config/application_{run_mode}")).required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("_")
                    .list_separator(",")
                    .ignore_empty(true),
            )
            .set_default("database.database_url", database_url)?
            .set_default("env", run_mode)?
            .build()
            .context("Failed to build config")?;
        let app_settings: AppSettings = settings
            .try_deserialize()
            .context("Failed to parse config")?;
        app_settings.validate().context("Invalid config")?;

        Ok(app_settings)
    }
}
