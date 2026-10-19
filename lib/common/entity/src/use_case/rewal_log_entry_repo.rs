use std::collections::BTreeMap;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

use rewal::{Batch, SyncMode, Wal};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::log_entry::LogEntry;
use super::log_entry_repo::LogEntryRepo;
use crate::LogEntrySequencer;

const RECORD_VERSION: u8 = 2;
const FRAME_HEADER_LEN: usize = 24;

#[derive(Debug, Error)]
pub enum RewalLogEntryRepoError {
    #[error("WAL I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("WAL error: {0}")]
    Wal(#[from] rewal::Error),
    #[error("record codec error: {0}")]
    Codec(#[from] bincode::Error),
    #[error("empty log batch")]
    EmptyBatch,
    #[error("LSN {next} does not follow {previous}")]
    LsnOrder { previous: u64, next: u64 },
    #[error("corrupt log record: {0}")]
    CorruptRecord(&'static str),
    #[error("incomplete WAL tail")]
    IncompleteTail,
    #[error("WAL sync failed: {0}")]
    Sync(rewal::Error),
    #[error("repository requires reopening after an uncertain write")]
    UncertainWrite,
}

#[derive(Serialize, Deserialize)]
struct StoredRecord<P> {
    version: u8,
    lsn: u64,
    logical_ts: u64,
    client_id: u64,
    request_id: u64,
    t: u8,
    p: u8,
    payload: P,
}

/// 同步持久化的日志仓库。业务 LSN 与 rewal 自身的物理 LSN 分开保存。
pub struct RewalLogEntryRepo<P> {
    wal: Wal,
    index: BTreeMap<u64, u64>,
    tail_lsn: Option<u64>,
    uncertain_write: bool,
    marker: PhantomData<P>,
}

impl<P: Serialize + DeserializeOwned> RewalLogEntryRepo<P> {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, RewalLogEntryRepoError> {
        let path = path.as_ref();
        if path.exists() {
            check_wal_frames(&fs::read(path)?)?;
        }
        let wal = Wal::open(path).sync_mode(SyncMode::Never).build()?;
        let mut index = BTreeMap::new();
        let mut iterator = wal.iterator(0)?;
        let mut tail_lsn = None;
        while let Some(event) = iterator.next()? {
            let record: StoredRecord<P> = decode_record(&event.payload)?;
            if let Some(previous) = tail_lsn {
                if record.lsn <= previous {
                    return Err(RewalLogEntryRepoError::LsnOrder { previous, next: record.lsn });
                }
            }
            index.insert(record.lsn, event.lsn);
            tail_lsn = Some(record.lsn);
        }
        Ok(Self { wal, index, tail_lsn, uncertain_write: false, marker: PhantomData })
    }
}

impl<P: Serialize + DeserializeOwned> LogEntryRepo<P> for RewalLogEntryRepo<P> {
    type Error = RewalLogEntryRepoError;

    fn append_batch(&mut self, entries: Vec<LogEntry<P>>) -> Result<(), Self::Error> {
        if self.uncertain_write {
            return Err(RewalLogEntryRepoError::UncertainWrite);
        }
        if entries.is_empty() {
            return Err(RewalLogEntryRepoError::EmptyBatch);
        }
        let mut previous = self.tail_lsn;
        for entry in &entries {
            if let Some(last) = previous {
                if entry.lsn() <= last {
                    return Err(RewalLogEntryRepoError::LsnOrder {
                        previous: last,
                        next: entry.lsn(),
                    });
                }
            }
            previous = Some(entry.lsn());
        }

        let first_lsn = entries[0].lsn();
        let last_lsn = entries[entries.len() - 1].lsn();
        let count = entries.len();
        let mut batch = Batch::with_capacity(count);
        let mut lsns = Vec::with_capacity(count);
        for entry in entries {
            lsns.push(entry.lsn());
            let record = StoredRecord {
                version: RECORD_VERSION,
                lsn: entry.lsn(),
                logical_ts: entry.logical_ts(),
                client_id: entry.client_id(),
                request_id: entry.request_id(),
                t: entry.topic(),
                p: entry.partition(),
                payload: entry.into_payload(),
            };
            batch.add(bincode::serialize(&record)?);
        }
        let wal_last = match self.wal.append_batch(batch) {
            Ok(lsn) => lsn,
            Err(error) => {
                self.uncertain_write = true;
                return Err(error.into());
            }
        };
        if let Err(error) = self.wal.flush() {
            self.uncertain_write = true;
            return Err(error.into());
        }
        if let Err(error) = self.wal.sync() {
            self.uncertain_write = true;
            return Err(RewalLogEntryRepoError::Sync(error));
        }
        let wal_first = wal_last - count as u64 + 1;
        for (offset, lsn) in lsns.into_iter().enumerate() {
            self.index.insert(lsn, wal_first + offset as u64);
        }
        debug_assert_eq!(self.index.get(&first_lsn), Some(&wal_first));
        self.tail_lsn = Some(last_lsn);
        Ok(())
    }

    fn read_from(&self, from_lsn: u64, limit: usize) -> Result<Vec<LogEntry<P>>, Self::Error> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let Some((_, &wal_lsn)) = self.index.range(from_lsn..).next() else {
            return Ok(Vec::new());
        };
        let mut iterator = self.wal.iterator(wal_lsn)?;
        let mut result = Vec::with_capacity(limit.min(self.index.len()));
        while result.len() < limit {
            let Some(event) = iterator.next()? else { break };
            let record: StoredRecord<P> = decode_record(&event.payload)?;
            result.push(LogEntrySequencer::from_metadata_with_topic(
                record.t,
                record.p,
                record.lsn,
                record.logical_ts,
                record.client_id,
                record.request_id,
                record.payload,
            ));
        }
        Ok(result)
    }
}

fn decode_record<P: DeserializeOwned>(
    bytes: &[u8],
) -> Result<StoredRecord<P>, RewalLogEntryRepoError> {
    let record: StoredRecord<P> = bincode::deserialize(bytes)?;
    if record.version != RECORD_VERSION {
        return Err(RewalLogEntryRepoError::CorruptRecord("unsupported record version"));
    }
    Ok(record)
}

// rewal 0.1.0 截断任何无法解码的尾帧；预检区分完整帧损坏与写入中断。
fn check_wal_frames(data: &[u8]) -> Result<(), RewalLogEntryRepoError> {
    let mut offset = 0;
    while offset < data.len() {
        let tail = &data[offset..];
        if tail.len() < FRAME_HEADER_LEN {
            return if b"EWAL".starts_with(&tail[..tail.len().min(4)]) || tail.starts_with(b"EWAL") {
                Ok(())
            } else {
                Err(RewalLogEntryRepoError::CorruptRecord("invalid frame header"))
            };
        }
        if &tail[..4] != b"EWAL" {
            return Err(RewalLogEntryRepoError::CorruptRecord("invalid frame magic"));
        }
        if u16::from_le_bytes([tail[4], tail[5]]) != 2 {
            return Err(RewalLogEntryRepoError::CorruptRecord("unsupported frame version"));
        }
        let size = u32::from_le_bytes(
            tail[20..24].try_into().map_err(|_| RewalLogEntryRepoError::IncompleteTail)?,
        ) as usize;
        if size < 28 {
            return Err(RewalLogEntryRepoError::CorruptRecord("invalid frame size"));
        }
        if size > tail.len() {
            return Ok(());
        }
        let (_, _, frame_size) =
            rewal::encoding::decode_batch_header(data, offset).map_err(|_| {
                RewalLogEntryRepoError::CorruptRecord("invalid frame checksum or header")
            })?;
        offset += frame_size;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    static NEXT_PATH: AtomicU64 = AtomicU64::new(0);

    fn path() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "entity-rewal-{}-{}.wal",
            std::process::id(),
            NEXT_PATH.fetch_add(1, Ordering::Relaxed)
        ))
    }

    fn entry(lsn: u64, payload: &str) -> LogEntry<String> {
        LogEntrySequencer::from_metadata(lsn, lsn + 100, 7, lsn, payload.to_owned())
    }

    #[test]
    fn empty_append_recovery_and_boundaries() {
        let path = path();
        let mut repo = RewalLogEntryRepo::<String>::open(&path).unwrap();
        assert!(repo.read_from(0, 10).unwrap().is_empty());
        assert!(matches!(repo.append_batch(vec![]), Err(RewalLogEntryRepoError::EmptyBatch)));
        repo.append_batch(vec![entry(4, "a"), entry(8, "non-copy")]).unwrap();
        assert!(matches!(
            repo.append(entry(8, "duplicate")),
            Err(RewalLogEntryRepoError::LsnOrder { .. })
        ));
        assert!(matches!(
            repo.append_batch(vec![entry(10, "a"), entry(9, "b")]),
            Err(RewalLogEntryRepoError::LsnOrder { .. })
        ));
        assert_eq!(repo.read_from(5, 1).unwrap(), vec![entry(8, "non-copy")]);
        assert!(repo.read_from(0, 0).unwrap().is_empty());
        drop(repo);

        let mut repo = RewalLogEntryRepo::<String>::open(&path).unwrap();
        assert_eq!(repo.read_from(0, 10).unwrap(), vec![entry(4, "a"), entry(8, "non-copy")]);
        repo.append(entry(12, "later")).unwrap();
        assert_eq!(repo.read_from(9, 10).unwrap(), vec![entry(12, "later")]);
        drop(repo);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn truncated_tail_is_recovered_but_corrupt_complete_frame_is_rejected() {
        let path = path();
        let mut repo = RewalLogEntryRepo::<String>::open(&path).unwrap();
        repo.append(entry(1, "first")).unwrap();
        drop(repo);
        let mut bytes = fs::read(&path).unwrap();
        bytes.extend_from_slice(b"EWA");
        fs::write(&path, &bytes).unwrap();
        let repo = RewalLogEntryRepo::<String>::open(&path).unwrap();
        assert_eq!(repo.read_from(0, 10).unwrap(), vec![entry(1, "first")]);
        drop(repo);
        let mut bytes = fs::read(&path).unwrap();
        bytes[30] ^= 1;
        fs::write(&path, bytes).unwrap();
        assert!(matches!(
            RewalLogEntryRepo::<String>::open(&path),
            Err(RewalLogEntryRepoError::CorruptRecord(_))
        ));
        fs::remove_file(path).unwrap();
    }
}
