mod biz;
mod config;
mod http;

use crate::config::{AppConfig, AppState};
use anyhow::Result;
use axum::{
    Form, Router,
    error_handling::HandleErrorLayer,
    extract::{Json, Path, Query},
    http::{
        HeaderMap, HeaderName, HeaderValue, StatusCode,
        header::{self, COOKIE},
    },
    response::IntoResponse,
    routing::{get, post},
};
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::fs::File;
use std::sync::Arc;
use std::{borrow::Borrow, collections::HashMap, env, time::Duration};
use tower::Service;
use tower::ServiceBuilder;
use tower_http::{auth, timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};

#[tokio::main]
async fn main() -> Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let config = Arc::new(AppConfig::init()?);

    let json_log = File::create("stars.log")?;
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_names(true)
                .with_filter(LevelFilter::DEBUG),
        )
        .with(
            fmt::layer()
                .json()
                .with_target(true)
                .with_thread_names(true)
                .with_writer(json_log)
                .with_filter(LevelFilter::INFO),
        )
        .init();

    let state = AppState::from(Arc::clone(&config))
        .await
        .expect("invalid configuration");

    http::serve(state).await;
    log::info!("shutting down");
    Ok(())
}
