use crate::config::AppState;
use crate::http::handler::{
    login_by_password, login_by_sms, profile, profile_me, register_by_email,
};
use axum::error_handling::HandleErrorLayer;
use axum::http::{Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{BoxError, Router};
use std::borrow::Cow;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn init_router(state: AppState) -> Router {
    let login_router = Router::new()
        .route("/login_by_password", post(login_by_password))
        .route("/login_by_sms", post(login_by_sms));
    let register_router = Router::new().route("/register_by_email", post(register_by_email));
    let profiles_router = Router::new()
        .route("/profile/me", get(profile_me))
        .route("/profile/{user_id}", get(profile));
    let router = Router::new()
        .merge(login_router)
        .merge(register_router)
        .merge(profiles_router);

    Router::new()
        .nest("/api", router)
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
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
