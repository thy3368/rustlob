use cache_analyzer_derive::CacheAnalyzer;
use cache_analyzer_types::CacheAnalysisReport;

/// 未优化的结构体 - 字段顺序导致大量填充
#[derive(CacheAnalyzer, Debug)]
pub struct UnoptimizedStruct {
    flag: bool,        // 1 字节
    value: u64,        // 8 字节
    counter: u32,      // 4 字节
    small_flag: bool,  // 1 字节
}

/// 优化后的结构体 - 按大小降序排列
#[derive(CacheAnalyzer, Debug)]
pub struct OptimizedStruct {
    value: u64,        // 8 字节
    counter: u32,      // 4 字节
    flag: bool,        // 1 字节
    small_flag: bool,  // 1 字节
}

/// 带热点标记的交易订单
#[derive(CacheAnalyzer, Debug)]
pub struct TradingOrder {
    #[hot]
    price: f64,

    #[hot]
    quantity: f64,

    order_id: u64,
    timestamp: u64,

    #[cold]
    user_metadata: String,
}

fn main() {
    println!("=== 缓存友好性分析示例 ===\n");

    // 分析未优化的结构体
    println!("1. 未优化的结构体分析:");
    println!("{:-<60}", "");
    analyze_struct::<UnoptimizedStruct>();

    println!("\n");

    // 分析优化后的结构体
    println!("2. 优化后的结构体分析:");
    println!("{:-<60}", "");
    analyze_struct::<OptimizedStruct>();

    println!("\n");

    // 分析带热点标记的结构体
    println!("3. 交易订单结构体分析:");
    println!("{:-<60}", "");
    analyze_trading_order();
}

fn analyze_struct<T>()
where
    T: HasCacheAnalysis,
{
    let report = T::get_cache_analysis();

    println!("结构体名称: {}", report.struct_name);
    println!("总大小: {} 字节", report.total_size);
    println!("对齐要求: {} 字节", report.alignment);
    println!("缓存行大小: {} 字节", report.cache_line_size);
    println!("需要缓存行数: {}", report.cache_lines_needed);
    println!("填充字节: {} 字节 ({:.1}%)",
             report.padding_bytes,
             report.padding_percentage);
    println!("字段顺序是否最优: {}",
             if report.is_current_order_optimal { "是" } else { "否" });

    println!("\n字段详情:");
    for field in &report.field_analyses {
        println!("  - {}: offset={}, size={}, align={}",
                 field.name,
                 field.offset,
                 field.size,
                 field.alignment);
    }

    if !report.suggestions.is_empty() {
        println!("\n优化建议:");
        for suggestion in &report.suggestions {
            println!("  • {}", suggestion);
        }
    }
}

fn analyze_trading_order() {
    let report = TradingOrder::detailed_cache_analysis();

    println!("结构体名称: {}", report.struct_name);
    println!("总大小: {} 字节", report.total_size);
    println!("对齐要求: {} 字节", report.alignment);
    println!("填充字节: {} 字节 ({:.1}%)",
             report.padding_bytes,
             report.padding_percentage);

    println!("\n字段分析（标记热点）:");
    for field in &report.field_analyses {
        let marker = if field.is_hot { "[HOT]" } else { "" };
        println!("  - {} {}: offset={}, size={}, align={}",
                 marker,
                 field.name,
                 field.offset,
                 field.size,
                 field.alignment);
    }

    if !report.suggestions.is_empty() {
        println!("\n优化建议:");
        for suggestion in &report.suggestions {
            println!("  • {}", suggestion);
        }
    }

    // 内存布局信息
    let layout = TradingOrder::memory_layout();
    println!("\n内存布局信息:");
    println!("  名称: {}", layout.name);
    println!("  大小: {} 字节", layout.size);
    println!("  对齐: {} 字节", layout.alignment);
    println!("  紧凑布局: {}", if layout.is_packed { "是" } else { "否" });
}

// 辅助 trait 用于统一接口
trait HasCacheAnalysis {
    fn get_cache_analysis() -> CacheAnalysisReport;
}

impl HasCacheAnalysis for UnoptimizedStruct {
    fn get_cache_analysis() -> CacheAnalysisReport {
        Self::detailed_cache_analysis()
    }
}

impl HasCacheAnalysis for OptimizedStruct {
    fn get_cache_analysis() -> CacheAnalysisReport {
        Self::detailed_cache_analysis()
    }
}
