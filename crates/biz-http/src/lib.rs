mod config;
mod handler;
mod intercepter;
mod vo;

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Json, Path, Query, Request},
    http::{
        header::{self, COOKIE},
        HeaderMap, HeaderName, HeaderValue, StatusCode,
    },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    BoxError, Form, Router,
};
use axum_extra::{response::Html, TypedHeader};
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    time::Duration,
};
use tower::ServiceBuilder;
use tower::{layer, Service};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::handler::login::*;

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/login_by_password", post(login_by_password))
        .route("/login_by_sms", post(login_by_sms))
        .route("/profile/me", get(profile_me))
        .route("/profile/:user_id", get(profile))
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(intercepter::auth)),
        )
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}
