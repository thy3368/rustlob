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

/// Derive macro for automatically implementing the Replay trait
///
/// # Examples
///
/// ```ignore
/// use diff::Replay;
/// use diff_derive::Replay;
///
/// #[derive(Replay)]
/// struct Order {
///     id: String,
///     amount: i64,
///     status: String,
/// }
/// ```
#[proc_macro_derive(Replay)]
pub fn derive_replay(input: TokenStream) -> TokenStream {
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
                    "Replay can only be derived for structs with named fields"
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input,
                "Replay can only be derived for structs"
            )
            .to_compile_error()
            .into();
        }
    };

    // 为每个字段生成回放代码
    let field_replays = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_type = &field.ty;

        // 生成解析代码
        quote! {
            #field_name_str => {
                self.#field_name = change.new_value.parse::<#field_type>()
                    .map_err(|e| format!("Failed to parse field '{}': {}", #field_name_str, e))?;
            }
        }
    });

    // 生成完整的 Replay trait 实现
    let expanded = quote! {
        impl #impl_generics diff::Replay for #name #ty_generics #where_clause {
            fn replay(&mut self, entry: &diff::ChangeLogEntry) -> Result<(), String> {
                if let diff::ChangeType::Updated { changed_fields } = &entry.change_type {
                    for change in changed_fields {
                        match change.field_name.as_str() {
                            #(#field_replays)*
                            _ => {}  // 忽略未知字段
                        }
                    }
                    Ok(())
                } else {
                    Err("Cannot replay: not an Update change".to_string())
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for automatically generating tracked update methods
///
/// This macro generates a `tracked_update` method that automatically tracks
/// all changes made to the struct, eliminating the need to call `track_auto`.
///
/// # Requirements
///
/// The struct must implement `Clone` and `Diff` traits.
///
/// # Examples
///
/// ```ignore
/// use diff_derive::{Diff, Tracked};
///
/// #[derive(Clone, Diff, Tracked)]
/// struct Order {
///     id: String,
///     price: i64,
///     status: String,
/// }
///
/// let mut order = Order { id: "1".into(), price: 100, status: "pending".into() };
///
/// // 使用自动生成的方法
/// let entry = order.tracked_update(|o| {
///     o.price = 200;
///     o.status = "confirmed".to_string();
/// }).unwrap();
/// ```
///
/// # Generated Methods
///
/// - `tracked_update<F>(&mut self, f: F) -> Result<ChangeLogEntry, Box<dyn std::error::Error>>`
///   - Tracks changes made by the closure `f`
///   - Returns a `ChangeLogEntry` with all detected changes
#[proc_macro_derive(Tracked)]
pub fn derive_tracked(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 生成 tracked_update 方法
    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// 自动追踪变更的更新方法
            ///
            /// # Arguments
            ///
            /// * `updater` - 更新闭包，在其中修改字段
            ///
            /// # Returns
            ///
            /// 返回包含所有变更的 `ChangeLogEntry`
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let mut order = Order::new();
            /// let entry = order.tracked_update(|o| {
            ///     o.price = 200;
            ///     o.status = "confirmed".to_string();
            /// }).unwrap();
            /// ```
            pub fn tracked_update<F>(&mut self, updater: F) -> Result<diff::ChangeLogEntry, Box<dyn std::error::Error>>
            where
                Self: Clone + diff::Diff + 'static,
                F: FnOnce(&mut Self)
            {
                // 1. 克隆旧状态
                let old_self = self.clone();

                // 2. 执行更新
                updater(self);

                // 3. 自动 diff 检测变更
                let field_changes = old_self.diff(self);

                // 4. 构造 ChangeLogEntry
                let entry = diff::ChangeLogEntry {
                    entity_id: "auto_generated".to_string(),
                    entity_type: std::any::type_name::<Self>().to_string(),
                    change_type: diff::ChangeType::Updated {
                        changed_fields: field_changes
                    },
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs(),
                };

                Ok(entry)
            }
        }
    };

    TokenStream::from(expanded)
}
