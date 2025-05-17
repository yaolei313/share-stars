mod config;

mod http;

use axum::{
    BoxError, Form, Router,
    error_handling::HandleErrorLayer,
    extract::{Json, Path, Query},
    http::{
        HeaderMap, HeaderName, HeaderValue, StatusCode,
        header::{self, COOKIE},
    },
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::{TypedHeader, response::Html};
use headers::UserAgent;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{borrow::Borrow, collections::HashMap, time::Duration};
use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use tower::Service;
use tower::ServiceBuilder;
use tower_http::{auth, timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};
use lib_core::AppState;
use crate::config::AppConfig;

#[tokio::main]
async fn main() {
    let config = AppConfig::init();

    env_logger::init();

    let json_log = File::create("stars.log")?;
    tracing_subscriber::registry()
        .with(fmt::layer().with_target(true).with_thread_names(true))
        .with(
            fmt::layer()
                .json()
                .with_target(true)
                .with_thread_names(true)
                .with_filter(LevelFilter::Info)
                .with_writer(json_log),
        )
        .init();

    let state = Arc::new(config::to_app_state(&config).await.expect("invalid configuration"));

    let app = http::router::init_router(state);
    let address = concat!("0.0.0.0:", &config.port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("bind address fail");
    tracing::debug!("listening on {}", address);
    axum::serve(listener, app).await.expect("serve fail");
    log::info!("shutting down");
}
