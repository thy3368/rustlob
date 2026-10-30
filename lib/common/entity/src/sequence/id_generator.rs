use super::single_thread_sequencer::SequenceExhausted;

/// 运行时生成实体 ID 的独立能力。
///
/// 该 trait 不要求实现者同时实现 [`Entity`](crate::Entity)，用于让实体创建
/// 流程依赖 ID 生成能力，而不把 ID 生成职责绑定到实体生命周期契约。
pub trait EntityIdGenerator {
    /// 分配下一个实体 ID。
    ///
    /// 实现者应在分配状态上保持单调递增；当可用的 `u64` ID 已耗尽时返回
    /// [`SequenceExhausted`]。
    fn next_id(&mut self) -> Result<u64, SequenceExhausted>;
}
