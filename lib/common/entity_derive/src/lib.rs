use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Fields, Ident, Meta, Token, Type, parse_macro_input};

/// Entity derive macro - 自动实现 Entity trait 和 FromCreatedEvent trait
///
/// # 属性
/// - `#[entity(id = "field_name")]` - 指定ID字段（默认为 `id`）
/// - `#[entity(type_name = "CustomName")]` - 指定实体类型名称（默认为结构体名）
/// - `#[diff(skip)]` - 跳过该字段的 diff 检测
/// - `#[replay(skip)]` - 跳过该字段的 replay 更新
/// - `#[created(skip)]` - 跳过该字段的 Created 事件重构
///
/// # 示例
/// ```ignore
/// use diff::{Entity, FromCreatedEvent};
///
/// #[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
/// struct Order {
///     id: u64,
///     symbol: String,
///     price: f64,
///     #[diff(skip)]
///     #[replay(skip)]
///     #[created(skip)]
///     cached_value: String,
/// }
///
/// // 自动生成 FromCreatedEvent 的实现，可直接使用：
/// // let order = Order::from_created_event(&event)?;
/// ```
#[proc_macro_derive(Entity, attributes(entity, diff, replay, created))]
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

    // 生成 FromCreatedEvent 实现
    let from_created_impl = generate_from_created_impl(&input);

    // 生成 table_schema() 方法
    let table_schema_method = generate_table_schema_method(&input, &type_name);

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

        // 为实体类型实现 table_schema 相关方法
        impl #impl_generics #name #ty_generics #where_clause {
            #table_schema_method
        }

        // 自动实现 FromCreatedEvent trait
        #from_created_impl
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
            if let Ok(meta) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            {
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
            if let Ok(meta) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            {
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
                        && attr.parse_args::<Ident>().map(|i| i == "skip").unwrap_or(false)
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
                        actual: entry.entity_id().to_string(),
                    });
                }

                match entry.change_type() {
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
                        actual: entry.entity_id().to_string(),
                    });
                }

                match entry.change_type() {
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
                        && attr.parse_args::<Ident>().map(|i| i == "skip").unwrap_or(false)
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
        "u8" | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "f32"
            | "f64"
            | "bool"
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

// ============================================================================
// FromCreatedEvent 代码生成
// ============================================================================

/// 生成 FromCreatedEvent trait 实现
fn generate_from_created_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // 生成字段构造代码
    let field_constructions = generate_field_constructions(input);

    quote! {
        impl #impl_generics diff::FromCreatedEvent for #name #ty_generics #where_clause {
            fn from_created_event(entry: &diff::ChangeLogEntry) -> Result<Self, diff::EntityError> {
                let fields = diff::extract_fields_from_created_event(entry)?;
                Self::from_field_map(&fields)
            }

            fn from_field_map(
                fields: &std::collections::HashMap<String, String>,
            ) -> Result<Self, diff::EntityError> {
                Ok(Self {
                    #(#field_constructions),*
                })
            }
        }
    }
}

/// 为每个字段生成构造代码
fn generate_field_constructions(input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut constructions = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                // 检查是否有 #[created(skip)] 属性
                let skip = field.attrs.iter().any(|attr| {
                    attr.path().is_ident("created")
                        && attr.parse_args::<Ident>().map(|i| i == "skip").unwrap_or(false)
                });

                if skip {
                    // 跳过该字段，使用 Default::default()
                    if let Some(ident) = &field.ident {
                        constructions.push(quote! {
                            #ident: {
                                // Field skipped - using Default
                                Default::default()
                            }
                        });
                    }
                    continue;
                }

                if let Some(ident) = &field.ident {
                    let field_name = ident.to_string();
                    let ty = &field.ty;
                    let type_str = quote!(#ty).to_string();

                    // 根据类型生成解析代码
                    let parse_code =
                        generate_field_parse_code_for_created(ident, &type_str, &field_name);
                    constructions.push(parse_code);
                }
            }
        }
    }

    constructions
}

/// 为 Created 事件生成字段解析代码
fn generate_field_parse_code_for_created(
    field_ident: &Ident,
    type_str: &str,
    field_name: &str,
) -> proc_macro2::TokenStream {
    match type_str {
        // 整数类型
        "u64" | "u32" | "u16" | "u8" | "i64" | "i32" | "i16" | "i8" | "usize" | "isize" => {
            let type_ident = Ident::new(&type_str.replace(" ", ""), proc_macro2::Span::call_site());
            quote! {
                #field_ident: fields
                    .get(#field_name)
                    .and_then(|v| v.parse::<#type_ident>().ok())
                    .ok_or(diff::EntityError::FieldParseError {
                        field: #field_name.to_string(),
                        reason: format!("Cannot parse '{}' as {}", #field_name, stringify!(#type_ident)),
                    })?
            }
        }
        // 浮点数类型
        "f64" | "f32" => {
            let type_ident = Ident::new(&type_str.replace(" ", ""), proc_macro2::Span::call_site());
            quote! {
                #field_ident: fields
                    .get(#field_name)
                    .and_then(|v| v.parse::<#type_ident>().ok())
                    .ok_or(diff::EntityError::FieldParseError {
                        field: #field_name.to_string(),
                        reason: format!("Cannot parse '{}' as {}", #field_name, stringify!(#type_ident)),
                    })?
            }
        }
        // 布尔值
        "bool" => {
            quote! {
                #field_ident: fields
                    .get(#field_name)
                    .and_then(|v| v.parse::<bool>().ok())
                    .ok_or(diff::EntityError::FieldParseError {
                        field: #field_name.to_string(),
                        reason: format!("Cannot parse '{}' as bool", #field_name),
                    })?
            }
        }
        // String 类型
        "String" => {
            quote! {
                #field_ident: fields
                    .get(#field_name)
                    .map(|v| {
                        // String 类型：去掉 Debug 格式的引号
                        if v.starts_with('\"') && v.ends_with('\"') && v.len() >= 2 {
                            v[1..v.len() - 1].to_string()
                        } else {
                            v.clone()
                        }
                    })
                    .ok_or(diff::EntityError::FieldParseError {
                        field: #field_name.to_string(),
                        reason: format!("Missing field '{}'", #field_name),
                    })?
            }
        }
        // 其他类型：尝试调用 from_created_event（如果类型也实现了 FromCreatedEvent）
        _ => {
            quote! {
                #field_ident: {
                    // 复杂类型：尝试使用 Default
                    // 如果需要自定义解析，请为该字段添加 #[created(skip)] 并使用 Default::default()
                    Default::default()
                }
            }
        }
    }
}

// ============================================================================
// TableSchema 方法代码生成
// ============================================================================

/// 生成 table_schema() 方法
///
/// 自动从结构体字段生成 TableSchema，包含所有字段的元数据
fn generate_table_schema_method(input: &DeriveInput, type_name: &str) -> proc_macro2::TokenStream {
    let table_name = type_name.to_lowercase();
    let field_schemas = generate_field_schemas(input);

    quote! {
        /// 获取实体对应的数据库表结构定义
        ///
        /// 自动从结构体字段生成 TableSchema，包含表名和所有字段的元数据
        #[inline]
        pub fn table_schema() -> ::diff::diff_types::TableSchema {
            let mut schema = ::diff::diff_types::TableSchema {
                table_name: #table_name.to_string(),
                fields: vec![
                    #(#field_schemas),*
                ],
            };
            schema
        }

        /// 获取实体对应的表名
        #[inline]
        pub const fn table_name() -> &'static str {
            #table_name
        }
    }
}

/// 从结构体字段生成 FieldSchema 列表
fn generate_field_schemas(input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut schemas = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                // 检查是否有 #[schema(skip)] 属性
                let skip = field.attrs.iter().any(|attr| {
                    attr.path().is_ident("schema")
                        && attr.parse_args::<Ident>().map(|i| i == "skip").unwrap_or(false)
                });

                if skip {
                    continue;
                }

                if let Some(ident) = &field.ident {
                    let field_name = ident.to_string();
                    let ty = &field.ty;
                    let type_str = quote!(#ty).to_string();

                    // 获取默认值（如果指定了）
                    let default_value = extract_default_value(&field)
                        .unwrap_or_else(|| get_type_default(&type_str).to_string());

                    schemas.push(quote! {
                        ::diff::diff_types::FieldSchema {
                            field_name: #field_name.to_string(),
                            field_type: stringify!(#ty).to_string(),
                            default_value: #default_value.to_string(),
                        }
                    });
                }
            }
        }
    }

    schemas
}

/// 从字段属性提取默认值
fn extract_default_value(field: &syn::Field) -> Option<String> {
    for attr in &field.attrs {
        if attr.path().is_ident("schema") {
            if let Ok(meta) = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            {
                for item in meta {
                    if let Meta::NameValue(nv) = item {
                        if nv.path.is_ident("default") {
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

/// 根据类型获取默认值字符串
fn get_type_default(type_str: &str) -> &'static str {
    match type_str {
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128"
        | "isize" => "0",
        "f32" | "f64" => "0.0",
        "bool" => "false",
        "String" => "\"\"",
        _ => "",
    }
}
