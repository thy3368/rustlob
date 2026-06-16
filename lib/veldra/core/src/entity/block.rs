use cmd_handler::EntityReplayableEvent;
use serde::{Deserialize, Serialize};

use super::stable_hash_hex;
use crate::entity::{CommandEnvelope, ProductCommand};

/// 新区块的业务承诺快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewBlock {
    /// 区块高度。
    pub block_height: u64,
    /// 父区块哈希。
    pub parent_block_hash: String,
    /// command envelope 列表的稳定承诺。
    pub commands_root: String,
    /// replayable events 列表的稳定承诺。
    pub events_root: String,
    /// 执行后交易所状态快照的稳定承诺。
    pub post_state_root: String,
    /// 当前区块头字段的稳定哈希。
    pub block_hash: String,
}

/// 区块执行体，保留可用于回放和持久化的规范事实。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockExecutionBody {
    /// 当前区块哈希，必须与区块头承诺一致。
    pub block_hash: String,
    /// 当前区块高度，必须与区块头一致。
    pub block_height: u64,
    /// 本区块按 canonical 顺序执行的命令。
    pub commands: Vec<CommandEnvelope<ProductCommand>>,
    /// 本区块按稳定 sequence 排列的可重放事件。
    pub replayable_events: Vec<EntityReplayableEvent>,
}

impl NewBlock {
    /// 用已经计算好的根字段生成新区块承诺。
    pub fn new(
        block_height: u64,
        parent_block_hash: String,
        commands_root: String,
        events_root: String,
        post_state_root: String,
    ) -> Self {
        let block_hash = stable_hash_hex(&[
            block_height.to_string(),
            parent_block_hash.clone(),
            commands_root.clone(),
            events_root.clone(),
            post_state_root.clone(),
        ]);
        Self {
            block_height,
            parent_block_hash,
            commands_root,
            events_root,
            post_state_root,
            block_hash,
        }
    }
}
