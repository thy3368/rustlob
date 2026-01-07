//! HotStuff 节点实现

use std::collections::HashMap;

use super::{
    consensus::HotStuffConsensus,
    entities::{Phase, Proposal, QuorumCertificate, ViewNumber, Vote}
};
use crate::crypto::{PrivateKey, PublicKey};

/// 消息类型
#[derive(Debug, Clone)]
pub enum Message {
    /// 提案消息
    Proposal(Proposal),
    /// 投票消息
    Vote(Vote),
    /// QC 消息
    NewQC(QuorumCertificate, Phase),
    /// 视图切换消息
    ViewChange(ViewNumber)
}

/// 节点角色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRole {
    Leader,
    Replica
}

/// HotStuff 节点
pub struct Node {
    /// 节点 ID
    id: u64,
    /// 共识引擎
    consensus: HotStuffConsensus,
    /// 当前角色
    role: NodeRole,
    /// 所有节点的公钥
    validators: HashMap<PublicKey, u64>,
    /// 消息队列
    message_queue: Vec<Message>,
    /// 是否启用详细日志
    verbose: bool
}

impl Node {
    /// 创建新节点
    pub fn new(id: u64, private_key: PrivateKey, validators: Vec<PublicKey>, verbose: bool) -> Self {
        let total_nodes = validators.len();

        let mut validator_map = HashMap::new();
        for (idx, pk) in validators.iter().enumerate() {
            validator_map.insert(*pk, idx as u64);
        }

        let role = Self::determine_role(id, ViewNumber::new(1));

        Self {
            id,
            consensus: HotStuffConsensus::new(private_key, total_nodes),
            role,
            validators: validator_map,
            message_queue: Vec::new(),
            verbose
        }
    }

    pub fn id(&self) -> u64 { self.id }

    pub fn public_key(&self) -> PublicKey { self.consensus.public_key() }

    pub fn consensus(&self) -> &HotStuffConsensus { &self.consensus }

    pub fn consensus_mut(&mut self) -> &mut HotStuffConsensus { &mut self.consensus }

    pub fn role(&self) -> NodeRole { self.role }

    /// 确定当前视图的 Leader
    fn determine_role(node_id: u64, view: ViewNumber) -> NodeRole {
        // 简单的 round-robin Leader 选举
        // Leader = view % total_validators
        let leader_id = view.as_u64() % 4; // 假设 4 个节点
        if node_id == leader_id {
            NodeRole::Leader
        } else {
            NodeRole::Replica
        }
    }

    /// 更新角色（视图切换时）
    pub fn update_role(&mut self) {
        let current_view = self.consensus.state().current_view();
        self.role = Self::determine_role(self.id, current_view);

        if self.verbose {
            println!("[Node {}] Role updated to {:?} for {}", self.id, self.role, current_view);
        }
    }

    /// Leader 创建并广播提案
    pub fn propose(&mut self, commands: Vec<Vec<u8>>) -> Vec<Message> {
        if self.role != NodeRole::Leader {
            if self.verbose {
                println!("[Node {}] Cannot propose: not a leader", self.id);
            }
            return Vec::new();
        }

        let proposal = self.consensus.create_proposal(commands);

        if self.verbose {
            println!(
                "[Node {}] Proposing block at height {} for {}",
                self.id,
                proposal.block.height(),
                proposal.block.view()
            );
        }

        vec![Message::Proposal(proposal)]
    }

    /// 处理收到的消息
    pub fn handle_message(&mut self, message: Message) -> Vec<Message> {
        match message {
            Message::Proposal(proposal) => self.handle_proposal(proposal),
            Message::Vote(vote) => self.handle_vote(vote),
            Message::NewQC(qc, phase) => self.handle_new_qc(qc, phase),
            Message::ViewChange(new_view) => self.handle_view_change(new_view)
        }
    }

    /// 处理提案
    fn handle_proposal(&mut self, proposal: Proposal) -> Vec<Message> {
        if self.verbose {
            println!(
                "[Node {}] Received proposal for block {} at {}",
                self.id,
                proposal.block.hash(),
                proposal.block.view()
            );
        }

        match self.consensus.on_receive_proposal(proposal) {
            Ok(vote) => {
                if self.verbose {
                    println!("[Node {}] Voting for block {} in phase {}", self.id, vote.block_hash(), vote.phase());
                }
                vec![Message::Vote(vote)]
            }
            Err(e) => {
                if self.verbose {
                    println!("[Node {}] Rejected proposal: {}", self.id, e);
                }
                Vec::new()
            }
        }
    }

    /// 处理投票
    fn handle_vote(&mut self, vote: Vote) -> Vec<Message> {
        if self.role != NodeRole::Leader {
            // 只有 Leader 收集投票
            return Vec::new();
        }

        if self.verbose {
            println!(
                "[Node {}] Received vote from {} for block {} in phase {}",
                self.id,
                vote.voter(),
                vote.block_hash(),
                vote.phase()
            );
        }

        if let Some(qc) = self.consensus.on_receive_vote(vote.clone()) {
            if self.verbose {
                println!("[Node {}] Formed QC for block {} in phase {}", self.id, qc.block_hash(), vote.phase());
            }

            // 广播新形成的 QC
            return vec![Message::NewQC(qc, vote.phase())];
        }

        Vec::new()
    }

    /// 处理新 QC
    fn handle_new_qc(&mut self, qc: QuorumCertificate, phase: Phase) -> Vec<Message> {
        if self.verbose {
            println!("[Node {}] Received QC for block {} in phase {}", self.id, qc.block_hash(), phase);
        }

        self.consensus.state_mut().update_high_qc(qc.clone());

        // 根据阶段决定下一步
        let next_phase = match phase {
            Phase::Prepare => Phase::PreCommit,
            Phase::PreCommit => {
                // 锁定 QC
                self.consensus.state_mut().set_locked_qc(qc.clone());
                Phase::Commit
            }
            Phase::Commit => Phase::Decide,
            Phase::Decide => return Vec::new()
        };

        // 所有节点对下一阶段投票
        if let Some(block) = self.consensus.state().get_block(&qc.block_hash()) {
            if self.role == NodeRole::Leader {
                // Leader 创建下一阶段的提案
                let proposal = Proposal::new(block.clone(), next_phase);
                return vec![Message::Proposal(proposal)];
            } else {
                // Replica 直接对下一阶段投票
                if let Ok(vote) = self.consensus.create_vote(block, next_phase) {
                    return vec![Message::Vote(vote)];
                }
            }
        }

        Vec::new()
    }

    /// 处理视图切换
    fn handle_view_change(&mut self, new_view: ViewNumber) -> Vec<Message> {
        if self.verbose {
            println!("[Node {}] Advancing to {}", self.id, new_view);
        }

        self.consensus.state_mut().advance_view(new_view);
        self.update_role();

        Vec::new()
    }

    /// 推进到下一个视图
    pub fn advance_view(&mut self) -> Vec<Message> {
        let next_view = self.consensus.state().current_view().next();
        self.handle_view_change(next_view)
    }

    /// 获取已提交的区块高度
    pub fn committed_height(&self) -> u64 { self.consensus.state().committed_height().as_u64() }

    /// 打印状态
    pub fn print_status(&self) {
        println!(
            "[Node {}] View: {}, Role: {:?}, Committed: {}",
            self.id,
            self.consensus.state().current_view(),
            self.role,
            self.committed_height()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::PrivateKey;

    fn create_test_nodes(n: usize) -> Vec<Node> {
        let mut validators = Vec::new();
        for i in 0..n {
            let pk = PrivateKey::from_u64(i as u64).public_key();
            validators.push(pk);
        }

        let mut nodes = Vec::new();
        for i in 0..n {
            let private_key = PrivateKey::from_u64(i as u64);
            let node = Node::new(i as u64, private_key, validators.clone(), false);
            nodes.push(node);
        }

        nodes
    }

    #[test]
    fn test_node_creation() {
        let nodes = create_test_nodes(4);
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[1].role(), NodeRole::Leader); // View 1, Leader = 1 % 4
                                                       // = 1
    }

    #[test]
    fn test_leader_proposal() {
        let mut nodes = create_test_nodes(4);
        let leader_node = &mut nodes[1]; // Node 1 是 Leader

        let messages = leader_node.propose(vec![b"test_tx".to_vec()]);
        assert_eq!(messages.len(), 1);

        match &messages[0] {
            Message::Proposal(proposal) => {
                assert_eq!(proposal.block.height().as_u64(), 1);
            }
            _ => panic!("Expected proposal message")
        }
    }

    #[test]
    fn test_replica_receives_proposal() {
        let mut nodes = create_test_nodes(4);

        // Leader 创建提案
        let messages = nodes[1].propose(vec![b"test".to_vec()]);
        let proposal = match &messages[0] {
            Message::Proposal(p) => p.clone(),
            _ => panic!("Expected proposal")
        };

        // Replica 处理提案
        let replica_node = &mut nodes[0];
        let vote_messages = replica_node.handle_message(Message::Proposal(proposal));

        assert_eq!(vote_messages.len(), 1);
        match &vote_messages[0] {
            Message::Vote(vote) => {
                assert_eq!(vote.phase(), Phase::Prepare);
            }
            _ => panic!("Expected vote message")
        }
    }
}
