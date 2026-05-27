use crate::component::AppState;
use crate::config::AppSettings;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;

mod handler;
mod mw;
mod router;
pub mod vo;

pub fn init_components() -> Result<()> {
    mw::init_ua_extractor()
}

pub async fn serve(settings: Arc<AppSettings>) -> Result<()> {
    let state = AppState::new(settings.clone()).await?;
    let app = router::init_router(state.clone());

    let address = settings.server.get_bind_addr();
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::info!("Application listening on {}", &address);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
