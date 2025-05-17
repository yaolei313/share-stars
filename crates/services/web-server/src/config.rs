use anyhow::Context;
use clap::{Parser, ValueEnum};
use lib_core::AppState;
use sqlx::postgres::PgPoolOptions;
use std::str::FromStr;
use std::sync::Arc;

#[derive(ValueEnum, Debug)]
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

#[derive(Parser, Debug)]
pub struct AppConfig {
    #[clap(env)]
    pub env: Env,
    #[clap(env)]
    pub database_url: String,
    #[clap(env)]
    pub jwt_secret: String,
    #[clap(env)]
    pub jwt_max_age: u64,
    #[clap(env)]
    pub port: u16,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenv::from_filename(".env").ok();

        // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        // let jwt_max_age: u64 = std::env::var("JWT_MAX_AGE")
        //     .map(|v| v.parse().expect("JWT_MAX_AGE must be u64"))
        //     .expect("JWT_MAX_AGE must be set");
        // let port: u16 = std::env::var("PORT").map_or_else(
        //     |_| 8080,
        //     |port| port.parse::<u16>().expect("PORT must be u16"),
        // );
        // let env = dotenv::var("ENV").expect("ENV must be set").try_into().expect("ENV must be set");
        // AppConfig {
        //     database_url,
        //     jwt_secret,
        //     jwt_max_age,
        //     port,
        //     env
        // }
        AppConfig::parse()
    }

}

pub async fn to_app_state(config: &AppConfig) -> anyhow::Result<AppState> {
    let db_pool = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;
    let state = AppState{
        db_pool,
        jwt_secret: "".to_string(),
        jwt_max_age: 0,
    };
    Ok(state)
}
