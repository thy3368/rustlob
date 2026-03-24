use serde::{Deserialize, Serialize};

/// 完整区块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    #[serde(flatten)]
    pub header: BlockHeader,
    #[serde(rename = "txs")]
    pub transactions: Vec<Transaction>,
}

/// 区块头
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub hash: String,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    pub height: u64,
    pub time: u64,
    pub proposer: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    #[serde(rename = "txsRoot")]
    pub transactions_root: String,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: String,
}

/// 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "txIdx")]
    pub tx_index: u32,
    #[serde(rename = "user")]
    pub user_address: String,
    #[serde(rename = "hash")]
    pub tx_hash: String,
    pub nonce: u64,
    #[serde(flatten)]
    pub data: TransactionData,
    #[serde(default)]
    pub status: TxStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 交易数据枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionData {
    #[serde(rename = "order")]
    Order(OrderTx),
    #[serde(rename = "cancel")]
    Cancel(CancelTx),
    #[serde(rename = "cancelByCloid")]
    CancelByCloid(CancelByCloidTx),
    #[serde(rename = "noop")]
    Noop(NoopTx),
    #[serde(rename = "batchModify")]
    BatchModify(BatchModifyTx),
    // 其他类型用通用结构
    #[serde(untagged)]
    Unknown {
        #[serde(rename = "type")]
        tx_type: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
}

/// Order 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTx {
    pub asset: u32,
    #[serde(rename = "isBuy")]
    pub is_buy: bool,
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    pub sz: String,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    pub cloid: Option<String>,
}

/// 订单类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit { tif: TimeInForce },
    #[serde(rename = "trigger")]
    Trigger {
        #[serde(rename = "triggerPx")]
        trigger_px: String,
        #[serde(rename = "isMarket")]
        is_market: bool,
        tpsl: String,
    },
}

/// 时间有效性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Alo,
}

/// Cancel 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelTx {
    pub asset: u32,
    pub oid: u64,
}

/// CancelByCloid 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelByCloidTx {
    pub asset: u32,
    pub cloid: String,
}

/// Noop 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoopTx {}

/// BatchModify 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchModifyTx {
    pub modifies: Vec<ModifyOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyOrder {
    pub oid: u64,
    pub order: OrderTx,
}

/// 交易状态
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TxStatus {
    #[default]
    Success,
    Error,
}
