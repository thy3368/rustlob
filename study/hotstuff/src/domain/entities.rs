//! HotStuff 核心实体定义

use std::collections::HashMap;

use crate::crypto::{Hash, PublicKey, Signature};

/// 视图编号
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewNumber(pub u64);

impl ViewNumber {
    pub const fn new(n: u64) -> Self { Self(n) }

    pub const fn as_u64(&self) -> u64 { self.0 }

    pub fn next(&self) -> Self { Self(self.0 + 1) }
}

impl std::fmt::Display for ViewNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "View({})", self.0) }
}

/// 区块高度
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Height(pub u64);

impl Height {
    pub const fn new(n: u64) -> Self { Self(n) }

    pub const fn as_u64(&self) -> u64 { self.0 }

    pub fn next(&self) -> Self { Self(self.0 + 1) }
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Height({})", self.0) }
}

/// 区块阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Prepare,
    PreCommit,
    Commit,
    Decide
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Prepare => write!(f, "PREPARE"),
            Phase::PreCommit => write!(f, "PRECOMMIT"),
            Phase::Commit => write!(f, "COMMIT"),
            Phase::Decide => write!(f, "DECIDE")
        }
    }
}

/// 区块（命令的集合）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// 区块哈希
    hash: Hash,
    /// 父区块哈希
    parent_hash: Hash,
    /// 视图编号
    view: ViewNumber,
    /// 区块高度
    height: Height,
    /// 提议者公钥
    proposer: PublicKey,
    /// QC（父区块的仲裁证书）
    justify: QuorumCertificate,
    /// 命令数据（简化为字节数组）
    commands: Vec<Vec<u8>>
}

impl Block {
    /// 创建创世区块
    pub fn genesis() -> Self {
        let genesis_qc = QuorumCertificate::genesis();
        let mut block = Self {
            hash: Hash::zero(),
            parent_hash: Hash::zero(),
            view: ViewNumber::new(0),
            height: Height::new(0),
            proposer: PublicKey::from_u64(0),
            justify: genesis_qc,
            commands: Vec::new()
        };
        block.hash = block.compute_hash();
        block
    }

    /// 创建新区块
    pub fn new(
        parent: &Block, view: ViewNumber, proposer: PublicKey, justify: QuorumCertificate, commands: Vec<Vec<u8>>
    ) -> Self {
        let mut block = Self {
            hash: Hash::zero(),
            parent_hash: parent.hash,
            view,
            height: parent.height.next(),
            proposer,
            justify,
            commands
        };
        block.hash = block.compute_hash();
        block
    }

    /// 计算区块哈希
    fn compute_hash(&self) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(self.parent_hash.as_bytes());
        data.extend_from_slice(&self.view.0.to_le_bytes());
        data.extend_from_slice(&self.height.0.to_le_bytes());
        data.extend_from_slice(self.proposer.as_bytes());
        for cmd in &self.commands {
            data.extend_from_slice(cmd);
        }
        Hash::compute(&data)
    }

    pub fn hash(&self) -> Hash { self.hash }

    pub fn parent_hash(&self) -> Hash { self.parent_hash }

    pub fn view(&self) -> ViewNumber { self.view }

    pub fn height(&self) -> Height { self.height }

    pub fn proposer(&self) -> PublicKey { self.proposer }

    pub fn justify(&self) -> &QuorumCertificate { &self.justify }

    pub fn commands(&self) -> &[Vec<u8>] { &self.commands }

    /// 扩展当前区块（创建子区块）
    pub fn extends(&self, block: &Block) -> bool { self.parent_hash == block.hash }
}

/// 投票
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    /// 投票的区块哈希
    block_hash: Hash,
    /// 视图编号
    view: ViewNumber,
    /// 投票阶段
    phase: Phase,
    /// 投票者公钥
    voter: PublicKey,
    /// 签名
    signature: Signature
}

impl Vote {
    /// 创建投票
    pub fn new(block_hash: Hash, view: ViewNumber, phase: Phase, voter: PublicKey, signature: Signature) -> Self {
        Self {
            block_hash,
            view,
            phase,
            voter,
            signature
        }
    }

    pub fn block_hash(&self) -> Hash { self.block_hash }

    pub fn view(&self) -> ViewNumber { self.view }

    pub fn phase(&self) -> Phase { self.phase }

    pub fn voter(&self) -> PublicKey { self.voter }

    pub fn signature(&self) -> Signature { self.signature }

    /// 验证投票签名
    pub fn verify(&self) -> bool {
        // 简化实现
        true
    }
}

/// 仲裁证书（Quorum Certificate）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuorumCertificate {
    /// QC 对应的区块哈希
    block_hash: Hash,
    /// 视图编号
    view: ViewNumber,
    /// 阶段
    phase: Phase,
    /// 投票集合
    votes: HashMap<PublicKey, Vote>
}

impl QuorumCertificate {
    /// 创建创世 QC
    pub fn genesis() -> Self {
        Self {
            block_hash: Hash::zero(),
            view: ViewNumber::new(0),
            phase: Phase::Decide,
            votes: HashMap::new()
        }
    }

    /// 创建带指定区块哈希的创世 QC
    pub fn genesis_with_hash(block_hash: Hash) -> Self {
        Self {
            block_hash,
            view: ViewNumber::new(0),
            phase: Phase::Decide,
            votes: HashMap::new()
        }
    }

    /// 创建新 QC
    pub fn new(block_hash: Hash, view: ViewNumber, phase: Phase) -> Self {
        Self {
            block_hash,
            view,
            phase,
            votes: HashMap::new()
        }
    }

    /// 添加投票
    pub fn add_vote(&mut self, vote: Vote) -> bool {
        if vote.block_hash != self.block_hash || vote.view != self.view || vote.phase != self.phase {
            return false;
        }

        if !vote.verify() {
            return false;
        }

        self.votes.insert(vote.voter, vote);
        true
    }

    /// 检查是否达到仲裁（2f+1 票）
    pub fn has_quorum(&self, total_nodes: usize) -> bool {
        let f = (total_nodes - 1) / 3; // 最多容忍 f 个拜占庭节点
        let quorum_size = 2 * f + 1;
        self.votes.len() >= quorum_size
    }

    pub fn block_hash(&self) -> Hash { self.block_hash }

    pub fn view(&self) -> ViewNumber { self.view }

    pub fn phase(&self) -> Phase { self.phase }

    pub fn vote_count(&self) -> usize { self.votes.len() }

    pub fn votes(&self) -> &HashMap<PublicKey, Vote> { &self.votes }
}

/// 提案消息
#[derive(Debug, Clone)]
pub struct Proposal {
    pub block: Block,
    pub phase: Phase
}

impl Proposal {
    pub fn new(block: Block, phase: Phase) -> Self {
        Self {
            block,
            phase
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert_eq!(genesis.height().as_u64(), 0);
        assert_eq!(genesis.view().as_u64(), 0);
        assert_eq!(genesis.parent_hash(), Hash::zero());
    }

    #[test]
    fn test_block_creation() {
        let genesis = Block::genesis();
        let proposer = PublicKey::from_u64(1);
        let qc = QuorumCertificate::genesis();

        let block = Block::new(&genesis, ViewNumber::new(1), proposer, qc, vec![b"command1".to_vec()]);

        assert_eq!(block.height().as_u64(), 1);
        assert_eq!(block.view().as_u64(), 1);
        assert_eq!(block.parent_hash(), genesis.hash());
    }

    #[test]
    fn test_quorum_certificate() {
        let block_hash = Hash::compute(&"test");
        let view = ViewNumber::new(1);
        let mut qc = QuorumCertificate::new(block_hash, view, Phase::Prepare);

        // 添加 3 个投票（假设总共 4 个节点，f=1，需要 2f+1=3 票）
        for i in 0..3 {
            let voter = PublicKey::from_u64(i);
            let vote = Vote::new(block_hash, view, Phase::Prepare, voter, Signature::zero());
            assert!(qc.add_vote(vote));
        }

        assert!(qc.has_quorum(4));
    }

    #[test]
    fn test_view_number_increment() {
        let view = ViewNumber::new(5);
        let next = view.next();
        assert_eq!(next.as_u64(), 6);
    }
}
