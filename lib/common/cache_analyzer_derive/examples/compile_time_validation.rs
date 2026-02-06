/// 编译时缓存友好性验证示例
///
/// 此文件展示如何使用 CacheAnalyzer 的编译时检查功能
use cache_analyzer_derive::CacheAnalyzer;

// ✅ 示例 1: 基础使用（无编译时检查）
#[derive(CacheAnalyzer, Debug)]
pub struct BasicStruct {
    value: u64,
    counter: u32,
    flag: bool,
}

// ✅ 示例 2: 最优字段顺序（按对齐降序）
#[derive(CacheAnalyzer, Debug)]
#[cache(enforce_order)]
pub struct OptimalOrderStruct {
    // 8 字节对齐
    value: u64,
    // 4 字节对齐
    counter: u32,
    // 1 字节对齐
    flag: bool,
}

// ❌ 示例 3: 错误的字段顺序（会导致编译错误）
// 取消注释下面的代码会导致编译错误
/*
#[derive(CacheAnalyzer, Debug)]
#[cache(enforce_order)]
pub struct BadOrderStruct {
    flag: bool,    // ❌ 错误! 小对齐字段不应该在前面
    value: u64,
    counter: u32,
}
// Error: 结构体 BadOrderStruct 的字段顺序不是最优的
 */

// ✅ 示例 4: 严格模式（热点字段必须在前）
#[derive(CacheAnalyzer, Debug)]
#[cache(strict)]
pub struct StrictModeStruct {
    #[hot]
    price: f64,
    #[hot]
    quantity: f64,
    timestamp: u64, // 非热点字段
}

// ❌ 示例 5: 严格模式 - 热点字段顺序错误
// 取消注释会导致编译错误
/*
#[derive(CacheAnalyzer, Debug)]
#[cache(strict)]
pub struct BadHotFieldsStruct {
    timestamp: u64,  // 非热点字段
    #[hot]        // ❌ 错误! 热点字段应该在前面
    price: f64,
}
// Error: 结构体 BadHotFieldsStruct 的热点字段 'price' 不在结构体开头
*/

// ✅ 示例 6: 自定义大小限制
#[derive(CacheAnalyzer, Debug)]
#[cache(max_size = 64)]
pub struct SizeLimitedStruct {
    data1: u64,
    data2: u64,
    data3: u64,
    data4: u64,
    // 总共 32 字节，符合 64 字节限制
}

// ❌ 示例 7: 超过大小限制
// 取消注释会导致编译错误
/*
#[derive(CacheAnalyzer, Debug)]
#[cache(max_size = 32)]
pub struct OversizedStruct {
    data1: u64,
    data2: u64,
    data3: u64,
    data4: u64,
    data5: u64,
    // 总共 40 字节，超过 32 字节限制
}
// Error: 结构体 OversizedStruct 估算大小 40 字节超过最大限制 32 字节
*/

// ✅ 示例 8: 填充比例限制
#[derive(CacheAnalyzer, Debug)]
#[cache(max_padding = 15.0)]
pub struct LowPaddingStruct {
    value1: u64,  // 8 字节
    value2: u64,  // 8 字节
    counter: u32, // 4 字节
    flag: bool,   // 1 字节 + 3 字节填充
                  // 总共 24 字节，填充 3 字节 (12.5%)
}

// ✅ 示例 9: 组合多个检查
#[derive(CacheAnalyzer, Debug)]
#[cache(enforce_order, max_size = 64, max_padding = 20.0)]
pub struct ComprehensiveStruct {
    // 按对齐降序排列
    large_value: u64,
    medium_value: u32,
    small_value: u8,
}

// ✅ 示例 10: 缓存行限制
#[derive(CacheAnalyzer, Debug)]
#[cache(max_cache_lines = 1)]
pub struct SingleCacheLineStruct {
    data1: u64,
    data2: u64,
    data3: u32,
    // 总共 20 字节，适合单个缓存行
}

fn main() {
    println!("=== 缓存友好性编译时检查示例 ===\n");

    // 示例 1: 基础结构
    let report1 = BasicStruct::detailed_cache_analysis();
    println!("1. 基础结构:");
    println!("   大小: {} 字节", report1.total_size);
    println!("   填充: {:.1}%", report1.padding_percentage);
    println!("   字段顺序最优: {}\n", report1.is_current_order_optimal);

    // 示例 2: 最优字段顺序
    let report2 = OptimalOrderStruct::detailed_cache_analysis();
    println!("2. 最优字段顺序:");
    println!("   大小: {} 字节", report2.total_size);
    println!("   填充: {:.1}%", report2.padding_percentage);
    println!("   ✓ 通过编译时顺序检查\n");

    // 示例 4: 严格模式
    let report4 = StrictModeStruct::detailed_cache_analysis();
    println!("4. 严格模式（热点字段在前）:");
    println!("   大小: {} 字节", report4.total_size);
    for field in &report4.field_analyses {
        let marker = if field.is_hot { "[HOT]" } else { "" };
        println!("   {} {} at offset {}", marker, field.name, field.offset);
    }
    println!("   ✓ 通过编译时热点字段检查\n");

    // 示例 6: 大小限制
    let report6 = SizeLimitedStruct::detailed_cache_analysis();
    println!("6. 大小限制检查:");
    println!("   大小: {} 字节 (限制: 64 字节)", report6.total_size);
    println!("   缓存行数: {}", report6.cache_lines_needed);
    println!("   ✓ 通过编译时大小检查\n");

    // 示例 9: 组合检查
    let report9 = ComprehensiveStruct::detailed_cache_analysis();
    println!("9. 组合检查:");
    println!("   大小: {} 字节", report9.total_size);
    println!("   填充: {:.1}%", report9.padding_percentage);
    println!("   字段顺序最优: {}", report9.is_current_order_optimal);
    println!("   ✓ 通过所有编译时检查\n");

    println!("编译时检查总结:");
    println!("  ✓ enforce_order - 强制字段按对齐顺序排列");
    println!("  ✓ strict - 严格模式，热点字段必须在前");
    println!("  ✓ max_size - 限制结构体最大大小");
    println!("  ✓ max_padding - 限制填充比例");
    println!("  ✓ max_cache_lines - 限制缓存行数量");
}
