use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

/// 区块持久化示例
///
/// 基于 geth 的区块持久化机制实现
/// 参考：go-ethereum/core/rawdb/database.go
///
/// 展示如何使用 MPT 和持久化存储保存以太坊区块数据
use crate::block_data::{Block, BlockHeader, Receipt, Transaction};
use crate::persistent_storage::PersistentStorage;
use crate::trie::MerklePatriciaTrie;
use crate::usecases::{GetUseCase, InsertUseCase, RootHashUseCase};

/// 数据库元数据
struct DatabaseMetadata {
    tx_root: [u8; 32],
    receipt_root: [u8; 32],
    blocks_processed: u64,
}

/// 区块数据库
///
/// 模拟 geth 的 ChainDB，用于存储区块、交易、收据
pub struct BlockDatabase {
    /// 数据目录
    data_dir: String,

    /// 交易树
    tx_trie: MerklePatriciaTrie<PersistentStorage>,

    /// 收据树
    receipt_trie: MerklePatriciaTrie<PersistentStorage>,

    /// 状态树（暂不实现完整状态树）
    // state_trie: MerklePatriciaTrie<PersistentStorage>,
    /// 已处理的区块数量
    blocks_processed: u64,
}

impl BlockDatabase {
    /// 创建新的区块数据库
    pub fn new(data_dir: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir_str = data_dir.as_ref().to_string_lossy().to_string();
        let metadata_path = data_dir.as_ref().join("metadata");

        // 尝试加载现有元数据
        let metadata = Self::load_metadata(&metadata_path);

        // 创建交易树持久化存储
        let tx_storage_path = data_dir.as_ref().join("transactions");
        let tx_storage = PersistentStorage::new(tx_storage_path, 1000)?;

        // 根据是否有元数据决定如何创建trie
        let tx_trie = if let Some(ref meta) = metadata {
            MerklePatriciaTrie::from_root(tx_storage, meta.tx_root)
        } else {
            MerklePatriciaTrie::new(tx_storage)
        };

        // 创建收据树持久化存储
        let receipt_storage_path = data_dir.as_ref().join("receipts");
        let receipt_storage = PersistentStorage::new(receipt_storage_path, 1000)?;

        let receipt_trie = if let Some(ref meta) = metadata {
            MerklePatriciaTrie::from_root(receipt_storage, meta.receipt_root)
        } else {
            MerklePatriciaTrie::new(receipt_storage)
        };

        let blocks_processed = metadata.map(|m| m.blocks_processed).unwrap_or(0);

        Ok(Self { data_dir: data_dir_str, tx_trie, receipt_trie, blocks_processed })
    }

    /// 加载元数据
    fn load_metadata(path: &Path) -> Option<DatabaseMetadata> {
        let mut file = File::open(path).ok()?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).ok()?;

        if buffer.len() < 72 {
            return None;
        }

        let mut tx_root = [0u8; 32];
        let mut receipt_root = [0u8; 32];

        tx_root.copy_from_slice(&buffer[0..32]);
        receipt_root.copy_from_slice(&buffer[32..64]);

        let blocks_processed = u64::from_le_bytes([
            buffer[64], buffer[65], buffer[66], buffer[67], buffer[68], buffer[69], buffer[70],
            buffer[71],
        ]);

        Some(DatabaseMetadata { tx_root, receipt_root, blocks_processed })
    }

    /// 保存元数据
    fn save_metadata(&self) -> Result<(), Box<dyn std::error::Error>> {
        let metadata_path = Path::new(&self.data_dir).join("metadata");
        let mut file =
            OpenOptions::new().write(true).create(true).truncate(true).open(metadata_path)?;

        let tx_root = self.tx_trie.root_hash();
        let receipt_root = self.receipt_trie.root_hash();

        file.write_all(&tx_root)?;
        file.write_all(&receipt_root)?;
        file.write_all(&self.blocks_processed.to_le_bytes())?;

        file.sync_all()?;
        Ok(())
    }

    /// 处理并持久化区块
    ///
    /// 类似 geth 的 WriteBlock 函数
    pub fn write_block(
        &mut self,
        block: &Block,
    ) -> Result<BlockReceipt, Box<dyn std::error::Error>> {
        let start = Instant::now();

        // 1. 持久化交易到交易树
        let tx_start = Instant::now();
        for (index, tx) in block.transactions.iter().enumerate() {
            let key = format!("{}", index);
            let value = tx.serialize();
            self.tx_trie.insert(key.as_bytes(), &value)?;
        }
        let tx_duration = tx_start.elapsed();

        // 2. 持久化收据到收据树
        let receipt_start = Instant::now();
        for (index, receipt) in block.receipts.iter().enumerate() {
            let key = format!("{}", index);
            let value = receipt.serialize();
            self.receipt_trie.insert(key.as_bytes(), &value)?;
        }
        let receipt_duration = receipt_start.elapsed();

        // 3. 获取根哈希
        let tx_root = self.tx_trie.root_hash();
        let receipt_root = self.receipt_trie.root_hash();

        self.blocks_processed += 1;

        // 4. 保存元数据（每写一个区块就保存，确保数据持久化）
        self.save_metadata()?;

        Ok(BlockReceipt {
            block_number: block.number(),
            block_hash: block.hash(),
            tx_count: block.transaction_count(),
            tx_root,
            receipt_root,
            tx_write_duration: tx_duration,
            receipt_write_duration: receipt_duration,
            total_duration: start.elapsed(),
        })
    }

    /// 读取交易
    pub fn read_transaction(
        &self,
        index: usize,
    ) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        let key = format!("{}", index);
        Ok(self.tx_trie.get(key.as_bytes())?)
    }

    /// 读取收据
    pub fn read_receipt(
        &self,
        index: usize,
    ) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        let key = format!("{}", index);
        Ok(self.receipt_trie.get(key.as_bytes())?)
    }

    /// 获取存储统计信息
    pub fn get_stats(&self) -> DatabaseStats {
        DatabaseStats {
            data_dir: self.data_dir.clone(),
            blocks_processed: self.blocks_processed,
            tx_root: hex::encode(self.tx_trie.root_hash()),
            receipt_root: hex::encode(self.receipt_trie.root_hash()),
        }
    }
}

/// 实现 Drop，确保数据库关闭时保存元数据
impl Drop for BlockDatabase {
    fn drop(&mut self) {
        let _ = self.save_metadata();
    }
}

/// 区块写入回执
#[derive(Debug, Clone)]
pub struct BlockReceipt {
    pub block_number: u64,
    pub block_hash: [u8; 32],
    pub tx_count: usize,
    pub tx_root: [u8; 32],
    pub receipt_root: [u8; 32],
    pub tx_write_duration: std::time::Duration,
    pub receipt_write_duration: std::time::Duration,
    pub total_duration: std::time::Duration,
}

/// 数据库统计信息
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub data_dir: String,
    pub blocks_processed: u64,
    pub tx_root: String,
    pub receipt_root: String,
}

/// 运行区块持久化示例
pub fn run_block_persistence_example() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    {
        let l = "=".repeat(70);
        println!("{}", l);
    }
    println!("💾 区块持久化示例 - 基于 Geth 架构");
    {
        let l = "=".repeat(70);
        println!("{}", l);
    }
    println!();

    println!("📖 说明:");
    println!("   本示例模拟 geth 的区块持久化机制");
    println!("   - 使用 MPT 存储交易和收据");
    println!("   - 使用文件系统作为持久化后端");
    println!("   - 支持数据持久化和重启恢复");
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 1: 创建区块数据库
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("🗄️  场景 1: 创建区块数据库");
    println!("{}", "─".repeat(70));
    println!();

    // 使用临时目录
    let temp_dir = std::env::temp_dir().join("mpt_block_db");
    println!("   数据目录: {}", temp_dir.display());

    // 清理旧数据
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
    }

    let mut db = BlockDatabase::new(&temp_dir)?;
    println!("   ✓ 区块数据库创建成功");
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 2: 生成并持久化区块
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("📦 场景 2: 生成并持久化区块");
    println!("{}", "─".repeat(70));
    println!();

    let block_count = 10;
    let tx_per_block = 100;

    println!("   生成 {} 个区块，每个区块 {} 笔交易", block_count, tx_per_block);
    println!();

    let mut total_tx_duration = std::time::Duration::ZERO;
    let mut total_receipt_duration = std::time::Duration::ZERO;
    let mut total_blocks_duration = std::time::Duration::ZERO;

    for block_num in 0..block_count {
        // 创建区块头
        let parent_hash = if block_num == 0 { [0u8; 32] } else { [block_num as u8; 32] };

        let mut header = BlockHeader::new(block_num, parent_hash);
        header.timestamp = 1700000000 + block_num * 12; // 12s 区块时间
        header.gas_limit = 30_000_000;
        header.coinbase = [block_num as u8; 20];

        let mut block = Block::new(header);

        // 添加交易和收据
        let mut cumulative_gas = 0u64;
        for tx_index in 0..tx_per_block {
            let to_addr = [(block_num * 100 + tx_index) as u8; 20];

            let tx = Transaction::eip1559(
                tx_index,
                2_000_000_000, // max fee
                1_000_000_000, // priority fee
                21000,
                Some(to_addr),
                1_000_000_000_000_000_000, // 1 ETH
                Vec::new(),
            );

            cumulative_gas += 21000;
            let receipt = Receipt::new(2, 1, cumulative_gas);

            block.add_transaction(tx, receipt);
        }

        block.header.gas_used = cumulative_gas;

        // 持久化区块
        let receipt = db.write_block(&block)?;

        total_tx_duration += receipt.tx_write_duration;
        total_receipt_duration += receipt.receipt_write_duration;
        total_blocks_duration += receipt.total_duration;

        println!(
            "   ✓ 区块 #{}: {} 笔交易, 耗时 {:.2?}",
            receipt.block_number, receipt.tx_count, receipt.total_duration
        );
    }

    println!();
    println!("   📊 持久化统计:");
    println!("      - 总交易数: {}", block_count * tx_per_block);
    println!("      - 交易写入总耗时: {:.2?}", total_tx_duration);
    println!("      - 收据写入总耗时: {:.2?}", total_receipt_duration);
    println!("      - 平均每区块耗时: {:.2?}", total_blocks_duration / block_count as u32);
    println!(
        "      - 平均每笔交易耗时: {:.2?}",
        total_blocks_duration / (block_count * tx_per_block) as u32
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 3: 读取持久化数据
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("🔍 场景 3: 读取持久化数据");
    println!("{}", "─".repeat(70));
    println!();

    println!("   随机读取交易和收据...");
    let sample_indices = [0, 25, 50, 75, 99];

    let read_start = Instant::now();
    let mut read_count = 0;

    for &index in &sample_indices {
        let tx = db.read_transaction(index)?;
        let receipt = db.read_receipt(index)?;

        if tx.is_some() && receipt.is_some() {
            read_count += 1;
            println!(
                "      ✓ 索引 {}: 交易 {} bytes, 收据 {} bytes",
                index,
                tx.unwrap().len(),
                receipt.unwrap().len()
            );
        }
    }

    let read_duration = read_start.elapsed();

    println!();
    println!("   📊 读取统计:");
    println!("      - 成功读取: {}/{}", read_count, sample_indices.len());
    println!("      - 总耗时: {:.2?}", read_duration);
    println!("      - 平均每次读取: {:.2?}", read_duration / sample_indices.len() as u32);
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 4: 数据库统计信息
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("📈 场景 4: 数据库统计信息");
    println!("{}", "─".repeat(70));
    println!();

    let stats = db.get_stats();

    println!("   🗄️  数据库信息:");
    println!("      - 数据目录: {}", stats.data_dir);
    println!("      - 已处理区块: {}", stats.blocks_processed);
    println!("      - 交易根哈希: {}", &stats.tx_root[..16]);
    println!("      - 收据根哈希: {}", &stats.receipt_root[..16]);
    println!();

    // 计算磁盘使用
    let dir_size = calculate_dir_size(&temp_dir);
    println!("   💾 磁盘使用:");
    println!("      - 总大小: {:.2} MB", dir_size as f64 / 1024.0 / 1024.0);
    println!("      - 平均每区块: {:.2} KB", dir_size as f64 / block_count as f64 / 1024.0);
    println!(
        "      - 平均每笔交易: {:.2} bytes",
        dir_size as f64 / (block_count * tx_per_block) as f64
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 5: 数据持久化验证
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("✅ 场景 5: 数据持久化验证");
    println!("{}", "─".repeat(70));
    println!();

    println!("   模拟程序重启...");
    drop(db); // 关闭数据库

    println!("   重新打开数据库...");
    let db2 = BlockDatabase::new(&temp_dir)?;

    println!("   验证持久化数据...");
    let verify_indices = [0, 50, 99];
    let mut verified = 0;

    for &index in &verify_indices {
        let tx = db2.read_transaction(index)?;
        let receipt = db2.read_receipt(index)?;

        if tx.is_some() && receipt.is_some() {
            verified += 1;
            println!("      ✓ 索引 {}: 数据完整", index);
        } else {
            println!("      ✗ 索引 {}: 数据丢失", index);
        }
    }

    println!();
    println!("   验证结果: {}/{} 成功", verified, verify_indices.len());
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 总结
    // ═══════════════════════════════════════════════════════════════════════
    {
        let l = "=".repeat(70);
        println!("{}", l);
    }
    println!("✨ 区块持久化示例完成！");
    println!();
    println!("📈 性能总结:");
    println!(
        "   • 写入吞吐: {:.0} tx/s",
        (block_count * tx_per_block) as f64 / total_blocks_duration.as_secs_f64()
    );
    println!(
        "   • 平均写入延迟: {:.2?} per tx",
        total_blocks_duration / (block_count * tx_per_block) as u32
    );
    println!("   • 平均读取延迟: {:.2?}", read_duration / sample_indices.len() as u32);
    println!("   • 磁盘使用: {:.2} MB", dir_size as f64 / 1024.0 / 1024.0);
    println!();
    println!("💡 Geth 兼容特性:");
    println!("   ✓ MPT 存储结构（与以太坊一致）");
    println!("   ✓ 交易树和收据树分离存储");
    println!("   ✓ Keccak256 哈希算法");
    println!("   ✓ 持久化存储支持");
    println!("   ✓ 数据重启后可恢复");
    println!();
    println!("🎯 生产环境改进建议:");
    println!("   • 使用 RocksDB 或 LevelDB 作为存储后端");
    println!("   • 实现完整的 RLP 编码/解码");
    println!("   • 添加区块索引（按区块号/哈希快速查询）");
    println!("   • 实现区块链分叉处理");
    println!("   • 添加状态树和账户存储");
    println!("   • 实现区块和交易的验证逻辑");
    {
        let l = "=".repeat(70);
        println!("{}", l);
    }

    Ok(())
}

/// 计算目录大小
fn calculate_dir_size(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }

    let mut total = 0u64;

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total += metadata.len();
                } else if metadata.is_dir() {
                    total += calculate_dir_size(&entry.path());
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_block_database() {
        let temp_dir = tempdir().unwrap();
        let mut db = BlockDatabase::new(temp_dir.path()).unwrap();

        // 创建测试区块
        let header = BlockHeader::new(1, [0u8; 32]);
        let mut block = Block::new(header);

        let tx = Transaction::legacy(
            0,
            1000000000,
            21000,
            Some([1u8; 20]),
            1000000000000000000,
            Vec::new(),
        );
        let receipt = Receipt::new(0, 1, 21000);

        block.add_transaction(tx, receipt);

        // 写入区块
        let result = db.write_block(&block).unwrap();
        assert_eq!(result.block_number, 1);
        assert_eq!(result.tx_count, 1);

        // 读取交易
        let tx_data = db.read_transaction(0).unwrap();
        assert!(tx_data.is_some());

        // 读取收据
        let receipt_data = db.read_receipt(0).unwrap();
        assert!(receipt_data.is_some());
    }
}
