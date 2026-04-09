BlockBody 不包含收据和状态

简短回答：在以太坊和Reth中，BlockBody 不包含收据和状态。收据和状态是独立存储的，通过Merkle根在区块头中进行承诺。

核心原则

以太坊采用分层存储设计：
1. 区块头：包含各种Merkle根，承诺了对应数据的完整性和一致性
2. 区块体：只包含原始交易数据和叔块信息
3. 收据：独立存储，通过receipts_root承诺
4. 状态：独立存储，通过state_root承诺

为什么这样设计？

1. 数据分离的合理性

┌─────────────────────────────────────┐
│             区块头                   │
│  state_root  │  receipts_root       │
└───────┬──────┴─────────┬─────────────┘
│                │
▼                ▼
┌──────────────┐  ┌──────────────┐
│    状态树     │  │    收据树     │
│ (独立存储)    │  │ (独立存储)    │
└──────────────┘  └──────────────┘


优势：
• 轻客户端支持：只下载区块头即可验证Merkle证明

• 并行处理：执行交易、生成收据、更新状态可并行

• 存储优化：不同类型数据独立压缩和存储

• 增量同步：节点可只同步缺少的部分

2. 实际存储结构

在Reth数据库中，这些数据分开存储：

表名 存储内容 示例键值对

Headers 区块头 block_number → Header

BlockBodyIndices 区块体索引 block_number → (first_tx_num, tx_count)

Transactions 交易数据 tx_number → Transaction

Receipts 交易收据 tx_number → Receipt

PlainAccountState 账户状态 address → Account

详细分析

1. 区块体（BlockBody）的真实内容

// reth-primitives/src/block.rs
pub struct BlockBody<T> {
/// 交易列表
pub transactions: Vec<T>,

    /// 叔块头列表（PoW链）
    pub ommers: Vec<Header>,
    
    /// 提款数据（上海升级后）
    pub withdrawals: Option<Vec<Withdrawal>>,
    
    // 注意：没有 receipts 字段！
    // 注意：没有 state 字段！
}


为什么？
• 交易是输入，收据是输出

• 状态是执行结果，不是输入数据

• 收据和状态可以从交易推导出来（确定性执行）

2. 收据（Receipts）的独立存储

收据包含交易执行结果信息：
// 收据结构
pub struct Receipt {
pub tx_type: TxType,           // 交易类型
pub success: bool,             // 执行是否成功
pub cumulative_gas_used: u64,  // 累计Gas使用
pub logs: Vec<Log>,            // 事件日志
pub bloom: Bloom,              // 日志Bloom过滤器
}

// 存储方式
// 不在BlockBody中，而是在独立的Receipts表中
// 键：tx_number（交易编号） → 值：Receipt


收据的用途：
• 验证交易执行结果

• 索引和查询事件日志

• 计算Gas退款

• 支持EVM的RETURN和REVERT

3. 状态（State）的独立存储

状态是全局的账户存储：
// 账户状态
pub struct Account {
pub nonce: u64,         // 交易计数
pub balance: U256,      // 余额
pub bytecode_hash: B256, // 合约代码哈希
pub storage_root: B256,  // 存储树根哈希
}

// 存储方式
// 不在BlockBody中，而是在PlainAccountState表中
// 键：address（地址） → 值：Account


执行流程演示

区块验证流程

fn validate_and_execute_block(block: Block) -> Result<()> {
// 1. 获取区块头和区块体
let header = block.header;
let body = block.body;

    // 2. 执行前状态
    let parent_state = db.get_state(header.parent_hash)?;
    
    // 3. 逐笔执行交易
    let mut receipts = Vec::new();
    let mut state = parent_state.clone();
    let mut cumulative_gas_used = 0;
    
    for (i, tx) in body.transactions.iter().enumerate() {
        // 执行交易
        let (receipt, gas_used) = execute_transaction(&mut state, tx)?;
        cumulative_gas_used += gas_used;
        receipt.cumulative_gas_used = cumulative_gas_used;
        
        receipts.push(receipt);
    }
    
    // 4. 生成收据Merkle树
    let receipts_root = merkleize_receipts(&receipts);
    
    // 5. 验证收据根
    if header.receipts_root != receipts_root {
        return Err("Receipts root mismatch");
    }
    
    // 6. 生成状态Merkle树
    let state_root = merkleize_state(&state);
    
    // 7. 验证状态根
    if header.state_root != state_root {
        return Err("State root mismatch");
    }
    
    // 8. 存储到数据库
    db.store_transactions(block.number, &body.transactions)?;
    db.store_receipts(block.number, &receipts)?;
    db.update_state(block.number, &state)?;
    
    Ok(())
}


数据关联性

通过Merkle根关联

区块头包含：
transactions_root = merkle_root(transactions)  ← 承诺交易数据
receipts_root = merkle_root(receipts)         ← 承诺收据数据
state_root = merkle_root(state)              ← 承诺状态数据


查询示例

// 查询一个区块的完整信息
fn get_full_block_info(block_number: BlockNumber) -> FullBlockInfo {
// 1. 从Headers表获取区块头
let header = db.get::<Headers>(block_number)?;

    // 2. 从BlockBodyIndices表获取交易范围
    let indices = db.get::<BlockBodyIndices>(block_number)?;
    let tx_range = indices.first_tx_num..indices.first_tx_num + indices.tx_count;
    
    // 3. 从Transactions表获取交易
    let transactions = db.get_range::<Transactions>(tx_range)?;
    
    // 4. 从Receipts表获取收据（同样的tx_range）
    let receipts = db.get_range::<Receipts>(tx_range)?;
    
    // 5. 状态需要从状态表重建
    let state = db.reconstruct_state(block_number)?;
    
    FullBlockInfo {
        header,
        transactions,
        receipts,
        state_root: header.state_root,
    }
}


网络传输优化

不同消息类型传输不同数据

消息类型 传输内容 用途

NewBlock 区块头 + 交易列表 新区块广播

NewBlockHashes 仅区块哈希列表 轻量级公告

GetBlockBodies 请求交易列表 同步时获取交易数据

GetReceipts 请求收据列表 同步或查询收据

GetNodeData 请求状态数据 状态同步

轻节点支持

轻节点只下载区块头，可以：
1. 通过Merkle证明验证特定交易
2. 通过Merkle证明验证特定账户状态
3. 不下载完整的交易、收据和状态数据

总结

BlockBody 不包含收据和状态的原因：

1. 架构分离：输入（交易）、输出（收据）、结果（状态）分离
2. 存储优化：不同类型数据使用不同存储策略
3. 验证效率：通过Merkle根快速验证，无需下载全部数据
4. 扩展性：支持轻节点、状态通道等扩展功能
5. 并行处理：执行、收据生成、状态更新可并行

关键结论：
• ✅ BlockBody 只包含交易数据、叔块、提款

• ✅ 收据独立存储，通过receipts_root承诺

• ✅ 状态独立存储，通过state_root承诺

• ✅ 这种设计是以太坊可扩展性和轻客户端支持的基础

这种设计使得以太坊节点可以根据自己的需求选择同步和存储哪些数据，从全节点（存储所有数据）到归档节点（存储所有历史状态）再到轻节点（只存储区块头），提供了灵活的部署选项。