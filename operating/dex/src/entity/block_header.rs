use alloy_primitives::{Address, BlockNumber, Bloom, Log, B256, U256};

use crate::cmd_handler::ExchangeCommandEnvelope;

pub struct Block {
    pub header: BlockHeader,
    pub body: BlockBody,
}

pub struct BlockBody {
    pub trans: Vec<ExchangeCommandEnvelope>,
}

pub struct BlockHeader {
    pub parent_hash: B256,
    pub transactions_root: B256,
    pub receipts_root: B256,
    pub state_root: B256,
    pub beneficiary: Address,
    pub number: BlockNumber,
    pub timestamp: u64,
}
impl BlockHeader {}


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

// 收据结构
pub struct Receipt {
    // pub tx_type: TxType,           // 交易类型
    pub success: bool,             // 执行是否成功
    pub cumulative_gas_used: u64,  // 累计Gas使用
    pub logs: Vec<Log>,            // 事件日志
    pub bloom: Bloom,              // 日志Bloom过滤器
}

// 存储方式
// 不在BlockBody中，而是在独立的Receipts表中
// 键：tx_number（交易编号） → 值：Receipt