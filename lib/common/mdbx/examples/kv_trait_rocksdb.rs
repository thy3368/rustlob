use anyhow::{Result, anyhow};
use db_repo::KvStore;
use mdbx::RocksDbKvStore;

fn main() -> Result<()> {
    let store = RocksDbKvStore::open("/tmp/rustlob-kv-trait-rocksdb")?;

    store.put(b"user:1", b"Alice")?;

    let value = store
        .get(b"user:1")?
        .ok_or_else(|| anyhow!("missing key: user:1"))?;

    println!("loaded value: {}", String::from_utf8_lossy(&value));
    Ok(())
}
