use axum::{
    error_handling::HandleErrorLayer,
    extract::{Json, Path, Query},
    http::{
        header::{self, COOKIE},
        HeaderMap, HeaderName, HeaderValue, StatusCode,
    },
    response::IntoResponse,
    routing::{get, post},
    BoxError, Form, Router,
};
use axum_extra::{response::Html, TypedHeader};
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, time::Duration};
use tower::Service;
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = biz_http::init_router();
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("bind address fail");
    tracing::debug!("listening on {}", address);
    axum::serve(listener, app).await.expect("serve fail");
}
