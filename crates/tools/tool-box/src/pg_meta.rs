use anyhow::Result;
use sqlx::{FromRow, PgPool, Pool, Postgres, Row};

#[derive(Debug, FromRow)]
pub struct TableInfo {
    #[sqlx(rename = "schemaname")]
    pub schema_name: String,
    #[sqlx(rename = "tablename")]
    pub table_name: String,
}
pub struct ColumnInfo {
    pub col_name: String,
    pub col_type: String,
    pub nullable: bool,
}

pub struct PgMeta {
    pool: Pool<Postgres>,
}

impl PgMeta {
    pub async fn new(database_url: &str) -> Result<PgMeta> {
        let pool = PgPool::connect(&database_url).await?;
        Ok(PgMeta { pool })
    }

    pub async fn get_all_table_infos(&self) -> Result<Vec<TableInfo>> {
        let tables: Vec<TableInfo> = sqlx::query_as(
            r#"
        SELECT tablename, schemaname
        FROM pg_tables
        WHERE schemaname NOT IN ('pg_catalog', 'information_schema', 'topology')
        ORDER BY schemaname, tablename;
        "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(tables)
    }

    pub async fn get_all_column_infos(
        &self,
        schema_name: &str,
        table_name: &str,
    ) -> Result<Vec<ColumnInfo>> {
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
        .bind(schema_name)
        .bind(table_name)
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::with_capacity(columns.len());
        for row in columns {
            let col_name: String = row.try_get("column_name")?;
            let col_type: String = row.try_get("data_type")?;
            let nullable_desc: String = row.try_get("is_nullable")?;

            let nullable: bool = "yes".eq_ignore_ascii_case(&nullable_desc);

            let info = ColumnInfo {
                col_name,
                col_type,
                nullable,
            };

            result.push(info);
        }

        Ok(result)
    }
}
