use axum::{
    error_handling::HandleErrorLayer,
    extract::{Json, Path, Query, Request},
    http::{
        header::{self, COOKIE},
        HeaderMap, HeaderName, HeaderValue, StatusCode,
    },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    BoxError, Form, Router,
};
use axum_extra::{response::Html, TypedHeader};
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, time::Duration};
use tower::Service;
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

#[derive(Clone)]
struct CurrentUser {
    user_id: u64,
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|item| item.to_str().ok());

    let token = if let Some(token) = auth_header {
        token
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(user) = authorize_current_user(token).await {
        req.extensions_mut().insert(user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    None
}
