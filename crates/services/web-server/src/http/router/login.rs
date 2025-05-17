use crate::http::handler::{login_by_password, login_by_sms};
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/login_by_password", post(login_by_password))
        .route("/login_by_sms", post(login_by_sms))
}
