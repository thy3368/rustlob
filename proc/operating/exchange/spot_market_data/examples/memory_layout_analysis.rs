//! OrderDelta 内存布局分析工具
//!
//! 用于分析和优化 OrderDelta 结构体的内存布局
//!
//! 运行方式：
//! ```bash
//! cargo run --example memory_layout_analysis
//! ```

use std::mem::{align_of, offset_of, size_of};

use lob::lob::{Side, TraderId};
use spot_market_data::domain::entity::level_types::{OrderChangeType, OrderDelta};

fn main() {
    println!("=== OrderDelta 内存布局分析 ===\n");

    // 整体信息
    println!("📦 整体信息:");
    println!("  总大小: {} 字节", size_of::<OrderDelta>());
    println!("  对齐要求: {} 字节", align_of::<OrderDelta>());
    println!();

    // 各字段类型大小
    println!("📏 字段类型大小:");
    println!("  u32 (SymbolId, Price, Quantity): {} 字节", size_of::<u32>());
    println!("  u64 (timestamp, sequence, OrderId): {} 字节", size_of::<u64>());
    println!("  OrderChangeType (enum): {} 字节", size_of::<OrderChangeType>());
    println!("  Side (enum): {} 字节", size_of::<Side>());
    println!("  TraderId ([u8; 8]): {} 字节", size_of::<TraderId>());
    println!("  Option<TraderId>: {} 字节", size_of::<Option<TraderId>>());
    println!();

    // 对齐要求
    println!("🎯 对齐要求:");
    println!("  u32: {} 字节", align_of::<u32>());
    println!("  u64: {} 字节", align_of::<u64>());
    println!("  OrderChangeType: {} 字节", align_of::<OrderChangeType>());
    println!("  Side: {} 字节", align_of::<Side>());
    println!("  Option<TraderId>: {} 字节", align_of::<Option<TraderId>>());
    println!();

    // 当前布局分析
    println!("🔍 当前字段布局（推测）:");
    println!("  当前顺序:");
    println!("    1. symbol_id: u32        (4 字节)");
    println!("    2. timestamp: u64        (8 字节) - 需要 4 字节 padding");
    println!("    3. sequence: u64         (8 字节)");
    println!("    4. change_type: enum     (1 字节)");
    println!("    5. order_id: u64         (8 字节) - 需要 7 字节 padding");
    println!("    6. side: enum            (1 字节)");
    println!("    7. price: u32            (4 字节) - 需要 3 字节 padding");
    println!("    8. quantity: u32         (4 字节)");
    println!("    9. trader_id: Option<[u8;8]> (9 字节) - 需要 padding");
    println!();

    // 计算理论最小大小
    let theoretical_min = 4 +  // symbol_id
        8 +  // timestamp
        8 +  // sequence
        1 +  // change_type
        8 +  // order_id
        1 +  // side
        4 +  // price
        4 +  // quantity
        9; // trader_id (Option<[u8;8]>)

    let actual_size = size_of::<OrderDelta>();
    let padding = actual_size - theoretical_min;

    println!("💾 内存使用统计:");
    println!("  理论最小大小: {} 字节", theoretical_min);
    println!("  实际大小: {} 字节", actual_size);
    println!("  Padding 浪费: {} 字节 ({:.1}%)", padding, (padding as f64 / actual_size as f64) * 100.0);
    println!();

    // 优化建议
    println!("✨ 优化建议:");
    println!();

    println!("方案 1: 字段重排（减少 padding）");
    println!("  建议顺序（按对齐要求从大到小）:");
    println!("    // 8 字节对齐字段");
    println!("    timestamp: u64,");
    println!("    sequence: u64,");
    println!("    order_id: u64,");
    println!("    trader_id: Option<TraderId>,  // 9 字节");
    println!("    // 4 字节对齐字段");
    println!("    symbol_id: u32,");
    println!("    price: u32,");
    println!("    quantity: u32,");
    println!("    // 1 字节字段（放在最后）");
    println!("    change_type: OrderChangeType,");
    println!("    side: Side,");
    println!("  预期大小: 48-56 字节");
    println!();

    println!("方案 2: 使用 #[repr(C)] 控制布局");
    println!("  优点: 可预测的内存布局");
    println!("  缺点: 可能增加 padding");
    println!();

    println!("方案 3: 位域压缩（极致优化）");
    println!("  将 change_type (3种状态) 和 side (2种状态) 压缩到 1 字节");
    println!("  使用 bitflags 或手动位操作");
    println!("  节省: 1 字节 + 减少 padding");
    println!();

    println!("方案 4: 分离可选字段");
    println!("  将 Option<TraderId> 移到单独的结构体");
    println!("  使用 HashMap<OrderId, TraderId> 存储");
    println!("  优点: 大多数情况下节省 9 字节");
    println!("  缺点: 需要额外查找");
    println!();

    println!("方案 5: 使用更紧凑的类型");
    println!("  如果业务允许:");
    println!("    - timestamp: u64 -> u32 (相对时间戳)");
    println!("    - sequence: u64 -> u32 (如果范围足够)");
    println!("  潜在节省: 8-16 字节");
    println!();

    // 缓存行分析
    println!("🚀 缓存行分析:");
    let cache_line_size = 64;
    let structs_per_cache_line = cache_line_size / actual_size;
    println!("  缓存行大小: {} 字节", cache_line_size);
    println!("  每个缓存行可容纳: {} 个 OrderDelta", structs_per_cache_line);
    println!("  100 个实例占用: {} 个缓存行", (100 * actual_size + cache_line_size - 1) / cache_line_size);
    println!();

    // 批量分配分析
    println!("📊 批量分配分析:");
    for count in [10, 50, 100, 500, 1000] {
        let total_bytes = count * actual_size;
        let kb = total_bytes as f64 / 1024.0;
        println!("  {} 个实例: {} 字节 ({:.2} KB)", count, total_bytes, kb);
    }
}
