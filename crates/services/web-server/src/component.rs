use crate::biz::ServiceState;
use crate::config::{AppSettings, Env};
use anyhow::Context;
use axum::extract::FromRef;
use lib_core::db::DbServiceState;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub env: Env,
    pub redis_client: Arc<redis::Client>,
    pub db_service_state: Arc<DbServiceState>,
    pub service_state: Arc<ServiceState>,
}

impl AppState {
    pub async fn new(config: Arc<AppSettings>) -> anyhow::Result<AppState> {
        tracing::info!("Initializing application state...");
        let db_pool = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
            // Since we're using the default superuser we don't have to worry about this too much,
            // although we should leave some connections available for manual access.
            .min_connections(config.database.min_connections)
            .max_connections(config.database.max_connections)
            .acquire_timeout(Duration::from_secs(2))
            .connect(&config.database.database_url)
            .await
            .context("Database pool connection failed. Check URL and server status.")?;
        tracing::info!("Database pool established successfully.");

        let redis_client = Arc::new(
            redis::Client::open(config.redis.url.as_str())
                .context("Failed to open Redis client. Check URL format.")?,
        );
        tracing::info!("Redis client connected successfully.");

        let db_service_state = Arc::new(DbServiceState::new(db_pool));
        let service_state = Arc::new(
            ServiceState::new(
                db_service_state.clone(),
                redis_client.clone(),
                config.clone(),
            )
            .context("Failed to initialize ServiceState")?,
        );
        let state = AppState {
            env: config.env,
            redis_client,
            db_service_state,
            service_state,
        };
        tracing::info!("Application state initialized successfully.");
        Ok(state)
    }
}
