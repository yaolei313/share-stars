use crate::config::{AppSettings, AppState};
use crate::http;
use anyhow::Result;
use axum::ServiceExt;
use std::net::SocketAddr;
use std::sync::Arc;

mod handler;
mod mw;
mod router;
pub mod vo;

pub async fn serve(settings: Arc<AppSettings>) -> Result<()> {
    let state = AppState::from(settings.clone()).await?;

    let app = router::init_router(state.clone());

    let address = settings.get_bind_addr();
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::debug!("listening on {}", &address);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
