use anyhow::Result;

use phf::phf_map;
use std::env;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use tool_box::convert_schema_to_struct;
use tool_box::pg_meta::PgMeta;

#[tokio::main]
async fn main() -> Result<()> {
    let current_dir: PathBuf = env::current_dir()?;
    println!("Current working directory: {:?}", current_dir);

    dotenv::from_filename(".tool.env").ok(); // 加载 .env 文件
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");
    let target_dir =
        env::var("TARGET_DIR").expect("TARGET_DIR must be set in .env file or environment");

    let meta = PgMeta::new(&database_url).await?;

    let dir_path = Path::new(&target_dir);
    let file_path = dir_path.join("model.rs");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    let schema = "public";
    let table = "user_credential";
    let column_infos = meta.get_all_column_infos(schema, table).await?;
    convert_schema_to_struct(&mut file, table, column_infos).expect("convert fail");

    let table = "user_identity";
    let column_infos = meta.get_all_column_infos(schema, table).await?;
    convert_schema_to_struct(&mut file, table, column_infos).expect("convert fail");

    let table = "user_identity_map";
    let column_infos = meta.get_all_column_infos(schema, table).await?;
    convert_schema_to_struct(&mut file, table, column_infos).expect("convert fail");

    let table = "trusted_device";
    let column_infos = meta.get_all_column_infos(schema, table).await?;
    convert_schema_to_struct(&mut file, table, column_infos).expect("convert fail");

    Ok(())
}

static _CONFIG_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "app_name" => "My Awesome App",
    "version" => "1.0.0",
    "env" => "production",
    "debug_mode" => "false",
};
