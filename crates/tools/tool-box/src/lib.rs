use crate::pg_meta::ColumnInfo;
use sqlx::FromRow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;

pub mod pg_meta;

pub fn convert_schema_to_struct(
    target_dir: &str,
    name: &str,
    column_infos: Vec<ColumnInfo>,
) -> anyhow::Result<()> {
    // let file = File::create_new();
    let dir_path = Path::new(target_dir);
    let file_path = dir_path.join("model.rs");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    let mut header = String::new();
    let mut body = String::new();
    header.push_str("use sqlx::FromRow;\n");

    body.push_str("\n#[derive(Debug, FromRow)]\n");
    body.push_str(&format!(
        "pub struct {} {{\n",
        lib_utils::snake_to_pascal_case(name)
    ));
    for col in column_infos {
        let field_name = col.col_name;
        if let Some(rust_type) = convert_pg_type_to_rust_type(&col.col_type, col.nullable) {
            body.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
        } else {
            body.push_str(&format!("    pub {}: {},\n", field_name, ""));
        }
    }
    body.push_str("}");

    file.write_all(header.as_bytes())?;
    file.write_all(body.as_bytes())?;

    Ok(())
}

pub fn convert_pg_type_to_rust_type(pg_type: &str, nullable: bool) -> Option<String> {
    let t = get_rust_type(pg_type)?;
    if nullable {
        Some(format!("Option<{}>", t))
    } else {
        Some(t.to_string())
    }
}

pub fn get_rust_type(pg_type: &str) -> Option<&'static str> {
    static HOLDER: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    let m = HOLDER.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("smallint", "i16");
        m.insert("integer", "i32");
        m.insert("bigint", "i64");
        m.insert("numeric", "rust_decimal::Decimal");
        m.insert("decimal", "rust_decimal::Decimal");
        m.insert("real", "f32");
        m.insert("double precision", "f64");

        m.insert("character", "String");
        m.insert("character varying", "String");
        m.insert("bpchar", "String");
        m.insert("text", "String");

        m.insert("boolean", "bool");

        m.insert("bytea", "Vec<u8>");

        m.insert("date", "chrono::NaiveDate");
        m.insert("time", "chrono::NaiveTime");
        m.insert("timestamp without time zone", "chrono::NaiveDateTime");
        m.insert("timestamp with time zone", "chrono::DateTime<Utc>");
        m.insert("interval", "chrono::Duration");
        m.insert("JSON, JSONB", "serde_json::Value");

        m.insert("inet", "std::net::IpAddr");
        m.insert("macaddr", "[u8; 6]");
        m.insert("macaddr8", "[u8; 8]");
        m.insert("uuid", "uuid::Uuid");

        m
    });
    m.get(pg_type).map(|s| *s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 5 - 1;
        assert_eq!(result, 4);
    }
}
