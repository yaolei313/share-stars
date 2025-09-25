use lib_macro_derive::BindCode;
use serde::Deserialize;

#[derive(Debug, Deserialize, BindCode)]
pub enum ProviderType {
    #[code(1)]
    PhoneNumber,
    #[code(2)]
    Email,

    #[code(11)]
    Facebook,
    #[code(12)]
    Google,
    #[code(13)]
    Apple,
}
