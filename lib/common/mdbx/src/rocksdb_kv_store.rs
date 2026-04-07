use db_repo::{KvStore, StorageError};
use rocksdb::{DB, Options};
use std::path::Path;

pub struct RocksDbKvStore {
    db: DB,
}

impl RocksDbKvStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let mut options = Options::default();
        options.create_if_missing(true);
        let db = DB::open(&options, path).map_err(|e| StorageError::Open {
            source: Box::new(e),
        })?;
        Ok(Self { db })
    }
}

impl KvStore for RocksDbKvStore {
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), StorageError> {
        self.db.put(key, value).map_err(|e| StorageError::Write {
            source: Box::new(e),
        })
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        self.db.get(key).map_err(|e| StorageError::Read {
            source: Box::new(e),
        })
    }

    fn delete(&self, key: &[u8]) -> Result<(), StorageError> {
        self.db.delete(key).map_err(|e| StorageError::Delete {
            source: Box::new(e),
        })
    }
}
