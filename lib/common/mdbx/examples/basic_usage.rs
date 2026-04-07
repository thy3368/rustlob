use anyhow::Result;
use libmdbx::{Database, DatabaseOptions, NoWriteMap, TableFlags, WriteFlags};
use rkyv::{
    access,
    api::{deserialize_using, high::to_bytes},
    de::Pool,
    rancor::Error,
    Archive, Deserialize, Serialize,
};
use std::fs;
use std::path::Path;

type Mdbx = Database<NoWriteMap>;

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
    email: String,
}

fn main() -> Result<()> {
    let db_path = Path::new("/tmp/rustlob-mdbx-basic-example");
    fs::create_dir_all(db_path)?;

    let db = Mdbx::open_with_options(
        db_path,
        DatabaseOptions {
            max_tables: Some(4),
            ..Default::default()
        },
    )?;

    {
        let txn = db.begin_rw_txn()?;
        let table = txn.create_table(Some("users"), TableFlags::empty())?;

        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };

        let encoded = to_bytes::<Error>(&user)?;
        txn.put(&table, user.id.to_be_bytes(), encoded.as_slice(), WriteFlags::empty())?;
        txn.commit()?;
    }

    {
        let txn = db.begin_ro_txn()?;
        let table = txn.open_table(Some("users"))?;
        let key = 1u64.to_be_bytes();

        if let Some(raw) = txn.get::<Vec<u8>>(&table, &key)? {
            let archived = access::<ArchivedUser, Error>(&raw)?;
            let user = deserialize_using::<User, _, Error>(archived, &mut Pool::new())?;
            println!("loaded user: {user:?}");
        }

        txn.commit()?;
    }

    Ok(())
}
