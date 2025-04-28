use std::borrow::Cow;
use axum::BoxError;
use axum::http::{Method, StatusCode, Uri};
use axum::response::IntoResponse;

mod login;
mod profile;
mod root;
mod register;

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

