use axum::http::{Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::{BoxError, Router};
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;
use crate::config::{AppConfig, AppState};
use crate::http::handler::{login_by_password, login_by_sms};

mod login;
mod profile;
mod register;

pub fn init_router(state: AppState) -> Router {
    let router= Router::new()
        .merge(login::routes())
        .merge(profile::routes());
    Router::new().nest("/api", router)
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(1))
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(state)
}

async fn handle_error(method: Method, uri: Uri, error: BoxError) -> impl IntoResponse {
    println!("handle error {} {} {}", method, uri, error);
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

async fn route_not_found() -> impl IntoResponse {
    (StatusCode::BAD_REQUEST, "route not found")
}
