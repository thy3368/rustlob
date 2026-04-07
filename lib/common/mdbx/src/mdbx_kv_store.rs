use db_repo::{KvStore, StorageError};
use libmdbx::{Database, DatabaseOptions, NoWriteMap, TableFlags, WriteFlags};
use std::fs;
use std::path::Path;

type MdbxDatabase = Database<NoWriteMap>;

pub struct MdbxKvStore {
    db: MdbxDatabase,
    table_name: String,
}

impl MdbxKvStore {
    pub fn open(path: impl AsRef<Path>, table_name: impl Into<String>) -> Result<Self, StorageError> {
        fs::create_dir_all(path.as_ref()).map_err(|e| StorageError::Open {
            source: Box::new(e),
        })?;

        let db = MdbxDatabase::open_with_options(
            path,
            DatabaseOptions {
                max_tables: Some(4),
                ..Default::default()
            },
        )
        .map_err(|e| StorageError::Open {
            source: Box::new(e),
        })?;

        Ok(Self {
            db,
            table_name: table_name.into(),
        })
    }
}

impl KvStore for MdbxKvStore {
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), StorageError> {
        let txn = self.db.begin_rw_txn().map_err(|e| StorageError::Write {
            source: Box::new(e),
        })?;
        //todo 每次put都 create_table 会不会有性能问题？
        let table = txn
            .create_table(Some(&self.table_name), TableFlags::empty())
            .map_err(|e| StorageError::Write {
                source: Box::new(e),
            })?;
        txn.put(&table, key, value, WriteFlags::empty())
            .map_err(|e| StorageError::Write {
                source: Box::new(e),
            })?;
        txn.commit().map_err(|e| StorageError::Write {
            source: Box::new(e),
        })?;
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        let txn = self.db.begin_ro_txn().map_err(|e| StorageError::Read {
            source: Box::new(e),
        })?;
        let table = match txn.open_table(Some(&self.table_name)) {
            Ok(table) => table,
            Err(libmdbx::Error::NotFound) => return Ok(None),
            Err(e) => {
                return Err(StorageError::Read {
                    source: Box::new(e),
                })
            }
        };
        let value = txn.get::<Vec<u8>>(&table, key).map_err(|e| StorageError::Read {
            source: Box::new(e),
        })?;
        txn.commit().map_err(|e| StorageError::Read {
            source: Box::new(e),
        })?;
        Ok(value)
    }

    fn delete(&self, key: &[u8]) -> Result<(), StorageError> {
        let txn = self.db.begin_rw_txn().map_err(|e| StorageError::Delete {
            source: Box::new(e),
        })?;
        let table = match txn.open_table(Some(&self.table_name)) {
            Ok(table) => table,
            Err(libmdbx::Error::NotFound) => return Ok(()),
            Err(e) => {
                return Err(StorageError::Delete {
                    source: Box::new(e),
                })
            }
        };
        match txn.del(&table, key, None) {
            Ok(_) | Err(libmdbx::Error::NotFound) => {}
            Err(e) => {
                return Err(StorageError::Delete {
                    source: Box::new(e),
                })
            }
        }
        txn.commit().map_err(|e| StorageError::Delete {
            source: Box::new(e),
        })?;
        Ok(())
    }
}
