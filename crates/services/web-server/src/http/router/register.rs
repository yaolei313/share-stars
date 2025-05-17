use crate::http::handler::register_by_email;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .route("/register_by_email", post(register_by_email))
}
