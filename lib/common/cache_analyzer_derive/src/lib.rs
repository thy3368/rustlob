use cache_analyzer_types::validation::{CompileTimeValidation, validate_cache_friendly};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// CacheAnalyzer 派生宏 - 分析结构体缓存友好性
///
/// # 功能
/// - 分析结构体的内存布局
/// - 计算填充字节和对齐要求
/// - 提供缓存行使用分析
/// - 生成优化建议
/// - **编译时验证缓存友好性**
///
/// # 属性
/// - `#[hot]` - 标记热点字段（频繁访问）
/// - `#[cold]` - 标记冷字段（不频繁访问）
/// - `#[cache(strict)]` - 启用严格模式，强制缓存友好性检查
/// - `#[cache(max_size = N)]` - 设置最大结构体大小限制
/// - `#[cache(max_padding = N)]` - 设置最大填充比例（百分比）
/// - `#[cache(enforce_order)]` - 强制字段按对齐顺序排列
///
/// # 编译时检查
/// 当使用特定属性时，宏会在编译时验证：
/// - 结构体大小不超过限制
/// - 填充比例在合理范围内
/// - 字段顺序是否最优
/// - 热点字段是否在结构体前部
///
/// # 生成的方法
/// - `detailed_cache_analysis()` - 获取详细的缓存分析报告
/// - `optimization_suggestions()` - 获取优化建议列表
/// - `memory_layout()` - 获取内存布局信息
///
/// # 生成的公共类型（所有结构体共享）
/// - `cache_analyzer_types::FieldAnalysis` - 字段分析信息
/// - `cache_analyzer_types::CacheAnalysisReport` - 详细缓存分析报告
/// - `cache_analyzer_types::MemoryLayout` - 内存布局信息
///
/// # 示例
///
/// ## 基础使用
/// ```ignore
/// use cache_analyzer_derive::CacheAnalyzer;
///
/// #[derive(CacheAnalyzer)]
/// pub struct OrderBook {
///     #[hot]
///     best_bid: f64,
///     #[hot]
///     best_ask: f64,
///     #[cold]
///     last_update_time: u64,
/// }
///
/// let report = OrderBook::detailed_cache_analysis();
/// println!("结构体大小: {} 字节", report.total_size);
/// ```
///
/// ## 严格模式（编译时检查）
/// ```ignore
/// // 这将在编译时验证缓存友好性
/// #[derive(CacheAnalyzer)]
/// #[cache(enforce_order)]  // 强制字段按对齐顺序排列
/// pub struct HighPerfStruct {
///     value: u64,    // 8 字节对齐
///     counter: u32,  // 4 字节对齐
///     flag: bool,    // 1 字节对齐
/// }
/// ```
///
/// ## 编译时错误示例
/// ```ignore
/// // ❌ 编译错误：字段顺序不优
/// #[derive(CacheAnalyzer)]
/// #[cache(enforce_order)]
/// pub struct BadLayout {
///     flag: bool,    // 1 字节 - 错误！应该放在最后
///     value: u64,    // 8 字节
///     counter: u32,  // 4 字节
/// }
/// // Error: 结构体 BadLayout 的字段顺序不是最优的
/// ```
#[proc_macro_derive(CacheAnalyzer, attributes(cache, hot, cold))]
pub fn cache_analyzer_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let expanded = impl_cache_analyzer(&ast);

    TokenStream::from(expanded)
}

fn impl_cache_analyzer(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let cache_line_size: usize = 64; // 默认缓存行大小

    // 解析 cache 属性配置
    let validation_config = parse_cache_attributes_from_ast(ast);

    // 编译时验证缓存友好性
    if let Err(error_msg) = validate_cache_friendly(ast, &validation_config) {
        // 使用 compile_error! 在编译时报错
        return quote! {
            compile_error!(#error_msg);
        };
    }
    // 收集字段信息
    let mut fields_info = Vec::new();

    if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for (i, field) in fields_named.named.iter().enumerate() {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                // 检查字段属性（hot/cold）
                let mut is_hot = false;
                for attr in &field.attrs {
                    if attr.path().is_ident("hot") {
                        is_hot = true;
                    }
                }

                fields_info.push((i, field_name.clone(), field_type.clone(), is_hot));
            }
        }
    }

    // 生成字段分析代码
    let field_analyses: Vec<_> = fields_info
        .iter()
        .enumerate()
        .map(|(_field_idx, (_idx, field_name, field_type, is_hot))| {
            let field_name_str = field_name.to_string();

            quote! {
                cache_analyzer_types::FieldAnalysis {
                    name: #field_name_str.to_string(),
                    offset: unsafe {
                        let base = core::ptr::null::<#name>();
                        let field = core::ptr::addr_of!((*base).#field_name);
                        (field as usize).wrapping_sub(base as usize)
                    },
                    size: core::mem::size_of::<#field_type>(),
                    alignment: core::mem::align_of::<#field_type>(),
                    is_hot: #is_hot,
                }
            }
        })
        .collect();

    let field_count_lit = syn::Index::from(fields_info.len());

    quote! {
        impl #name {
            /// 详细的缓存分析报告
            ///
            /// 返回包含完整内存布局分析的报告，包括：
            /// - 结构体大小和对齐
            /// - 缓存行使用情况
            /// - 字段详细信息
            /// - 填充字节分析
            /// - 优化建议
            pub fn detailed_cache_analysis() -> cache_analyzer_types::CacheAnalysisReport {
                let field_analyses = vec![
                    #(#field_analyses),*
                ];

                let total_size = std::mem::size_of::<Self>();
                let alignment = std::mem::align_of::<Self>();
                let cache_line_size_usize: usize = #cache_line_size;

                // 计算各种指标
                let cache_lines_needed = (total_size + cache_line_size_usize - 1) / cache_line_size_usize;

                // 分析字段排序
                let optimal_order = cache_analyzer_types::CacheAnalysisReport::calculate_optimal_field_order(&field_analyses);
                let current_order: Vec<usize> = (0..field_analyses.len()).collect();
                let is_optimal = cache_analyzer_types::CacheAnalysisReport::is_order_optimal(&current_order, &optimal_order, &field_analyses);

                // 计算填充
                let padding = cache_analyzer_types::CacheAnalysisReport::calculate_padding(&field_analyses);
                let padding_percentage = if total_size > 0 {
                    (padding as f32 / total_size as f32) * 100.0
                } else {
                    0.0
                };

                // 生成优化建议
                let mut suggestions = Vec::new();

                if padding_percentage > 20.0 {
                    suggestions.push(format!(
                        "结构体 {} 有 {:.1}% 的填充空间，考虑重新排列字段",
                        stringify!(#name), padding_percentage
                    ));
                }

                if !is_optimal {
                    suggestions.push("当前字段顺序不是最优的，建议按照对齐和大小降序排列".to_string());
                }

                if total_size > 64 {
                    suggestions.push(format!(
                        "结构体大小 {} 字节超过常见缓存行大小(64字节)，考虑拆分",
                        total_size
                    ));
                }

                if cache_lines_needed > 1 {
                    suggestions.push(format!(
                        "需要访问 {} 个缓存行，考虑优化布局",
                        cache_lines_needed
                    ));
                }

                cache_analyzer_types::CacheAnalysisReport {
                    struct_name: stringify!(#name).to_string(),
                    total_size,
                    alignment,
                    cache_line_size: cache_line_size_usize,
                    cache_lines_needed,
                    field_count: #field_count_lit,
                    field_analyses,
                    padding_bytes: padding,
                    padding_percentage,
                    optimal_field_order: optimal_order,
                    is_current_order_optimal: is_optimal,
                    suggestions,
                }
            }

            /// 获取优化建议
            ///
            /// 返回基于缓存分析的优化建议列表
            pub fn optimization_suggestions() -> Vec<String> {
                let report = Self::detailed_cache_analysis();
                report.suggestions
            }

            /// 获取内存布局信息
            ///
            /// 返回结构体的基本内存布局信息
            pub fn memory_layout() -> cache_analyzer_types::MemoryLayout {
                cache_analyzer_types::MemoryLayout {
                    name: stringify!(#name).to_string(),
                    size: std::mem::size_of::<Self>(),
                    alignment: std::mem::align_of::<Self>(),
                    is_packed: false,
                }
            }
        }
    }
}

/// 从 AST 解析 cache 属性配置
fn parse_cache_attributes_from_ast(ast: &DeriveInput) -> CompileTimeValidation {
    let mut config = CompileTimeValidation::default();

    for attr in &ast.attrs {
        if attr.path().is_ident("cache") {
            // 解析属性内容
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("strict") {
                    config.strict_mode = true;
                    return Ok(());
                }

                if meta.path.is_ident("enforce_order") {
                    config.enforce_optimal_order = true;
                    return Ok(());
                }

                if meta.path.is_ident("max_size") {
                    if let Ok(lit) = meta.value()?.parse::<syn::LitInt>() {
                        if let Ok(val) = lit.base10_parse::<usize>() {
                            config.max_struct_size = val;
                        }
                    }
                    return Ok(());
                }

                if meta.path.is_ident("max_padding") {
                    if let Ok(lit) = meta.value()?.parse::<syn::LitFloat>() {
                        if let Ok(val) = lit.base10_parse::<f32>() {
                            config.max_padding_percentage = val;
                        }
                    }
                    return Ok(());
                }

                if meta.path.is_ident("max_cache_lines") {
                    if let Ok(lit) = meta.value()?.parse::<syn::LitInt>() {
                        if let Ok(val) = lit.base10_parse::<usize>() {
                            config.max_cache_lines = val;
                        }
                    }
                    return Ok(());
                }

                if meta.path.is_ident("check_false_sharing") {
                    config.check_false_sharing = true;
                    return Ok(());
                }

                Ok(())
            });
        }
    }

    config
}
