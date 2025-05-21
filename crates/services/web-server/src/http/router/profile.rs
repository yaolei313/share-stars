use crate::http::handler::{profile, profile_me};
use axum::routing::get;
use axum::Router;
use crate::config::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/profile/me", get(profile_me))
        .route("/profile/:user_id", get(profile))
}
