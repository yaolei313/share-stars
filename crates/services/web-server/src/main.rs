mod http;
mod config;

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
use headers::UserAgent;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::fs::File;
use std::sync::Arc;
use std::{borrow::Borrow, collections::HashMap, time::Duration};
use tower::Service;
use tower::ServiceBuilder;
use tower_http::{auth, timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};
use crate::config::{AppConfig, AppState};

#[tokio::main]
async fn main() {
    let config = Arc::new(AppConfig::init());

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

    let state = Arc::new(
        AppState::from(Arc::clone(config))
            .await
            .expect("invalid configuration"),
    );

    http::serve(state).await;;
    log::info!("shutting down");
}
