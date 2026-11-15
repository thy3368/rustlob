pub trait Shard {
    fn topic(&self) -> u8 {
        0
    }
    fn partition(&self) -> u8;

    fn party_id(&self) -> u64 {
        0
    }
}

/// 提供状态机 executor 投影事件时所需的日志元数据。
pub trait LogAttr: Shard {
    fn lsn(&self) -> u64;
    fn logical_ts(&self) -> u64;
}
