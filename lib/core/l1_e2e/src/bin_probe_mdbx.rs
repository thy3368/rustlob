use l1_adapter::MdbxStateStore;

fn main() {
    let path = std::env::current_dir().unwrap().join("tmp/l1_e2e_mdbx_probe");
    std::fs::create_dir_all(&path).unwrap();
    println!("path={}", path.display());
    match libmdbx::Database::<libmdbx::NoWriteMap>::open_with_options(
        &path,
        libmdbx::DatabaseOptions {
            max_tables: Some(16),
            ..Default::default()
        },
    ) {
        Ok(_) => println!("raw_db_ok"),
        Err(err) => println!("raw_db_err: {:?}", err),
    }
    match MdbxStateStore::open(&path) {
        Ok(_) => println!("store_ok"),
        Err(err) => println!("store_err: {:?}", err),
    }
}
