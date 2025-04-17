use axum::Router;

mod router;
mod app;

pub fn init_router() -> Router {
    Router::new()
        .merge(crate::config::router::get_login_routes())
        .merge(crate::config::router::get_user_routes())
        .merge(crate::config::router::get_other_routes())
    //.fallback(any(route_not_found))
}