use anyhow::{Result, anyhow};
use db_repo::{RkyvKvStoreExt, StorageError};
use mdbx::RocksDbKvStore;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug)]
struct ValueMismatch;

impl std::fmt::Display for ValueMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("loaded value mismatch")
    }
}

impl std::error::Error for ValueMismatch {}

fn main() -> Result<()> {
    let store = RocksDbKvStore::open("/tmp/rustlob-kv-typed-rocksdb")?;

    let alice = User {
        id: 1,
        name: "Alice".to_string(),
    };
    store.put_obj(b"user:1", &alice)?;

    let loaded = store
        .get_obj::<User>(b"user:1")?
        .ok_or_else(|| anyhow!("missing key: user:1"))?;

    if loaded != alice {
        return Err(StorageError::Codec {
            source: Box::new(ValueMismatch),
        }
        .into());
    }

    println!("loaded user: {loaded:?}");
    Ok(())
}
