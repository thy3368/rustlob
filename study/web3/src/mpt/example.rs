/// MPT 使用示例
///
/// 展示如何使用 trait-based MPT 接口

use crate::{
    storage::InMemoryStorage,
    trie::MerklePatriciaTrie,
    usecases::{DeleteUseCase, GetUseCase, InsertUseCase, IteratorUseCase, MptUseCases, RootHashUseCase},
};

/// 运行 MPT 基本操作示例
pub fn run_basic_example() -> Result<(), Box<dyn std::error::Error>> {
    let line = "=".repeat(70);
    println!("{}", line);
    println!("🌳 Merkle Patricia Trie (MPT) 基本操作示例");
    println!("{}", line);
    println!();

    // 1. 创建 MPT
    println!("📦 步骤 1: 创建 Merkle Patricia Trie");
    let mut trie = MerklePatriciaTrie::new(InMemoryStorage::new());
    println!("   ✓ MPT 创建成功");
    println!("   初始根哈希: {:?}", hex::encode(trie.root_hash()));
    println!();

    // 2. 插入数据
    println!("➕ 步骤 2: 插入键值对");
    let entries = [
        (b"alice" as &[u8], b"100 ETH" as &[u8]),
        (b"bob", b"50 ETH"),
        (b"charlie", b"75 ETH"),
        (b"david", b"200 ETH"),
    ];

    for (key, value) in &entries {
        trie.insert(key, value)?;
        println!("   ✓ 插入: {} => {}", String::from_utf8_lossy(key), String::from_utf8_lossy(value));
    }
    println!("   根哈希更新: {:?}", hex::encode(trie.root_hash()));
    println!();

    // 3. 查询数据
    println!("🔍 步骤 3: 查询键值对");
    for (key, expected_value) in &entries {
        let value = trie.get(key)?;
        match value {
            Some(v) => {
                println!("   ✓ 查询 {}: {}", String::from_utf8_lossy(key), String::from_utf8_lossy(&v));
                assert_eq!(&v, expected_value);
            }
            None => {
                println!("   ✗ 键不存在: {}", String::from_utf8_lossy(key));
            }
        }
    }
    println!();

    // 4. 更新数据
    println!("🔄 步骤 4: 更新键值对");
    trie.insert(b"alice", b"150 ETH")?;
    let new_value = trie.get(b"alice")?;
    println!("   ✓ Alice 的余额更新: {}", String::from_utf8_lossy(&new_value.unwrap()));
    println!("   根哈希更新: {:?}", hex::encode(trie.root_hash()));
    println!();

    // 5. 删除数据
    println!("🗑️  步骤 5: 删除键值对");
    let deleted = trie.delete(b"bob")?;
    match deleted {
        Some(v) => {
            println!("   ✓ 删除 Bob: {}", String::from_utf8_lossy(&v));
        }
        None => {
            println!("   ✗ Bob 不存在");
        }
    }
    println!();

    // 6. 遍历所有键值对
    println!("📋 步骤 6: 遍历所有键值对");
    println!("   总数: {} 个键值对", trie.len());
    for (key, value) in trie.entries() {
        println!("   - {} => {}", String::from_utf8_lossy(&key), String::from_utf8_lossy(&value));
    }
    println!();

    // 7. 快照和恢复
    println!("📸 步骤 7: 创建快照并恢复");
    let snapshot = trie.snapshot()?;
    println!("   ✓ 快照创建成功");
    println!("   - 快照大小: {} bytes", snapshot.size_bytes());
    println!("   - 包含 {} 个键值对", snapshot.len());

    trie.clear()?;
    println!("   ✓ 清空 MPT");
    println!("   - 剩余键值对: {}", trie.len());

    trie.restore(&snapshot)?;
    println!("   ✓ 从快照恢复");
    println!("   - 恢复后键值对: {}", trie.len());
    println!();

    {let l = "=".repeat(70); println!("{}", l);}
    println!("✨ MPT 基本操作示例完成！");
    {let l = "=".repeat(70); println!("{}", l);}

    Ok(())
}

/// 运行 MPT 高级操作示例
pub fn run_advanced_example() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    {let l = "=".repeat(70); println!("{}", l);}
    println!("🚀 Merkle Patricia Trie (MPT) 高级操作示例");
    {let l = "=".repeat(70); println!("{}", l);}
    println!();

    let mut trie = MerklePatriciaTrie::new(InMemoryStorage::new());

    // 1. 批量插入
    println!("📦 步骤 1: 批量插入");
    let batch_entries: Vec<(&[u8], &[u8])> = (0..10)
        .map(|i| {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            (
                Box::leak(key.into_boxed_str()).as_bytes(),
                Box::leak(value.into_boxed_str()).as_bytes(),
            )
        })
        .collect();

    let count = trie.batch_insert(&batch_entries)?;
    println!("   ✓ 批量插入成功: {} 个键值对", count);
    println!();

    // 2. 批量查询
    println!("🔍 步骤 2: 批量查询");
    let keys: Vec<&[u8]> = batch_entries.iter().map(|(k, _)| *k).take(5).collect();
    let results = trie.batch_get(&keys)?;
    println!("   查询结果:");
    for (i, result) in results.iter().enumerate() {
        match result {
            Some(v) => println!("      ✓ {} => {}", String::from_utf8_lossy(keys[i]), String::from_utf8_lossy(v)),
            None => println!("      ✗ {} 不存在", String::from_utf8_lossy(keys[i])),
        }
    }
    println!();

    // 3. 性能测试
    println!("⚡ 步骤 3: 性能测试");
    let start = std::time::Instant::now();

    for i in 0..1000 {
        let key = format!("perf_key_{}", i);
        let value = format!("perf_value_{}", i);
        trie.insert(key.as_bytes(), value.as_bytes())?;
    }

    let insert_duration = start.elapsed();
    println!("   ✓ 1000 次插入耗时: {:?}", insert_duration);
    println!("   - 平均每次插入: {:?}", insert_duration / 1000);

    let start = std::time::Instant::now();
    for i in 0..1000 {
        let key = format!("perf_key_{}", i);
        let _ = trie.get(key.as_bytes())?;
    }
    let get_duration = start.elapsed();
    println!("   ✓ 1000 次查询耗时: {:?}", get_duration);
    println!("   - 平均每次查询: {:?}", get_duration / 1000);
    println!();

    // 4. 根哈希验证
    println!("🔐 步骤 4: 根哈希验证");
    let root1 = trie.root_hash();
    println!("   初始根哈希: {}", hex::encode(root1));

    trie.insert(b"test", b"data")?;
    let root2 = trie.root_hash();
    println!("   插入后根哈希: {}", hex::encode(root2));
    assert_ne!(root1, root2, "根哈希应该改变");

    trie.delete(b"test")?;
    let root3 = trie.root_hash();
    println!("   删除后根哈希: {}", hex::encode(root3));
    println!();

    {let l = "=".repeat(70); println!("{}", l);}
    println!("✨ MPT 高级操作示例完成！");
    {let l = "=".repeat(70); println!("{}", l);}

    Ok(())
}

/// 运行以太坊状态树示例
pub fn run_ethereum_state_example() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    {let l = "=".repeat(70); println!("{}", l);}
    println!("⛓️  以太坊状态树模拟示例");
    {let l = "=".repeat(70); println!("{}", l);}
    println!();

    let mut state_trie = MerklePatriciaTrie::new(InMemoryStorage::new());

    // 模拟以太坊账户状态
    println!("📝 步骤 1: 创建账户状态");

    #[derive(Debug)]
    struct Account {
        address: &'static str,
        balance: u64,
        nonce: u64,
    }

    let accounts = vec![
        Account {
            address: "0x1234567890abcdef1234567890abcdef12345678",
            balance: 1_000_000_000_000_000_000,
            nonce: 0,
        },
        Account {
            address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
            balance: 500_000_000_000_000_000,
            nonce: 5,
        },
        Account {
            address: "0xfedcba9876543210fedcba9876543210fedcba98",
            balance: 2_000_000_000_000_000_000,
            nonce: 10,
        },
    ];

    for account in &accounts {
        let value = format!("balance:{},nonce:{}", account.balance, account.nonce);
        state_trie.insert(account.address.as_bytes(), value.as_bytes())?;
        println!("   ✓ 账户: {}", account.address);
        println!("      余额: {} wei", account.balance);
        println!("      Nonce: {}", account.nonce);
    }
    println!();

    // 计算状态根
    println!("🌳 步骤 2: 计算状态根");
    let state_root = state_trie.root_hash();
    println!("   状态根: {}", hex::encode(state_root));
    println!("   账户总数: {}", state_trie.len());
    println!();

    // 模拟交易：转账
    println!("💸 步骤 3: 执行交易");
    let from_addr = accounts[0].address;
    let to_addr = accounts[1].address;
    let amount = 100_000_000_000_000_000u64;

    println!("   交易: {} -> {}", from_addr, to_addr);
    println!("   金额: {} wei", amount);

    // 更新发送方
    let from_state = state_trie.get(from_addr.as_bytes())?.unwrap();
    let from_state_str = String::from_utf8_lossy(&from_state);
    let from_parts: Vec<&str> = from_state_str.split(',').collect();
    let from_balance: u64 = from_parts[0].split(':').nth(1).unwrap().parse()?;
    let from_nonce: u64 = from_parts[1].split(':').nth(1).unwrap().parse()?;

    let new_from_state = format!("balance:{},nonce:{}", from_balance - amount, from_nonce + 1);
    state_trie.insert(from_addr.as_bytes(), new_from_state.as_bytes())?;

    // 更新接收方
    let to_state = state_trie.get(to_addr.as_bytes())?.unwrap();
    let to_state_str = String::from_utf8_lossy(&to_state);
    let to_parts: Vec<&str> = to_state_str.split(',').collect();
    let to_balance: u64 = to_parts[0].split(':').nth(1).unwrap().parse()?;
    let to_nonce: u64 = to_parts[1].split(':').nth(1).unwrap().parse()?;

    let new_to_state = format!("balance:{},nonce:{}", to_balance + amount, to_nonce);
    state_trie.insert(to_addr.as_bytes(), new_to_state.as_bytes())?;

    println!("   ✓ 交易执行成功");
    println!();

    // 新状态根
    println!("🔄 步骤 4: 更新状态根");
    let new_state_root = state_trie.root_hash();
    println!("   旧状态根: {}", hex::encode(state_root));
    println!("   新状态根: {}", hex::encode(new_state_root));
    assert_ne!(state_root, new_state_root);
    println!("   ✓ 状态根已更新");
    println!();

    {let l = "=".repeat(70); println!("{}", l);}
    println!("✨ 以太坊状态树模拟完成！");
    {let l = "=".repeat(70); println!("{}", l);}

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert!(run_basic_example().is_ok());
    }

    #[test]
    fn test_advanced_example() {
        assert!(run_advanced_example().is_ok());
    }

    #[test]
    fn test_ethereum_state_example() {
        assert!(run_ethereum_state_example().is_ok());
    }

    #[test]
    fn test_ethereum_transaction_example() {
        assert!(run_ethereum_transaction_example().is_ok());
    }

    #[test]
    fn test_light_client_example() {
        assert!(run_light_client_example().is_ok());
    }
}

/// 运行以太坊交易树和收据树高频场景示例
///
/// 模拟以太坊区块中的交易树和收据树，展示高性能插入和查询
pub fn run_ethereum_transaction_example() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    {let l = "=".repeat(70); println!("{}", l);}
    println!("🚀 以太坊交易树与收据树 - 高频场景示例");
    {let l = "=".repeat(70); println!("{}", l);}
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 1: 交易树 (Transaction Trie) - 高频交易处理
    // ═══════════════════════════════════════════════════════════════════════
    println!("📋 场景 1: 交易树 - 区块交易批量处理");
    println!("{}", "─".repeat(70));
    println!();

    let mut tx_trie = MerklePatriciaTrie::new(InMemoryStorage::new());

    // 模拟以太坊交易结构
    #[derive(Debug, Clone)]
    struct Transaction {
        from: &'static str,
        to: &'static str,
        value: u64,
        gas_price: u64,
        gas_limit: u64,
        nonce: u64,
        data: &'static str,
    }

    impl Transaction {
        fn serialize(&self) -> String {
            format!(
                "from:{},to:{},value:{},gas_price:{},gas_limit:{},nonce:{},data:{}",
                self.from, self.to, self.value, self.gas_price, self.gas_limit, self.nonce, self.data
            )
        }

        fn hash(&self, index: usize) -> Vec<u8> {
            use sha3::{Digest, Keccak256};
            let mut hasher = Keccak256::new();
            hasher.update(format!("{}{}", index, self.serialize()));
            hasher.finalize().to_vec()
        }
    }

    // 模拟一个典型区块的交易（100-200笔交易）
    println!("📊 准备模拟交易数据...");
    let block_number = 18_000_000u64;
    let tx_count = 150;

    let mut transactions = Vec::new();
    for i in 0..tx_count {
        transactions.push(Transaction {
            from: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
            to: "0x8ac76a51cc950d9822d68b83fE1Ad97B32Cd580d",
            value: 1_000_000_000_000_000_000 + i as u64 * 1000,
            gas_price: 50_000_000_000,
            gas_limit: 21000,
            nonce: i as u64,
            data: "0x",
        });
    }
    println!("   ✓ 生成 {} 笔交易", tx_count);
    println!();

    // 高性能批量插入交易
    println!("⚡ 执行高频插入操作...");
    let start = std::time::Instant::now();

    for (index, tx) in transactions.iter().enumerate() {
        // 使用交易索引作为键（RLP 编码的索引）
        let key = format!("{}", index);
        let value = tx.serialize();
        tx_trie.insert(key.as_bytes(), value.as_bytes())?;
    }

    let insert_duration = start.elapsed();
    let tx_root = tx_trie.root_hash();

    println!("   ✓ 插入 {} 笔交易完成", tx_count);
    println!("   - 总耗时: {:?}", insert_duration);
    println!("   - 平均每笔: {:.2?}", insert_duration / tx_count);
    println!("   - 吞吐量: {:.0} tx/s", tx_count as f64 / insert_duration.as_secs_f64());
    println!("   📍 交易根: {}", hex::encode(tx_root));
    println!();

    // 高频查询测试
    println!("🔍 执行高频查询操作...");
    let query_count = 50;
    let start = std::time::Instant::now();

    for i in 0..query_count {
        let key = format!("{}", i);
        let result = tx_trie.get(key.as_bytes())?;
        assert!(result.is_some(), "Transaction {} should exist", i);
    }

    let query_duration = start.elapsed();
    println!("   ✓ 查询 {} 笔交易完成", query_count);
    println!("   - 总耗时: {:?}", query_duration);
    println!("   - 平均每次: {:.2?}", query_duration / query_count);
    println!("   - QPS: {:.0} queries/s", query_count as f64 / query_duration.as_secs_f64());
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 2: 收据树 (Receipt Trie) - 交易执行结果
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("📜 场景 2: 收据树 - 交易执行结果批量处理");
    println!("{}", "─".repeat(70));
    println!();

    let mut receipt_trie = MerklePatriciaTrie::new(InMemoryStorage::new());

    // 模拟交易收据结构
    #[derive(Debug, Clone)]
    struct Receipt {
        tx_hash: Vec<u8>,
        status: bool,
        gas_used: u64,
        cumulative_gas: u64,
        logs_count: usize,
        contract_address: Option<&'static str>,
    }

    impl Receipt {
        fn serialize(&self) -> String {
            format!(
                "tx_hash:{},status:{},gas_used:{},cumulative_gas:{},logs:{},contract:{}",
                hex::encode(&self.tx_hash),
                if self.status { "1" } else { "0" },
                self.gas_used,
                self.cumulative_gas,
                self.logs_count,
                self.contract_address.unwrap_or("null")
            )
        }
    }

    // 为每笔交易生成收据
    println!("📊 生成交易收据...");
    let mut receipts = Vec::new();
    let mut cumulative_gas = 0u64;

    for (index, tx) in transactions.iter().enumerate() {
        let gas_used = 21000 + (index % 5) as u64 * 10000;
        cumulative_gas += gas_used;

        receipts.push(Receipt {
            tx_hash: tx.hash(index),
            status: true,
            gas_used,
            cumulative_gas,
            logs_count: index % 3,
            contract_address: if index % 10 == 0 {
                Some("0x1234567890abcdef1234567890abcdef12345678")
            } else {
                None
            },
        });
    }
    println!("   ✓ 生成 {} 个收据", receipts.len());
    println!();

    // 高性能批量插入收据
    println!("⚡ 执行收据批量插入...");
    let start = std::time::Instant::now();

    for (index, receipt) in receipts.iter().enumerate() {
        let key = format!("{}", index);
        let value = receipt.serialize();
        receipt_trie.insert(key.as_bytes(), value.as_bytes())?;
    }

    let receipt_insert_duration = start.elapsed();
    let receipt_root = receipt_trie.root_hash();

    println!("   ✓ 插入 {} 个收据完成", receipts.len());
    println!("   - 总耗时: {:?}", receipt_insert_duration);
    println!("   - 平均每个: {:.2?}", receipt_insert_duration / receipts.len() as u32);
    println!("   - 吞吐量: {:.0} receipt/s", receipts.len() as f64 / receipt_insert_duration.as_secs_f64());
    println!("   📍 收据根: {}", hex::encode(receipt_root));
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 3: 区块验证 - 根哈希一致性检查
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("🔐 场景 3: 区块完整性验证");
    println!("{}", "─".repeat(70));
    println!();

    println!("📋 区块头信息:");
    println!("   区块号: {}", block_number);
    println!("   交易数: {}", tx_count);
    println!("   Gas 使用: {}/{}", cumulative_gas, 30_000_000);
    println!("   Gas 使用率: {:.1}%", cumulative_gas as f64 / 30_000_000.0 * 100.0);
    println!();

    println!("🌳 Merkle 树根:");
    println!("   📍 交易根:  {}", hex::encode(tx_root));
    println!("   📍 收据根:  {}", hex::encode(receipt_root));
    println!();

    // 验证数据完整性
    println!("✅ 验证数据完整性...");
    let sample_indices = [0, 50, 99, 149];

    for &index in &sample_indices {
        let tx_key = format!("{}", index);
        let tx_exists = tx_trie.get(tx_key.as_bytes())?.is_some();
        let receipt_exists = receipt_trie.get(tx_key.as_bytes())?.is_some();

        if tx_exists && receipt_exists {
            println!("   ✓ 交易 #{}: 交易 ✓  收据 ✓", index);
        } else {
            println!("   ✗ 交易 #{}: 数据不完整", index);
        }
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 4: 性能对比与统计
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("📊 场景 4: 性能统计与分析");
    println!("{}", "─".repeat(70));
    println!();

    println!("⚡ 写入性能:");
    println!("   交易树: {:.0} tx/s  (avg: {:.2?}/tx)",
        tx_count as f64 / insert_duration.as_secs_f64(),
        insert_duration / tx_count);
    println!("   收据树: {:.0} receipt/s  (avg: {:.2?}/receipt)",
        receipts.len() as f64 / receipt_insert_duration.as_secs_f64(),
        receipt_insert_duration / receipts.len() as u32);
    println!();

    println!("🔍 读取性能:");
    println!("   QPS: {:.0} queries/s", query_count as f64 / query_duration.as_secs_f64());
    println!("   延迟: {:.2?} per query", query_duration / query_count);
    println!();

    println!("💾 存储统计:");
    println!("   交易树条目: {}", tx_trie.len());
    println!("   收据树条目: {}", receipt_trie.len());
    println!("   总条目: {}", tx_trie.len() + receipt_trie.len());
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 5: 极限压力测试
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("🔥 场景 5: 极限压力测试 (1000笔交易)");
    println!("{}", "─".repeat(70));
    println!();

    let mut stress_trie = MerklePatriciaTrie::new(InMemoryStorage::new());
    let stress_tx_count = 1000;

    println!("🚀 执行极限压力测试...");
    let start = std::time::Instant::now();

    for i in 0..stress_tx_count {
        let tx = Transaction {
            from: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
            to: "0x8ac76a51cc950d9822d68b83fE1Ad97B32Cd580d",
            value: 1_000_000_000_000_000_000 + i as u64,
            gas_price: 50_000_000_000,
            gas_limit: 21000,
            nonce: i as u64,
            data: "0x",
        };

        let key = format!("{}", i);
        let value = tx.serialize();
        stress_trie.insert(key.as_bytes(), value.as_bytes())?;
    }

    let stress_duration = start.elapsed();
    let stress_root = stress_trie.root_hash();

    println!("   ✓ 压力测试完成");
    println!("   - 交易数: {}", stress_tx_count);
    println!("   - 总耗时: {:?}", stress_duration);
    println!("   - 平均延迟: {:.2?}/tx", stress_duration / stress_tx_count);
    println!("   - 峰值吞吐: {:.0} tx/s", stress_tx_count as f64 / stress_duration.as_secs_f64());
    println!("   📍 根哈希: {}", hex::encode(stress_root));
    println!();

    // 压力测试查询性能
    println!("🔍 压力测试 - 随机查询...");
    let stress_query_count = 100;
    let start = std::time::Instant::now();

    let step_size = (stress_tx_count / stress_query_count) as usize;
    for i in (0..stress_tx_count as usize).step_by(step_size) {
        let key = format!("{}", i);
        let _ = stress_trie.get(key.as_bytes())?;
    }

    let stress_query_duration = start.elapsed();
    println!("   ✓ 查询 {} 次完成", stress_query_count);
    println!("   - 总耗时: {:?}", stress_query_duration);
    println!("   - 平均延迟: {:.2?}/query", stress_query_duration / stress_query_count);
    println!("   - QPS: {:.0} queries/s", stress_query_count as f64 / stress_query_duration.as_secs_f64());
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 总结
    // ═══════════════════════════════════════════════════════════════════════
    {let l = "=".repeat(70); println!("{}", l);}
    println!("✨ 以太坊交易树与收据树高频场景测试完成！");
    println!();
    println!("📈 性能总结:");
    println!("   • 标准区块处理: {:.0} tx/s", tx_count as f64 / insert_duration.as_secs_f64());
    println!("   • 极限性能: {:.0} tx/s", stress_tx_count as f64 / stress_duration.as_secs_f64());
    println!("   • 平均插入延迟: {:.2?}", stress_duration / stress_tx_count);
    println!("   • 平均查询延迟: {:.2?}", stress_query_duration / stress_query_count);
    println!();
    println!("💡 应用场景:");
    println!("   ✓ 区块验证器：快速验证交易和收据根哈希");
    println!("   ✓ 轻客户端：生成 Merkle 证明验证交易存在性");
    println!("   ✓ 归档节点：高效存储和检索历史交易数据");
    println!("   ✓ 状态同步：批量处理区块数据实现快速同步");
    {let l = "=".repeat(70); println!("{}", l);}

    Ok(())
}

/// 运行轻客户端验证高频场景示例
///
/// 模拟轻客户端使用 Merkle 证明验证交易和收据的存在性
pub fn run_light_client_example() -> Result<(), Box<dyn std::error::Error>> {
    use crate::usecases::ProveUseCase;

    println!();
    {let l = "=".repeat(70); println!("{}", l);}
    println!("💡 轻客户端 Merkle 证明验证 - 高频场景示例");
    {let l = "=".repeat(70); println!("{}", l);}
    println!();

    println!("📖 场景说明:");
    println!("   轻客户端无需下载完整区块链数据，只需要：");
    println!("   1️⃣  区块头（包含交易根哈希和收据根哈希）");
    println!("   2️⃣  关心的交易/收据的 Merkle 证明");
    println!("   3️⃣  验证证明的有效性");
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 1: 全节点构建交易树和收据树
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("🖥️  场景 1: 全节点构建区块数据");
    println!("{}", "─".repeat(70));
    println!();

    // 模拟交易结构
    #[derive(Debug, Clone)]
    struct Transaction {
        from: &'static str,
        to: &'static str,
        value: u64,
        nonce: u64,
    }

    impl Transaction {
        fn serialize(&self) -> String {
            format!("from:{},to:{},value:{},nonce:{}", self.from, self.to, self.value, self.nonce)
        }
    }

    // 模拟收据结构
    #[derive(Debug, Clone)]
    struct Receipt {
        status: bool,
        gas_used: u64,
        logs_count: usize,
    }

    impl Receipt {
        fn serialize(&self) -> String {
            format!("status:{},gas_used:{},logs:{}",
                if self.status { "1" } else { "0" },
                self.gas_used,
                self.logs_count)
        }
    }

    // 构建交易树
    let mut tx_trie = MerklePatriciaTrie::new(InMemoryStorage::new());
    let tx_count = 200;

    println!("📊 全节点构建交易树...");
    let start = std::time::Instant::now();

    let mut transactions = Vec::new();
    for i in 0..tx_count {
        let tx = Transaction {
            from: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
            to: "0x8ac76a51cc950d9822d68b83fE1Ad97B32Cd580d",
            value: 1_000_000_000_000_000_000 + i as u64 * 1000,
            nonce: i as u64,
        };
        transactions.push(tx.clone());

        let key = format!("{}", i);
        tx_trie.insert(key.as_bytes(), tx.serialize().as_bytes())?;
    }

    let build_duration = start.elapsed();
    let tx_root = tx_trie.root_hash();

    println!("   ✓ 交易树构建完成");
    println!("   - 交易数: {}", tx_count);
    println!("   - 构建耗时: {:?}", build_duration);
    println!("   - 平均延迟: {:.2?}/tx", build_duration / tx_count);
    println!("   📍 交易根: {}", hex::encode(tx_root));
    println!();

    // 构建收据树
    let mut receipt_trie = MerklePatriciaTrie::new(InMemoryStorage::new());

    println!("📊 全节点构建收据树...");
    let start = std::time::Instant::now();

    let mut receipts = Vec::new();
    for i in 0..tx_count {
        let receipt = Receipt {
            status: true,
            gas_used: 21000 + (i % 10) as u64 * 5000,
            logs_count: (i % 3) as usize,
        };
        receipts.push(receipt.clone());

        let key = format!("{}", i);
        receipt_trie.insert(key.as_bytes(), receipt.serialize().as_bytes())?;
    }

    let receipt_build_duration = start.elapsed();
    let receipt_root = receipt_trie.root_hash();

    println!("   ✓ 收据树构建完成");
    println!("   - 收据数: {}", tx_count);
    println!("   - 构建耗时: {:?}", receipt_build_duration);
    println!("   📍 收据根: {}", hex::encode(receipt_root));
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 2: 生成 Merkle 证明
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("🔐 场景 2: 全节点生成 Merkle 证明");
    println!("{}", "─".repeat(70));
    println!();

    // 轻客户端请求的交易索引
    let requested_indices = vec![0, 50, 99, 150, 199];

    println!("📋 轻客户端请求证明的交易: {:?}", requested_indices);
    println!();

    println!("⚡ 生成交易 Merkle 证明...");
    let start = std::time::Instant::now();

    let mut tx_proofs = Vec::new();
    for &idx in &requested_indices {
        let key = format!("{}", idx);
        let proof = tx_trie.prove(key.as_bytes())?;
        tx_proofs.push((idx, proof));
    }

    let proof_gen_duration = start.elapsed();

    println!("   ✓ 生成 {} 个交易证明完成", tx_proofs.len());
    println!("   - 总耗时: {:?}", proof_gen_duration);
    println!("   - 平均每个: {:.2?}", proof_gen_duration / tx_proofs.len() as u32);
    println!();

    // 统计证明大小
    let mut total_proof_size = 0;
    for (idx, proof) in &tx_proofs {
        let size = proof.proof_size();
        total_proof_size += size;
        println!("   📦 交易 #{}: 证明大小 {} bytes, 深度 {}",
            idx, size, proof.depth());
    }
    println!("   📊 平均证明大小: {} bytes", total_proof_size / tx_proofs.len());
    println!();

    // 生成收据证明
    println!("⚡ 生成收据 Merkle 证明...");
    let start = std::time::Instant::now();

    let mut receipt_proofs = Vec::new();
    for &idx in &requested_indices {
        let key = format!("{}", idx);
        let proof = receipt_trie.prove(key.as_bytes())?;
        receipt_proofs.push((idx, proof));
    }

    let receipt_proof_gen_duration = start.elapsed();

    println!("   ✓ 生成 {} 个收据证明完成", receipt_proofs.len());
    println!("   - 总耗时: {:?}", receipt_proof_gen_duration);
    println!("   - 平均每个: {:.2?}", receipt_proof_gen_duration / receipt_proofs.len() as u32);
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 3: 轻客户端验证证明
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("✅ 场景 3: 轻客户端验证 Merkle 证明");
    println!("{}", "─".repeat(70));
    println!();

    println!("📱 轻客户端持有的数据:");
    println!("   - 区块头中的交易根: {}", hex::encode(tx_root));
    println!("   - 区块头中的收据根: {}", hex::encode(receipt_root));
    println!("   - {} 个交易的 Merkle 证明", tx_proofs.len());
    println!("   - {} 个收据的 Merkle 证明", receipt_proofs.len());
    println!();

    // 验证交易证明
    println!("🔍 验证交易 Merkle 证明...");
    let start = std::time::Instant::now();

    let mut verified_count = 0;
    for (idx, proof) in &tx_proofs {
        // 验证根哈希匹配
        if proof.root_hash != tx_root {
            println!("   ✗ 交易 #{}: 根哈希不匹配", idx);
            continue;
        }

        // 验证证明有效性
        if proof.verify()? {
            verified_count += 1;
            let value = proof.value.as_ref().unwrap();
            println!("   ✓ 交易 #{}: 验证成功 (值: {} bytes)", idx, value.len());
        } else {
            println!("   ✗ 交易 #{}: 证明无效", idx);
        }
    }

    let verify_duration = start.elapsed();

    println!();
    println!("   ✓ 验证完成: {}/{} 成功", verified_count, tx_proofs.len());
    println!("   - 总耗时: {:?}", verify_duration);
    println!("   - 平均每个: {:.2?}", verify_duration / tx_proofs.len() as u32);
    println!("   - 验证速率: {:.0} proofs/s",
        tx_proofs.len() as f64 / verify_duration.as_secs_f64());
    println!();

    // 验证收据证明
    println!("🔍 验证收据 Merkle 证明...");
    let start = std::time::Instant::now();

    let mut receipt_verified_count = 0;
    for (idx, proof) in &receipt_proofs {
        if proof.root_hash != receipt_root {
            println!("   ✗ 收据 #{}: 根哈希不匹配", idx);
            continue;
        }

        if proof.verify()? {
            receipt_verified_count += 1;
        }
    }

    let receipt_verify_duration = start.elapsed();

    println!("   ✓ 验证完成: {}/{} 成功", receipt_verified_count, receipt_proofs.len());
    println!("   - 总耗时: {:?}", receipt_verify_duration);
    println!("   - 平均每个: {:.2?}", receipt_verify_duration / receipt_proofs.len() as u32);
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 4: 批量验证性能测试
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("⚡ 场景 4: 批量验证性能测试");
    println!("{}", "─".repeat(70));
    println!();

    let batch_size = 100;
    println!("📊 批量生成和验证 {} 个证明...", batch_size);
    println!();

    // 批量生成证明
    let start = std::time::Instant::now();
    let mut batch_proofs = Vec::new();

    for i in 0..batch_size {
        let key = format!("{}", i);
        let proof = tx_trie.prove(key.as_bytes())?;
        batch_proofs.push(proof);
    }

    let batch_gen_duration = start.elapsed();

    println!("   ✓ 批量生成证明完成");
    println!("   - 总耗时: {:?}", batch_gen_duration);
    println!("   - 平均每个: {:.2?}", batch_gen_duration / batch_size);
    println!("   - 生成速率: {:.0} proofs/s",
        batch_size as f64 / batch_gen_duration.as_secs_f64());
    println!();

    // 批量验证证明
    let start = std::time::Instant::now();
    let mut batch_verified = 0;

    for proof in &batch_proofs {
        if proof.root_hash == tx_root && proof.verify()? {
            batch_verified += 1;
        }
    }

    let batch_verify_duration = start.elapsed();

    println!("   ✓ 批量验证证明完成");
    println!("   - 验证成功: {}/{}", batch_verified, batch_size);
    println!("   - 总耗时: {:?}", batch_verify_duration);
    println!("   - 平均每个: {:.2?}", batch_verify_duration / batch_size);
    println!("   - 验证速率: {:.0} proofs/s",
        batch_size as f64 / batch_verify_duration.as_secs_f64());
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 场景 5: 数据传输效率对比
    // ═══════════════════════════════════════════════════════════════════════
    println!("{}", "=".repeat(70));
    println!("📊 场景 5: 数据传输效率对比");
    println!("{}", "─".repeat(70));
    println!();

    // 计算完整数据大小
    let mut full_data_size = 0usize;
    for tx in &transactions {
        full_data_size += tx.serialize().as_bytes().len();
    }

    // 计算证明数据大小
    let mut proof_data_size = 0usize;
    for proof in &batch_proofs {
        proof_data_size += proof.proof_size();
    }

    println!("📦 数据大小对比 (验证 {} 笔交易):", batch_size);
    println!();
    println!("   方案 1: 下载完整区块数据");
    println!("   - 所有交易数据: {} KB", full_data_size / 1024);
    println!("   - 包含不需要的数据: {} 笔交易", tx_count - batch_size);
    println!();
    println!("   方案 2: 只下载 Merkle 证明");
    println!("   - 区块头: ~0.5 KB (仅包含根哈希)");
    println!("   - Merkle 证明: {:.2} KB", proof_data_size as f64 / 1024.0);
    println!("   - 总计: {:.2} KB", (500 + proof_data_size) as f64 / 1024.0);
    println!();
    println!("   📈 数据节省: {:.1}%",
        (1.0 - (proof_data_size as f64 / full_data_size as f64)) * 100.0);
    println!("   📉 传输减少: {:.1}x",
        full_data_size as f64 / proof_data_size as f64);
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // 总结
    // ═══════════════════════════════════════════════════════════════════════
    {let l = "=".repeat(70); println!("{}", l);}
    println!("✨ 轻客户端 Merkle 证明验证示例完成！");
    println!();
    println!("📈 性能总结:");
    println!("   • 证明生成速率: {:.0} proofs/s",
        batch_size as f64 / batch_gen_duration.as_secs_f64());
    println!("   • 证明验证速率: {:.0} proofs/s",
        batch_size as f64 / batch_verify_duration.as_secs_f64());
    println!("   • 平均证明大小: {} bytes", total_proof_size / tx_proofs.len());
    println!("   • 平均生成延迟: {:.2?}", batch_gen_duration / batch_size);
    println!("   • 平均验证延迟: {:.2?}", batch_verify_duration / batch_size);
    println!();
    println!("💡 轻客户端优势:");
    println!("   ✓ 无需下载完整区块链 (节省 99%+ 存储空间)");
    println!("   ✓ 快速验证交易存在性 (微秒级延迟)");
    println!("   ✓ 降低网络带宽消耗 (减少 {:.1}x 数据传输)",
        full_data_size as f64 / proof_data_size as f64);
    println!("   ✓ 保持安全性 (密码学证明保证)");
    println!();
    println!("🎯 典型应用场景:");
    println!("   • 移动钱包: 快速验证交易状态");
    println!("   • IoT 设备: 资源受限环境下的区块链访问");
    println!("   • 跨链桥: 验证源链交易的存在性");
    println!("   • 支付终端: 实时验证支付交易");
    println!("   • 审计工具: 抽查验证特定交易");
    {let l = "=".repeat(70); println!("{}", l);}

    Ok(())
}
