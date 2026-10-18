use super::log_entry_sequencer::LogEntrySequencer;

/// 定序器检查点的持久化端口。
pub trait LogEntrySequencerRepo {
    type Error;

    /// 从持久化检查点恢复定序器；无检查点时返回新定序器。
    fn load(&self) -> Result<LogEntrySequencer, Self::Error>;

    /// 保存当前定序器的检查点。
    fn save(&mut self, sequencer: &LogEntrySequencer) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum RepoError {
        Unavailable,
    }

    #[derive(Default)]
    struct MemoryRepo {
        checkpoint: Option<(u64, u64)>,
        unavailable: bool,
    }

    impl LogEntrySequencerRepo for MemoryRepo {
        type Error = RepoError;

        fn load(&self) -> Result<LogEntrySequencer, Self::Error> {
            if self.unavailable {
                return Err(RepoError::Unavailable);
            }
            Ok(LogEntrySequencer::new(self.checkpoint))
        }

        fn save(&mut self, sequencer: &LogEntrySequencer) -> Result<(), Self::Error> {
            if self.unavailable {
                return Err(RepoError::Unavailable);
            }
            self.checkpoint = sequencer.last_committed();
            Ok(())
        }
    }

    #[test]
    fn restores_checkpoint_and_continues_sequence() {
        let mut repo = MemoryRepo::default();
        let mut sequencer = repo.load().unwrap();
        assert_eq!(sequencer, LogEntrySequencer::new(None));
        assert_eq!(sequencer.last_committed(), None);

        let first = sequencer.sequence(1, 2, ()).unwrap();
        repo.save(&sequencer).unwrap();
        let mut restored = repo.load().unwrap();
        assert_eq!(restored.last_committed(), sequencer.last_committed());

        let second = restored.sequence(1, 3, ()).unwrap();
        assert_eq!(second.lsn, first.lsn + 1);
        assert!(second.logical_ts > first.logical_ts);
    }

    #[test]
    fn propagates_repository_errors() {
        let mut repo = MemoryRepo { unavailable: true, ..MemoryRepo::default() };
        assert_eq!(repo.load(), Err(RepoError::Unavailable));
        assert_eq!(repo.save(&LogEntrySequencer::new(None)), Err(RepoError::Unavailable));
    }
}
