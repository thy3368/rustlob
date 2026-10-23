use super::log_entry::LogEntry;

/// 日志条目的存取端口。
///
/// 调用方负责分配 LSN。实现应保证批次按 LSN 严格递增，且整批原子提交。
pub trait LogEntryRepo<P> {
    type Error;

    /// 原子追加一批日志条目。
    fn append_batch(&mut self, entries: Vec<LogEntry<P>>) -> Result<(), Self::Error>;

    /// 追加一条日志条目，复用批量追加的原子语义。
    fn append(&mut self, entry: LogEntry<P>) -> Result<(), Self::Error> {
        self.append_batch(vec![entry])
    }

    /// 从指定 LSN（包含）开始按升序读取至多 `limit` 条日志。
    fn read_from(&self, from_lsn: u64, limit: usize) -> Result<Vec<LogEntry<P>>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::{LogEntry, LogEntryRepo};

    struct RecordingRepo {
        batches: Vec<Vec<LogEntry<String>>>,
    }

    impl LogEntryRepo<String> for RecordingRepo {
        type Error = ();

        fn append_batch(&mut self, entries: Vec<LogEntry<String>>) -> Result<(), Self::Error> {
            self.batches.push(entries);
            Ok(())
        }

        fn read_from(
            &self,
            _from_lsn: u64,
            _limit: usize,
        ) -> Result<Vec<LogEntry<String>>, Self::Error> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn append_delegates_to_append_batch_for_non_copy_payload() {
        let mut repo = RecordingRepo {
            batches: Vec::new(),
        };
        let entry = crate::LogEntrySequencer::from_metadata(
            7,
            11,
            13,
            17,
            String::from("non-copy payload"),
        );

        repo.append(entry.clone()).expect("append should succeed");

        assert_eq!(repo.batches, vec![vec![entry]]);
    }
}
