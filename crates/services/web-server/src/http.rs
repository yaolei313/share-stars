use crate::config::{AppSettings, AppState};
use crate::http;
use axum::ServiceExt;
use std::net::SocketAddr;
use std::sync::Arc;

mod handler;
mod mw;
mod router;
pub mod vo;

pub async fn serve(settings: Arc<AppSettings>) {
    let state = AppState::from(settings.clone())
        .await
        .expect("invalid configuration");

    let app = router::init_router(state.clone());
    let address = settings.get_bind_addr();
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("bind address fail");
    tracing::debug!("listening on {}", &address);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("error running HTTP server");
}
