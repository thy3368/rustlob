mod entities;
mod usecases;
mod trie;
mod storage;
mod example;

use example::{run_advanced_example, run_basic_example, run_ethereum_state_example, run_ethereum_transaction_example, run_light_client_example};

/// MPT 演示程序
///
/// 展示 Merkle Patricia Trie 的完整功能
fn main() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     Merkle Patricia Trie (MPT) 演示程序                         ║");
    println!("║     基于 Clean Architecture 的 trait-based 实现                 ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // 运行基本示例
    if let Err(e) = run_basic_example() {
        eprintln!("❌ 基本示例失败: {}", e);
        std::process::exit(1);
    }

    // 运行高级示例
    if let Err(e) = run_advanced_example() {
        eprintln!("❌ 高级示例失败: {}", e);
        std::process::exit(1);
    }

    // 运行以太坊状态树示例
    if let Err(e) = run_ethereum_state_example() {
        eprintln!("❌ 以太坊状态树示例失败: {}", e);
        std::process::exit(1);
    }

    // 运行以太坊交易树、收据树高频场景示例
    if let Err(e) = run_ethereum_transaction_example() {
        eprintln!("❌ 以太坊交易树与收据树示例失败: {}", e);
        std::process::exit(1);
    }

    // 运行轻客户端验证高频场景示例
    if let Err(e) = run_light_client_example() {
        eprintln!("❌ 轻客户端验证示例失败: {}", e);
        std::process::exit(1);
    }


    // 打印架构信息
    print_architecture_info();

    println!();
    println!("✅ 所有示例运行成功！");
    println!();
}

fn print_architecture_info() {
    println!();
    {let l = "=".repeat(70); println!("{}", l);}
    println!("📐 Clean Architecture 分层说明");
    {let l = "=".repeat(70); println!("{}", l);}
    println!();
    println!("1️⃣  Entities Layer (entities.rs)");
    println!("   - Node: MPT 节点类型（Empty, Leaf, Extension, Branch）");
    println!("   - Path: Nibble 路径表示");
    println!("   - MptError: 错误类型定义");
    println!("   - MerkleProof: Merkle 证明结构");
    println!("   ✓ 无外部依赖，纯领域模型");
    println!();
    println!("2️⃣  Use Cases Layer (usecases.rs)");
    println!("   - InsertUseCase: 插入操作 trait");
    println!("   - GetUseCase: 查询操作 trait");
    println!("   - DeleteUseCase: 删除操作 trait");
    println!("   - ProveUseCase: 证明生成 trait");
    println!("   - RootHashUseCase: 根哈希操作 trait");
    println!("   - IteratorUseCase: 遍历操作 trait");
    println!("   - MptUseCases: 组合所有用例的总接口");
    println!("   ✓ 用 trait 表达业务用例，单一职责");
    println!();
    println!("3️⃣  Interface Adapters Layer (storage.rs)");
    println!("   - Storage trait: 存储抽象接口");
    println!("   - InMemoryStorage: 内存存储实现");
    println!("   - CachedStorage: 缓存装饰器");
    println!("   ✓ 依赖倒置，可替换实现");
    println!();
    println!("4️⃣  Core Implementation (trie.rs)");
    println!("   - MerklePatriciaTrie: MPT 核心实现");
    println!("   - 实现所有 UseCase trait");
    println!("   - 递归插入、查询算法");
    println!("   ✓ 业务逻辑与基础设施分离");
    println!();
    println!("5️⃣  Presentation Layer (example.rs, main.rs)");
    println!("   - 命令行界面");
    println!("   - 使用示例展示");
    println!("   ✓ 依赖内层接口");
    println!();
    {let l = "=".repeat(70); println!("{}", l);}
    println!("🎯 设计原则");
    {let l = "=".repeat(70); println!("{}", l);}
    println!();
    println!("✅ 依赖倒置原则 (DIP): 高层模块不依赖低层模块");
    println!("✅ 单一职责原则 (SRP): 每个 trait 负责一个用例");
    println!("✅ 开闭原则 (OCP): 对扩展开放，对修改关闭");
    println!("✅ 接口隔离原则 (ISP): 细粒度的 trait 接口");
    println!("✅ 里氏替换原则 (LSP): Storage trait 可替换实现");
    println!();
    {let l = "=".repeat(70); println!("{}", l);}
    println!("📚 参考资源");
    {let l = "=".repeat(70); println!("{}", l);}
    println!();
    println!("   - 以太坊黄皮书: https://ethereum.github.io/yellowpaper/");
    println!("   - Merkle Patricia Trie 规范: https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/");
    println!("   - Clean Architecture: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html");
    println!();
}
