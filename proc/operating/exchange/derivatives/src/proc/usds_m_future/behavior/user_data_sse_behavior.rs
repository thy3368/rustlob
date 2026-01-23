// 参考 ## user data stream Endpoints /Users/hongyaotang/src/rustlob/design/other/binance_derivatives_api/usds-margined-futures/user-data-streams 定义所有 user data stream 接口

use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};

// ============================================================================
// 枚举类型定义
// ============================================================================

/// 执行类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionType {
    /// 新订单
    New,
    /// 已取消
    Canceled,
    /// 已计算（强平执行）
    Calculated,
    /// 已过期
    Expired,
    /// 成交
    Trade,
    /// 修改（订单修改）
    Amendment,
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Expired,
    ExpiredInMatch,
}

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
    Liquidation,
}

/// 有效时间类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTX,
    GTD,
}

/// 工作类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

/// 持仓方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

/// 保证金类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginType {
    Isolated,
    Crossed,
}

/// 账户更新原因类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountUpdateReasonType {
    /// 充值
    Deposit,
    /// 提现
    Withdraw,
    /// 订单
    Order,
    /// 资金费用
    FundingFee,
    /// 提现拒绝
    WithdrawReject,
    /// 调整
    Adjustment,
    /// 保险清算
    InsuranceClear,
    /// 管理员充值
    AdminDeposit,
    /// 管理员提现
    AdminWithdraw,
    /// 保证金转账
    MarginTransfer,
    /// 保证金类型变更
    MarginTypeChange,
    /// 资产转账
    AssetTransfer,
    /// 期权权利金费用
    OptionsPremiumFee,
    /// 期权结算收益
    OptionsSettleProfit,
    /// 自动兑换
    AutoExchange,
    /// 币币兑换充值
    CoinSwapDeposit,
    /// 币币兑换提现
    CoinSwapWithdraw,
}

/// 自成交防护模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfTradePreventionMode {
    None,
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
}

/// 价格匹配模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceMatchMode {
    None,
    Opponent,
    Opponent5,
    Opponent10,
    Opponent20,
    Queue,
    Queue5,
    Queue10,
    Queue20,
}

// ============================================================================
// User Data Stream 命令枚举
// ============================================================================

/// USDS-M期货用户数据流命令枚举
#[derive(Debug, Clone)]
pub enum UsdsMFutureUserDataStreamCmdAny {
    // ========== ListenKey管理 ==========
    /// 创建ListenKey POST /fapi/v1/listenKey
    /// Weight: 1
    StartUserDataStream(StartUserDataStreamCmd),

    /// 延长ListenKey有效期 PUT /fapi/v1/listenKey
    /// Weight: 1
    KeepaliveUserDataStream(KeepaliveUserDataStreamCmd),

    /// 关闭用户数据流 DELETE /fapi/v1/listenKey
    /// Weight: 1
    CloseUserDataStream(CloseUserDataStreamCmd),

    // ========== 用户数据事件（接收） ==========
    /// 订单更新事件
    OrderTradeUpdateEvent(OrderTradeUpdateEvent),

    /// 账户余额和持仓更新事件
    AccountUpdateEvent(AccountUpdateEvent),

    /// 账户配置更新事件
    AccountConfigUpdateEvent(AccountConfigUpdateEvent),

    /// 保证金追缴事件
    MarginCallEvent(MarginCallEvent),

    /// 条件订单触发拒绝事件
    ConditionalOrderTriggerRejectEvent(ConditionalOrderTriggerRejectEvent),

    /// 成交Lite事件
    TradeLiteEvent(TradeLiteEvent),

    /// 用户数据流过期事件
    ListenKeyExpiredEvent(ListenKeyExpiredEvent),

    /// 策略更新事件
    StrategyUpdateEvent(StrategyUpdateEvent),

    /// 网格更新事件
    GridUpdateEvent(GridUpdateEvent),
}

// ============================================================================
// ListenKey管理命令
// ============================================================================

/// 创建ListenKey命令
/// POST /fapi/v1/listenKey
/// Weight: 1
#[derive(Debug, Clone)]
pub struct StartUserDataStreamCmd {
    pub metadata: CMetadata,
}

/// 延长ListenKey有效期命令
/// PUT /fapi/v1/listenKey
/// Weight: 1
#[derive(Debug, Clone)]
pub struct KeepaliveUserDataStreamCmd {
    pub metadata: CMetadata,
    /// ListenKey
    pub listen_key: String,
}

/// 关闭用户数据流命令
/// DELETE /fapi/v1/listenKey
/// Weight: 1
#[derive(Debug, Clone)]
pub struct CloseUserDataStreamCmd {
    pub metadata: CMetadata,
    /// ListenKey
    pub listen_key: String,
}

// ============================================================================
// 用户数据事件定义（接收的推送数据）
// ============================================================================

/// 订单更新事件
/// Event: ORDER_TRADE_UPDATE
/// 当新订单创建、订单状态变化时推送此事件
#[derive(Debug, Clone)]
pub struct OrderTradeUpdateEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易时间
    pub transaction_time: i64,
    /// 订单详情
    pub order: OrderUpdate,
}

/// 订单更新详情
#[derive(Debug, Clone)]
pub struct OrderUpdate {
    /// 交易对
    pub symbol: String,
    /// 客户端订单ID
    /// 特殊值：
    /// - "autoclose-*": 强平订单
    /// - "adl_autoclose": ADL自动平仓订单
    /// - "settlement_autoclose-*": 交割或下架结算订单
    pub client_order_id: String,
    /// 订单方向
    pub side: OrderSide,
    /// 订单类型
    pub order_type: OrderType,
    /// 有效时间类型
    pub time_in_force: TimeInForce,
    /// 原始数量
    pub original_quantity: String,
    /// 原始价格
    pub original_price: String,
    /// 平均价格
    pub average_price: String,
    /// 止损价格（TRAILING_STOP_MARKET订单忽略）
    pub stop_price: String,
    /// 执行类型
    pub execution_type: ExecutionType,
    /// 订单状态
    pub order_status: OrderStatus,
    /// 订单ID
    pub order_id: i64,
    /// 最后成交数量
    pub last_filled_quantity: String,
    /// 累计成交数量
    pub accumulated_filled_quantity: String,
    /// 最后成交价格
    pub last_filled_price: String,
    /// 手续费资产（无手续费时不推送）
    pub commission_asset: Option<String>,
    /// 手续费（无手续费时不推送）
    pub commission: Option<String>,
    /// 成交时间
    pub trade_time: i64,
    /// 成交ID
    pub trade_id: i64,
    /// 买单名义价值
    pub bids_notional: String,
    /// 卖单名义价值
    pub asks_notional: String,
    /// 是否为做市方
    pub is_maker_side: bool,
    /// 是否只减仓
    pub is_reduce_only: bool,
    /// 止损价格工作类型
    pub stop_price_working_type: WorkingType,
    /// 原始订单类型
    pub original_order_type: OrderType,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 是否全平仓（条件订单推送）
    pub is_close_all: bool,
    /// 激活价格（仅TRAILING_STOP_MARKET订单）
    pub activation_price: Option<String>,
    /// 回调比率（仅TRAILING_STOP_MARKET订单）
    pub callback_rate: Option<String>,
    /// 是否开启价格保护
    pub price_protect: bool,
    /// 忽略字段1
    pub ignore_si: i64,
    /// 忽略字段2
    pub ignore_ss: i64,
    /// 本次成交实现盈亏
    pub realized_profit: String,
    /// 自成交防护模式
    pub stp_mode: SelfTradePreventionMode,
    /// 价格匹配模式
    pub price_match: PriceMatchMode,
    /// GTD订单自动取消时间
    pub gtd_auto_cancel_time: i64,
}

/// 账户更新事件
/// Event: ACCOUNT_UPDATE
/// 当余额或持仓更新时推送此事件
#[derive(Debug, Clone)]
pub struct AccountUpdateEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易时间
    pub transaction_time: i64,
    /// 更新数据
    pub account_update: AccountUpdateData,
}

/// 账户更新数据
#[derive(Debug, Clone)]
pub struct AccountUpdateData {
    /// 事件原因类型
    pub reason_type: AccountUpdateReasonType,
    /// 余额更新列表
    pub balances: Vec<BalanceUpdate>,
    /// 持仓更新列表（仅包含变化的持仓）
    pub positions: Vec<PositionUpdate>,
}

/// 余额更新
#[derive(Debug, Clone)]
pub struct BalanceUpdate {
    /// 资产
    pub asset: String,
    /// 钱包余额
    pub wallet_balance: String,
    /// 全仓钱包余额
    pub cross_wallet_balance: String,
    /// 余额变化（不包括盈亏和手续费）
    pub balance_change: String,
}

/// 持仓更新
#[derive(Debug, Clone)]
pub struct PositionUpdate {
    /// 交易对
    pub symbol: String,
    /// 持仓数量
    pub position_amount: String,
    /// 入场价格
    pub entry_price: String,
    /// 盈亏平衡价格
    pub breakeven_price: String,
    /// 累计实现盈亏（扣除手续费前）
    pub accumulated_realized: String,
    /// 未实现盈亏
    pub unrealized_pnl: String,
    /// 保证金类型
    pub margin_type: MarginType,
    /// 逐仓钱包余额（逐仓持仓）
    pub isolated_wallet: String,
    /// 持仓方向
    pub position_side: PositionSide,
}

/// 账户配置更新事件
/// Event: ACCOUNT_CONFIG_UPDATE
/// 当账户配置变更时推送此事件
#[derive(Debug, Clone)]
pub struct AccountConfigUpdateEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易时间
    pub transaction_time: i64,
    /// 账户配置（杠杆变更）
    pub account_config: Option<AccountConfig>,
    /// 账户信息（多资产模式变更）
    pub account_info: Option<AccountInfo>,
}

/// 账户配置（杠杆）
#[derive(Debug, Clone)]
pub struct AccountConfig {
    /// 交易对
    pub symbol: String,
    /// 杠杆倍数
    pub leverage: i32,
}

/// 账户信息（多资产模式）
#[derive(Debug, Clone)]
pub struct AccountInfo {
    /// 多资产模式
    pub multi_assets_mode: bool,
}

/// 保证金追缴事件
/// Event: MARGIN_CALL
/// 当用户持仓风险率过高时推送此事件
/// 注意：此消息仅作为风险指导信息，不建议作为投资策略使用
#[derive(Debug, Clone)]
pub struct MarginCallEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 全仓钱包余额（仅全仓保证金追缴推送）
    pub cross_wallet_balance: Option<String>,
    /// 保证金追缴持仓列表
    pub positions: Vec<MarginCallPosition>,
}

/// 保证金追缴持仓
#[derive(Debug, Clone)]
pub struct MarginCallPosition {
    /// 交易对
    pub symbol: String,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 持仓数量
    pub position_amount: String,
    /// 保证金类型
    pub margin_type: MarginType,
    /// 逐仓钱包余额（逐仓持仓）
    pub isolated_wallet: String,
    /// 标记价格
    pub mark_price: String,
    /// 未实现盈亏
    pub unrealized_pnl: String,
    /// 维持保证金要求
    pub maintenance_margin_required: String,
}

/// 条件订单触发拒绝事件
/// Event: conditional_order_trigger_reject
#[derive(Debug, Clone)]
pub struct ConditionalOrderTriggerRejectEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 订单ID
    pub order_id: i64,
    /// 拒绝原因
    pub reject_reason: String,
}

/// 成交Lite事件
/// Event: ACCOUNT_TRADE_UPDATE
#[derive(Debug, Clone)]
pub struct TradeLiteEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易时间
    pub transaction_time: i64,
    /// 交易对
    pub symbol: String,
    /// 订单ID
    pub order_id: i64,
    /// 成交ID
    pub trade_id: i64,
    /// 订单方向
    pub side: OrderSide,
    /// 成交价格
    pub price: String,
    /// 成交数量
    pub quantity: String,
    /// 是否为做市方
    pub is_maker: bool,
    /// 手续费
    pub commission: String,
    /// 手续费资产
    pub commission_asset: String,
}

/// 用户数据流过期事件
/// Event: listenKeyExpired
#[derive(Debug, Clone)]
pub struct ListenKeyExpiredEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 过期的ListenKey
    pub listen_key: String,
}

/// 策略更新事件
/// Event: STRATEGY_UPDATE
#[derive(Debug, Clone)]
pub struct StrategyUpdateEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易时间
    pub transaction_time: i64,
    /// 策略ID
    pub strategy_id: i64,
    /// 策略类型
    pub strategy_type: String,
    /// 策略状态
    pub strategy_status: String,
    /// 交易对
    pub symbol: String,
}

/// 网格更新事件
/// Event: GRID_UPDATE
#[derive(Debug, Clone)]
pub struct GridUpdateEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易时间
    pub transaction_time: i64,
    /// 网格ID
    pub grid_id: i64,
    /// 网格状态
    pub grid_status: String,
    /// 交易对
    pub symbol: String,
}

// ============================================================================
// 响应类型定义
// ============================================================================

/// User Data Stream 响应枚举
#[derive(Debug, Clone)]
pub enum UsdsMFutureUserDataStreamRes {
    /// 启动用户数据流响应
    StartUserDataStream(StartUserDataStreamResponse),
    /// Keepalive响应
    KeepaliveUserDataStream(KeepaliveUserDataStreamResponse),
    /// 关闭用户数据流响应
    CloseUserDataStream(CloseUserDataStreamResponse),
    /// 事件已处理
    EventProcessed,
}

/// 启动用户数据流响应
#[derive(Debug, Clone)]
pub struct StartUserDataStreamResponse {
    /// ListenKey（60分钟有效期）
    pub listen_key: String,
}

/// Keepalive响应（空响应，返回成功即可）
#[derive(Debug, Clone)]
pub struct KeepaliveUserDataStreamResponse {
    pub success: bool,
}

/// 关闭用户数据流响应（空响应，返回成功即可）
#[derive(Debug, Clone)]
pub struct CloseUserDataStreamResponse {
    pub success: bool,
}

// ============================================================================
// 错误类型定义
// ============================================================================

/// User Data Stream 命令错误
#[derive(Debug, Clone)]
pub enum UsdsMFutureUserDataStreamCmdError {
    /// 无效的ListenKey
    InvalidListenKey(String),
    /// ListenKey已过期
    ListenKeyExpired(String),
    /// WebSocket连接错误
    ConnectionError(String),
    /// 序列化/反序列化错误
    SerializationError(String),
    /// API错误
    ApiError { code: i32, msg: String },
    /// 未知错误
    Unknown(String),
}

// ============================================================================
// User Data Stream 行为接口
// ============================================================================

/// USDS-M期货用户数据流行为接口
pub trait UsdsMFutureUserDataSSEBehavior: Send + Sync {
    /// 处理用户数据流命令
    fn handle(
        &mut self,
        cmd: UsdsMFutureUserDataStreamCmdAny,
    ) -> Result<CmdResp<UsdsMFutureUserDataStreamRes>, UsdsMFutureUserDataStreamCmdError>;
}

// ============================================================================
// 辅助工具
// ============================================================================

/// ListenKey管理器接口
pub trait ListenKeyManager: Send + Sync {
    /// 创建或刷新ListenKey
    fn create_or_refresh(&mut self) -> Result<String, UsdsMFutureUserDataStreamCmdError>;

    /// 延长ListenKey有效期（建议每30分钟调用一次）
    fn keepalive(&mut self, listen_key: &str) -> Result<(), UsdsMFutureUserDataStreamCmdError>;

    /// 关闭ListenKey
    fn close(&mut self, listen_key: &str) -> Result<(), UsdsMFutureUserDataStreamCmdError>;

    /// 检查ListenKey是否有效
    fn is_valid(&self, listen_key: &str) -> bool;
}

/// WebSocket URL构建器
pub struct UserDataStreamUrlBuilder;

impl UserDataStreamUrlBuilder {
    /// 构建用户数据流WebSocket URL
    /// wss://fstream.binance.com/ws/<listenKey>
    pub fn build(listen_key: &str) -> String {
        format!("wss://fstream.binance.com/ws/{}", listen_key)
    }

    /// 构建测试网用户数据流WebSocket URL
    /// wss://stream.binancefuture.com/ws/<listenKey>
    pub fn build_testnet(listen_key: &str) -> String {
        format!("wss://stream.binancefuture.com/ws/{}", listen_key)
    }
}
