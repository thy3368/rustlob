use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};

// ============================================================================
// REST API Commands - User Data Stream Management
// ============================================================================

/// Option User Data Stream 命令枚举
#[derive(Debug, Clone)]
pub enum OptionUserDataStreamCmdAny {
    /// 启动用户数据流 POST /eapi/v1/listenKey
    /// Weight: 1
    StartListenKey(StartListenKeyCmd),

    /// 保持用户数据流活跃 PUT /eapi/v1/listenKey
    /// Weight: 1
    KeepaliveListenKey(KeepaliveListenKeyCmd),

    /// 关闭用户数据流 DELETE /eapi/v1/listenKey
    /// Weight: 1
    CloseListenKey(CloseListenKeyCmd),
}

/// 启动用户数据流命令
/// POST /eapi/v1/listenKey
/// Weight: 1
///
/// 启动新的用户数据流。流将在 60 分钟后关闭，除非发送 keepalive。
/// 如果账户已有活跃的 listenKey，将返回该 listenKey 并延长其有效期 60 分钟。
#[derive(Debug, Clone)]
pub struct StartListenKeyCmd {
    pub metadata: CMetadata,
}

/// 保持用户数据流活跃命令
/// PUT /eapi/v1/listenKey
/// Weight: 1
///
/// 保持用户数据流活跃以防止超时。用户数据流将在 60 分钟后关闭。
/// 建议每 60 分钟发送一次 ping。
#[derive(Debug, Clone)]
pub struct KeepaliveListenKeyCmd {
    pub metadata: CMetadata,
}

/// 关闭用户数据流命令
/// DELETE /eapi/v1/listenKey
/// Weight: 1
///
/// 关闭用户数据流。
#[derive(Debug, Clone)]
pub struct CloseListenKeyCmd {
    pub metadata: CMetadata,
}

// ============================================================================
// REST API Responses
// ============================================================================

/// Option User Data Stream 响应枚举
#[derive(Debug, Clone)]
pub enum OptionUserDataStreamRes {
    /// 启动 listenKey 响应
    StartListenKey(StartListenKeyRes),

    /// Keepalive 响应（空响应）
    KeepaliveListenKey,

    /// 关闭 listenKey 响应（空响应）
    CloseListenKey,
}

/// 启动 listenKey 响应
#[derive(Debug, Clone)]
pub struct StartListenKeyRes {
    /// listenKey，用于 WebSocket 连接
    pub listen_key: String,
}

// ============================================================================
// WebSocket Events - User Data Stream
// ============================================================================

/// Option User Data Stream WebSocket 事件枚举
#[derive(Debug, Clone)]
pub enum OptionUserDataStreamEvent {
    /// 订单更新事件
    OrderTradeUpdate(OrderTradeUpdateEvent),

    /// 账户数据更新事件
    AccountUpdate(AccountUpdateEvent),

    /// 风险等级变化事件
    RiskLevelChange(RiskLevelChangeEvent),
}

// ============================================================================
// Event: ORDER_TRADE_UPDATE
// ============================================================================

/// 订单更新事件
/// Event Name: ORDER_TRADE_UPDATE
/// Update Speed: 50ms
///
/// 在以下情况下触发：
/// - 订单成交
/// - 订单下单
/// - 订单取消
#[derive(Debug, Clone)]
pub struct OrderTradeUpdateEvent {
    /// 事件类型
    pub e: String, // "ORDER_TRADE_UPDATE"

    /// 事件时间
    pub event_time: i64,

    /// 订单列表
    pub orders: Vec<OrderUpdate>,
}

/// 订单更新详情
#[derive(Debug, Clone)]
pub struct OrderUpdate {
    /// 订单创建时间
    pub create_time: i64,

    /// 订单更新时间
    pub update_time: i64,

    /// 合约符号
    pub symbol: String,

    /// 客户端订单 ID
    pub client_order_id: String,

    /// 订单 ID
    pub order_id: String,

    /// 订单价格
    pub price: String,

    /// 订单数量（正数为 BUY，负数为 SELL）
    pub quantity: String,

    /// 暂未使用
    pub stp: i32,

    /// 只减仓
    pub reduce_only: bool,

    /// 只做 maker
    pub post_only: bool,

    /// 订单状态
    pub status: String,

    /// 已完成交易量（合约数）
    pub executed_qty: String,

    /// 已完成交易金额（计价资产）
    pub executed_amount: String,

    /// 手续费
    pub fee: String,

    /// 有效期类型
    pub time_in_force: String,

    /// 订单类型
    pub order_type: String,

    /// 成交明细列表
    pub fills: Vec<FillInfo>,
}

/// 成交明细
#[derive(Debug, Clone)]
pub struct FillInfo {
    /// 交易 ID
    pub trade_id: String,

    /// 成交价格
    pub price: String,

    /// 成交数量
    pub quantity: String,

    /// 成交时间
    pub trade_time: i64,

    /// taker 或 maker
    pub maker_or_taker: String,

    /// 手续费（>0）或返佣（<0）
    pub fee: String,
}

// ============================================================================
// Event: ACCOUNT_UPDATE
// ============================================================================

/// 账户数据更新事件
/// Event Name: ACCOUNT_UPDATE
/// Update Speed: 50ms
///
/// 在以下情况下触发：
/// - 账户充值或提现
/// - 持仓信息变化（包含 P 属性，否则不包含）
/// - 希腊值更新
#[derive(Debug, Clone)]
pub struct AccountUpdateEvent {
    /// 事件类型
    pub e: String, // "ACCOUNT_UPDATE"

    /// 事件时间
    pub event_time: i64,

    /// 余额信息列表
    pub balances: Vec<BalanceInfo>,

    /// 希腊值列表
    pub greeks: Vec<GreekInfo>,

    /// 持仓信息列表（可选）
    pub positions: Option<Vec<PositionInfo>>,

    /// 用户 ID
    pub uid: i64,
}

/// 余额信息
#[derive(Debug, Clone)]
pub struct BalanceInfo {
    /// 账户余额
    pub balance: String,

    /// 持仓价值
    pub position_value: String,

    /// 未实现盈亏
    pub unrealized_pnl: String,

    /// 多头未实现盈利
    pub unrealized_pnl_long: String,

    /// 维持保证金
    pub maintenance_margin: String,

    /// 初始保证金
    pub initial_margin: String,

    /// 保证金资产
    pub asset: String,
}

/// 希腊值信息
#[derive(Debug, Clone)]
pub struct GreekInfo {
    /// 标的资产
    pub underlying: String,

    /// Delta
    pub delta: String,

    /// Theta
    pub theta: String,

    /// Gamma
    pub gamma: String,

    /// Vega
    pub vega: String,
}

/// 持仓信息
#[derive(Debug, Clone)]
pub struct PositionInfo {
    /// 合约符号
    pub symbol: String,

    /// 当前持仓数量
    pub quantity: String,

    /// 可减仓数量
    pub reducible_qty: String,

    /// 持仓价值
    pub position_value: String,

    /// 平均入场价格
    pub avg_price: String,
}

// ============================================================================
// Event: RISK_LEVEL_CHANGE
// ============================================================================

/// 风险等级变化事件
/// Event Name: RISK_LEVEL_CHANGE
/// Update Speed: 50ms
///
/// 当账户风险等级发生变化时触发。可能的值：
/// - NORMAL
/// - REDUCE_ONLY
///
/// 注意：风险等级变化仅适用于 VIP 和做市商账户。
/// VIP 和某些做市商账户在保证金余额不足以满足维持保证金要求时，
/// 将自动进入 REDUCE_ONLY 模式。进入 REDUCE_ONLY 模式后，
/// 系统仅在以下事件发生时重新评估风险等级：
/// - 资金转账
/// - 交易成交
/// - 期权到期
#[derive(Debug, Clone)]
pub struct RiskLevelChangeEvent {
    /// 事件类型
    pub e: String, // "RISK_LEVEL_CHANGE"

    /// 事件时间
    pub event_time: i64,

    /// 风险等级（NORMAL 或 REDUCE_ONLY）
    pub risk_level: String,

    /// 保证金余额
    pub margin_balance: String,

    /// 维持保证金
    pub maintenance_margin: String,
}

// ============================================================================
// Behavior Trait
// ============================================================================

/// Option User Data Stream 行为接口
pub trait OptionUserDataStreamBehavior: Send + Sync {
    /// 处理 User Data Stream 命令
    fn handle(
        &mut self,
        cmd: OptionUserDataStreamCmdAny,
    ) -> Result<CmdResp<OptionUserDataStreamRes>, SpotCmdErrorAny>;

    /// 处理 WebSocket 事件（可选实现）
    fn handle_event(&mut self, event: OptionUserDataStreamEvent) {
        // 默认实现：不处理事件
        let _ = event;
    }
}

