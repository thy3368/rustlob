use super::single_thread_sequencer::{SequenceExhausted, SingleThreadSequencer};

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

/// 使用单线程递增序列生成实体 ID。
///
/// 起始值由 [`SingleThreadSequencer::new`] 指定，`u64::MAX` 可以分配一次，
/// 后续调用返回 [`SequenceExhausted`]。
impl EntityIdGenerator for SingleThreadSequencer {
    #[inline]
    fn next_id(&mut self) -> Result<u64, SequenceExhausted> {
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::{EntityIdGenerator, SequenceExhausted, SingleThreadSequencer};

    #[test]
    fn generates_contiguous_ids_from_custom_start() {
        let mut generator = SingleThreadSequencer::new(41);

        assert_eq!(generator.next_id(), Ok(41));
        assert_eq!(generator.next_id(), Ok(42));
        assert_eq!(generator.next_id(), Ok(43));
    }

    #[test]
    fn allocates_maximum_once_then_returns_exhausted() {
        let mut generator = SingleThreadSequencer::new(u64::MAX);

        assert_eq!(generator.next_id(), Ok(u64::MAX));
        assert_eq!(generator.next_id(), Err(SequenceExhausted));
    }

    #[test]
    fn independent_generators_do_not_share_state() {
        let mut first = SingleThreadSequencer::new(7);
        let mut second = SingleThreadSequencer::new(7);

        assert_eq!(first.next_id(), Ok(7));
        assert_eq!(first.next_id(), Ok(8));
        assert_eq!(second.next_id(), Ok(7));
    }

    #[test]
    fn trait_call_matches_sequencer_next() {
        let mut trait_generator = SingleThreadSequencer::new(99);
        let mut direct_generator = SingleThreadSequencer::new(99);

        assert_eq!(EntityIdGenerator::next_id(&mut trait_generator), direct_generator.next());
    }
}
