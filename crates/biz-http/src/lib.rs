mod config;
mod handler;
mod vo;
mod intercepter;

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
use std::{borrow::Borrow, collections::HashMap, time::Duration};
use tower::Service;
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::handler::login::*;


pub fn init_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/login_by_password", post(login_by_password))
        .route("/login_by_sms", post(login_by_sms))
        .route("/profile/me", get(profile_me))
        .route("/profile/:user_id", get(profile))
        // .layer(
        //     ServiceBuilder::new()
        //     .layer(TimeoutLayer::new(Duration::from_secs(3)))
        //     .layer(TraceLayer::new_for_http()).into())
        // .route_layer(middleware::from_fn(auth))
}