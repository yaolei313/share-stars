use axum::Router;
use axum::routing::get;
use crate::handler::root;

pub fn get_other_routes() -> Router {
    Router::new().route("/", get(root))
}
