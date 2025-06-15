use crate::config::AppState;
use crate::http;
use axum::ServiceExt;
use std::net::SocketAddr;

mod handler;
mod router;
pub(crate) mod vo;

mod middleware;

pub async fn serve(state: AppState) {
    let app = router::init_router(state.clone());
    let address = format!("0.0.0.0:{}", 8080);
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
