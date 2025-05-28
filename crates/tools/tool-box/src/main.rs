use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // 加载 .env 文件
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file or environment");
    let target_dir =
        env::var("TARGET_DIR").expect("TARGET_DIR must be set in .env file or enviroment");

    let pool = PgPool::connect(&database_url).await?;

    println!("Connected to PostgreSQL database.");

    // ==========================================================
    // 示例 1: 获取所有用户表的名称
    // ==========================================================
    println!("\n--- All User Tables ---");
    let tables: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT tablename, schemaname
        FROM pg_tables
        WHERE schemaname NOT IN ('pg_catalog', 'information_schema', 'topology')
        ORDER BY schemaname, tablename;
        "#,
    )
    .fetch_all(&pool)
    .await?;

    for (table_name, schema_name) in tables {
        println!("Schema: {}, Table: {}", schema_name, table_name);
    }

    let target_table = "passport"; // 替换为你要查询的表名
    let target_schema = "public"; // 替换为你要查询的 schema

    let columns = sqlx::query(
        r#"
        SELECT
            column_name,
            data_type,
            is_nullable,
            column_default
        FROM
            information_schema.columns
        WHERE
            table_schema = $1 AND table_name = $2
        ORDER BY
            ordinal_position;
        "#,
    )
    .bind(target_schema)
    .bind(target_table)
    .fetch_all(&pool)
    .await?;

    for row in columns {
        let column_name: String = row.try_get("column_name")?;
        let data_type: String = row.try_get("data_type")?;
        let is_nullable: String = row.try_get("is_nullable")?;
        let column_default: Option<String> = row.try_get("column_default")?;

        println!(
            "{:<20} {:<20} {:<10} {}",
            column_name,
            data_type,
            if is_nullable == "YES" { "Yes" } else { "No" },
            column_default.unwrap_or_else(|| "NULL".to_string())
        );
    }

    Ok(())
}

pub fn schema_to_struct(name: &str, columns: Vec<PgRow>) -> Result<()> {
    let path_buf = PathBuf::new();
    // let file = File::create_new();
    todo!()
}
