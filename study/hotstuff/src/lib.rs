//! HotStuff BFT Consensus Implementation
//!
//! 本实现遵循 HotStuff 论文的三阶段共识协议：
//! - Prepare Phase
//! - Pre-commit Phase
//! - Commit Phase
//!
//! 架构遵循 Clean Architecture 原则，分层如下：
//! - domain: 核心领域模型和业务逻辑
//! - crypto: 密码学原语（简化实现）
//!
//! # 示例
//!
//! ```rust
//! use hotstuff::{Node, crypto::PrivateKey};
//!
//! // 创建 4 个验证者
//! let validators: Vec<_> = (0..4)
//!     .map(|i| PrivateKey::from_u64(i).public_key())
//!     .collect();
//!
//! // 创建节点
//! let private_key = PrivateKey::from_u64(0);
//! let node = Node::new(0, private_key, validators, false);
//! ```

pub mod crypto;
pub mod domain;

#[cfg(test)]
mod tests;

pub use domain::{
    consensus::HotStuffConsensus,
    entities::{Block, QuorumCertificate, Vote},
    node::Node
};
