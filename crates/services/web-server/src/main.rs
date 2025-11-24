mod biz;
mod config;
mod http;

use crate::config::AppSettings;
use anyhow::{Context, Result};
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

    // 1. 初始化配置
    let settings = AppSettings::init().context("Failed to initialize application settings")?;

    // 2. 初始化日志
    init_logging().context("Failed to initialize logging")?;

    // 3. 打印启动信息
    tracing::info!("Application starting up...");

    http::serve(Arc::new(settings))
        .await
        .context("HTTP server failed to start or run")?;

    tracing::info!("Application shut down cleanly.");
    Ok(())
}

// 提取日志初始化逻辑到单独的函数
fn init_logging() -> Result<()> {
    // 假设 settings 中有一个 log_path 字段
    let log_file_path = "stars.log";

    let json_log = File::create(log_file_path)
        .context(format!("Failed to create log file at {}", log_file_path))?;

    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_names(true)
        .with_filter(LevelFilter::DEBUG);

    let file_layer = fmt::layer()
        .json()
        .with_target(true)
        .with_thread_names(true)
        .with_writer(json_log) // 使用同步的文件 writer
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    Ok(())
}
