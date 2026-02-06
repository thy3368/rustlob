use syn::{Data, DeriveInput, Fields, Ident, Type};

/// 编译时验证配置
#[derive(Debug, Clone)]
pub struct CompileTimeValidation {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 最大允许的填充比例（百分比）
    pub max_padding_percentage: f32,
    /// 最大允许的缓存行数
    pub max_cache_lines: usize,
    /// 是否强制字段按对齐顺序排列
    pub enforce_optimal_order: bool,
    /// 最大结构体大小（字节）
    pub max_struct_size: usize,
    /// 是否检查伪共享风险
    pub check_false_sharing: bool,
    /// 缓存行大小（字节，默认64）
    pub cache_line_size: usize,
}

impl Default for CompileTimeValidation {
    fn default() -> Self {
        Self {
            strict_mode: false,
            max_padding_percentage: 30.0,
            max_cache_lines: 2,
            enforce_optimal_order: false,
            max_struct_size: 128,
            check_false_sharing: false,
            cache_line_size: 64,
        }
    }
}

/// 字段信息
#[derive(Clone)]
pub struct FieldInfo {
    pub name: String,
    pub ty: Type,
    pub is_hot: bool,
    /// 是否标记为多线程访问
    pub is_thread_local: bool,
}

impl FieldInfo {
    /// 估算字段大小（编译时）
    pub fn estimated_size(&self) -> Option<usize> {
        estimate_type_size(&self.ty)
    }

    /// 估算字段对齐（编译时）
    pub fn estimated_alignment(&self) -> Option<usize> {
        estimate_type_alignment(&self.ty)
    }

    /// 检查字段是否可能被多线程访问
    pub fn is_potentially_shared(&self) -> bool {
        // 检查类型是否包含原子类型或同步原语
        is_atomic_or_sync_type(&self.ty) || !self.is_thread_local
    }
}

/// 验证结构体是否缓存友好
///
/// 如果启用了严格模式或特定检查,会在编译时报错
pub fn validate_cache_friendly(
    ast: &DeriveInput,
    config: &CompileTimeValidation,
) -> Result<(), String> {
    let struct_name = &ast.ident;
    let fields = extract_fields(ast)?;

    // 1. 检查字段顺序是否最优
    if config.enforce_optimal_order {
        check_optimal_field_order(&fields, struct_name)?;
    }

    // 2. 检查结构体是否按缓存行大小对齐
    check_cache_line_alignment(ast, &fields, struct_name, config.cache_line_size)?;

    // 3. 检查伪共享风险
    if config.check_false_sharing {
        check_false_sharing_risk(&fields, struct_name, config.cache_line_size)?;
    }

    // 3. 估算结构体大小
    let estimated_size = estimate_struct_size(&fields);
    if let Some(size) = estimated_size {
        if size > config.max_struct_size {
            return Err(format!(
                "结构体 {} 估算大小 {} 字节超过最大限制 {} 字节\n\
                建议:\n\
                - 将大字段拆分到单独的结构体\n\
                - 使用 Box<T> 或引用类型减小内联大小\n\
                - 考虑使用 #[repr(C)] 并手动优化布局",
                struct_name, size, config.max_struct_size
            ));
        }

        // 计算需要的缓存行数
        let cache_lines_needed = (size + 63) / 64;
        if cache_lines_needed > config.max_cache_lines {
            return Err(format!(
                "结构体 {} 需要 {} 个缓存行（估算大小 {} 字节），超过限制 {} 个缓存行\n\
                建议:\n\
                - 拆分为更小的结构体\n\
                - 将冷数据字段移到单独的结构体\n\
                - 使用指针或引用间接访问大字段",
                struct_name, cache_lines_needed, size, config.max_cache_lines
            ));
        }
    }

    // 3. 估算填充比例
    if let Some(padding_pct) = estimate_padding_percentage(&fields) {
        if padding_pct > config.max_padding_percentage {
            return Err(format!(
                "结构体 {} 估算填充比例 {:.1}% 超过限制 {:.1}%\n\
                建议:\n\
                - 按对齐要求降序排列字段（大对齐在前）\n\
                - 将小字段聚合在一起\n\
                - 使用 #[repr(C)] 或 #[repr(packed)] 控制布局",
                struct_name, padding_pct, config.max_padding_percentage
            ));
        }
    }

    // 4. 检查热点字段是否在结构体开头
    if config.strict_mode {
        check_hot_fields_first(&fields, struct_name)?;
    }

    Ok(())
}

/// 检查结构体是否按缓存行大小对齐
///
/// 此函数检查以下几个方面：
/// 1. 结构体是否跨越多个缓存行（可能导致缓存行分割）
/// 2. 如果启用了 #[repr(align(N))]，检查对齐值是否合理
/// 3. 检测可能导致缓存行分割的边界情况
fn check_cache_line_alignment(
    ast: &DeriveInput,
    fields: &[FieldInfo],
    struct_name: &Ident,
    cache_line_size: usize,
) -> Result<(), String> {
    // 1. 检查是否有 repr(align) 属性
    let mut explicit_alignment: Option<usize> = None;
    let mut has_repr_c = false;
    let mut has_repr_packed = false;

    for attr in &ast.attrs {
        if attr.path().is_ident("repr") {
            // 解析 repr 属性
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("C") {
                    has_repr_c = true;
                    Ok(())
                } else if meta.path.is_ident("packed") {
                    has_repr_packed = true;
                    Ok(())
                } else if meta.path.is_ident("align") {
                    // 解析 align(N) 的值
                    let content;
                    syn::parenthesized!(content in meta.input);
                    let lit: syn::LitInt = content.parse()?;
                    explicit_alignment = lit.base10_parse().ok();
                    Ok(())
                } else {
                    Ok(())
                }
            });
        }
    }

    // 2. 估算结构体的实际大小和自然对齐
    let struct_size = estimate_struct_size(fields);
    let max_field_alignment =
        fields.iter().filter_map(|f| f.estimated_alignment()).max().unwrap_or(1);

    let natural_alignment = if has_repr_packed { 1 } else { max_field_alignment };

    let effective_alignment = explicit_alignment.unwrap_or(natural_alignment);

    // 3. 检查结构体大小是否跨越多个缓存行
    if let Some(size) = struct_size {
        let cache_lines_needed = (size + cache_line_size - 1) / cache_line_size;

        // 3.1 检查是否有缓存行分割风险
        if cache_lines_needed > 1 && effective_alignment < cache_line_size {
            let mut warnings = Vec::new();

            warnings.push(format!(
                "结构体大小为 {} 字节，跨越 {} 个缓存行（缓存行大小 {} 字节）",
                size, cache_lines_needed, cache_line_size
            ));

            warnings
                .push(format!("当前对齐为 {} 字节，可能导致缓存行分割问题", effective_alignment));

            // 计算最坏情况：结构体实例可能跨越的缓存行数
            let worst_case_lines = if size % cache_line_size == 0 {
                cache_lines_needed
            } else {
                cache_lines_needed + 1
            };

            if worst_case_lines > cache_lines_needed {
                warnings
                    .push(format!("最坏情况下，单个实例可能跨越 {} 个缓存行", worst_case_lines));
            }

            return Err(format!(
                "结构体 {} 存在缓存行对齐问题：\n{}\n\n\
                缓存行分割会导致：\n\
                - 单次访问需要加载多个缓存行\n\
                - 增加缓存未命中率\n\
                - 降低内存带宽利用率\n\
                - 在多核场景下增加缓存一致性开销\n\n\
                建议的优化方案：\n\
                1. 使用 #[repr(align({}))] 将结构体对齐到缓存行边界：\n\
                   #[repr(align({}))]\n\
                   struct {} {{ ... }}\n\n\
                2. 如果结构体较大，考虑拆分为更小的结构体：\n\
                   - 将热点字段（频繁访问）分离到单独的结构体\n\
                   - 使用 Box<T> 或引用间接访问冷数据\n\n\
                3. 如果结构体必须跨越多个缓存行，确保：\n\
                   - 热点字段在第一个缓存行内\n\
                   - 使用 #[repr(C)] 固定字段布局\n\
                   - 按访问频率组织字段",
                struct_name,
                warnings.join("\n"),
                cache_line_size,
                cache_line_size,
                struct_name
            ));
        }

        // 3.2 检查显式对齐是否合理
        if let Some(align) = explicit_alignment {
            let mut warnings = Vec::new();

            // 检查对齐是否为 2 的幂
            if !align.is_power_of_two() {
                return Err(format!(
                    "结构体 {} 的 repr(align({})) 不是 2 的幂\n\
                    对齐值必须是 2 的幂（1, 2, 4, 8, 16, 32, 64, 128...）",
                    struct_name, align
                ));
            }

            // 检查过度对齐（对齐远大于结构体大小）
            if align > size * 2 && align > cache_line_size {
                warnings.push(format!(
                    "结构体 {} 的对齐 {} 字节远大于实际大小 {} 字节\n\
                    这可能导致内存浪费。考虑：\n\
                    - 使用 #[repr(align({}))] 对齐到缓存行\n\
                    - 或者移除不必要的显式对齐",
                    struct_name, align, size, cache_line_size
                ));
            }

            // 建议对齐到缓存行大小
            if align != cache_line_size && cache_lines_needed > 1 {
                warnings.push(format!(
                    "建议将对齐改为 {} 字节（缓存行大小）以优化缓存性能",
                    cache_line_size
                ));
            }

            if !warnings.is_empty() {
                eprintln!(
                    "警告: 结构体 {} 的对齐配置需要注意：\n{}",
                    struct_name,
                    warnings.join("\n")
                );
            }
        }

        // 3.3 检查热点字段是否在缓存行边界
        check_hot_fields_cache_alignment(fields, struct_name, cache_line_size)?;
    }

    Ok(())
}

/// 检查热点字段是否跨越缓存行边界
fn check_hot_fields_cache_alignment(
    fields: &[FieldInfo],
    struct_name: &Ident,
    cache_line_size: usize,
) -> Result<(), String> {
    let mut current_offset = 0;
    let mut warnings = Vec::new();

    for field in fields {
        let size = field.estimated_size().unwrap_or(8);
        let alignment = field.estimated_alignment().unwrap_or(8);

        // 计算对齐后的偏移
        let padding = (alignment - (current_offset % alignment)) % alignment;
        current_offset += padding;

        // 只检查热点字段
        if field.is_hot {
            // 检查字段是否跨越缓存行边界
            let field_start_line = current_offset / cache_line_size;
            let field_end_line = (current_offset + size - 1) / cache_line_size;

            if field_start_line != field_end_line {
                warnings.push(format!(
                    "热点字段 '{}' (偏移 {}, 大小 {}) 跨越缓存行 {} 和 {}",
                    field.name, current_offset, size, field_start_line, field_end_line
                ));
            }
        }

        current_offset += size;
    }

    if !warnings.is_empty() {
        return Err(format!(
            "结构体 {} 的热点字段存在缓存行对齐问题：\n{}\n\n\
            热点字段跨越缓存行边界会导致：\n\
            - 访问单个字段需要加载两个缓存行\n\
            - 显著降低缓存效率\n\
            - 增加内存访问延迟\n\n\
            建议：\n\
            1. 重新排列字段，确保热点字段不跨越 64 字节边界\n\
            2. 在热点字段前添加适当的填充：\n\
               struct {} {{\n\
                   _pad: [u8; PADDING_SIZE],  // 确保下一个字段对齐到缓存行\n\
                   #[hot]\n\
                   hot_field: Type,\n\
               }}\n\
            3. 将热点字段移到结构体开头，确保它们在第一个缓存行内",
            struct_name,
            warnings.join("\n"),
            struct_name
        ));
    }

    Ok(())
}

/// 提取结构体字段
fn extract_fields(ast: &DeriveInput) -> Result<Vec<FieldInfo>, String> {
    let mut fields_info = Vec::new();

    if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in fields_named.named.iter() {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let field_type = field.ty.clone();

                // 检查是否有 #[hot] 或 #[thread_local] 属性
                let mut is_hot = false;
                let mut is_thread_local = false;
                for attr in &field.attrs {
                    if attr.path().is_ident("hot") {
                        is_hot = true;
                    }
                    if attr.path().is_ident("thread_local") {
                        is_thread_local = true;
                    }
                }

                fields_info.push(FieldInfo {
                    name: field_name,
                    ty: field_type,
                    is_hot,
                    is_thread_local,
                });
            }
        } else {
            return Err("CacheAnalyzer 仅支持具名字段的结构体".to_string());
        }
    } else {
        return Err("CacheAnalyzer 仅支持结构体".to_string());
    }

    Ok(fields_info)
}

/// 检查字段顺序是否最优
fn check_optimal_field_order(fields: &[FieldInfo], struct_name: &Ident) -> Result<(), String> {
    let mut prev_alignment = usize::MAX;

    for (idx, field) in fields.iter().enumerate() {
        if let Some(alignment) = field.estimated_alignment() {
            if alignment > prev_alignment {
                // 找到建议的顺序
                let mut sorted_fields: Vec<_> = fields.iter().enumerate().collect();
                sorted_fields.sort_by(|(_, a), (_, b)| {
                    let align_a = a.estimated_alignment().unwrap_or(1);
                    let align_b = b.estimated_alignment().unwrap_or(1);
                    align_b.cmp(&align_a)
                });

                let suggested_order: Vec<String> =
                    sorted_fields.iter().map(|(_, f)| f.name.clone()).collect();

                return Err(format!(
                    "结构体 {} 的字段顺序不是最优的\n\
                    字段 '{}' (索引 {}) 的对齐 {} 大于前面字段的对齐 {}\n\n\
                    建议的字段顺序（按对齐降序）:\n{}\n\n\
                    当前字段顺序会产生不必要的填充字节，影响缓存效率。",
                    struct_name,
                    field.name,
                    idx,
                    alignment,
                    prev_alignment,
                    suggested_order.join("\n")
                ));
            }
            prev_alignment = alignment;
        }
    }

    Ok(())
}

/// 检查热点字段是否在前面
fn check_hot_fields_first(fields: &[FieldInfo], struct_name: &Ident) -> Result<(), String> {
    let mut seen_cold = false;

    for field in fields {
        if !field.is_hot && !seen_cold {
            seen_cold = true;
        }
        if field.is_hot && seen_cold {
            return Err(format!(
                "结构体 {} 的热点字段 '{}' 不在结构体开头\n\
                为了更好的缓存预取效果，所有 #[hot] 字段应该放在结构体前面",
                struct_name, field.name
            ));
        }
    }

    Ok(())
}

/// 估算结构体总大小
fn estimate_struct_size(fields: &[FieldInfo]) -> Option<usize> {
    let mut total_size = 0;
    let mut current_offset = 0;

    for field in fields {
        let size = field.estimated_size()?;
        let alignment = field.estimated_alignment()?;

        // 计算对齐填充
        let padding = (alignment - (current_offset % alignment)) % alignment;
        current_offset += padding;
        current_offset += size;
        total_size = current_offset;
    }

    // 最终对齐到结构体对齐
    let max_alignment = fields.iter().filter_map(|f| f.estimated_alignment()).max()?;

    let final_padding = (max_alignment - (total_size % max_alignment)) % max_alignment;
    total_size += final_padding;

    Some(total_size)
}

/// 估算填充比例
fn estimate_padding_percentage(fields: &[FieldInfo]) -> Option<f32> {
    let mut current_offset = 0;
    let mut total_padding = 0;

    for field in fields {
        let size = field.estimated_size()?;
        let alignment = field.estimated_alignment()?;

        // 计算对齐填充
        let padding = (alignment - (current_offset % alignment)) % alignment;
        total_padding += padding;
        current_offset += padding + size;
    }

    let total_size = estimate_struct_size(fields)?;

    if total_size > 0 {
        Some((total_padding as f32 / total_size as f32) * 100.0)
    } else {
        Some(0.0)
    }
}

/// 估算类型大小（编译时）
fn estimate_type_size(ty: &Type) -> Option<usize> {
    match ty {
        Type::Path(type_path) => {
            let segments = &type_path.path.segments;
            if segments.is_empty() {
                return None;
            }

            let last_segment = segments.last().unwrap();
            let type_name = last_segment.ident.to_string();

            match type_name.as_str() {
                // 原生类型
                "bool" => Some(1),
                "i8" | "u8" => Some(1),
                "i16" | "u16" => Some(2),
                "i32" | "u32" | "f32" => Some(4),
                "i64" | "u64" | "f64" => Some(8),
                "i128" | "u128" => Some(16),
                "isize" | "usize" => Some(8), // 假设 64 位平台

                // 指针和引用
                "Box" | "Arc" | "Rc" => Some(8),

                // 字符串
                "String" => Some(24), // Vec<u8> 的大小

                // Vec
                "Vec" => Some(24), // ptr + len + cap

                // Option 估算（保守估计）
                "Option" => Some(16),

                _ => None,
            }
        }
        Type::Reference(_) => Some(8), // 引用是指针大小
        Type::Ptr(_) => Some(8),       // 裸指针
        Type::Array(arr) => {
            // 数组大小 = 元素大小 * 长度
            if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(len), .. }) = &arr.len {
                let element_size = estimate_type_size(&arr.elem)?;
                let length: usize = len.base10_parse().ok()?;
                Some(element_size * length)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// 估算类型对齐（编译时）
fn estimate_type_alignment(ty: &Type) -> Option<usize> {
    match ty {
        Type::Path(type_path) => {
            let segments = &type_path.path.segments;
            if segments.is_empty() {
                return None;
            }

            let last_segment = segments.last().unwrap();
            let type_name = last_segment.ident.to_string();

            match type_name.as_str() {
                "bool" | "i8" | "u8" => Some(1),
                "i16" | "u16" => Some(2),
                "i32" | "u32" | "f32" => Some(4),
                "i64" | "u64" | "f64" | "isize" | "usize" => Some(8),
                "i128" | "u128" => Some(16),

                // 指针类型
                "Box" | "Arc" | "Rc" => Some(8),

                // 复合类型（保守估计）
                "String" | "Vec" | "Option" => Some(8),

                _ => None,
            }
        }
        Type::Reference(_) => Some(8),
        Type::Ptr(_) => Some(8),
        Type::Array(arr) => estimate_type_alignment(&arr.elem),
        _ => None,
    }
}

/// 解析 cache 属性配置
pub fn parse_cache_attributes(ast: &DeriveInput) -> CompileTimeValidation {
    let config = CompileTimeValidation::default();

    for attr in &ast.attrs {
        if attr.path().is_ident("cache") {
            let _ = attr.parse_args::<syn::MetaList>();
        }
    }

    config
}

/// 检查伪共享风险
///
/// 伪共享发生在多个线程访问同一缓存行中的不同变量时
/// 即使线程访问的是不同的变量,由于它们在同一缓存行中,会导致缓存行在CPU之间不断失效和同步
fn check_false_sharing_risk(
    fields: &[FieldInfo],
    struct_name: &Ident,
    cache_line_size: usize,
) -> Result<(), String> {
    if fields.len() < 2 {
        return Ok(()); // 单字段不可能有伪共享
    }

    // 计算每个字段的偏移量
    let mut field_offsets = Vec::new();
    let mut current_offset = 0;

    for field in fields {
        let size = field.estimated_size().unwrap_or(8);
        let alignment = field.estimated_alignment().unwrap_or(8);

        // 对齐
        let padding = (alignment - (current_offset % alignment)) % alignment;
        current_offset += padding;

        field_offsets.push((field, current_offset, size));
        current_offset += size;
    }

    // 检查是否有多个可能被不同线程访问的字段在同一缓存行
    let mut warnings = Vec::new();

    for i in 0..field_offsets.len() {
        for j in (i + 1)..field_offsets.len() {
            let (field_i, offset_i, _size_i) = &field_offsets[i];
            let (field_j, offset_j, _size_j) = &field_offsets[j];

            // 计算两个字段所在的缓存行
            let cache_line_i = offset_i / cache_line_size;
            let cache_line_j = offset_j / cache_line_size;

            // 如果在同一缓存行且可能被不同线程访问
            if cache_line_i == cache_line_j {
                let both_shared =
                    field_i.is_potentially_shared() && field_j.is_potentially_shared();
                let both_atomic =
                    is_atomic_or_sync_type(&field_i.ty) && is_atomic_or_sync_type(&field_j.ty);

                if both_shared || both_atomic {
                    warnings.push(format!(
                        "字段 '{}' (偏移 {}) 和 '{}' (偏移 {}) 在同一缓存行 {} 中",
                        field_i.name, offset_i, field_j.name, offset_j, cache_line_i
                    ));
                }
            }
        }
    }

    if !warnings.is_empty() {
        return Err(format!(
            "结构体 {} 存在伪共享(False Sharing)风险:\n{}\n\n\
            伪共享会导致严重的性能下降。建议:\n\
            - 使用 #[repr(align({}))] 将字段对齐到缓存行边界\n\
            - 在多线程访问的字段间添加填充(padding)隔离\n\
            - 将不同线程访问的字段分离到不同的结构体\n\
            - 使用 #[thread_local] 标记仅在单线程访问的字段\n\n\
            示例:\n\
            #[repr(align({}))]\n\
            struct {} {{\n\
                field1: AtomicU64,\n                _pad: [u8; {} - 8],  // 填充到缓存行边界\n\
                field2: AtomicU64,\n\
            }}",
            struct_name,
            warnings.join("\n"),
            cache_line_size,
            cache_line_size,
            struct_name,
            cache_line_size
        ));
    }

    Ok(())
}

/// 检查类型是否为原子类型或同步原语
fn is_atomic_or_sync_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        let segments = &type_path.path.segments;
        if segments.is_empty() {
            return false;
        }

        let last_segment = segments.last().unwrap();
        let type_name = last_segment.ident.to_string();

        matches!(
            type_name.as_str(),
            "AtomicBool"
                | "AtomicI8"
                | "AtomicI16"
                | "AtomicI32"
                | "AtomicI64"
                | "AtomicIsize"
                | "AtomicU8"
                | "AtomicU16"
                | "AtomicU32"
                | "AtomicU64"
                | "AtomicUsize"
                | "AtomicPtr"
                | "Mutex"
                | "RwLock"
                | "Atomic"
        )
    } else {
        false
    }
}
