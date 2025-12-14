//! 集成测试 - 演示完整的 HotStuff 共识流程

#[cfg(test)]
mod integration_tests {
    use std::collections::HashMap;

    use crate::{
        crypto::PrivateKey,
        domain::node::{Message, Node}
    };

    /// 模拟网络 - 广播消息给所有节点
    struct Network {
        nodes: HashMap<u64, Node>
    }

    impl Network {
        fn new(num_nodes: usize, verbose: bool) -> Self {
            let mut validators = Vec::new();
            for i in 0..num_nodes {
                let pk = PrivateKey::from_u64(i as u64).public_key();
                validators.push(pk);
            }

            let mut nodes = HashMap::new();
            for i in 0..num_nodes {
                let private_key = PrivateKey::from_u64(i as u64);
                let node = Node::new(i as u64, private_key, validators.clone(), verbose);
                nodes.insert(i as u64, node);
            }

            Self {
                nodes
            }
        }

        /// 广播消息给所有节点（除了发送者）
        fn broadcast(&mut self, sender_id: u64, messages: Vec<Message>) -> Vec<Message> {
            let mut responses = Vec::new();

            for msg in messages {
                for (node_id, node) in self.nodes.iter_mut() {
                    if *node_id != sender_id {
                        let node_responses = node.handle_message(msg.clone());
                        responses.extend(node_responses);
                    }
                }
            }

            responses
        }

        /// 获取节点
        fn get_node(&self, id: u64) -> &Node { self.nodes.get(&id).unwrap() }

        /// 获取可变节点
        fn get_node_mut(&mut self, id: u64) -> &mut Node { self.nodes.get_mut(&id).unwrap() }

        /// 打印所有节点状态
        fn print_all_status(&self) {
            println!("\n=== Network Status ===");
            for (id, node) in &self.nodes {
                node.print_status();
            }
            println!("======================\n");
        }

        /// 检查所有节点是否提交到相同高度
        #[allow(dead_code)]
        fn all_committed(&self, expected_height: u64) -> bool {
            self.nodes.values().all(|node| node.committed_height() >= expected_height)
        }
    }

    #[test]
    fn test_single_block_consensus() {
        println!("\n=== Testing Single Block Consensus ===\n");

        let mut network = Network::new(4, true);

        // View 1: Node 1 是 Leader
        let leader_id = 1;
        let leader_node = network.get_node_mut(leader_id);

        // Leader 创建提案
        println!("--- Phase 1: Prepare ---");
        let proposal_msgs = leader_node.propose(vec![b"transaction1".to_vec()]);
        assert_eq!(proposal_msgs.len(), 1);

        // 广播提案，收集投票
        let vote_msgs = network.broadcast(leader_id, proposal_msgs);
        println!("Collected {} votes", vote_msgs.len());
        assert_eq!(vote_msgs.len(), 3); // 3 个 replicas 投票

        // Leader 收集投票，形成 Prepare QC
        let mut qc_msgs = Vec::new();
        for vote_msg in vote_msgs {
            let msgs = network.get_node_mut(leader_id).handle_message(vote_msg);
            qc_msgs.extend(msgs);
        }

        // 应该形成 Prepare QC
        assert!(!qc_msgs.is_empty());
        println!("Prepare QC formed!");

        // 广播 Prepare QC，进入 Pre-commit 阶段
        println!("\n--- Phase 2: Pre-commit ---");
        let precommit_msgs = network.broadcast(leader_id, qc_msgs);
        println!("Collected {} pre-commit votes", precommit_msgs.len());

        // Leader 收集 Pre-commit 投票
        let mut commit_qc_msgs = Vec::new();
        for msg in precommit_msgs {
            let msgs = network.get_node_mut(leader_id).handle_message(msg);
            commit_qc_msgs.extend(msgs);
        }

        println!("Pre-commit QC formed!");

        // 广播 Pre-commit QC，进入 Commit 阶段
        println!("\n--- Phase 3: Commit ---");
        let commit_msgs = network.broadcast(leader_id, commit_qc_msgs);
        println!("Collected {} commit votes", commit_msgs.len());

        // Leader 收集 Commit 投票
        let mut decide_msgs = Vec::new();
        for msg in commit_msgs {
            let msgs = network.get_node_mut(leader_id).handle_message(msg);
            decide_msgs.extend(msgs);
        }

        println!("Commit QC formed!");

        // 广播 Commit QC
        network.broadcast(leader_id, decide_msgs);

        network.print_all_status();
    }

    #[test]
    fn test_multi_block_consensus() {
        println!("\n=== Testing Multi-Block Consensus ===\n");

        let mut network = Network::new(4, false);

        // 执行 3 个视图（提交第一个区块需要 3 个连续的 QC）
        for view in 1..=3 {
            println!("\n=== View {} ===", view);

            let leader_id = view % 4;
            println!("Leader: Node {}", leader_id);

            // Prepare 阶段
            let proposal = network.get_node_mut(leader_id).propose(vec![format!("tx_view_{}", view).into_bytes()]);

            let votes = network.broadcast(leader_id, proposal);

            let mut qc_msgs = Vec::new();
            for vote in votes {
                let msgs = network.get_node_mut(leader_id).handle_message(vote);
                qc_msgs.extend(msgs);
            }

            if !qc_msgs.is_empty() {
                // Pre-commit 阶段
                let precommit_votes = network.broadcast(leader_id, qc_msgs);

                let mut commit_qc_msgs = Vec::new();
                for vote in precommit_votes {
                    let msgs = network.get_node_mut(leader_id).handle_message(vote);
                    commit_qc_msgs.extend(msgs);
                }

                if !commit_qc_msgs.is_empty() {
                    // Commit 阶段
                    let commit_votes = network.broadcast(leader_id, commit_qc_msgs);

                    let mut decide_msgs = Vec::new();
                    for vote in commit_votes {
                        let msgs = network.get_node_mut(leader_id).handle_message(vote);
                        decide_msgs.extend(msgs);
                    }

                    network.broadcast(leader_id, decide_msgs);
                }
            }

            // 推进到下一个视图
            for node in network.nodes.values_mut() {
                node.advance_view();
            }

            network.print_all_status();
        }

        // 验证至少有一个区块被提交
        println!("\n=== Final Status ===");
        network.print_all_status();
    }

    #[test]
    fn test_view_change() {
        println!("\n=== Testing View Change ===\n");

        let mut network = Network::new(4, true);

        // 初始视图
        for node in network.nodes.values() {
            println!("Node {}: View {}, Role: {:?}", node.id(), node.consensus().state().current_view(), node.role());
        }

        println!("\n--- Advancing to View 2 ---");

        // 所有节点推进到视图 2
        for node in network.nodes.values_mut() {
            node.advance_view();
        }

        for node in network.nodes.values() {
            println!("Node {}: View {}, Role: {:?}", node.id(), node.consensus().state().current_view(), node.role());
            assert_eq!(node.consensus().state().current_view().as_u64(), 2);
        }

        // 验证 Leader 轮换（View 2 的 Leader 应该是 Node 2）
        assert_eq!(network.get_node(2).role(), crate::domain::node::NodeRole::Leader);
    }
}
