mod contracts;
mod example;
mod executor;

use example::run_counter_example;

/// REVM Counter 合约演示程序
///
/// 这个程序展示了如何在 REVM（Rust Ethereum Virtual Machine）上：
/// - 部署智能合约
/// - 执行合约函数
/// - 查询合约状态
///
/// REVM 是一个高性能的 EVM 实现，专为低延迟场景优化
fn main() {
    println!();
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║         REVM 智能合约执行演示                         ║");
    println!("║    Rust Ethereum Virtual Machine Example             ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();

    // 运行 Counter 合约示例
    match run_counter_example() {
        Ok(_) => {
            println!();
            println!("✅ 所有测试通过！");
            println!();
            print_system_info();
        }
        Err(e) => {
            eprintln!();
            eprintln!("❌ 错误: {}", e);
            eprintln!();
            std::process::exit(1);
        }
    }
}

/// 打印系统信息
fn print_system_info() {
    println!("ℹ️  系统信息:");
    println!("   - REVM 版本: 18.0.0");
    println!("   - Alloy Primitives: 0.8");
    println!("   - 执行模式: 单线程内存数据库");
    println!();
    println!("📚 相关资源:");
    println!("   - REVM: https://github.com/bluealloy/revm");
    println!("   - Alloy: https://github.com/alloy-rs");
    println!("   - Solidity 文档: https://docs.soliditylang.org");
    println!();
}
