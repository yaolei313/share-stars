use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::{middleware, Router};
use axum::routing::get;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use crate::handler::{profile, profile_me};
use lib_auth::interceptor;

pub fn get_user_routes() -> Router {
    Router::new()
        .route("/profile/me", get(profile_me))
        .route("/profile/:user_id", get(profile))
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(crate::router::handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(5))
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(interceptor::auth)),
        )
}