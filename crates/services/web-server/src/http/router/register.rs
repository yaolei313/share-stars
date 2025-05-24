use crate::config::AppState;
use crate::http::handler::register_by_email;
use axum::Router;
use axum::routing::post;

pub fn routes() -> Router<AppState> {
    Router::new().route("/register_by_email", post(register_by_email))
}
