use anyhow::Result;

use futures::future::join_all;
use phf::phf_map;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{env, fs};
use tokio::sync::Mutex;
use tool_box::pg_meta::PgMeta;
use tool_box::StructGenerator;

#[tokio::main]
async fn main() -> Result<()> {
    let current_dir: PathBuf = env::current_dir()?;
    println!("Current working directory: {:?}", current_dir);

    dotenv::from_filename(".tool.env").ok(); // 加载 .env 文件
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");
    let target_dir =
        env::var("TARGET_DIR").expect("TARGET_DIR must be set in .env file or environment");

    let schema = "public";

    let shared_generator = Arc::new(Mutex::new(StructGenerator::new(
        PgMeta::new(&database_url).await?,
    )));

    let tables = vec![
        "account",
        "account_identity",
        "account_device",
        "lookup_account",
        "device",
        "lookup_device",
        "prefilter_device",
        "sms_template",
        "email_template",
    ];
    let mut futures = Vec::new();
    for table in tables {
        let generator_clone = Arc::clone(&shared_generator);
        let future = async move {
            // 在异步块内部，获取 Mutex 的锁
            // 这会阻塞当前任务直到获得锁，但不会阻塞整个线程
            let mut gen1 = generator_clone.lock().await;
            gen1.convert_schema_to_struct(schema, table)
                .await
                .expect("convert fail"); // 错误处理可以更精细
        };
        futures.push(future);
    }
    // 4. 并发等待所有任务完成
    join_all(futures).await;
    println!("All schema conversions completed.");

    let output_dir = Path::new(&target_dir);
    fs::create_dir_all(output_dir)?;
    let final_generator = shared_generator.lock().await;
    final_generator.to_file(output_dir)?;

    println!(
        "Generated model.rs file at {:?}",
        output_dir.join("model.rs")
    );

    Ok(())
}

static _CONFIG_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "app_name" => "My Awesome App",
    "version" => "1.0.0",
    "env" => "production",
    "debug_mode" => "false",
};
