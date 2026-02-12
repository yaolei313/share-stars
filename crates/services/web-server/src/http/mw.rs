mod mw_access_token_auth;
mod mw_base;
mod mw_client_ctx;

pub use mw_access_token_auth::*;
pub use mw_client_ctx::*;
use std::fs::read_to_string;

use std::sync::OnceLock;
use ua_parser::{Extractor, Regexes};

static EXTRACTOR: OnceLock<Extractor<'_>> = OnceLock::new();

pub fn init_ua_extractor() -> anyhow::Result<()> {
    let yaml_content = read_to_string("config/regexes.yaml")?;
    let yaml_static: &'static str = Box::leak(yaml_content.into_boxed_str());
    let regexes: Regexes<'static> = serde_yaml::from_str(yaml_static)?;
    let extractor = ua_parser::Extractor::try_from(regexes)?;
    EXTRACTOR
        .set(extractor)
        .map_err(|_| anyhow::anyhow!("Extractor has already been initialized"))?;
    Ok(())
}

pub fn get_ua_extractor() -> Option<&'static Extractor<'static>> {
    EXTRACTOR.get()
}
