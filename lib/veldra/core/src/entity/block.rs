use super::stable_hash_hex;

/// 新区块的业务承诺快照。
#[derive(Debug, Clone, PartialEq, Eq)]
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
