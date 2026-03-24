use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    #[serde(rename = "blockDetails")]
    pub block_details: Block,
}

/// 完整区块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: u64,
    #[serde(rename = "blockTime")]
    pub block_time: u64,
    pub hash: String,
    pub proposer: String,
    #[serde(rename = "numTxs")]
    pub num_txs: u64,
    #[serde(rename = "txs")]
    pub transactions: Vec<Transaction>,
}

/// 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub time: u64,
    pub user: String,
    pub action: TransactionAction,
    pub block: u64,
    pub hash: String,
    pub error: Option<String>,
}

/// 交易动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionAction {
    #[serde(rename = "order")]
    Order(OrderAction),
    #[serde(rename = "cancel")]
    Cancel(CancelTx),
    #[serde(rename = "cancelByCloid")]
    CancelByCloid(CancelByCloidTx),
    #[serde(rename = "noop")]
    Noop,
    #[serde(rename = "batchModify")]
    BatchModify(BatchModifyTx),
    #[serde(rename = "modify")]
    Modify(ModifyAction),
    #[serde(rename = "scheduleCancel")]
    ScheduleCancel(ScheduleCancelTx),
    #[serde(rename = "updateLeverage")]
    UpdateLeverage(UpdateLeverageTx),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAction {
    pub orders: Vec<OrderTx>,
    pub grouping: Option<String>,
}

/// Order 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTx {
    #[serde(rename = "a")]
    pub asset: u32,
    #[serde(rename = "b")]
    pub is_buy: bool,
    #[serde(rename = "p")]
    pub limit_px: String,
    #[serde(rename = "s")]
    pub sz: String,
    #[serde(rename = "r")]
    pub reduce_only: bool,
    #[serde(rename = "t")]
    pub order_type: OrderType,
    #[serde(rename = "c")]
    pub cloid: Option<String>,
}

/// 订单类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderType {
    #[serde(rename = "limit")]
    pub limit: Option<LimitOrder>,
    #[serde(rename = "trigger")]
    pub trigger: Option<TriggerOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOrder {
    pub tif: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerOrder {
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    #[serde(rename = "isMarket")]
    pub is_market: bool,
    pub tpsl: String,
}

/// Cancel 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelTx {
    #[serde(rename = "cancels")]
    pub cancels: Vec<CancelItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelItem {
    #[serde(rename = "a")]
    pub asset: u32,
    #[serde(rename = "o")]
    pub oid: JsonValue,
}

/// CancelByCloid 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelByCloidTx {
    #[serde(rename = "cancels")]
    pub cancels: Vec<CancelByCloidItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelByCloidItem {
    pub asset: u32,
    pub cloid: String,
}

/// BatchModify 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchModifyTx {
    pub modifies: Vec<ModifyOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyOrder {
    #[serde(rename = "oid")]
    pub oid: JsonValue,
    pub order: OrderTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyAction {
    #[serde(rename = "oid")]
    pub oid: String,
    pub order: OrderTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleCancelTx {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLeverageTx {
    pub asset: u32,
    #[serde(rename = "isCross")]
    pub is_cross: bool,
    pub leverage: u32,
}

// ============================================================
// 用户信息类型
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearinghouseState {
    #[serde(rename = "assetPositions")]
    pub asset_positions: Vec<AssetPosition>,
    #[serde(rename = "crossMarginSummary")]
    pub cross_margin_summary: MarginSummary,
    #[serde(rename = "marginSummary")]
    pub margin_summary: MarginSummary,
    pub withdrawable: String,
    #[serde(rename = "crossMaintenanceMarginUsed")]
    pub cross_maintenance_margin_used: Option<String>,
    pub time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetPosition {
    #[serde(rename = "type")]
    pub position_type: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub coin: String,
    pub sz: String,
    pub side: String,
    #[serde(rename = "entryPx")]
    pub entry_px: String,
    pub leverage: Leverage,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leverage {
    pub value: u32,
    #[serde(rename = "isCross")]
    pub is_cross: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSummary {
    #[serde(rename = "accountValue")]
    pub account_value: String,
    #[serde(rename = "totalNtlPos")]
    pub total_ntl_pos: String,
    #[serde(rename = "totalRawUsd")]
    pub total_raw_usd: String,
    #[serde(rename = "totalMarginUsed")]
    pub total_margin_used: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotClearinghouseState {
    pub balances: Vec<SpotBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotBalance {
    pub coin: String,
    pub hold: String,
    pub total: String,
    pub entry_ntl: Option<String>,
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetails {
    pub address: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "isRegistered")]
    pub is_registered: bool,
    #[serde(rename = "email")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrdersResponse {
    #[serde(rename = "type")]
    pub order_type: String,
    pub order: OpenOrderInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrderInfo {
    pub coin: String,
    pub side: String,
    pub sz: String,
    pub limit_px: String,
    pub oid: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFills {
    pub fills: Vec<Fill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub coin: String,
    pub side: String,
    pub sz: String,
    pub px: String,
    pub oid: u64,
    pub hash: String,
    pub time: u64,
    pub fee: String,
    pub closed_pnl: Option<String>,
}
