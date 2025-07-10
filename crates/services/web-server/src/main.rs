mod biz;
mod config;
mod http;

use crate::config::AppSettings;
use anyhow::Result;
use std::env;
use std::fs::File;
use std::sync::Arc;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Layer};

#[tokio::main]
async fn main() -> Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    println!("Cargo Manifest Directory: {}", manifest_dir);

    let settings = Arc::new(AppSettings::init()?);

    let json_log = File::create("stars.log")?;
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_names(true)
                .with_filter(LevelFilter::DEBUG),
        )
        .with(
            fmt::layer()
                .json()
                .with_target(true)
                .with_thread_names(true)
                .with_writer(json_log)
                .with_filter(LevelFilter::INFO),
        )
        .init();

    http::serve(settings).await?;
    log::info!("shutting down");
    Ok(())
}
