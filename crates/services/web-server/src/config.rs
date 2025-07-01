use anyhow::Result;
use config::{Config, File};
use serde::Deserialize;
use std::env;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ServerSetting {
    pub host: String,
    pub port: u16,
    pub worker_id: u16,
}

impl ServerSetting {
    pub fn get_bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSetting {
    pub database_url: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub idle_timeout_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct RedisSetting {
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
    pub audience: String,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
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
}
