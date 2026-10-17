/// 单线程序号耗尽错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceExhausted;

impl std::fmt::Display for SequenceExhausted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("sequence exhausted")
    }
}

impl std::error::Error for SequenceExhausted {}

/// 为单线程调用方分配递增的 `u64` 序号。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleThreadSequencer {
    next_sequence: u64,
    exhausted: bool,
}

impl SingleThreadSequencer {
    /// 创建从 `start` 开始分配序号的定序器。
    #[inline]
    pub const fn new(start: u64) -> Self {
        Self { next_sequence: start, exhausted: false }
    }

    /// 分配下一个序号；`u64::MAX` 只能成功分配一次。
    #[inline]
    pub fn next(&mut self) -> Result<u64, SequenceExhausted> {
        if self.exhausted {
            return Err(SequenceExhausted);
        }

        let sequence = self.next_sequence;
        if sequence == u64::MAX {
            self.exhausted = true;
        } else {
            self.next_sequence += 1;
        }
        Ok(sequence)
    }
}

#[cfg(test)]
mod tests {
    use super::{SequenceExhausted, SingleThreadSequencer};

    #[test]
    fn allocates_from_custom_start_and_increments() {
        let mut sequencer = SingleThreadSequencer::new(41);

        assert_eq!(sequencer.next(), Ok(41));
        assert_eq!(sequencer.next(), Ok(42));
        assert_eq!(sequencer.next(), Ok(43));
    }

    #[test]
    fn allocates_maximum_once_then_stays_exhausted() {
        let mut sequencer = SingleThreadSequencer::new(u64::MAX - 1);

        assert_eq!(sequencer.next(), Ok(u64::MAX - 1));
        assert_eq!(sequencer.next(), Ok(u64::MAX));
        assert_eq!(sequencer.next(), Err(SequenceExhausted));
        assert_eq!(sequencer.next(), Err(SequenceExhausted));
    }

    #[test]
    fn independent_instances_do_not_share_sequence() {
        let mut first = SingleThreadSequencer::new(7);
        let mut second = SingleThreadSequencer::new(7);

        assert_eq!(first.next(), Ok(7));
        assert_eq!(first.next(), Ok(8));
        assert_eq!(second.next(), Ok(7));
    }
}
