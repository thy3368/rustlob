/// Halo2 零知识证明示例运行程序
///
/// 运行方式：
/// cargo run --bin zevm_demo

mod zevm;

use zevm::ZEVM;

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║  Halo2 零知识证明示例演示              ║");
    println!("╚════════════════════════════════════════╝\n");

    // 运行平方电路示例
    if let Err(e) = ZEVM::run_square_example() {
        eprintln!("平方电路示例出错: {:?}", e);
    }

    println!("\n{}\n", "=".repeat(50));

    // 运行加法电路示例
    if let Err(e) = ZEVM::run_add_example() {
        eprintln!("加法电路示例出错: {:?}", e);
    }

    println!("\n{}\n", "=".repeat(50));

    // 运行错误证明示例
    if let Err(e) = ZEVM::run_invalid_example() {
        eprintln!("错误证明示例出错: {:?}", e);
    }

    println!("\n╔════════════════════════════════════════╗");
    println!("║  零知识证明的核心特性：                ║");
    println!("║  1. 完备性：真实的声明能被证明         ║");
    println!("║  2. 可靠性：虚假的声明无法被证明       ║");
    println!("║  3. 零知识性：不泄露任何额外信息       ║");
    println!("╚════════════════════════════════════════╝");
}
