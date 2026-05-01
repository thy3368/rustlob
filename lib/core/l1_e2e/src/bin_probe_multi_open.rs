use db_repo::StorageError;
use libmdbx::{Database, DatabaseOptions, NoWriteMap};
use std::fs;
use std::path::Path;

type MdbxDatabase = Database<NoWriteMap>;

fn open_one(path: &Path, label: &str) -> Result<(), StorageError> {
    let db = MdbxDatabase::open_with_options(
        path,
        DatabaseOptions {
            max_tables: Some(16),
            ..Default::default()
        },
    )
    .map_err(|e| StorageError::Open { source: Box::new(e) })?;
    println!("opened {label}: {:p}", &db);
    Ok(())
}

fn main() {
    let path = std::env::current_dir().unwrap().join("tmp/l1_e2e_mdbx_probe_multi");
    fs::create_dir_all(&path).unwrap();
    println!("path={}", path.display());
    println!("first={:?}", open_one(&path, "first"));
    println!("second={:?}", open_one(&path, "second"));
}
