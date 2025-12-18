use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, Ident, Meta, Token, Type,
    punctuated::Punctuated,
};

/// Entity derive macro - 自动实现 Entity trait
///
/// # 属性
/// - `#[entity(id = "field_name")]` - 指定ID字段（默认为 `id`）
/// - `#[entity(type_name = "CustomName")]` - 指定实体类型名称（默认为结构体名）
/// - `#[diff(skip)]` - 跳过该字段的 diff 检测
/// - `#[replay(skip)]` - 跳过该字段的 replay 更新
///
/// # 示例
/// ```ignore
/// use diff::Entity;
///
/// #[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
/// struct Order {
///     id: u64,
///     symbol: String,
///     price: f64,
///     #[diff(skip)]
///     #[replay(skip)]
///     cached_value: String,
/// }
/// ```
#[proc_macro_derive(Entity, attributes(entity, diff, replay))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // 解析属性
    let id_field = extract_id_field(&input).unwrap_or_else(|| quote! { id });
    let type_name = extract_type_name(&input).unwrap_or_else(|| name.to_string());

    // 推断ID类型
    let id_type = infer_id_type(&input, &id_field.to_string());

    // 生成 diff 实现
    let diff_fields = generate_diff_fields(&input);

    // 生成 replay 实现
    let replay_impl = generate_replay_impl(&input);

    let expanded = quote! {
        impl #impl_generics diff::Entity for #name #ty_generics #where_clause {
            type Id = #id_type;

            fn entity_id(&self) -> Self::Id {
                self.#id_field.clone()
            }

            fn entity_type() -> &'static str {
                #type_name
            }

            fn diff(&self, other: &Self) -> Vec<diff::FieldChange> {
                let mut changes = Vec::new();
                #(#diff_fields)*
                changes
            }

            #replay_impl
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Helper Functions
// ============================================================================

/// 提取 ID 字段名称
fn extract_id_field(input: &DeriveInput) -> Option<proc_macro2::TokenStream> {
    for attr in &input.attrs {
        if attr.path().is_ident("entity") {
            if let Ok(meta) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                for item in meta {
                    if let Meta::NameValue(nv) = item {
                        if nv.path.is_ident("id") {
                            if let syn::Expr::Lit(expr_lit) = &nv.value {
                                if let syn::Lit::Str(s) = &expr_lit.lit {
                                    let ident = Ident::new(&s.value(), s.span());
                                    return Some(quote! { #ident });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// 提取自定义类型名称
fn extract_type_name(input: &DeriveInput) -> Option<String> {
    for attr in &input.attrs {
        if attr.path().is_ident("entity") {
            if let Ok(meta) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                for item in meta {
                    if let Meta::NameValue(nv) = item {
                        if nv.path.is_ident("type_name") {
                            if let syn::Expr::Lit(expr_lit) = &nv.value {
                                if let syn::Lit::Str(s) = &expr_lit.lit {
                                    return Some(s.value());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// 推断 ID 类型
fn infer_id_type(input: &DeriveInput, id_field_name: &str) -> proc_macro2::TokenStream {
    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                if let Some(ident) = &field.ident {
                    if ident == id_field_name {
                        let ty = &field.ty;
                        return quote! { #ty };
                    }
                }
            }
        }
    }

    // 默认为 u64
    quote! { u64 }
}

/// 生成 diff 字段比较逻辑
fn generate_diff_fields(input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut field_diffs = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                // 检查是否有 #[diff(skip)] 属性
                let skip = field.attrs.iter().any(|attr| {
                    attr.path().is_ident("diff")
                        && attr
                            .parse_args::<Ident>()
                            .map(|i| i == "skip")
                            .unwrap_or(false)
                });

                if skip {
                    continue;
                }

                if let Some(ident) = &field.ident {
                    let field_name = ident.to_string();

                    field_diffs.push(quote! {
                        if self.#ident != other.#ident {
                            changes.push(diff::FieldChange::new(
                                #field_name,
                                format!("{:?}", self.#ident),
                                format!("{:?}", other.#ident),
                            ));
                        }
                    });
                }
            }
        }
    }

    field_diffs
}

/// 生成 replay 实现
fn generate_replay_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let replay_fields = generate_replay_fields(input);

    if replay_fields.is_empty() {
        // 如果所有字段都跳过，返回简单实现
        quote! {
            fn replay(&mut self, entry: &diff::ChangeLogEntry) -> Result<(), diff::EntityError> {
                if !self.can_replay(entry) {
                    return Err(diff::EntityError::EntityIdMismatch {
                        expected: self.entity_id().to_string(),
                        actual: entry.entity_id.clone(),
                    });
                }

                match &entry.change_type {
                    diff::ChangeType::Deleted => {
                        Err(diff::EntityError::CannotReplayOnDeleted)
                    }
                    _ => Ok(())
                }
            }
        }
    } else {
        // 生成完整的 replay 实现
        quote! {
            fn replay(&mut self, entry: &diff::ChangeLogEntry) -> Result<(), diff::EntityError> {
                if !self.can_replay(entry) {
                    return Err(diff::EntityError::EntityIdMismatch {
                        expected: self.entity_id().to_string(),
                        actual: entry.entity_id.clone(),
                    });
                }

                match &entry.change_type {
                    diff::ChangeType::Updated { changed_fields } => {
                        for field in changed_fields {
                            match field.field_name.as_ref() {
                                #(#replay_fields)*
                                _ => {
                                    // 忽略未知字段
                                }
                            }
                        }
                        Ok(())
                    }
                    diff::ChangeType::Deleted => {
                        Err(diff::EntityError::CannotReplayOnDeleted)
                    }
                    diff::ChangeType::Created { fields: _ } => Ok(())
                }
            }
        }
    }
}

/// 生成 replay 字段解析逻辑
fn generate_replay_fields(input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut field_replays = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                // 检查是否有 #[replay(skip)] 属性
                let skip = field.attrs.iter().any(|attr| {
                    attr.path().is_ident("replay")
                        && attr
                            .parse_args::<Ident>()
                            .map(|i| i == "skip")
                            .unwrap_or(false)
                });

                if skip {
                    continue;
                }

                if let Some(ident) = &field.ident {
                    let field_name = ident.to_string();
                    let ty = &field.ty;

                    // 生成类型特定的解析逻辑
                    let parse_logic = generate_parse_logic_for_type(ident, ty, &field_name);

                    field_replays.push(quote! {
                        #field_name => {
                            #parse_logic
                        }
                    });
                }
            }
        }
    }

    field_replays
}

/// 为不同类型生成解析逻辑
fn generate_parse_logic_for_type(
    field_ident: &Ident,
    ty: &Type,
    field_name: &str,
) -> proc_macro2::TokenStream {
    // 获取类型的字符串表示
    let type_str = quote!(#ty).to_string();

    // String 类型特殊处理（Debug 格式会有引号）
    if type_str == "String" {
        return quote! {
            // String 类型：去掉 Debug 格式的引号
            let value_str = field.new_value.trim();
            if value_str.starts_with('"') && value_str.ends_with('"') && value_str.len() >= 2 {
                self.#field_ident = value_str[1..value_str.len()-1]
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\")
                    .replace("\\n", "\n")
                    .replace("\\r", "\r")
                    .replace("\\t", "\t");
            } else {
                self.#field_ident = value_str.to_string();
            }
        };
    }

    // 基础数值类型
    if matches!(
        type_str.as_str(),
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
        "f32" | "f64" | "bool"
    ) {
        return quote! {
            // 基础类型：直接解析
            self.#field_ident = field.new_value.parse::<#ty>()
                .map_err(|e| diff::EntityError::FieldParseError {
                    field: #field_name.to_string(),
                    reason: format!("Failed to parse {}: {}", #field_name, e),
                })?;
        };
    }

    // 其他类型：尝试直接克隆字符串（对于简单的 Copy 类型）
    // 或者返回错误提示需要手动实现
    quote! {
        // 复杂类型：需要自定义解析逻辑
        // 如果该字段不需要 replay，请添加 #[replay(skip)]
        // 否则需要手动实现 Entity trait
        return Err(diff::EntityError::FieldParseError {
            field: #field_name.to_string(),
            reason: format!(
                "Cannot auto-parse field '{}' of type '{}'. Use #[replay(skip)] or implement Entity manually.",
                #field_name,
                stringify!(#ty)
            ),
        });
    }
}
