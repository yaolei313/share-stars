use crate::pg_meta::PgMeta;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;

pub mod pg_meta;

pub struct StructGenerator {
    pub import_part: HashSet<String>,
    pub body_part: String,
    pub meta: PgMeta,
}

impl StructGenerator {
    pub fn new(meta: PgMeta) -> Self {
        Self {
            import_part: Default::default(),
            body_part: String::with_capacity(10 * 1024 * 1024),
            meta,
        }
    }

    pub async fn convert_schema_to_struct(
        &mut self,
        schema: &str,
        table: &str,
    ) -> anyhow::Result<()> {
        self.import_part.insert("sqlx::FromRow".to_string());

        self.body_part.push_str("#[derive(Debug, FromRow)]\n");
        self.body_part.push_str(&format!(
            "pub struct {} {{\n",
            lib_utils::snake_to_pascal_case(table)
        ));

        let column_infos = self.meta.get_all_column_infos(schema, table).await?;

        for col in column_infos {
            let field_name = col.col_name;
            if let Some(t) = get_rust_type(&col.col_type) {
                for it in t.import_type.iter() {
                    self.import_part.insert(it.to_string());
                }
                let ty = if col.nullable {
                    format!("    pub {}: Option<{}>,\n", field_name, t.short_name)
                } else {
                    format!("    pub {}: {},\n", field_name, t.short_name)
                };
                self.body_part.push_str(&ty);
            } else {
                println!("not found type for {} {}", field_name, col.col_type);
                self.body_part
                    .push_str(&format!("    pub {}: {},\n", field_name, "unknown"));
            };
        }
        self.body_part.push_str("}\n");

        Ok(())
    }

    pub fn to_file(&self, output_dir: &Path) -> anyhow::Result<()> {
        let file_path = output_dir.join("gen_model.rs");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        file.write_all("/// auto generated\n".as_bytes())?;

        for row in self.import_part.iter() {
            writeln!(file, "use {};", row)?;
        }
        writeln!(file)?;

        file.write_all(self.body_part.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DbType {
    pub import_type: Vec<&'static str>,
    pub short_name: String,
}

impl DbType {
    pub fn from_str(s: &'static str) -> Self {
        let mut import_type = Vec::new();
        let mut short_name = String::new();
        if let Some((idx, generic_full)) = inner_generic_type(s) {
            let out_full = &s[0..idx];
            if let Some(out_short) = short_type_name(out_full) {
                import_type.push(out_full);
                short_name.push_str(out_short);
            } else {
                short_name.push_str(out_full);
            }
            if let Some(generic_short) = short_type_name(generic_full) {
                import_type.push(generic_full);
                short_name.push_str(&format!("<{}>", generic_short));
            } else {
                short_name.push_str(&format!("<{}>", generic_full));
            }
        } else {
            if let Some(out_short) = short_type_name(s) {
                import_type.push(s);
                short_name.push_str(out_short);
            } else {
                short_name.push_str(s);
            }
        }

        Self {
            import_type,
            short_name,
        }
    }
}

#[macro_export]
macro_rules! define_db_type_map {
    ( $( ($pg_type:expr, $rust_type:expr) ),* $(,)? ) => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($pg_type, DbType::from_str($rust_type));
            )*
            m
        }
    };
}

pub fn get_rust_type(pg_type: &str) -> Option<&DbType> {
    static HOLDER: OnceLock<HashMap<&'static str, DbType>> = OnceLock::new();
    let m = HOLDER.get_or_init(|| {
        let r = define_db_type_map!(
            ("smallint", "i16"),
            ("integer", "i32"),
            ("bigint", "i64"),
            ("numeric", "rust_decimal::Decimal"),
            ("decimal", "rust_decimal::Decimal"),
            ("real", "f32"),
            ("double precision", "f64"),
            ("character", "String"),
            ("character varying", "String"),
            ("bpchar", "String"),
            ("text", "String"),
            ("boolean", "bool"),
            ("bytea", "Vec<u8>"),
            ("date", "chrono::NaiveDate"),
            ("time", "chrono::NaiveTime"),
            ("timestamp without time zone", "chrono::NaiveDateTime"),
            ("timestamp with time zone", "chrono::DateTime<chrono::Utc>"),
            ("interval", "chrono::Duration"),
            ("json", "serde_json::Value"),
            ("jsonb", "serde_json::Value"),
            ("inet", "std::net::IpAddr"),
            ("macaddr", "[u8; 6]"),
            ("macaddr8", "[u8; 8]"),
            ("uuid", "uuid::Uuid"),
        );
        r
    });
    m.get(pg_type)
}

fn inner_generic_type(s: &str) -> Option<(usize, &str)> {
    let start_index = s.find('<')?; // Find the first '<'
    let end_index = s.rfind('>')?; // Find the last '>'

    if start_index < end_index {
        Some((start_index, &s[start_index + 1..end_index]))
    } else {
        None
    }
}

fn short_type_name(s: &str) -> Option<&str> {
    s.rsplit_once("::").map(|(_, suffix)| suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let input = "chrono::DateTime<chrono::Utc>";
        let type1 = DbType::from_str(input);
        println!("{:?}", type1);
        Ok(())
    }
}
