use crate::http::AppState;
use crate::http::handler::{
    login_by_password, login_by_sms, profile, profile_me, register_by_email, send_sms, test,
};
use crate::http::mw;
use crate::http::vo::error::AppError;
use axum::error_handling::HandleErrorLayer;
use axum::http::{Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{BoxError, Router, middleware};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn init_router(state: AppState) -> Router {
    let login_router = Router::new()
        .route("/login_by_password", post(login_by_password))
        .route("/send_sms", post(send_sms))
        .route("/login_by_sms", post(login_by_sms));
    let register_router = Router::new().route("/register_by_email", post(register_by_email));
    let profiles_router = Router::new()
        .route("/profile/me", get(profile_me))
        .route("/profile/{user_id}", get(profile));
    let test_router = Router::new().route("/test", get(test));
    let router = Router::new()
        .merge(login_router)
        .merge(register_router)
        .merge(profiles_router)
        .merge(test_router);

    let service_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .layer(TraceLayer::new_for_http())
        .concurrency_limit(1024)
        .timeout(Duration::from_secs(1))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw::auth_middleware,
        ));

    Router::new()
        .nest("/api", router)
        .layer(service_layer)
        .with_state(state)
}

async fn handle_error(method: Method, uri: Uri, error: BoxError) -> Response {
    println!("handle error {} {} {}", method, uri, error);
    if error.is::<tower::timeout::error::Elapsed>() {
        return AppError::RequestTimeout.into_response();
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return AppError::ServiceUnavailable.into_response();
    }

    AppError::InternalServerError(error).into_response()
}

async fn route_not_found() -> impl IntoResponse {
    (StatusCode::BAD_REQUEST, "route not found")
}
