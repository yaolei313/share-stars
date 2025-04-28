use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::Router;
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use crate::handler::{login_by_password, login_by_sms};

pub fn get_login_routes() -> Router {
    Router::new()
        .route("/login_by_password", post(login_by_password))
        .route("/login_by_sms", post(login_by_sms))
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(crate::router::handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(5))
                .layer(TraceLayer::new_for_http()),
        )
}