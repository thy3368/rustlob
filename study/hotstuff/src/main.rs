//! HotStuff 命令行工具
//!
//! 运行方式：cargo run

use hotstuff::{crypto::PrivateKey, Node};

fn main() {
    println!("\n╔═══════════════════════════════════════╗");
    println!("║   HotStuff BFT Consensus CLI         ║");
    println!("╚═══════════════════════════════════════╝\n");

    // 创建 4 个节点
    let validators: Vec<_> = (0..4).map(|i| PrivateKey::from_u64(i).public_key()).collect();

    println!("Initializing 4 nodes...\n");

    let mut nodes = Vec::new();
    for i in 0..4 {
        let private_key = PrivateKey::from_u64(i);
        let node = Node::new(i, private_key, validators.clone(), false);
        println!("✓ Node {} initialized (Role: {:?})", i, node.role());
        nodes.push(node);
    }

    println!("\n✨ HotStuff network ready!");
    println!("\n💡 Tip: Run the example for a full consensus demo:");
    println!("   cargo run --example basic_consensus\n");
    println!("📚 Documentation:");
    println!("   cargo doc --open\n");
    println!("🧪 Run tests:");
    println!("   cargo test\n");
}
