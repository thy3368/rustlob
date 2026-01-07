//! HotStuff 基本共识演示
//!
//! 运行方式：cargo run --example basic_consensus

use std::collections::HashMap;

use hotstuff::{
    crypto::PrivateKey,
    domain::node::{Message, Node, NodeRole}
};

/// 模拟网络环境
struct SimulatedNetwork {
    nodes: HashMap<u64, Node>
}

impl SimulatedNetwork {
    /// 创建包含 n 个节点的网络
    fn new(num_nodes: usize) -> Self {
        println!("Initializing network with {} nodes...\n", num_nodes);

        // 生成所有验证者的公钥
        let validators: Vec<_> = (0..num_nodes).map(|i| PrivateKey::from_u64(i as u64).public_key()).collect();

        // 创建节点
        let mut nodes = HashMap::new();
        for i in 0..num_nodes {
            let private_key = PrivateKey::from_u64(i as u64);
            let node = Node::new(i as u64, private_key, validators.clone(), true);
            println!("Node {} initialized - Role: {:?}", i, node.role());
            nodes.insert(i as u64, node);
        }

        println!();
        Self {
            nodes
        }
    }

    /// 广播消息给所有节点（除了发送者）
    fn broadcast(&mut self, sender_id: u64, messages: Vec<Message>) -> Vec<Message> {
        let mut responses = Vec::new();

        for msg in messages {
            println!("Broadcasting message from Node {}", sender_id);

            for (node_id, node) in self.nodes.iter_mut() {
                if *node_id != sender_id {
                    let node_responses = node.handle_message(msg.clone());
                    responses.extend(node_responses);
                }
            }
        }

        responses
    }

    /// 获取当前 Leader
    fn get_leader_id(&self) -> u64 {
        for (id, node) in &self.nodes {
            if node.role() == NodeRole::Leader {
                return *id;
            }
        }
        panic!("No leader found");
    }

    /// 运行一轮完整的共识（Prepare -> Pre-commit -> Commit）
    fn run_consensus_round(&mut self, transaction: Vec<u8>) -> bool {
        let leader_id = self.get_leader_id();

        println!("\n╔═══════════════════════════════════════╗");
        println!("║     Starting Consensus Round         ║");
        println!("║  Leader: Node {}                      ║", leader_id);
        println!("╚═══════════════════════════════════════╝\n");

        // ========== Prepare Phase ==========
        println!("┌─────────────────────────────────────┐");
        println!("│  Phase 1: PREPARE                   │");
        println!("└─────────────────────────────────────┘");

        let leader = self.nodes.get_mut(&leader_id).unwrap();
        let proposal_msgs = leader.propose(vec![transaction.clone()]);

        if proposal_msgs.is_empty() {
            println!("Failed to create proposal");
            return false;
        }

        let prepare_votes = self.broadcast(leader_id, proposal_msgs);
        println!("Collected {} Prepare votes\n", prepare_votes.len());

        // Leader 收集 Prepare 投票
        let mut prepare_qc_msgs = Vec::new();
        for vote in prepare_votes {
            let leader = self.nodes.get_mut(&leader_id).unwrap();
            let msgs = leader.handle_message(vote);
            prepare_qc_msgs.extend(msgs);
        }

        if prepare_qc_msgs.is_empty() {
            println!("Failed to form Prepare QC");
            return false;
        }

        // ========== Pre-commit Phase ==========
        println!("┌─────────────────────────────────────┐");
        println!("│  Phase 2: PRE-COMMIT                │");
        println!("└─────────────────────────────────────┘");

        let precommit_votes = self.broadcast(leader_id, prepare_qc_msgs);
        println!("Collected {} Pre-commit votes\n", precommit_votes.len());

        let mut precommit_qc_msgs = Vec::new();
        for vote in precommit_votes {
            let leader = self.nodes.get_mut(&leader_id).unwrap();
            let msgs = leader.handle_message(vote);
            precommit_qc_msgs.extend(msgs);
        }

        if precommit_qc_msgs.is_empty() {
            println!("Failed to form Pre-commit QC");
            return false;
        }

        // ========== Commit Phase ==========
        println!("┌─────────────────────────────────────┐");
        println!("│  Phase 3: COMMIT                    │");
        println!("└─────────────────────────────────────┘");

        let commit_votes = self.broadcast(leader_id, precommit_qc_msgs);
        println!("Collected {} Commit votes\n", commit_votes.len());

        let mut commit_qc_msgs = Vec::new();
        for vote in commit_votes {
            let leader = self.nodes.get_mut(&leader_id).unwrap();
            let msgs = leader.handle_message(vote);
            commit_qc_msgs.extend(msgs);
        }

        if !commit_qc_msgs.is_empty() {
            self.broadcast(leader_id, commit_qc_msgs);
            println!("✓ Consensus round completed successfully!\n");
            return true;
        }

        false
    }

    /// 推进到下一个视图
    fn advance_view(&mut self) {
        println!("\n╔═══════════════════════════════════════╗");
        println!("║      Advancing to Next View           ║");
        println!("╚═══════════════════════════════════════╝\n");

        for node in self.nodes.values_mut() {
            node.advance_view();
        }

        let new_leader = self.get_leader_id();
        println!("New Leader: Node {}\n", new_leader);
    }

    /// 打印网络状态
    fn print_status(&self) {
        println!("\n╔═══════════════════════════════════════╗");
        println!("║        Network Status                 ║");
        println!("╚═══════════════════════════════════════╝");

        for (id, node) in &self.nodes {
            node.print_status();
        }

        println!();
    }
}

fn main() {
    println!("\n");
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║                                                   ║");
    println!("║        HotStuff BFT Consensus Demo                ║");
    println!("║                                                   ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();

    // 创建 4 个节点的网络
    let mut network = SimulatedNetwork::new(4);

    // 执行多轮共识
    let num_rounds = 3;

    for round in 1..=num_rounds {
        println!("\n\n");
        println!("╔═══════════════════════════════════════════════════╗");
        println!("║                 ROUND {}                          ║", round);
        println!("╚═══════════════════════════════════════════════════╝");

        let transaction = format!("Transaction_{}", round).into_bytes();

        let success = network.run_consensus_round(transaction);

        if !success {
            println!("Consensus failed in round {}", round);
            break;
        }

        network.print_status();

        if round < num_rounds {
            network.advance_view();
        }
    }

    println!("\n");
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║             Final Network Status                  ║");
    println!("╚═══════════════════════════════════════════════════╝");
    network.print_status();

    println!("╔═══════════════════════════════════════════════════╗");
    println!("║           Demo Completed Successfully!            ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();
}
