use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DataEnum, DeriveInput, ExprLit, Lit, LitInt};
use syn::{Expr, Result};

pub(crate) fn try_expand_bind_code_derive(input: DeriveInput) -> Result<TokenStream> {
    // Ensure the macro is applied to an enum.
    match input.data {
        Data::Enum(data_enum) => {
            let enum_name = &input.ident; // The name of the enum (e.g., BizError)
            impl_enum(enum_name, data_enum)
        }
        _ => Err(syn::Error::new_spanned(
            input.ident,
            "BindCode can only be derived for enums",
        )),
    }
}

fn impl_enum(enum_name: &Ident, data_enum: DataEnum) -> Result<TokenStream> {
    // Collect all the match arms for the `code()` method.
    let mut match_arms = Vec::new();

    for variant in &data_enum.variants {
        let variant_name = &variant.ident; // The name of the variant (e.g., Success, Fail)
        let variant_span = variant.span();

        // 尝试从变体的属性中查找并解析 #[code(VALUE)]
        let code_value = match find_and_parse_code_attribute(&variant.attrs)? {
            Some(code) => code, // 找到了且解析成功
            None => {
                // 没有找到 #[code] 属性
                // 返回编译错误，指向当前变体的位置
                return Err(syn::Error::new(
                    variant_span,
                    "Missing #[code(VALUE)] attribute",
                ));
            }
        };

        // Determine if the variant has fields (e.g., `Fail(String)` or `InvalidArgument(String)`).
        // If it does, we need to match with `_` to ignore the field data.
        let match_pattern = match variant.fields {
            syn::Fields::Unnamed(_) => {
                // If it has unnamed fields (e.g., `Fail(String)`), match with `VariantName(_)`.
                quote! { #enum_name::#variant_name(_) }
            }
            _ => {
                // Otherwise (unit or named fields), match directly with `VariantName`.
                // Note: For named fields, you'd typically destructure them too, but for `code()`
                // which only depends on the variant name, we can treat them like unit variants.
                quote! { #enum_name::#variant_name }
            }
        };

        // Add the match arm: `BizError::VariantName => code_value,`
        // or `BizError::VariantName(_) => code_value,`
        match_arms.push(quote! {
            #match_pattern => #code_value,
        });
    }

    // Generate the `impl` block for the enum.
    let expanded = quote! {
        impl #enum_name {
            pub fn code(&self) -> i32 {
                match self {
                    #(#match_arms)* // Expand all collected match arms
                }
            }
        }
    };

    Ok(expanded.into()) // Convert the generated tokens into a TokenStream.
}

/// Helper function to extract the integer value from a `#[code(VALUE)]` attribute.
fn find_and_parse_code_attribute(attrs: &[Attribute]) -> Result<Option<LitInt>> {
    for attr in attrs {
        if attr.path().is_ident("code") {
            // attr.parse_args() attempts to parse the content inside `#[code(...)]`
            // as a generic expression (syn::Expr).
            let expr: Expr = attr.parse_args()?;
            // We expect this expression to be a literal (e.g., `0`, `1`, `401`).
            // So, we match against `Expr::Lit`.
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) = expr
            {
                // If it's an integer literal, return it.
                return Ok(Some(lit_int));
            } else {
                // If it's not an integer literal (e.g., `#[code("hello")]` or `#[code(1 + 2)]`),
                // return an error pointing to the attribute itself.
                return Err(syn::Error::new_spanned(
                    attr,
                    "Expected an integer literal (e.g., `0`, `1`) inside #[code(...)]",
                ));
            }
        }
    }
    Ok(None)
}
