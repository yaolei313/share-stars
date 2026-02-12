use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashSet;
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

/*
 * Ident 代表rust中的标识符
 * Span 代表位置信息
 */
fn impl_enum(enum_name: &Ident, data_enum: DataEnum) -> Result<TokenStream> {
    // Collect all the match arms for the `code()` method.
    let mut match_arms = Vec::new();
    let mut from_arms = Vec::new();
    let mut seen_codes = HashSet::new();

    let mut exists_unnamed = false;

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

        let val = code_value.to_string();
        if !seen_codes.insert(val) {
            // 如果 insert 返回 false，说明 code_value 已经存在于 HashSet 中
            return Err(syn::Error::new(
                code_value.span(), // 错误指向重复的 code 值本身
                format!(
                    "Duplicate code value `{}` found. Each `#[code(VALUE)]` must be unique.",
                    code_value
                ),
            ));
        }

        // Determine if the variant has fields (e.g., `Fail(String)` or `InvalidArgument(String)`).
        // If it does, we need to match with `_` to ignore the field data.
        let match_pattern = match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                exists_unnamed = true;

                let num_fields = fields.unnamed.len();
                let underscores = (0..num_fields).map(|_| quote! {_});

                // If it has unnamed fields (e.g., `Fail(String)`), match with `VariantName(_)`.
                quote! { #enum_name::#variant_name(#(#underscores),*) => #code_value }
            }
            syn::Fields::Named(_) => {
                // 对于命名字段 (e.g., StructVariant { a: i32 }), 我们使用 `..` 来忽略字段数据
                // 即使是命名字段，也需要模式匹配来忽略数据，否则会报“没有使用”的警告
                quote! { #enum_name::#variant_name{..} => #code_value }
            }
            syn::Fields::Unit => {
                // 单元变体 (e.g., Success)
                quote! { #enum_name::#variant_name => #code_value }
            }
        };

        // Add the match arm: `BizError::VariantName => code_value,`
        // or `BizError::VariantName(_) => code_value,`
        match_arms.push(match_pattern);

        from_arms.push(quote! {
            #code_value => Some(#enum_name::#variant_name)
        });
    }

    let from_code_fn = if !exists_unnamed {
        from_arms.push(quote! {
            _ => None
        });
        quote! {
            pub fn from_code(code: i32) -> Option<Self> {
                match code {
                    #(#from_arms),*
                }
            }
        }
    } else {
        quote! {}
    };

    // Generate the `impl` block for the enum.
    let expanded = quote! {
        impl #enum_name {
            pub fn code(&self) -> i32 {
                match self {
                    #(#match_arms),* // Expand all collected match arms
                }
            }

            #from_code_fn
        }
    };

    // panic!(
    //     "Generated code for {}:\n{}",
    //     enum_name,
    //     expanded.to_string()
    // );

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
