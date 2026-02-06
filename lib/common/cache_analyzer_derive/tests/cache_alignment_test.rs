use cache_analyzer_derive::CacheAnalyzer;

// 测试1：小结构体应该通过缓存行检查
#[derive(CacheAnalyzer)]
struct SmallStruct {
    a: u32,
    b: u32,
}

#[test]
fn test_small_struct_cache_analysis() {
    let report = SmallStruct::detailed_cache_analysis();

    println!("\n=== SmallStruct 分析 ===");
    println!("大小: {} 字节", report.total_size);
    println!("缓存行数: {}", report.cache_lines_needed);

    assert!(report.total_size <= 64);
    assert_eq!(report.cache_lines_needed, 1);
}

// 测试2：缓存行对齐的结构体
#[repr(align(64))]
#[derive(CacheAnalyzer)]
struct AlignedStruct {
    #[hot]
    counter: u64,
    data: [u8; 56],
}

#[test]
fn test_aligned_struct() {
    let report = AlignedStruct::detailed_cache_analysis();

    println!("\n=== AlignedStruct 分析 ===");
    println!("大小: {} 字节", report.total_size);
    println!("对齐: {} 字节", report.alignment);
    println!("缓存行数: {}", report.cache_lines_needed);

    // repr(align(64)) 应该将对齐设为 64
    assert_eq!(report.alignment, 64);
    assert_eq!(report.cache_lines_needed, 1);
}

// 测试3：带热点字段的结构体
#[derive(CacheAnalyzer)]
struct HotFieldStruct {
    #[hot]
    id: u64,
    #[hot]
    timestamp: u64,
    metadata: [u8; 48],
}

#[test]
fn test_hot_fields() {
    let report = HotFieldStruct::detailed_cache_analysis();

    println!("\n=== HotFieldStruct 分析 ===");
    for field in &report.field_analyses {
        let hot_marker = if field.is_hot { " [HOT]" } else { "" };
        println!("  {}{}: offset={}, size={}", field.name, hot_marker, field.offset, field.size);
    }

    // 检查热点字段标记
    let hot_fields: Vec<_> =
        report.field_analyses.iter().filter(|f| f.is_hot).map(|f| f.name.as_str()).collect();

    assert!(hot_fields.contains(&"id"));
    assert!(hot_fields.contains(&"timestamp"));
}

// 测试4：优化的字段顺序
#[derive(CacheAnalyzer)]
struct OptimizedOrder {
    b: u64, // 8 字节，最大对齐
    d: u32, // 4 字节
    c: u16, // 2 字节
    a: u8,  // 1 字节
}

#[test]
fn test_optimized_order() {
    let report = OptimizedOrder::detailed_cache_analysis();

    println!("\n=== OptimizedOrder 分析 ===");
    println!("填充: {} 字节 ({:.1}%)", report.padding_bytes, report.padding_percentage);
    println!("当前顺序是否最优: {}", report.is_current_order_optimal);

    // 优化的顺序应该减少填充
    assert!(report.padding_percentage < 10.0, "优化的顺序应该有较少的填充");
}

// 测试5：使用 repr(C) 的结构体
#[repr(C)]
#[derive(CacheAnalyzer)]
struct CReprStruct {
    a: u32,
    b: u64,
}

#[test]
fn test_c_repr() {
    let report = CReprStruct::detailed_cache_analysis();

    println!("\n=== CReprStruct 分析 ===");
    println!("大小: {} 字节", report.total_size);

    // repr(C) 确保字段按声明顺序排列
    assert!(report.total_size >= 12); // 至少 4 + 8 字节
}
