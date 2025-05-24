use crate::config::AppState;
use crate::http::handler::{profile, profile_me};
use axum::Router;
use axum::routing::get;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/profile/me", get(profile_me))
        .route("/profile/{user_id}", get(profile))
}
