//! HotStuff 共识算法核心逻辑

use std::collections::HashMap;

use super::entities::{Block, Height, Phase, Proposal, QuorumCertificate, ViewNumber, Vote};
use crate::crypto::{Hash, PrivateKey, PublicKey};

/// 共识状态
#[derive(Debug)]
pub struct ConsensusState {
    /// 当前视图
    current_view: ViewNumber,
    /// 最高 QC
    high_qc: QuorumCertificate,
    /// 锁定的 QC（Pre-commit QC）
    locked_qc: Option<QuorumCertificate>,
    /// 已提交的最高区块高度
    committed_height: Height,
    /// 区块存储
    blocks: HashMap<Hash, Block>,
    /// 待处理的投票（按阶段和区块哈希分组）
    pending_votes: HashMap<(Phase, Hash, ViewNumber), HashMap<PublicKey, Vote>>
}

impl ConsensusState {
    /// 创建新的共识状态
    pub fn new() -> Self {
        let genesis = Block::genesis();
        let genesis_hash = genesis.hash();
        let mut blocks = HashMap::new();
        blocks.insert(genesis_hash, genesis);

        // 创建指向创世区块的 QC
        let genesis_qc = QuorumCertificate::genesis_with_hash(genesis_hash);

        Self {
            current_view: ViewNumber::new(1),
            high_qc: genesis_qc,
            locked_qc: None,
            committed_height: Height::new(0),
            blocks,
            pending_votes: HashMap::new()
        }
    }

    pub fn current_view(&self) -> ViewNumber { self.current_view }

    pub fn high_qc(&self) -> &QuorumCertificate { &self.high_qc }

    pub fn locked_qc(&self) -> Option<&QuorumCertificate> { self.locked_qc.as_ref() }

    pub fn committed_height(&self) -> Height { self.committed_height }

    pub fn get_block(&self, hash: &Hash) -> Option<&Block> { self.blocks.get(hash) }

    pub fn store_block(&mut self, block: Block) {
        let hash = block.hash();
        self.blocks.insert(hash, block);
    }

    /// 更新视图
    pub fn advance_view(&mut self, new_view: ViewNumber) {
        if new_view > self.current_view {
            self.current_view = new_view;
        }
    }

    /// 更新 high_qc
    pub fn update_high_qc(&mut self, qc: QuorumCertificate) {
        if qc.view() > self.high_qc.view() {
            self.high_qc = qc;
        }
    }

    /// 设置 locked_qc
    pub fn set_locked_qc(&mut self, qc: QuorumCertificate) { self.locked_qc = Some(qc); }

    /// 更新已提交高度
    pub fn update_committed_height(&mut self, height: Height) {
        if height > self.committed_height {
            self.committed_height = height;
        }
    }

    /// 添加投票并尝试形成 QC
    pub fn add_vote(&mut self, vote: Vote, total_nodes: usize) -> Option<QuorumCertificate> {
        let key = (vote.phase(), vote.block_hash(), vote.view());
        let votes = self.pending_votes.entry(key).or_insert_with(HashMap::new);

        votes.insert(vote.voter(), vote.clone());

        // 检查是否达到仲裁
        let f = (total_nodes - 1) / 3;
        let quorum_size = 2 * f + 1;

        if votes.len() >= quorum_size {
            // 形成 QC
            let mut qc = QuorumCertificate::new(key.1, key.2, key.0);
            for (_, v) in votes.iter() {
                qc.add_vote(v.clone());
            }

            // 清理已使用的投票
            self.pending_votes.remove(&key);

            Some(qc)
        } else {
            None
        }
    }
}

impl Default for ConsensusState {
    fn default() -> Self { Self::new() }
}

/// HotStuff 共识引擎
pub struct HotStuffConsensus {
    /// 节点私钥
    private_key: PrivateKey,
    /// 节点公钥
    public_key: PublicKey,
    /// 共识状态
    state: ConsensusState,
    /// 节点总数
    total_nodes: usize
}

impl HotStuffConsensus {
    /// 创建新的共识引擎
    pub fn new(private_key: PrivateKey, total_nodes: usize) -> Self {
        let public_key = private_key.public_key();
        Self {
            private_key,
            public_key,
            state: ConsensusState::new(),
            total_nodes
        }
    }

    pub fn public_key(&self) -> PublicKey { self.public_key }

    pub fn state(&self) -> &ConsensusState { &self.state }

    pub fn state_mut(&mut self) -> &mut ConsensusState { &mut self.state }

    /// 创建新区块提案
    pub fn create_proposal(&mut self, commands: Vec<Vec<u8>>) -> Proposal {
        let parent_hash = self.state.high_qc.block_hash();
        let parent = self.state.get_block(&parent_hash).expect("Parent block must exist");

        let block = Block::new(parent, self.state.current_view, self.public_key, self.state.high_qc.clone(), commands);

        self.state.store_block(block.clone());

        Proposal::new(block, Phase::Prepare)
    }

    /// 处理收到的提案
    pub fn on_receive_proposal(&mut self, proposal: Proposal) -> Result<Vote, ConsensusError> {
        let block = &proposal.block;
        let phase = &proposal.phase;

        // 验证提案
        self.validate_proposal(block)?;

        // 存储区块
        self.state.store_block(block.clone());

        // 根据阶段创建投票
        let vote = self.create_vote(block, *phase)?;

        Ok(vote)
    }

    /// 验证提案
    fn validate_proposal(&self, block: &Block) -> Result<(), ConsensusError> {
        // 1. 检查视图是否匹配
        if block.view() != self.state.current_view {
            return Err(ConsensusError::InvalidView);
        }

        // 2. 检查父区块是否存在
        if !self.state.blocks.contains_key(&block.parent_hash()) {
            return Err(ConsensusError::MissingParent);
        }

        // 3. 检查 QC 是否有效（创世 QC 例外）
        if block.justify().view().as_u64() > 0 && !block.justify().has_quorum(self.total_nodes) {
            return Err(ConsensusError::InvalidQC);
        }

        // 4. 安全性检查：确保扩展自 locked_qc
        if let Some(locked_qc) = &self.state.locked_qc {
            // 必须扩展自锁定的 QC
            if block.justify().view() < locked_qc.view() {
                return Err(ConsensusError::ConflictWithLockedQC);
            }
        }

        Ok(())
    }

    /// 创建投票
    pub fn create_vote(&self, block: &Block, phase: Phase) -> Result<Vote, ConsensusError> {
        let block_hash = block.hash();
        let view = block.view();

        // 创建签名数据
        let mut sign_data = Vec::new();
        sign_data.extend_from_slice(block_hash.as_bytes());
        sign_data.extend_from_slice(&view.as_u64().to_le_bytes());
        sign_data.push(phase as u8);

        let signature = self.private_key.sign(&sign_data);

        Ok(Vote::new(block_hash, view, phase, self.public_key, signature))
    }

    /// 处理收到的投票
    pub fn on_receive_vote(&mut self, vote: Vote) -> Option<QuorumCertificate> {
        // 验证投票
        if !vote.verify() {
            return None;
        }

        // 添加投票并尝试形成 QC
        let qc = self.state.add_vote(vote.clone(), self.total_nodes)?;

        // 处理形成的 QC
        self.on_qc_formed(qc.clone(), vote.phase());

        Some(qc)
    }

    /// 处理形成的 QC
    fn on_qc_formed(&mut self, qc: QuorumCertificate, phase: Phase) {
        // 更新 high_qc
        self.state.update_high_qc(qc.clone());

        match phase {
            Phase::Prepare => {
                // Prepare QC 形成，进入 Pre-commit 阶段
                // 在实际实现中，Leader 会广播 Pre-commit 提案
            }
            Phase::PreCommit => {
                // Pre-commit QC 形成，锁定该 QC
                self.state.set_locked_qc(qc.clone());
                // 进入 Commit 阶段
            }
            Phase::Commit => {
                // Commit QC 形成，可以决定提交
                self.try_commit(&qc);
            }
            Phase::Decide => {
                // Decide QC（通常不会直接形成）
            }
        }
    }

    /// 尝试提交区块
    fn try_commit(&mut self, commit_qc: &QuorumCertificate) {
        // HotStuff 提交规则：
        // 当有三个连续的 QC（Prepare -> Pre-commit -> Commit）时，
        // 可以提交第一个 QC 对应的区块

        let commit_block_hash = commit_qc.block_hash();
        let commit_block = match self.state.get_block(&commit_block_hash) {
            Some(b) => b,
            None => return
        };

        // 获取 Pre-commit QC（commit_block 的 justify）
        let precommit_qc_hash = commit_block.justify().block_hash();
        let precommit_block = match self.state.get_block(&precommit_qc_hash) {
            Some(b) => b,
            None => return
        };

        // 获取 Prepare QC（precommit_block 的 justify）
        let prepare_qc_hash = precommit_block.justify().block_hash();
        let prepare_block = match self.state.get_block(&prepare_qc_hash) {
            Some(b) => b,
            None => return
        };

        // 检查是否是三个连续的区块
        let is_chain =
            commit_block.parent_hash() == precommit_qc_hash && precommit_block.parent_hash() == prepare_qc_hash;

        if is_chain {
            // 提交 prepare_block
            let prepare_height = prepare_block.height();
            if prepare_height > self.state.committed_height {
                self.state.update_committed_height(prepare_height);
                println!("✓ Committed block at height {}", prepare_height);
            }
        }
    }

    /// 进入下一个视图
    pub fn advance_to_next_view(&mut self) {
        let next_view = self.state.current_view.next();
        self.state.advance_view(next_view);
    }
}

/// 共识错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusError {
    InvalidView,
    MissingParent,
    InvalidQC,
    ConflictWithLockedQC,
    InvalidSignature
}

impl std::fmt::Display for ConsensusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsensusError::InvalidView => write!(f, "Invalid view number"),
            ConsensusError::MissingParent => write!(f, "Parent block not found"),
            ConsensusError::InvalidQC => write!(f, "Invalid quorum certificate"),
            ConsensusError::ConflictWithLockedQC => write!(f, "Conflicts with locked QC"),
            ConsensusError::InvalidSignature => write!(f, "Invalid signature")
        }
    }
}

impl std::error::Error for ConsensusError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_initialization() {
        let private_key = PrivateKey::from_u64(1);
        let consensus = HotStuffConsensus::new(private_key, 4);

        assert_eq!(consensus.state().current_view().as_u64(), 1);
        assert_eq!(consensus.state().committed_height().as_u64(), 0);
    }

    #[test]
    fn test_create_proposal() {
        let private_key = PrivateKey::from_u64(1);
        let mut consensus = HotStuffConsensus::new(private_key, 4);

        let commands = vec![b"tx1".to_vec(), b"tx2".to_vec()];
        let proposal = consensus.create_proposal(commands);

        assert_eq!(proposal.block.height().as_u64(), 1);
        assert_eq!(proposal.phase, Phase::Prepare);
    }

    #[test]
    fn test_vote_aggregation() {
        let mut consensus = HotStuffConsensus::new(PrivateKey::from_u64(0), 4);

        let proposal = consensus.create_proposal(vec![b"test".to_vec()]);
        let block_hash = proposal.block.hash();
        let view = proposal.block.view();

        // 模拟 3 个节点投票（2f+1 = 3，f=1）
        for i in 0..3 {
            let voter = PublicKey::from_u64(i);
            let vote = Vote::new(block_hash, view, Phase::Prepare, voter, crate::crypto::Signature::zero());

            let qc = consensus.on_receive_vote(vote);
            if i == 2 {
                assert!(qc.is_some());
            } else {
                assert!(qc.is_none());
            }
        }
    }
}
