use crate::biz::ServiceState;
use crate::config::{AppSettings, Env};
use crate::http;
use anyhow::{Context, Result};
use axum::ServiceExt;
use axum::extract::FromRef;
use lib_core::RepositoryState;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

mod handler;
mod mw;
mod router;
pub mod vo;

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
            config.env.clone(),
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

pub async fn serve(settings: Arc<AppSettings>) -> Result<()> {
    let state = AppState::from(settings.clone()).await?;
    let app = router::init_router(state.clone());

    let address = settings.server.get_bind_addr();
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::info!("listening on {}", &address);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
