// 参考 /Users/hongyaotang/src/rustlob/design/other/binance-spot-api-docs/
// user-data-stream.md 定义所有 user data 接口

use base_types::handler::handler::Handler;

use crate::proc::behavior::spot_trade_behavior::{CMetadata, SpotCmdErrorAny};
// ==================== User Data Stream 事件枚举 ====================

/// User Data Stream 事件类型（WebSocket 推送）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum UserDataStreamEventAny {
    /// 账户位置更新事件
    /// 事件类型: outboundAccountPosition
    /// 当账户余额发生变化时推送，包含可能被事件改变的资产
    OutboundAccountPosition(OutboundAccountPositionEvent),

    /// 余额更新事件
    /// 事件类型: balanceUpdate
    /// 发生在以下情况：
    /// - 账户的充值或提现
    /// - 账户间资金划转（例如现货到杠杆）
    BalanceUpdate(BalanceUpdateEvent),

    /// 订单更新事件（执行报告）
    /// 事件类型: executionReport
    /// 订单通过此事件进行更新
    ExecutionReport(ExecutionReportEvent),

    /// 订单列表状态事件
    /// 事件类型: listStatus
    /// 如果订单是订单列表（OCO/OTO/OTOCO），除了 executionReport
    /// 外还会发送此事件
    ListStatus(ListStatusEvent),

    /// 事件流终止事件
    /// 事件类型: eventStreamTerminated
    /// 当以下情况发生时推送：
    /// - listenKey 订阅因 token 过期而到期
    /// - logon 订阅在发送 session.logout 后结束
    /// - 通过 userDataStream.unsubscribe 方法停止订阅
    EventStreamTerminated(EventStreamTerminatedEvent),

    /// 外部锁定更新事件
    /// 事件类型: externalLockUpdate
    /// 当现货钱包余额的一部分被外部系统锁定/解锁时推送（例如用作保证金抵押品）
    ExternalLockUpdate(ExternalLockUpdateEvent)
}

// ==================== 账户位置更新事件 ====================

/// 账户位置更新事件
/// outboundAccountPosition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct OutboundAccountPositionEvent {
    /// 订阅 ID
    pub subscription_id: i32,
    /// 事件类型 "outboundAccountPosition"
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 最后一次账户更新时间
    pub last_account_update_time: i64,
    /// 余额数组
    pub balances: Vec<BalanceItem>
}

/// 余额项
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct BalanceItem {
    /// 资产名称
    pub asset: String,
    /// 可用余额
    pub free: String,
    /// 锁定余额
    pub locked: String
}

// ==================== 余额更新事件 ====================

/// 余额更新事件
/// balanceUpdate
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct BalanceUpdateEvent {
    /// 订阅 ID
    pub subscription_id: i32,
    /// 事件类型 "balanceUpdate"
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 资产名称
    pub asset: String,
    /// 余额变化量
    pub balance_delta: String,
    /// 清算时间
    pub clear_time: i64
}

// ==================== 订单更新事件（执行报告）====================

/// 执行报告事件（订单更新）
/// executionReport
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct ExecutionReportEvent {
    /// 订阅 ID
    pub subscription_id: i32,
    /// 事件类型 "executionReport"
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 客户端订单 ID
    pub client_order_id: String,
    /// 订单方向（BUY/SELL）
    pub side: OrderSide,
    /// 订单类型（LIMIT/MARKET等）
    pub order_type: OrderType,
    /// 有效方式（GTC/IOC/FOK）
    pub time_in_force: TimeInForce,
    /// 订单数量
    pub order_quantity: String,
    /// 订单价格
    pub order_price: String,
    /// 止损价格
    pub stop_price: String,
    /// 冰山订单数量
    pub iceberg_quantity: String,
    /// 订单列表 ID（-1 表示不属于订单列表）
    pub order_list_id: i64,
    /// 原始客户端订单 ID（被取消订单的 ID）
    pub original_client_order_id: String,
    /// 当前执行类型
    pub current_execution_type: ExecutionType,
    /// 当前订单状态
    pub current_order_status: OrderStatus,
    /// 订单拒绝原因
    pub order_reject_reason: OrderRejectReason,
    /// 订单 ID
    pub order_id: i64,
    /// 最后执行的数量
    pub last_executed_quantity: String,
    /// 累计成交数量
    pub cumulative_filled_quantity: String,
    /// 最后执行的价格
    pub last_executed_price: String,
    /// 佣金金额
    pub commission_amount: String,
    /// 佣金资产（可能为 null）
    pub commission_asset: Option<String>,
    /// 交易时间
    pub transaction_time: i64,
    /// 成交 ID
    pub trade_id: i64,
    /// 执行 ID
    pub execution_id: i64,
    /// 订单是否在订单簿上
    pub is_on_book: bool,
    /// 是否是挂单方
    pub is_maker: bool,
    /// 忽略（废弃字段）
    pub ignore: bool,
    /// 订单创建时间
    pub order_creation_time: i64,
    /// 累计成交金额
    pub cumulative_quote_transacted_quantity: String,
    /// 最后成交金额（lastPrice * lastQty）
    pub last_quote_transacted_quantity: String,
    /// 报价订单数量
    pub quote_order_quantity: String,
    /// 自成交保护模式
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    // ==================== 条件字段 ====================
    /// 跟踪止损回调幅度（仅用于跟踪止损订单）
    pub trailing_delta: Option<i64>,
    /// 跟踪止损激活时间（仅用于跟踪止损订单）
    pub trailing_time: Option<i64>,

    /// 策略 ID（仅当下单时提供了 strategyId 参数）
    pub strategy_id: Option<i64>,
    /// 策略类型（仅当下单时提供了 strategyType 参数）
    pub strategy_type: Option<i32>,

    /// 阻止匹配 ID（仅当订单因 STP 过期时可见）
    pub prevented_match_id: Option<i64>,
    /// 阻止数量（仅当订单因 STP 过期时）
    pub prevented_quantity: Option<String>,
    /// 最后阻止数量（仅当订单因 STP 过期时）
    pub last_prevented_quantity: Option<String>,
    /// 交易组 ID（仅当订单因 STP 过期时）
    pub trade_group_id: Option<i64>,
    /// 对手方订单 ID（仅当订单因 STP 过期时）
    pub counter_order_id: Option<i64>,
    /// 对手方交易对（仅当订单因 STP 过期时）
    pub counter_symbol: Option<String>,
    /// 阻止执行数量（仅当订单因 STP 过期时）
    pub prevented_execution_quantity: Option<String>,
    /// 阻止执行价格（仅当订单因 STP 过期时）
    pub prevented_execution_price: Option<String>,
    /// 阻止执行报价数量（仅当订单因 STP 过期时）
    pub prevented_execution_quote_quantity: Option<String>,

    /// 工作时间（当订单在订单簿上工作时出现）
    pub working_time: Option<i64>,

    /// 匹配类型（用于有分配的订单）
    pub match_type: Option<String>,
    /// 分配 ID（用于有分配的订单）
    pub allocation_id: Option<i64>,

    /// 工作层（用于可能有分配的订单）
    pub working_floor: Option<String>,

    /// 是否使用了 SOR（智能订单路由）
    pub used_sor: Option<bool>,

    /// 价格钉住类型（仅用于钉住订单）
    pub pegged_price_type: Option<String>,
    /// 价格偏移类型（仅用于钉住订单）
    pub pegged_offset_type: Option<String>,
    /// 价格偏移值（仅用于钉住订单）
    pub pegged_offset_value: Option<i32>,
    /// 当前钉住价格（仅用于钉住订单）
    pub pegged_price: Option<String>
}

// ==================== 订单列表状态事件 ====================

/// 订单列表状态事件
/// listStatus
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct ListStatusEvent {
    /// 订阅 ID
    pub subscription_id: i32,
    /// 事件类型 "listStatus"
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 订单列表 ID
    pub order_list_id: i64,
    /// 条件类型（OCO/OTO）
    pub contingency_type: String,
    /// 列表状态类型
    pub list_status_type: String,
    /// 列表订单状态
    pub list_order_status: String,
    /// 列表拒绝原因
    pub list_reject_reason: String,
    /// 列表客户端订单 ID
    pub list_client_order_id: String,
    /// 交易时间
    pub transaction_time: i64,
    /// 订单数组
    pub orders: Vec<ListOrderItem>
}

/// 订单列表中的订单项
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct ListOrderItem {
    /// 交易对
    pub symbol: String,
    /// 订单 ID
    pub order_id: i64,
    /// 客户端订单 ID
    pub client_order_id: String
}

// ==================== 事件流终止事件 ====================

/// 事件流终止事件
/// eventStreamTerminated
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct EventStreamTerminatedEvent {
    /// 订阅 ID
    pub subscription_id: i32,
    /// 事件类型 "eventStreamTerminated"
    pub event_type: String,
    /// 事件时间
    pub event_time: i64
}

// ==================== 外部锁定更新事件 ====================

/// 外部锁定更新事件
/// externalLockUpdate
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct ExternalLockUpdateEvent {
    /// 订阅 ID
    pub subscription_id: i32,
    /// 事件类型 "externalLockUpdate"
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 资产名称
    pub asset: String,
    /// 变化量
    pub delta: String,
    /// 交易时间
    pub transaction_time: i64
}

// ==================== 枚举类型定义 ====================

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OrderSide {
    /// 买入
    BUY,
    /// 卖出
    SELL
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OrderType {
    /// 限价单
    LIMIT,
    /// 市价单
    MARKET,
    /// 止损单
    STOP_LOSS,
    /// 止损限价单
    STOP_LOSS_LIMIT,
    /// 止盈单
    TAKE_PROFIT,
    /// 止盈限价单
    TAKE_PROFIT_LIMIT,
    /// 限价只挂单
    LIMIT_MAKER
}

/// 有效方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum TimeInForce {
    /// 成交为止（一直有效）
    GTC,
    /// 无法立即成交的部分就撤销
    IOC,
    /// 无法全部立即成交就撤销
    FOK
}

/// 执行类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum ExecutionType {
    /// 订单已被引擎接受
    NEW,
    /// 订单已被用户取消
    CANCELED,
    /// 订单已被修改（撤销替换）
    REPLACED,
    /// 订单被拒绝且未处理（例如撤销替换订单中新订单被拒绝但取消请求成功）
    REJECTED,
    /// 订单的部分或全部数量已成交
    TRADE,
    /// 订单根据订单类型规则或交易所规则被取消（例如无成交的 LIMIT FOK
    /// 订单，部分成交的 LIMIT IOC 或 MARKET 订单）
    EXPIRED,
    /// 订单因 STP（自成交保护）而过期
    TRADE_PREVENTION
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OrderStatus {
    /// 新建订单
    NEW,
    /// 部分成交
    PARTIALLY_FILLED,
    /// 全部成交
    FILLED,
    /// 已取消
    CANCELED,
    /// 待取消（当前未使用）
    PENDING_CANCEL,
    /// 订单被拒绝
    REJECTED,
    /// 订单过期
    EXPIRED,
    /// 订单过期（在匹配引擎中）
    EXPIRED_IN_MATCH
}

/// 订单拒绝原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OrderRejectReason {
    /// 无拒绝（订单未被拒绝）
    NONE,
    /// 账户余额不足
    /// 错误信息: "Account has insufficient balance for requested action."
    INSUFFICIENT_BALANCES,
    /// 止损价格会立即触发
    /// 错误信息: "Order would trigger immediately."
    STOP_PRICE_WOULD_TRIGGER_IMMEDIATELY,
    /// 会立即匹配
    /// 错误信息: "Order would immediately match and take."
    WOULD_MATCH_IMMEDIATELY,
    /// OCO 订单价格关系不正确
    /// 错误信息: "The relationship of the prices for the orders is not
    /// correct."
    OCO_BAD_PRICES
}

/// 自成交保护模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum SelfTradePreventionMode {
    /// 无保护
    NONE,
    /// 过期接受方
    EXPIRE_TAKER,
    /// 过期挂单方
    EXPIRE_MAKER,
    /// 过期双方
    EXPIRE_BOTH
}

// ==================== User Data Stream 命令枚举 ====================

/// User Data Stream 命令枚举（用于 REST API 管理订阅）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum SpotUserDataListenKeyCmdAny {
    /// 创建 Listen Key（开启 User Data Stream）
    /// POST /api/v3/userDataStream
    /// Weight: 2
    CreateListenKey(CreateListenKeyCmd),

    /// 延长 Listen Key 有效期（保活）
    /// PUT /api/v3/userDataStream
    /// Weight: 2
    KeepAliveListenKey(KeepAliveListenKeyCmd),

    /// 关闭 User Data Stream
    /// DELETE /api/v3/userDataStream
    /// Weight: 2
    CloseListenKey(CloseListenKeyCmd)
}

/// 创建 Listen Key 命令
/// POST /api/v3/userDataStream
/// Weight: 2
/// Data Source: Memory
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct CreateListenKeyCmd {
    pub metadata: CMetadata
}

/// 延长 Listen Key 有效期命令
/// PUT /api/v3/userDataStream
/// Weight: 2
/// Data Source: Memory
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct KeepAliveListenKeyCmd {
    pub metadata: CMetadata,
    /// Listen Key
    pub listen_key: String
}

/// 关闭 Listen Key 命令
/// DELETE /api/v3/userDataStream
/// Weight: 2
/// Data Source: Memory
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct CloseListenKeyCmd {
    pub metadata: CMetadata,
    /// Listen Key
    pub listen_key: String
}

// ==================== User Data Stream 响应枚举 ====================

/// User Data Stream 响应枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum SpotUserDataListenKeyResAny {
    /// 创建 Listen Key 响应
    CreateListenKey(ListenKeyResponse),
    /// 延长 Listen Key 响应（空响应）
    KeepAliveListenKey,
    /// 关闭 Listen Key 响应（空响应）
    CloseListenKey
}

/// Listen Key 响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct ListenKeyResponse {
    /// Listen Key（用于 WebSocket 订阅）
    /// 有效期：60 分钟
    pub listen_key: String
}

// ==================== 辅助类型和常量 ====================

/// User Data Stream 配置
pub struct UserDataStreamConfig {
    /// Listen Key 有效期（毫秒）
    /// 默认：60 分钟
    pub listen_key_ttl_ms: i64,
    /// 建议保活间隔（毫秒）
    /// 建议每 30 分钟延长一次有效期
    pub keep_alive_interval_ms: i64
}

impl Default for UserDataStreamConfig {
    fn default() -> Self {
        Self {
            listen_key_ttl_ms: 60 * 60 * 1000,      // 60 分钟
            keep_alive_interval_ms: 30 * 60 * 1000  // 30 分钟
        }
    }
}

/// User Data Stream 事件类型常量
pub mod event_types {
    /// 账户位置更新
    pub const OUTBOUND_ACCOUNT_POSITION: &str = "outboundAccountPosition";
    /// 余额更新
    pub const BALANCE_UPDATE: &str = "balanceUpdate";
    /// 执行报告（订单更新）
    pub const EXECUTION_REPORT: &str = "executionReport";
    /// 订单列表状态
    pub const LIST_STATUS: &str = "listStatus";
    /// 事件流终止
    pub const EVENT_STREAM_TERMINATED: &str = "eventStreamTerminated";
    /// 外部锁定更新
    pub const EXTERNAL_LOCK_UPDATE: &str = "externalLockUpdate";
}

// ==================== 使用说明 ====================

/// User Data Stream 使用说明
///
/// ## REST API 端点（管理订阅）
///
/// 1. **创建 Listen Key**
///    - POST /api/v3/userDataStream
///    - Weight: 2
///    - 返回一个 60 分钟有效的 listenKey
///
/// 2. **延长 Listen Key 有效期**
///    - PUT /api/v3/userDataStream
///    - Weight: 2
///    - 建议每 30 分钟调用一次
///
/// 3. **关闭 User Data Stream**
///    - DELETE /api/v3/userDataStream
///    - Weight: 2
///    - 关闭后 listenKey 失效
///
/// ## WebSocket 订阅
///
/// 使用返回的 listenKey 订阅 WebSocket：
///
/// ```text
/// wss://stream.binance.com:9443/ws/<listenKey>
/// ```
///
/// ## 事件类型
///
/// 1. **outboundAccountPosition** - 账户余额变化时推送
/// 2. **balanceUpdate** - 充值/提现/划转时推送
/// 3. **executionReport** - 订单状态变化时推送
/// 4. **listStatus** - OCO/OTO/OTOCO 订单状态变化时推送
/// 5. **eventStreamTerminated** - 订阅终止时推送
/// 6. **externalLockUpdate** - 外部系统锁定/解锁余额时推送
///
/// ## 注意事项
///
/// - 所有时间戳均为毫秒
/// - 账户事件为实时推送
/// - 支持 JSON 和 SBE 两种输出格式
/// - 如果资产或交易对名称包含非 ASCII 字符，事件可能包含 UTF-8 编码的非 ASCII
///   字符
/// - 平均价格可通过 `累计成交金额 / 累计成交数量` 计算
#[allow(dead_code)]
const USAGE_GUIDE: () = ();

// ==================== User Data Stream 行为接口 ====================

/// User Data Stream 行为接口
pub trait SpotUserDataListenKeyBehavior:
    Send + Sync + Handler<SpotUserDataListenKeyCmdAny, SpotUserDataListenKeyResAny, SpotCmdErrorAny>
{
}
