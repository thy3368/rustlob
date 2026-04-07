以下是Rust中使用MDBX（内存映射数据库）的完整指南，重点介绍libmdbx-rs库的使用方法：

MDBX简介

MDBX是LMDB的增强版，特点是：
• 内存映射数据库，支持零拷贝访问

• 无锁读写并发（读者无锁，写者单写）

• ACID事务支持

• 自动崩溃恢复

• 支持嵌套事务

1. 基本使用示例

Cargo.toml配置

[dependencies]
libmdbx = "0.6"
anyhow = "1.0"
rkyv = { version = "0.8", features = ["std"] }


基本数据库操作

use anyhow::Result;
use libmdbx::{Database, DatabaseOptions, NoWriteMap, TableFlags, WriteFlags};
use rkyv::{
    Archive, Deserialize, Serialize, api::high::{from_bytes, to_bytes_in}, rancor::Error,
    util::AlignedVec,
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

        let mut encoded: AlignedVec<16> = AlignedVec::new();
        to_bytes_in::<_, Error>(&user, &mut encoded)?;
        txn.put(&table, user.id.to_be_bytes(), encoded.as_slice(), WriteFlags::empty())?;
        txn.commit()?;
    }

    {
        let txn = db.begin_ro_txn()?;
        let table = txn.open_table(Some("users"))?;
        let key = 1u64.to_be_bytes();

        if let Some(raw) = txn.get::<Vec<u8>>(&table, &key)? {
            let user = from_bytes::<User, Error>(&raw)?;
            println!("loaded user: {user:?}");
        }

        txn.commit()?;
    }

    Ok(())
}


2. 高级功能示例

数据库配置选项

use libmdbx::{
Environment, EnvironmentKind, NoWriteMap,
Geometry, PageSize, SyncMode
};

fn open_with_options() -> Result<Environment<NoWriteMap>> {
let env = Environment::new()
.set_geometry(Geometry {
// 数据库大小配置
size: Some(0..(1024 * 1024 * 1024)),  // 0-1GB
growth_step: Some(1024 * 1024 * 64),  // 64MB增长步长
shrink_threshold: Some(0),
page_size: Some(PageSize::Set(4096)),  // 4KB页大小
})
.set_max_dbs(20)
.set_max_readers(256)
.set_flags(
libmdbx::EnvironmentFlags::NO_SUB_DIR    // 不创建子目录
| libmdbx::EnvironmentFlags::NO_TLS      // 禁用线程局部存储
| libmdbx::EnvironmentFlags::WRITE_MAP  // 使用写映射（Linux）
| libmdbx::EnvironmentFlags::NO_META_SYNC,  // 元数据异步
)
.set_sync_mode(SyncMode::Standard)  // 标准同步模式
.open(Path::new("data.mdb"))?;

    Ok(env)
}


游标操作

use libmdbx::{Cursor, Transaction, RO, Iter, Range};

fn cursor_examples(env: &Environment) -> Result<()> {
let db = env.open_db(Some("users"))?;
let txn = env.begin_ro_txn()?;

    // 1. 获取游标
    let mut cursor = txn.cursor(&db)?;
    
    // 2. 遍历所有键值对
    println!("All entries:");
    for item in cursor.iter() {
        let (key_bytes, value_bytes) = item?;
        let key = u64::from_be_bytes(key_bytes.try_into().unwrap());
        let user: User = bincode::deserialize(&value_bytes)?;
        println!("  {}: {:?}", key, user);
    }
    
    // 3. 范围查询
    let start_key = 10u64.to_be_bytes();
    let end_key = 20u64.to_be_bytes();
    println!("\nRange 10-20:");
    for item in cursor.iter_from::<Range<&[u8]>>(Range::from(start_key..=end_key))? {
        let (key_bytes, value_bytes) = item?;
        // 处理数据
    }
    
    // 4. 使用游标API
    println!("\nManual cursor navigation:");
    if let Some((first_key, first_value)) = cursor.first()? {
        println!("First: {:?}", first_key);
        
        // 移动到下一个
        if let Some((next_key, _)) = cursor.next()? {
            println!("Next: {:?}", next_key);
        }
        
        // 移动到最后一个
        if let Some((last_key, _)) = cursor.last()? {
            println!("Last: {:?}", last_key);
            
            // 移动到前一个
            if let Some((prev_key, _)) = cursor.prev()? {
                println!("Previous: {:?}", prev_key);
            }
        }
    }
    
    txn.commit()?;
    Ok(())
}


读写事务示例

fn transaction_examples(env: &Environment) -> Result<()> {
let db = env.open_db(Some("users"))?;

    // 1. 基本读写事务
    {
        let mut txn = env.begin_rw_txn()?;
        
        // 批量插入
        for i in 1..=1000 {
            let user = User {
                id: i,
                name: format!("User{}", i),
                email: format!("user{}@example.com", i),
            };
            let data = bincode::serialize(&user)?;
            txn.put(&db, &i.to_be_bytes(), &data, WriteFlags::default())?;
        }
        
        // 更新数据
        let key = 42u64.to_be_bytes();
        if let Some(existing) = txn.get(&db, &key)? {
            let mut user: User = bincode::deserialize(&existing)?;
            user.email = "updated@example.com".to_string();
            let new_data = bincode::serialize(&user)?;
            txn.put(&db, &key, &new_data, WriteFlags::UPSERT)?;
        }
        
        // 删除数据
        txn.del(&db, &999u64.to_be_bytes(), None)?;
        
        txn.commit()?;
    }
    
    // 2. 嵌套事务
    {
        let mut parent_txn = env.begin_rw_txn()?;
        
        // 父事务插入
        let parent_user = User { id: 1001, name: "Parent".to_string(), email: "parent@example.com".to_string() };
        let parent_data = bincode::serialize(&parent_user)?;
        parent_txn.put(&db, &1001u64.to_be_bytes(), &parent_data, WriteFlags::default())?;
        
        // 子事务（嵌套事务）
        {
            let mut child_txn = parent_txn.begin_nested_txn()?;
            
            let child_user = User { id: 1002, name: "Child".to_string(), email: "child@example.com".to_string() };
            let child_data = bincode::serialize(&child_user)?;
            child_txn.put(&db, &1002u64.to_be_bytes(), &child_data, WriteFlags::default())?;
            
            // 子事务可以单独提交或中止
            // child_txn.abort();  // 放弃子事务的修改
            child_txn.commit()?;  // 提交到父事务
        }
        
        // 父事务提交（包含子事务的修改）
        parent_txn.commit()?;
    }
    
    // 3. 事务中止示例
    {
        let mut txn = env.begin_rw_txn()?;
        
        txn.put(&db, &9999u64.to_be_bytes(), b"test", WriteFlags::default())?;
        
        // 发生错误，中止事务
        if some_error_condition {
            txn.abort();
            println!("Transaction aborted, changes discarded");
        } else {
            txn.commit()?;
        }
    }
    
    Ok(())
}


多数据库操作

fn multiple_databases(env: &Environment) -> Result<()> {
// 创建多个数据库
let users_db = env.create_db(Some("users"), libmdbx::DatabaseFlags::default())?;
let products_db = env.create_db(Some("products"), libmdbx::DatabaseFlags::default())?;
let orders_db = env.create_db(Some("orders"), libmdbx::DatabaseFlags::default())?;

    let mut txn = env.begin_rw_txn()?;
    
    // 跨数据库操作
    txn.put(&users_db, b"user1", b"Alice", WriteFlags::default())?;
    txn.put(&products_db, b"prod1", b"Laptop", WriteFlags::default())?;
    txn.put(&orders_db, b"order1", b"user1:prod1", WriteFlags::default())?;
    
    // 原子性跨数据库操作
    txn.commit()?;
    
    Ok(())
}


3. 错误处理和恢复

use libmdbx::{Error, Result as MdbxResult};
use std::time::Duration;

fn robust_transaction(env: &Environment) -> Result<()> {
let db = env.open_db(Some("critical_data"))?;

    // 重试机制
    let mut retries = 3;
    while retries > 0 {
        match env.begin_rw_txn() {
            Ok(mut txn) => {
                match txn.put(&db, b"key", b"value", WriteFlags::default()) {
                    Ok(_) => {
                        match txn.commit() {
                            Ok(_) => {
                                println!("Transaction committed successfully");
                                break;
                            }
                            Err(e) => {
                                eprintln!("Commit failed: {}", e);
                                retries -= 1;
                                std::thread::sleep(Duration::from_millis(100));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Put failed: {}", e);
                        txn.abort();
                        return Err(e.into());
                    }
                }
            }
            Err(e) => {
                eprintln!("Begin transaction failed: {}", e);
                retries -= 1;
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        
        if retries == 0 {
            return Err(anyhow::anyhow!("Failed after multiple retries"));
        }
    }
    
    Ok(())
}


4. 性能优化技巧

fn performance_optimization(env: &Environment) -> Result<()> {
let db = env.create_db(
Some("optimized"),
libmdbx::DatabaseFlags::DUP_SORT  // 允许重复键
| libmdbx::DatabaseFlags::DUP_FIXED  // 固定大小值
| libmdbx::DatabaseFlags::INTEGER_KEY  // 整数键优化
)?;

    let mut txn = env.begin_rw_txn()?;
    
    // 1. 批量插入使用事务
    for i in 0..10000 {
        txn.put(&db, &i.to_be_bytes(), &(i * 2).to_be_bytes(), WriteFlags::APPEND)?;
    }
    
    // 2. 使用APPEND标志加速顺序写入
    txn.put(&db, b"last", b"value", WriteFlags::APPEND)?;
    
    // 3. 使用UPSERT避免先读后写
    txn.put(&db, b"counter", b"100", WriteFlags::UPSERT)?;
    
    // 4. 批量删除
    let mut cursor = txn.cursor(&db)?;
    let start_key = 5000u64.to_be_bytes();
    
    if let Some((key, _)) = cursor.set_range(&start_key)? {
        while let Some((key_bytes, _)) = cursor.next() {
            let key_val = u64::from_be_bytes(key_bytes.try_into().unwrap());
            if key_val >= 6000 {
                break;
            }
            cursor.del(WriteFlags::default())?;
        }
    }
    
    txn.commit()?;
    
    Ok(())
}


5. 与Reth存储的集成示例

// 类似Reth的存储抽象
use libmdbx::{Environment, Transaction, RW, RO};
use std::collections::BTreeMap;
use std::sync::Arc;

trait Table {
const NAME: &'static str;
type Key: AsRef<[u8]>;
type Value: AsRef<[u8]>;
}

struct Database {
env: Arc<Environment>,
tables: BTreeMap<String, libmdbx::Database>,
}

impl Database {
fn open(path: &Path, tables: &[&str]) -> Result<Self> {
let env = Environment::new()
.set_max_dbs(tables.len() as u32 + 1)
.open(path)?;

        let mut table_map = BTreeMap::new();
        for &table_name in tables {
            let db = env.create_db(Some(table_name), libmdbx::DatabaseFlags::default())?;
            table_map.insert(table_name.to_string(), db);
        }
        
        Ok(Self {
            env: Arc::new(env),
            tables: table_map,
        })
    }
    
    fn get<K, V>(&self, table: &str, key: &K) -> Result<Option<V>>
    where
        K: AsRef<[u8]>,
        V: for<'a> serde::Deserialize<'a>,
    {
        let db = self.tables.get(table).ok_or_else(|| anyhow::anyhow!("Table not found"))?;
        let txn = self.env.begin_ro_txn()?;
        
        if let Some(data) = txn.get(db, key.as_ref())? {
            let value = bincode::deserialize(&data)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
    
    fn put<K, V>(&self, table: &str, key: K, value: V) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: serde::Serialize,
    {
        let db = self.tables.get(table).ok_or_else(|| anyhow::anyhow!("Table not found"))?;
        let mut txn = self.env.begin_rw_txn()?;
        
        let data = bincode::serialize(&value)?;
        txn.put(db, key.as_ref(), &data, WriteFlags::default())?;
        
        txn.commit()?;
        Ok(())
    }
}


6. 监控和统计

fn database_statistics(env: &Environment) -> Result<()> {
let txn = env.begin_ro_txn()?;

    // 获取环境统计
    let stat = env.stat()?;
    println!("Page size: {}", stat.page_size());
    println!("Tree depth: {}", stat.depth());
    println!("Entries: {}", stat.entries());
    println!("Branch pages: {}", stat.branch_pages());
    println!("Leaf pages: {}", stat.leaf_pages());
    println!("Overflow pages: {}", stat.overflow_pages());
    
    // 获取环境信息
    let info = env.info()?;
    println!("Map size: {}", info.map_size());
    println!("Last page number: {}", info.last_pgno());
    println!("Last transaction ID: {}", info.last_txnid());
    println!("Max readers: {}", info.max_readers());
    println!("Number of readers: {}", info.num_readers());
    
    txn.commit()?;
    Ok(())
}


最佳实践

1. 事务管理
   • 保持事务尽可能短

   • 批量操作使用单个事务

   • 及时提交或中止事务

2. 内存管理
   • 合理设置map_size

   • 监控内存使用

   • 定期压缩数据库

3. 并发控制
   • 读者可以并发访问

   • 写者需要序列化访问

   • 考虑使用读写分离

4. 错误处理
   • 始终检查事务结果

   • 实现重试机制

   • 处理MDBX_MAP_FULL等错误

5. 性能调优
   • 使用整数键(DUP_SORT + INTEGER_KEY)

   • 批量操作减少事务开销

   • 合理设置页大小和增长步长

这些示例展示了MDBX在Rust中的基本和高级用法。在实际项目中，如Reth，通常会在此基础上构建更高级的抽象层。