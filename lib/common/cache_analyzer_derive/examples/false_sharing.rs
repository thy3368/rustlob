/// 伪共享(False Sharing)检查示例
///
/// 伪共享是多线程程序中常见的性能杀手
/// 当多个线程访问同一缓存行中的不同变量时,会导致缓存行频繁失效

use cache_analyzer_derive::CacheAnalyzer;
use std::sync::atomic::{AtomicU64, AtomicBool};

// ✅ 示例 1: 无伪共享风险 - 单线程访问
#[derive(CacheAnalyzer, Debug)]
pub struct SafeSingleThreaded {
    counter1: u64,
    counter2: u64,
    flag: bool,
}

// ❌ 示例 2: 伪共享风险 - 多个原子变量在同一缓存行
// 取消注释会导致编译错误
/*
#[derive(CacheAnalyzer)]
#[cache(check_false_sharing)]
pub struct FalseSharingRisk {
    counter1: AtomicU64,  // 8 字节
    counter2: AtomicU64,  // 8 字节 - 与 counter1 在同一缓存行!
    flag: AtomicBool,     // 1 字节
}
// Error: 结构体存在伪共享风险
*/

// ✅ 示例 3: 正确做法 - 使用对齐隔离原子变量
#[derive(CacheAnalyzer, Debug)]
#[repr(align(64))]  // 对齐到缓存行边界
pub struct NoFalseSharing {
    counter1: AtomicU64,
    _pad1: [u8; 56],    // 填充到 64 字节
    counter2: AtomicU64,
    _pad2: [u8; 56],    // 填充到 64 字节
}

// ✅ 示例 4: 非原子类型不触发伪共享检查
#[derive(CacheAnalyzer, Debug)]
pub struct NonAtomicFields {
    local_counter1: u64,
    local_counter2: u64,
    shared_flag: bool,
}

// ✅ 示例 5: 分离热点和冷数据避免伪共享
#[derive(CacheAnalyzer, Debug)]
pub struct SeparatedHotCold {
    // 热点数据（频繁访问）
    #[hot]
    hot_counter: AtomicU64,
    _pad: [u8; 56],

    // 冷数据（不频繁访问）
    cold_timestamp: u64,
    cold_metadata: u64,
}

// ✅ 示例 6: 将不同线程访问的数据分离到不同结构体
#[derive(CacheAnalyzer, Debug)]
pub struct Thread1Data {
    counter: AtomicU64,
    flag: AtomicBool,
    _pad: [u8; 55],  // 确保独占缓存行
}

#[derive(CacheAnalyzer, Debug)]
pub struct Thread2Data {
    counter: AtomicU64,
    flag: AtomicBool,
    _pad: [u8; 55],
}

// ❌ 示例 7: 多个 Mutex 在同一缓存行
/*
use std::sync::Mutex;

#[derive(CacheAnalyzer)]
#[cache(check_false_sharing)]
pub struct MultipleMutexes {
    lock1: Mutex<u64>,
    lock2: Mutex<u64>,
}
// Error: Mutex 类型会被识别为可能的伪共享风险
*/

fn main() {
    println!("=== 伪共享(False Sharing)检查示例 ===\n");

    // 示例 1: 安全的单线程结构
    println!("1. 单线程访问（无伪共享风险）:");
    let report1 = SafeSingleThreaded::detailed_cache_analysis();
    println!("   结构体: {}", report1.struct_name);
    println!("   大小: {} 字节", report1.total_size);
    println!("   缓存行数: {}", report1.cache_lines_needed);
    println!("   ✓ 无伪共享风险\n");

    // 示例 3: 正确的缓存行对齐
    println!("3. 缓存行对齐隔离:");
    let report3 = NoFalseSharing::detailed_cache_analysis();
    println!("   结构体: {}", report3.struct_name);
    println!("   大小: {} 字节", report3.total_size);
    println!("   缓存行数: {}", report3.cache_lines_needed);
    println!("   对齐: {} 字节", report3.alignment);
    println!("   ✓ 使用填充字节隔离原子变量\n");

    // 示例 4: 非原子类型
    println!("4. 非原子类型字段:");
    let report4 = NonAtomicFields::detailed_cache_analysis();
    println!("   结构体: {}", report4.struct_name);
    println!("   大小: {} 字节", report4.total_size);
    println!("   ✓ 非原子类型不触发伪共享警告\n");

    // 示例 5: 热点和冷数据分离
    println!("5. 热点/冷数据分离:");
    let report5 = SeparatedHotCold::detailed_cache_analysis();
    println!("   结构体: {}", report5.struct_name);
    println!("   大小: {} 字节", report5.total_size);
    println!("   缓存行数: {}", report5.cache_lines_needed);
    println!("   ✓ 使用填充隔离热点和冷数据\n");

    println!("伪共享检查总结:");
    println!("  ⚠️  多个原子类型在同一缓存行会触发警告");
    println!("  ✅ 使用 #[repr(align(64))] 和填充字节隔离");
    println!("  ✅ 将不同线程的数据分离到不同结构体");
    println!("\n性能影响:");
    println!("  伪共享可能导致 10-100 倍的性能下降!");
    println!("  在高并发场景下尤为严重");
}
