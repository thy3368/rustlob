use std::time::{SystemTime, UNIX_EPOCH};

use crate::LogEntry;

/// 为日志条目分配序号或时间戳失败时返回的错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogEntrySequenceError {
    /// 系统时钟早于 Unix epoch。
    ClockBeforeUnixEpoch,
    /// Unix 纳秒时间戳无法表示为 `u64`。
    LogicalTimestampOverflow,
    /// LSN 已达到 `u64` 上限，无法继续分配。
    LsnOverflow,
}

impl std::fmt::Display for LogEntrySequenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClockBeforeUnixEpoch => f.write_str("system clock is before Unix epoch"),
            Self::LogicalTimestampOverflow => f.write_str("logical timestamp overflow"),
            Self::LsnOverflow => f.write_str("LSN overflow"),
        }
    }
}

impl std::error::Error for LogEntrySequenceError {}

/// 由单线程调用方持有的日志条目定序器。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntrySequencer {
    next_lsn: u64,
    last_logical_ts: u64,
}

impl LogEntrySequencer {
    /// 从最后一个已提交日志恢复；没有历史日志时首个 LSN 为 `1`。
    pub const fn new(last_committed: Option<(u64, u64)>) -> Self {
        let (next_lsn, last_logical_ts) = match last_committed {
            Some((lsn, logical_ts)) => (lsn, logical_ts),
            None => (0, 0),
        };
        Self { next_lsn, last_logical_ts }
    }

    /// 分配一个新的日志条目。
    pub fn sequence<P>(
        &mut self,
        client_id: u64,
        request_id: u64,
        payload: P,
    ) -> Result<LogEntry<P>, LogEntrySequenceError> {
        self.sequence_at(SystemTime::now(), client_id, request_id, payload)
    }

    fn sequence_at<P>(
        &mut self,
        now: SystemTime,
        client_id: u64,
        request_id: u64,
        payload: P,
    ) -> Result<LogEntry<P>, LogEntrySequenceError> {
        let unix_nanos = now
            .duration_since(UNIX_EPOCH)
            .map_err(|_| LogEntrySequenceError::ClockBeforeUnixEpoch)?
            .as_nanos();
        let now_ts = u64::try_from(unix_nanos)
            .map_err(|_| LogEntrySequenceError::LogicalTimestampOverflow)?;
        let logical_ts = now_ts.max(
            self.last_logical_ts
                .checked_add(1)
                .ok_or(LogEntrySequenceError::LogicalTimestampOverflow)?,
        );
        let lsn = self.next_lsn.checked_add(1).ok_or(LogEntrySequenceError::LsnOverflow)?;

        self.next_lsn = lsn;
        self.last_logical_ts = logical_ts;
        Ok(LogEntry { lsn, logical_ts, client_id, request_id, payload })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn at(nanos: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_nanos(nanos)
    }

    #[test]
    fn first_and_continuous_allocations_are_ordered() {
        let mut sequencer = LogEntrySequencer::new(None);
        let first = sequencer.sequence_at(at(10), 1, 2, "a").unwrap();
        let second = sequencer.sequence_at(at(10), 3, 4, "b").unwrap();
        assert_eq!((first.lsn, first.logical_ts), (1, 10));
        assert_eq!((second.lsn, second.logical_ts), (2, 11));
    }

    #[test]
    fn clock_stall_or_regression_still_advances_logical_time() {
        let mut sequencer = LogEntrySequencer::new(None);
        assert_eq!(sequencer.sequence_at(at(100), 0, 0, ()).unwrap().logical_ts, 100);
        assert_eq!(sequencer.sequence_at(at(90), 0, 1, ()).unwrap().logical_ts, 101);
    }

    #[test]
    fn recovers_from_last_committed_entry() {
        let mut sequencer = LogEntrySequencer::new(Some((41, 900)));
        let entry = sequencer.sequence_at(at(800), 1, 1, ()).unwrap();
        assert_eq!((entry.lsn, entry.logical_ts), (42, 901));
    }

    #[test]
    fn failures_do_not_change_state() {
        let mut sequencer = LogEntrySequencer::new(Some((7, 20)));
        assert_eq!(
            sequencer.sequence_at(UNIX_EPOCH - Duration::from_nanos(1), 0, 0, ()),
            Err(LogEntrySequenceError::ClockBeforeUnixEpoch)
        );
        assert_eq!(sequencer.sequence_at(at(21), 0, 0, ()).unwrap().lsn, 8);
    }

    #[test]
    fn detects_timestamp_and_lsn_overflow() {
        let mut timestamp = LogEntrySequencer::new(Some((1, u64::MAX)));
        assert_eq!(
            timestamp.sequence_at(at(0), 0, 0, ()),
            Err(LogEntrySequenceError::LogicalTimestampOverflow)
        );
        let mut lsn = LogEntrySequencer::new(Some((u64::MAX, 0)));
        assert_eq!(lsn.sequence_at(at(0), 0, 0, ()), Err(LogEntrySequenceError::LsnOverflow));
    }
}
