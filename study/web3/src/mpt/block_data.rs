/// 区块数据结构
///
/// 模拟 geth 的区块数据结构
/// 参考：go-ethereum/core/types/block.go
///
/// 以太坊区块结构包含：
/// - Header: 区块头（包含各种根哈希）
/// - Transactions: 交易列表
/// - Receipts: 收据列表
/// - Uncles: 叔块列表

use sha3::{Digest, Keccak256};

/// 区块头
///
/// 参考 geth Header 结构
/// https://github.com/ethereum/go-ethereum/blob/master/core/types/block.go#L67
#[derive(Debug, Clone, PartialEq)]
pub struct BlockHeader {
    /// 父区块哈希
    pub parent_hash: [u8; 32],

    /// Ommers 哈希（叔块哈希）
    pub uncle_hash: [u8; 32],

    /// Coinbase 地址（矿工地址）
    pub coinbase: [u8; 20],

    /// 状态根哈希
    pub state_root: [u8; 32],

    /// 交易根哈希
    pub transactions_root: [u8; 32],

    /// 收据根哈希
    pub receipts_root: [u8; 32],

    /// Logs bloom filter
    pub logs_bloom: [u8; 256],

    /// 难度值
    pub difficulty: u64,

    /// 区块号
    pub number: u64,

    /// Gas 限制
    pub gas_limit: u64,

    /// Gas 使用量
    pub gas_used: u64,

    /// 时间戳
    pub timestamp: u64,

    /// 额外数据
    pub extra_data: Vec<u8>,

    /// Mix hash (PoW 相关)
    pub mix_hash: [u8; 32],

    /// Nonce (PoW 相关)
    pub nonce: u64,

    /// Base fee (EIP-1559)
    pub base_fee_per_gas: Option<u64>,
}

impl BlockHeader {
    /// 创建新的区块头
    pub fn new(number: u64, parent_hash: [u8; 32]) -> Self {
        Self {
            parent_hash,
            uncle_hash: [0u8; 32],
            coinbase: [0u8; 20],
            state_root: [0u8; 32],
            transactions_root: [0u8; 32],
            receipts_root: [0u8; 32],
            logs_bloom: [0u8; 256],
            difficulty: 0,
            number,
            gas_limit: 30_000_000,
            gas_used: 0,
            timestamp: 0,
            extra_data: Vec::new(),
            mix_hash: [0u8; 32],
            nonce: 0,
            base_fee_per_gas: Some(1_000_000_000), // 1 Gwei
        }
    }

    /// 计算区块头哈希
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();

        hasher.update(&self.parent_hash);
        hasher.update(&self.uncle_hash);
        hasher.update(&self.coinbase);
        hasher.update(&self.state_root);
        hasher.update(&self.transactions_root);
        hasher.update(&self.receipts_root);
        hasher.update(&self.logs_bloom);
        hasher.update(&self.difficulty.to_be_bytes());
        hasher.update(&self.number.to_be_bytes());
        hasher.update(&self.gas_limit.to_be_bytes());
        hasher.update(&self.gas_used.to_be_bytes());
        hasher.update(&self.timestamp.to_be_bytes());
        hasher.update(&self.extra_data);
        hasher.update(&self.mix_hash);
        hasher.update(&self.nonce.to_be_bytes());

        hasher.finalize().into()
    }

    /// 序列化为字节
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.extend_from_slice(&self.parent_hash);
        data.extend_from_slice(&self.uncle_hash);
        data.extend_from_slice(&self.coinbase);
        data.extend_from_slice(&self.state_root);
        data.extend_from_slice(&self.transactions_root);
        data.extend_from_slice(&self.receipts_root);
        data.extend_from_slice(&self.logs_bloom);
        data.extend_from_slice(&self.difficulty.to_be_bytes());
        data.extend_from_slice(&self.number.to_be_bytes());
        data.extend_from_slice(&self.gas_limit.to_be_bytes());
        data.extend_from_slice(&self.gas_used.to_be_bytes());
        data.extend_from_slice(&self.timestamp.to_be_bytes());

        // 额外数据长度 + 数据
        data.extend_from_slice(&(self.extra_data.len() as u32).to_be_bytes());
        data.extend_from_slice(&self.extra_data);

        data.extend_from_slice(&self.mix_hash);
        data.extend_from_slice(&self.nonce.to_be_bytes());

        // Base fee (可选)
        if let Some(base_fee) = self.base_fee_per_gas {
            data.push(1);
            data.extend_from_slice(&base_fee.to_be_bytes());
        } else {
            data.push(0);
        }

        data
    }
}

/// 交易
///
/// 参考 geth Transaction 结构
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    /// 交易类型 (0: Legacy, 1: EIP-2930, 2: EIP-1559)
    pub tx_type: u8,

    /// Nonce
    pub nonce: u64,

    /// Gas price
    pub gas_price: u64,

    /// Gas limit
    pub gas_limit: u64,

    /// 接收地址 (None 表示合约创建)
    pub to: Option<[u8; 20]>,

    /// 转账金额
    pub value: u128,

    /// 数据/输入
    pub data: Vec<u8>,

    /// 签名 v
    pub v: u64,

    /// 签名 r
    pub r: [u8; 32],

    /// 签名 s
    pub s: [u8; 32],

    /// Max fee per gas (EIP-1559)
    pub max_fee_per_gas: Option<u64>,

    /// Max priority fee per gas (EIP-1559)
    pub max_priority_fee_per_gas: Option<u64>,
}

impl Transaction {
    /// 创建 Legacy 交易
    pub fn legacy(
        nonce: u64,
        gas_price: u64,
        gas_limit: u64,
        to: Option<[u8; 20]>,
        value: u128,
        data: Vec<u8>,
    ) -> Self {
        Self {
            tx_type: 0,
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
            v: 0,
            r: [0u8; 32],
            s: [0u8; 32],
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        }
    }

    /// 创建 EIP-1559 交易
    pub fn eip1559(
        nonce: u64,
        max_fee_per_gas: u64,
        max_priority_fee_per_gas: u64,
        gas_limit: u64,
        to: Option<[u8; 20]>,
        value: u128,
        data: Vec<u8>,
    ) -> Self {
        Self {
            tx_type: 2,
            nonce,
            gas_price: 0,
            gas_limit,
            to,
            value,
            data,
            v: 0,
            r: [0u8; 32],
            s: [0u8; 32],
            max_fee_per_gas: Some(max_fee_per_gas),
            max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
        }
    }

    /// 计算交易哈希
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();

        hasher.update(&[self.tx_type]);
        hasher.update(&self.nonce.to_be_bytes());
        hasher.update(&self.gas_price.to_be_bytes());
        hasher.update(&self.gas_limit.to_be_bytes());

        if let Some(to) = &self.to {
            hasher.update(&[1u8]);
            hasher.update(to);
        } else {
            hasher.update(&[0u8]);
        }

        hasher.update(&self.value.to_be_bytes());
        hasher.update(&self.data);
        hasher.update(&self.v.to_be_bytes());
        hasher.update(&self.r);
        hasher.update(&self.s);

        hasher.finalize().into()
    }

    /// 序列化交易
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.push(self.tx_type);
        data.extend_from_slice(&self.nonce.to_be_bytes());
        data.extend_from_slice(&self.gas_price.to_be_bytes());
        data.extend_from_slice(&self.gas_limit.to_be_bytes());

        if let Some(to) = &self.to {
            data.push(1);
            data.extend_from_slice(to);
        } else {
            data.push(0);
        }

        data.extend_from_slice(&self.value.to_be_bytes());

        // 数据长度 + 数据
        data.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        data.extend_from_slice(&self.data);

        data.extend_from_slice(&self.v.to_be_bytes());
        data.extend_from_slice(&self.r);
        data.extend_from_slice(&self.s);

        data
    }
}

/// 交易收据
///
/// 参考 geth Receipt 结构
#[derive(Debug, Clone, PartialEq)]
pub struct Receipt {
    /// 交易类型
    pub tx_type: u8,

    /// 状态根 (Pre-Byzantium) 或 状态 (Post-Byzantium)
    pub status: u8,

    /// 累计 Gas 使用量
    pub cumulative_gas_used: u64,

    /// Logs bloom filter
    pub logs_bloom: [u8; 256],

    /// 日志列表
    pub logs: Vec<Log>,

    /// 合约地址 (如果是合约创建交易)
    pub contract_address: Option<[u8; 20]>,
}

impl Receipt {
    /// 创建新的收据
    pub fn new(tx_type: u8, status: u8, cumulative_gas_used: u64) -> Self {
        Self {
            tx_type,
            status,
            cumulative_gas_used,
            logs_bloom: [0u8; 256],
            logs: Vec::new(),
            contract_address: None,
        }
    }

    /// 序列化收据
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.push(self.tx_type);
        data.push(self.status);
        data.extend_from_slice(&self.cumulative_gas_used.to_be_bytes());
        data.extend_from_slice(&self.logs_bloom);

        // 日志数量
        data.extend_from_slice(&(self.logs.len() as u32).to_be_bytes());
        for log in &self.logs {
            let log_data = log.serialize();
            data.extend_from_slice(&(log_data.len() as u32).to_be_bytes());
            data.extend_from_slice(&log_data);
        }

        // 合约地址
        if let Some(addr) = &self.contract_address {
            data.push(1);
            data.extend_from_slice(addr);
        } else {
            data.push(0);
        }

        data
    }
}

/// 日志
#[derive(Debug, Clone, PartialEq)]
pub struct Log {
    /// 合约地址
    pub address: [u8; 20],

    /// Topics
    pub topics: Vec<[u8; 32]>,

    /// 数据
    pub data: Vec<u8>,
}

impl Log {
    /// 创建新日志
    pub fn new(address: [u8; 20], topics: Vec<[u8; 32]>, data: Vec<u8>) -> Self {
        Self {
            address,
            topics,
            data,
        }
    }

    /// 序列化日志
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.extend_from_slice(&self.address);

        // Topics 数量 + Topics
        data.extend_from_slice(&(self.topics.len() as u32).to_be_bytes());
        for topic in &self.topics {
            data.extend_from_slice(topic);
        }

        // 数据长度 + 数据
        data.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        data.extend_from_slice(&self.data);

        data
    }
}

/// 完整区块
///
/// 参考 geth Block 结构
#[derive(Debug, Clone)]
pub struct Block {
    /// 区块头
    pub header: BlockHeader,

    /// 交易列表
    pub transactions: Vec<Transaction>,

    /// 收据列表
    pub receipts: Vec<Receipt>,

    /// 叔块列表
    pub uncles: Vec<BlockHeader>,
}

impl Block {
    /// 创建新区块
    pub fn new(header: BlockHeader) -> Self {
        Self {
            header,
            transactions: Vec::new(),
            receipts: Vec::new(),
            uncles: Vec::new(),
        }
    }

    /// 添加交易
    pub fn add_transaction(&mut self, tx: Transaction, receipt: Receipt) {
        self.transactions.push(tx);
        self.receipts.push(receipt);
    }

    /// 获取区块哈希
    pub fn hash(&self) -> [u8; 32] {
        self.header.hash()
    }

    /// 获取区块号
    pub fn number(&self) -> u64 {
        self.header.number
    }

    /// 获取交易数量
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// 验证区块完整性
    pub fn validate(&self) -> bool {
        // 验证交易和收据数量一致
        if self.transactions.len() != self.receipts.len() {
            return false;
        }

        // TODO: 验证根哈希
        // - 验证 transactions_root 与实际交易树的根哈希一致
        // - 验证 receipts_root 与实际收据树的根哈希一致

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_header_hash() {
        let header = BlockHeader::new(1, [0u8; 32]);
        let hash = header.hash();
        assert_ne!(hash, [0u8; 32]);
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction::legacy(0, 1000000000, 21000, Some([1u8; 20]), 1000000000000000000, Vec::new());
        let hash = tx.hash();
        assert_ne!(hash, [0u8; 32]);
    }

    #[test]
    fn test_block_creation() {
        let mut block = Block::new(BlockHeader::new(1, [0u8; 32]));

        let tx = Transaction::legacy(0, 1000000000, 21000, Some([1u8; 20]), 1000000000000000000, Vec::new());
        let receipt = Receipt::new(0, 1, 21000);

        block.add_transaction(tx, receipt);

        assert_eq!(block.transaction_count(), 1);
        assert!(block.validate());
    }
}
