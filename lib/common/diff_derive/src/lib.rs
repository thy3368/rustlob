use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive macro for automatically implementing the Diff trait
///
/// # Attributes
///
/// - `#[diff(skip)]` - Skip this field in diff comparison
/// - `#[diff(mask)]` - Mask the field value (show as "***") in diff output
///
/// # Examples
///
/// ```ignore
/// use diff::Diff;
/// use diff_derive::Diff;
///
/// #[derive(Clone, Diff)]
/// struct User {
///     id: String,
///     name: String,
///     age: i32,
///     #[diff(skip)]
///     internal_cache: Option<String>,
///     #[diff(mask)]
///     password: String,
/// }
/// ```
#[proc_macro_derive(Diff, attributes(diff))]
pub fn derive_diff(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 只支持结构体
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "Diff can only be derived for structs with named fields"
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input,
                "Diff can only be derived for structs"
            )
            .to_compile_error()
            .into();
        }
    };

    // 为每个字段生成比较代码
    let field_comparisons = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();

        // 解析字段属性
        let mut skip = false;
        let mut mask = false;

        for attr in &field.attrs {
            if attr.path().is_ident("diff") {
                if let Ok(meta_list) = attr.meta.require_list() {
                    meta_list.tokens.clone().into_iter().for_each(|token| {
                        let token_str = token.to_string();
                        if token_str == "skip" {
                            skip = true;
                        } else if token_str == "mask" {
                            mask = true;
                        }
                    });
                }
            }
        }

        if skip {
            // 跳过此字段
            quote! {}
        } else if mask {
            // 脱敏字段：只显示 "***"
            quote! {
                if self.#field_name != other.#field_name {
                    changes.push(diff::FieldChange {
                        field_name: #field_name_str.to_string(),
                        old_value: "***".to_string(),
                        new_value: "***".to_string(),
                    });
                }
            }
        } else {
            // 正常字段：比较并记录变更
            quote! {
                if self.#field_name != other.#field_name {
                    changes.push(diff::FieldChange {
                        field_name: #field_name_str.to_string(),
                        old_value: self.#field_name.to_string(),
                        new_value: other.#field_name.to_string(),
                    });
                }
            }
        }
    });

    // 生成完整的 Diff trait 实现
    let expanded = quote! {
        impl #impl_generics diff::Diff for #name #ty_generics #where_clause {
            fn diff(&self, other: &Self) -> Vec<diff::FieldChange> {
                let mut changes = Vec::new();
                #(#field_comparisons)*
                changes
            }
        }
    };

    TokenStream::from(expanded)
}
