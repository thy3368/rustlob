use anyhow::{Result, anyhow};
use rocksdb::{DB, Options};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let db_path = Path::new("/tmp/rustlob-rocksdb-basic-example");
    fs::create_dir_all(db_path)?;

    let mut options = Options::default();
    options.create_if_missing(true);

    let db = DB::open(&options, db_path)?;

    db.put(b"user:1", b"Alice")?;

    let value = db
        .get(b"user:1")?
        .ok_or_else(|| anyhow!("missing key: user:1"))?;

    println!("loaded value: {}", String::from_utf8_lossy(&value));
    Ok(())
}
