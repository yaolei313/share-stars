use anyhow::Result;
use dotenv::dotenv;

use phf::phf_map;
use std::env;
use std::path::PathBuf;
use tool_box::convert_schema_to_struct;
use tool_box::pg_meta::PgMeta;

#[tokio::main]
async fn main() -> Result<()> {
    let current_dir: PathBuf = env::current_dir()?;
    println!("Current working directory: {:?}", current_dir);

    dotenv().ok(); // 加载 .env 文件
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");
    let target_dir =
        env::var("TARGET_DIR").expect("TARGET_DIR must be set in .env file or environment");

    let meta = PgMeta::new(&database_url).await?;

    let schema = "public";
    let table = "passport";
    let column_infos = meta.get_all_column_infos(schema, table).await?;

    convert_schema_to_struct(&target_dir, table, column_infos).expect("convert fail");

    Ok(())
}

static CONFIG_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "app_name" => "My Awesome App",
    "version" => "1.0.0",
    "env" => "production",
    "debug_mode" => "false",
};
