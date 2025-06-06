use crate::config::AppState;
use crate::http;

mod handler;
mod router;
pub(crate) mod vo;

mod middleware;

pub async fn serve(state: AppState) {
    let app = router::init_router(state.clone());
    let address = format!("0.0.0.0:{}", state.config.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("bind address fail");
    tracing::debug!("listening on {}", &address);
    axum::serve(listener, app)
        .await
        .expect("error running HTTP server");
}
