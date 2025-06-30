use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod enum_ext;

#[proc_macro_derive(BindCode, attributes(code))]
pub fn bind_code_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a Rust syntax tree.
    let input = parse_macro_input!(input as DeriveInput);
    enum_ext::try_expand_bind_code_derive(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
