// 参考 Trading endpoints
// /Users/hongyaotang/src/rustlob/design/other/binance-spot-api-docs/rest-api.md
// 定义所有 Trading endpoints 接口;用中文注

use base_types::handler::handler::Handler;
use immutable_derive::immutable;
use crate::proc::behavior::spot_trade_behavior::{CMetadata, SpotCmdErrorAny};

/// Spot Trading 命令枚举 - 包含所有交易端点
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpotTradeCmdAny {
    /// 创建新订单 POST /api/v3/order
    /// Weight: 1
    /// Unfilled Order Count: 1
    NewOrder(NewOrderCmd),

    /// 测试下单 POST /api/v3/order/test
    /// Weight: 1 (不带 computeCommissionRates) 或 20 (带
    /// computeCommissionRates)
    TestNewOrder(TestNewOrderCmd),

    /// 取消订单 DELETE /api/v3/order
    /// Weight: 1
    CancelOrder(CancelOrderCmd),

    /// 取消某交易对的所有挂单 DELETE /api/v3/openOrders
    /// Weight: 1
    CancelAllOpenOrders(CancelAllOpenOrdersCmd),

    /// 撤销订单并创建新订单 POST /api/v3/order/cancelReplace
    /// Weight: 1
    /// Unfilled Order Count: 1
    CancelReplaceOrder(CancelReplaceOrderCmd),

    /// 查询订单 GET /api/v3/order
    /// Weight: 4
    QueryOrder(QueryOrderCmd),

    /// 当前挂单 GET /api/v3/openOrders
    /// Weight: 6 (单个交易对) 或 80 (所有交易对)
    CurrentOpenOrders(CurrentOpenOrdersCmd),

    /// 查询所有订单 GET /api/v3/allOrders
    /// Weight: 20
    AllOrders(AllOrdersCmd),

    /// 创建 OCO 订单 POST /api/v3/orderList/oco
    /// Weight: 1
    /// Unfilled Order Count: 2
    NewOcoOrder(NewOcoOrderCmd),

    /// 创建 OTO 订单 POST /api/v3/orderList/oto
    /// Weight: 1
    /// Unfilled Order Count: 2
    NewOtoOrder(NewOtoOrderCmd),

    /// 创建 OTOCO 订单 POST /api/v3/orderList/otoco
    /// Weight: 1
    /// Unfilled Order Count: 3
    NewOtocoOrder(NewOtocoOrderCmd),

    /// 取消 OCO 订单 DELETE /api/v3/orderList
    /// Weight: 1
    CancelOcoOrder(CancelOcoOrderCmd),

    /// 查询 OCO 订单 GET /api/v3/orderList
    /// Weight: 4
    QueryOcoOrder(QueryOcoOrderCmd),

    /// 查询所有 OCO 订单 GET /api/v3/allOrderList
    /// Weight: 20
    AllOcoOrders(AllOcoOrdersCmd),

    /// 查询挂起的 OCO 订单 GET /api/v3/openOrderList
    /// Weight: 6
    OpenOcoOrders(OpenOcoOrdersCmd),

    /// 账户信息查询 GET /api/v3/account
    /// Weight: 20
    Account(AccountCmd),

    /// 账户成交历史 GET /api/v3/myTrades
    /// Weight: 20
    MyTrades(MyTradesCmd),

    /// 查询目前下单数 GET /api/v3/rateLimit/order
    /// Weight: 40
    QueryUnfilledOrderCount(QueryUnfilledOrderCountCmd),

    /// 查询已阻止的匹配 GET /api/v3/myPreventedMatches
    /// Weight: 20
    QueryPreventedMatches(QueryPreventedMatchesCmd),

    /// 查询分配结果 GET /api/v3/myAllocations
    /// Weight: 20
    QueryAllocations(QueryAllocationsCmd),

    /// 查询佣金费率 GET /api/v3/account/commission
    /// Weight: 20
    QueryCommissionRates(QueryCommissionRatesCmd)
}

// ==================== 订单类型枚举 ====================

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OrderType {
    /// 限价单
    Limit,
    /// 市价单
    Market,
    /// 止损单
    StopLoss,
    /// 止损限价单
    StopLossLimit,
    /// 止盈单
    TakeProfit,
    /// 止盈限价单
    TakeProfitLimit,
    /// 限价只挂单
    LimitMaker
}

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OrderSide {
    /// 买入
    Buy,
    /// 卖出
    Sell
}

/// 有效方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum TimeInForce {
    /// 成交为止 (一直有效)
    GTC,
    /// 无法立即成交的部分就撤销
    IOC,
    /// 无法全部立即成交就撤销
    FOK
}

/// 订单响应类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum NewOrderRespType {
    /// 仅返回确认信息
    ACK,
    /// 返回订单结果
    RESULT,
    /// 返回完整订单信息（包含成交明细）
    FULL
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

/// 价格钉住类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum PegPriceType {
    /// 主要价格钉住（同侧最优价）
    PRIMARY_PEG,
    /// 市场价格钉住（对侧最优价）
    MARKET_PEG
}

/// 价格偏移类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum PegOffsetType {
    /// 价格级别
    PRICE_LEVEL
}

/// 取消限制
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum CancelRestrictions {
    /// 仅当订单状态为 NEW 时取消
    ONLY_NEW,
    /// 仅当订单状态为 PARTIALLY_FILLED 时取消
    ONLY_PARTIALLY_FILLED
}

/// 撤销替换模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum CancelReplaceMode {
    /// 取消失败则不下新单
    STOP_ON_FAILURE,
    /// 即使取消失败也尝试下新单
    ALLOW_FAILURE
}

/// 订单限流超限模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OrderRateLimitExceededMode {
    /// 不执行操作（默认）
    DO_NOTHING,
    /// 仅取消订单
    CANCEL_ONLY
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

/// OCO 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OcoOrderStatus {
    /// 执行中
    EXECUTING,
    /// 全部成交
    ALL_DONE,
    /// 订单被拒绝
    REJECTED
}

/// OCO 状态类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum OcoStatusType {
    /// 响应
    RESPONSE,
    /// 执行开始
    EXEC_STARTED,
    /// 全部成交
    ALL_DONE
}

/// 订单列表条件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum ContingencyType {
    /// OCO 订单
    OCO,
    /// OTO 订单
    OTO
}

// ==================== 订单命令定义 ====================

/// 创建新订单命令
/// POST /api/v3/order
/// Weight: 1
/// Unfilled Order Count: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct NewOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单方向
    side: OrderSide,
    /// 订单类型
    order_type: OrderType,
    /// 有效方式
    time_in_force: Option<TimeInForce>,
    /// 数量
    quantity: Option<f64>,
    /// 报价数量（市价单使用）
    quote_order_qty: Option<f64>,
    /// 价格
    price: Option<f64>,
    /// 用户自定义订单 ID
    new_client_order_id: Option<String>,
    /// 策略 ID
    strategy_id: Option<i64>,
    /// 策略类型（不能小于 1000000）
    strategy_type: Option<i32>,
    /// 止损价格
    stop_price: Option<f64>,
    /// 跟踪止损回调幅度
    trailing_delta: Option<i64>,
    /// 冰山订单数量
    iceberg_qty: Option<f64>,
    /// 订单响应类型
    new_order_resp_type: Option<NewOrderRespType>,
    /// 自成交保护模式
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// 价格钉住类型
    peg_price_type: Option<PegPriceType>,
    /// 价格偏移值（最大 100）
    peg_offset_value: Option<i32>,
    /// 价格偏移类型
    peg_offset_type: Option<PegOffsetType>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 测试下单命令
/// POST /api/v3/order/test
/// Weight: 1 (不带 computeCommissionRates) 或 20 (带 computeCommissionRates)
/// Data Source: Memory
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct TestNewOrderCmd {
    /// 继承 NewOrderCmd 的所有字段
    new_order: NewOrderCmd,
    /// 计算佣金费率（默认 false）
    compute_commission_rates: Option<bool>
}

/// 取消订单命令
/// DELETE /api/v3/order
/// Weight: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct CancelOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单 ID
    order_id: Option<i64>,
    /// 用户自定义订单 ID
    orig_client_order_id: Option<String>,
    /// 用于唯一标识此取消操作的 ID
    new_client_order_id: Option<String>,
    /// 取消限制
    cancel_restrictions: Option<CancelRestrictions>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 取消某交易对的所有挂单命令
/// DELETE /api/v3/openOrders
/// Weight: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct CancelAllOpenOrdersCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 撤销订单并创建新订单命令
/// POST /api/v3/order/cancelReplace
/// Weight: 1
/// Unfilled Order Count: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct CancelReplaceOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单方向
    side: OrderSide,
    /// 订单类型
    order_type: OrderType,
    /// 撤销替换模式
    cancel_replace_mode: CancelReplaceMode,
    /// 有效方式
    time_in_force: Option<TimeInForce>,
    /// 数量
    quantity: Option<f64>,
    /// 报价数量
    quote_order_qty: Option<f64>,
    /// 价格
    price: Option<f64>,
    /// 用于唯一标识取消操作的 ID
    cancel_new_client_order_id: Option<String>,
    /// 原始用户自定义订单 ID
    cancel_orig_client_order_id: Option<String>,
    /// 要取消的订单 ID
    cancel_order_id: Option<i64>,
    /// 新订单的用户自定义 ID
    new_client_order_id: Option<String>,
    /// 策略 ID
    strategy_id: Option<i64>,
    /// 策略类型
    strategy_type: Option<i32>,
    /// 止损价格
    stop_price: Option<f64>,
    /// 跟踪止损回调幅度
    trailing_delta: Option<i64>,
    /// 冰山订单数量
    iceberg_qty: Option<f64>,
    /// 订单响应类型
    new_order_resp_type: Option<NewOrderRespType>,
    /// 自成交保护模式
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// 取消限制
    cancel_restrictions: Option<CancelRestrictions>,
    /// 订单限流超限模式
    order_rate_limit_exceeded_mode: Option<OrderRateLimitExceededMode>,
    /// 价格钉住类型
    peg_price_type: Option<PegPriceType>,
    /// 价格偏移值
    peg_offset_value: Option<i32>,
    /// 价格偏移类型
    peg_offset_type: Option<PegOffsetType>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询订单命令
/// GET /api/v3/order
/// Weight: 4
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct QueryOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单 ID
    order_id: Option<i64>,
    /// 用户自定义订单 ID
    orig_client_order_id: Option<String>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 当前挂单查询命令
/// GET /api/v3/openOrders
/// Weight: 6 (单个交易对) 或 80 (所有交易对)
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct CurrentOpenOrdersCmd {
    metadata: CMetadata,
    /// 交易对（可选，不传则查询所有交易对）
    symbol: Option<String>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询所有订单命令
/// GET /api/v3/allOrders
/// Weight: 20
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct AllOrdersCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单 ID（返回订单 ID >= orderId 的订单）
    order_id: Option<i64>,
    /// 起始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 返回数量限制（默认 500，最大 1000）
    limit: Option<i32>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

// ==================== OCO 订单命令定义 ====================

/// 创建 OCO 订单命令
/// POST /api/v3/orderList/oco
/// Weight: 1
/// Unfilled Order Count: 2
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct NewOcoOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单列表的用户自定义 ID
    list_client_order_id: Option<String>,
    /// 订单方向
    side: OrderSide,
    /// 数量
    quantity: f64,
    /// 限价单价格
    price: f64,
    /// 止损/止盈价格
    stop_price: f64,
    /// 止损/止盈限价单价格（可选）
    stop_limit_price: Option<f64>,
    /// 止损/止盈限价单的有效方式
    stop_limit_time_in_force: Option<TimeInForce>,
    /// 限价单的用户自定义 ID
    limit_client_order_id: Option<String>,
    /// 止损/止盈单的用户自定义 ID
    stop_client_order_id: Option<String>,
    /// 限价单冰山数量
    limit_iceberg_qty: Option<f64>,
    /// 止损/止盈单冰山数量
    stop_iceberg_qty: Option<f64>,
    /// 跟踪止损回调幅度
    trailing_delta: Option<i64>,
    /// 限价单策略 ID
    limit_strategy_id: Option<i64>,
    /// 限价单策略类型
    limit_strategy_type: Option<i32>,
    /// 止损/止盈单策略 ID
    stop_strategy_id: Option<i64>,
    /// 止损/止盈单策略类型
    stop_strategy_type: Option<i32>,
    /// 订单响应类型
    new_order_resp_type: Option<NewOrderRespType>,
    /// 自成交保护模式
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 创建 OTO 订单命令（One-Triggers-the-Other）
/// POST /api/v3/orderList/oto
/// Weight: 1
/// Unfilled Order Count: 2
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct NewOtoOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单列表的用户自定义 ID
    list_client_order_id: Option<String>,
    /// 订单响应类型
    new_order_resp_type: Option<NewOrderRespType>,
    /// 自成交保护模式
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    // Working Order (触发订单)
    /// 触发订单方向
    working_side: OrderSide,
    /// 触发订单类型
    working_type: OrderType,
    /// 触发订单有效方式
    working_time_in_force: Option<TimeInForce>,
    /// 触发订单数量
    working_quantity: Option<f64>,
    /// 触发订单报价数量
    working_quote_order_qty: Option<f64>,
    /// 触发订单价格
    working_price: Option<f64>,
    /// 触发订单用户自定义 ID
    working_client_order_id: Option<String>,
    /// 触发订单冰山数量
    working_iceberg_qty: Option<f64>,
    /// 触发订单策略 ID
    working_strategy_id: Option<i64>,
    /// 触发订单策略类型
    working_strategy_type: Option<i32>,

    // Pending Order (待触发订单)
    /// 待触发订单方向
    pending_side: OrderSide,
    /// 待触发订单类型
    pending_type: OrderType,
    /// 待触发订单有效方式
    pending_time_in_force: Option<TimeInForce>,
    /// 待触发订单数量
    pending_quantity: Option<f64>,
    /// 待触发订单报价数量
    pending_quote_order_qty: Option<f64>,
    /// 待触发订单价格
    pending_price: Option<f64>,
    /// 待触发订单用户自定义 ID
    pending_client_order_id: Option<String>,
    /// 待触发订单止损价格
    pending_stop_price: Option<f64>,
    /// 待触发订单跟踪止损回调幅度
    pending_trailing_delta: Option<i64>,
    /// 待触发订单冰山数量
    pending_iceberg_qty: Option<f64>,
    /// 待触发订单策略 ID
    pending_strategy_id: Option<i64>,
    /// 待触发订单策略类型
    pending_strategy_type: Option<i32>,

    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 创建 OTOCO 订单命令（One-Triggers-OCO）
/// POST /api/v3/orderList/otoco
/// Weight: 1
/// Unfilled Order Count: 3
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct NewOtocoOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单列表的用户自定义 ID
    list_client_order_id: Option<String>,
    /// 订单响应类型
    new_order_resp_type: Option<NewOrderRespType>,
    /// 自成交保护模式
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    // Working Order (触发订单)
    /// 触发订单方向
    working_side: OrderSide,
    /// 触发订单类型
    working_type: OrderType,
    /// 触发订单有效方式
    working_time_in_force: Option<TimeInForce>,
    /// 触发订单数量
    working_quantity: Option<f64>,
    /// 触发订单报价数量
    working_quote_order_qty: Option<f64>,
    /// 触发订单价格
    working_price: Option<f64>,
    /// 触发订单用户自定义 ID
    working_client_order_id: Option<String>,
    /// 触发订单冰山数量
    working_iceberg_qty: Option<f64>,
    /// 触发订单策略 ID
    working_strategy_id: Option<i64>,
    /// 触发订单策略类型
    working_strategy_type: Option<i32>,

    // Pending Order Below (OCO 下方订单)
    /// OCO 下方订单方向
    pending_below_side: OrderSide,
    /// OCO 下方订单类型
    pending_below_type: OrderType,
    /// OCO 下方订单有效方式
    pending_below_time_in_force: Option<TimeInForce>,
    /// OCO 下方订单数量
    pending_below_quantity: Option<f64>,
    /// OCO 下方订单报价数量
    pending_below_quote_order_qty: Option<f64>,
    /// OCO 下方订单价格
    pending_below_price: Option<f64>,
    /// OCO 下方订单用户自定义 ID
    pending_below_client_order_id: Option<String>,
    /// OCO 下方订单止损价格
    pending_below_stop_price: Option<f64>,
    /// OCO 下方订单跟踪止损回调幅度
    pending_below_trailing_delta: Option<i64>,
    /// OCO 下方订单冰山数量
    pending_below_iceberg_qty: Option<f64>,
    /// OCO 下方订单策略 ID
    pending_below_strategy_id: Option<i64>,
    /// OCO 下方订单策略类型
    pending_below_strategy_type: Option<i32>,

    // Pending Order Above (OCO 上方订单)
    /// OCO 上方订单方向
    pending_above_side: OrderSide,
    /// OCO 上方订单类型
    pending_above_type: OrderType,
    /// OCO 上方订单有效方式
    pending_above_time_in_force: Option<TimeInForce>,
    /// OCO 上方订单数量
    pending_above_quantity: Option<f64>,
    /// OCO 上方订单报价数量
    pending_above_quote_order_qty: Option<f64>,
    /// OCO 上方订单价格
    pending_above_price: Option<f64>,
    /// OCO 上方订单用户自定义 ID
    pending_above_client_order_id: Option<String>,
    /// OCO 上方订单止损价格
    pending_above_stop_price: Option<f64>,
    /// OCO 上方订单跟踪止损回调幅度
    pending_above_trailing_delta: Option<i64>,
    /// OCO 上方订单冰山数量
    pending_above_iceberg_qty: Option<f64>,
    /// OCO 上方订单策略 ID
    pending_above_strategy_id: Option<i64>,
    /// OCO 上方订单策略类型
    pending_above_strategy_type: Option<i32>,

    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 取消 OCO 订单命令
/// DELETE /api/v3/orderList
/// Weight: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct CancelOcoOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单列表 ID
    order_list_id: Option<i64>,
    /// 用户自定义订单列表 ID
    list_client_order_id: Option<String>,
    /// 用于唯一标识此取消操作的 ID
    new_client_order_id: Option<String>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询 OCO 订单命令
/// GET /api/v3/orderList
/// Weight: 4
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct QueryOcoOrderCmd {
    metadata: CMetadata,
    /// 订单列表 ID
    order_list_id: Option<i64>,
    /// 用户自定义订单列表 ID
    orig_client_order_id: Option<String>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询所有 OCO 订单命令
/// GET /api/v3/allOrderList
/// Weight: 20
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct AllOcoOrdersCmd {
    metadata: CMetadata,
    /// 起始订单列表 ID（返回 ID >= fromId 的订单列表）
    from_id: Option<i64>,
    /// 起始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 返回数量限制（默认 500，最大 1000）
    limit: Option<i32>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询挂起的 OCO 订单命令
/// GET /api/v3/openOrderList
/// Weight: 6
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct OpenOcoOrdersCmd {
    metadata: CMetadata,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

// ==================== 账户查询命令定义 ====================

/// 账户信息查询命令
/// GET /api/v3/account
/// Weight: 20
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct AccountCmd {
    metadata: CMetadata,
    /// 仅返回非零余额，默认 false
    omit_zero_balances: Option<bool>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 账户成交历史查询命令
/// GET /api/v3/myTrades
/// Weight: 20
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct MyTradesCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单 ID（返回该订单的成交）
    order_id: Option<i64>,
    /// 起始交易 ID（返回交易 ID >= startTime 的成交）
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 起始交易 ID（返回交易 ID >= fromId 的成交）
    from_id: Option<i64>,
    /// 返回数量限制（默认 500，最大 1000）
    limit: Option<i32>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询目前下单数命令
/// GET /api/v3/rateLimit/order
/// Weight: 40
/// Data Source: Memory
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct QueryUnfilledOrderCountCmd {
    metadata: CMetadata,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询已阻止的匹配命令
/// GET /api/v3/myPreventedMatches
/// Weight: 20
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct QueryPreventedMatchesCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 阻止匹配 ID（与 symbol 一起使用可查询特定的阻止匹配）
    prevented_match_id: Option<i64>,
    /// 订单 ID（返回该订单的阻止匹配）
    order_id: Option<i64>,
    /// 起始阻止匹配 ID
    from_prevented_match_id: Option<i64>,
    /// 返回数量限制（默认 500，最大 1000）
    limit: Option<i32>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询分配结果命令
/// GET /api/v3/myAllocations
/// Weight: 20
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct QueryAllocationsCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 起始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 起始分配 ID
    from_allocation_id: Option<i64>,
    /// 返回数量限制（默认 500，最大 1000）
    limit: Option<i32>,
    /// 订单 ID
    order_id: Option<i64>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

/// 查询佣金费率命令
/// GET /api/v3/account/commission
/// Weight: 20
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct QueryCommissionRatesCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<f64>,
    /// 时间戳
    timestamp: i64
}

// ==================== 响应类型定义 ====================

/// Spot Trading 响应枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum SpotTradeResAny {
    /// 新订单响应（ACK 模式）
    NewOrderAck(NewOrderAck),
    /// 新订单响应（RESULT 模式）
    NewOrderResult(NewOrderResult),
    /// 新订单响应（FULL 模式）
    NewOrderFull(NewOrderFull),

    /// 测试下单响应（不带佣金计算）
    TestNewOrderEmpty,
    /// 测试下单响应（带佣金计算）
    TestNewOrderCommission(CommissionRates),

    /// 取消订单响应
    CancelOrder(OrderInfo),
    /// 取消所有挂单响应
    CancelAllOpenOrders(Vec<CancelOrderResult>),
    /// 撤销替换订单响应
    CancelReplaceOrder(CancelReplaceResult),

    /// 查询订单响应
    QueryOrder(OrderInfo),
    /// 当前挂单响应
    CurrentOpenOrders(Vec<OrderInfo>),
    /// 所有订单响应
    AllOrders(Vec<OrderInfo>),

    /// OCO 订单响应
    NewOcoOrder(OcoOrderResult),
    NewOtoOrder(OtoOrderResult),
    NewOtocoOrder(OtocoOrderResult),
    CancelOcoOrder(OcoOrderInfo),
    QueryOcoOrder(OcoOrderInfo),
    AllOcoOrders(Vec<OcoOrderInfo>),
    OpenOcoOrders(Vec<OcoOrderInfo>),

    /// 账户信息响应
    Account(AccountInfo),
    /// 成交历史响应
    MyTrades(Vec<TradeInfo>),
    /// 下单数查询响应
    UnfilledOrderCount(Vec<UnfilledOrderCount>),
    /// 阻止匹配查询响应
    PreventedMatches(Vec<PreventedMatch>),
    /// 分配结果查询响应
    Allocations(Vec<Allocation>),
    /// 佣金费率查询响应
    CommissionRates(CommissionRates)
}

// ==================== 订单响应结构 ====================

/// 新订单 ACK 响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct NewOrderAck {
    /// 交易对
    symbol: String,
    /// 订单 ID
    order_id: i64,
    /// 订单列表 ID（-1 表示不属于订单列表）
    order_list_id: i64,
    /// 用户自定义订单 ID
    client_order_id: String,
    /// 交易时间戳
    transact_time: i64
}

/// 新订单 RESULT 响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct NewOrderResult {
    /// 交易对
    symbol: String,
    /// 订单 ID
    order_id: i64,
    /// 订单列表 ID
    order_list_id: i64,
    /// 用户自定义订单 ID
    client_order_id: String,
    /// 交易时间戳
    transact_time: i64,
    /// 价格
    price: String,
    /// 原始数量
    orig_qty: String,
    /// 已成交数量
    executed_qty: String,
    /// 原始报价数量
    orig_quote_order_qty: String,
    /// 累计成交金额
    cummulative_quote_qty: String,
    /// 订单状态
    status: OrderStatus,
    /// 有效方式
    time_in_force: TimeInForce,
    /// 订单类型
    order_type: OrderType,
    /// 订单方向
    side: OrderSide,
    /// 订单开始时间
    working_time: i64,
    /// 自成交保护模式
    self_trade_prevention_mode: SelfTradePreventionMode,

    // 条件字段
    /// 冰山订单数量
    iceberg_qty: Option<String>,
    /// 阻止匹配 ID
    prevented_match_id: Option<i64>,
    /// 阻止数量
    prevented_quantity: Option<String>,
    /// 止损价格
    stop_price: Option<String>,
    /// 策略 ID
    strategy_id: Option<i64>,
    /// 策略类型
    strategy_type: Option<i32>,
    /// 跟踪止损回调幅度
    trailing_delta: Option<i64>,
    /// 跟踪止损激活时间
    trailing_time: Option<i64>,
    /// 是否使用 SOR
    used_sor: Option<bool>,
    /// SOR 工作层
    working_floor: Option<String>,
    /// 价格钉住类型
    peg_price_type: Option<PegPriceType>,
    /// 价格偏移类型
    peg_offset_type: Option<PegOffsetType>,
    /// 价格偏移值
    peg_offset_value: Option<i32>,
    /// 当前钉住价格
    pegged_price: Option<String>
}

/// 新订单 FULL 响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct NewOrderFull {
    /// 基础订单信息
    base: NewOrderResult,
    /// 成交明细
    fills: Vec<Fill>
}

/// 成交明细
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct Fill {
    /// 成交价格
    price: String,
    /// 成交数量
    qty: String,
    /// 佣金
    commission: String,
    /// 佣金资产
    commission_asset: String,
    /// 成交 ID
    trade_id: i64
}

/// 订单信息（通用）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct OrderInfo {
    /// 交易对
    symbol: String,
    /// 订单 ID
    order_id: i64,
    /// 订单列表 ID
    order_list_id: i64,
    /// 用户自定义订单 ID
    client_order_id: String,
    /// 价格
    price: String,
    /// 原始数量
    orig_qty: String,
    /// 已成交数量
    executed_qty: String,
    /// 累计成交金额
    cummulative_quote_qty: String,
    /// 订单状态
    status: OrderStatus,
    /// 有效方式
    time_in_force: TimeInForce,
    /// 订单类型
    order_type: OrderType,
    /// 订单方向
    side: OrderSide,
    /// 止损价格（可选）
    stop_price: Option<String>,
    /// 冰山订单数量（可选）
    iceberg_qty: Option<String>,
    /// 订单创建时间
    time: i64,
    /// 订单更新时间
    update_time: i64,
    /// 是否只挂单
    is_working: bool,
    /// 原始报价数量
    orig_quote_order_qty: String,
    /// 订单开始时间
    working_time: i64,
    /// 自成交保护模式
    self_trade_prevention_mode: SelfTradePreventionMode
}

/// 取消订单结果（可能是单个订单或 OCO 订单列表）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum CancelOrderResult {
    /// 单个订单
    Order(OrderInfo),
    /// OCO 订单列表
    OcoOrder(OcoOrderInfo)
}

/// 撤销替换订单结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct CancelReplaceResult {
    /// 取消结果（SUCCESS 或 FAILURE）
    cancel_result: String,
    /// 新订单结果（SUCCESS、FAILURE 或 NOT_ATTEMPTED）
    new_order_result: String,
    /// 取消响应
    cancel_response: Option<OrderInfo>,
    /// 新订单响应（根据 newOrderRespType 返回不同类型）
    new_order_response: Option<NewOrderResponse>
}

/// 新订单响应（用于撤销替换）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum NewOrderResponse {
    Ack(NewOrderAck),
    Result(NewOrderResult),
    Full(NewOrderFull)
}

// ==================== OCO 订单响应结构 ====================

/// OCO 订单结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct OcoOrderResult {
    /// 订单列表 ID
    order_list_id: i64,
    /// 条件类型
    contingency_type: ContingencyType,
    /// 状态类型
    list_status_type: OcoStatusType,
    /// 订单状态
    list_order_status: OcoOrderStatus,
    /// 用户自定义订单列表 ID
    list_client_order_id: String,
    /// 交易时间戳
    transaction_time: i64,
    /// 交易对
    symbol: String,
    /// 订单列表
    orders: Vec<OcoOrderEntry>,
    /// 订单报告
    order_reports: Vec<OrderInfo>
}

/// OTO 订单结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct OtoOrderResult {
    /// 订单列表 ID
    order_list_id: i64,
    /// 条件类型
    contingency_type: ContingencyType,
    /// 状态类型
    list_status_type: OcoStatusType,
    /// 订单状态
    list_order_status: OcoOrderStatus,
    /// 用户自定义订单列表 ID
    list_client_order_id: String,
    /// 交易时间戳
    transaction_time: i64,
    /// 交易对
    symbol: String,
    /// 订单列表
    orders: Vec<OcoOrderEntry>,
    /// 订单报告
    order_reports: Vec<OrderInfo>
}

/// OTOCO 订单结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct OtocoOrderResult {
    /// 订单列表 ID
    order_list_id: i64,
    /// 条件类型
    contingency_type: ContingencyType,
    /// 状态类型
    list_status_type: OcoStatusType,
    /// 订单状态
    list_order_status: OcoOrderStatus,
    /// 用户自定义订单列表 ID
    list_client_order_id: String,
    /// 交易时间戳
    transaction_time: i64,
    /// 交易对
    symbol: String,
    /// 订单列表
    orders: Vec<OcoOrderEntry>,
    /// 订单报告
    order_reports: Vec<OrderInfo>
}

/// OCO 订单条目
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct OcoOrderEntry {
    /// 交易对
    symbol: String,
    /// 订单 ID
    order_id: i64,
    /// 用户自定义订单 ID
    client_order_id: String
}

/// OCO 订单信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct OcoOrderInfo {
    /// 订单列表 ID
    order_list_id: i64,
    /// 条件类型
    contingency_type: ContingencyType,
    /// 状态类型
    list_status_type: OcoStatusType,
    /// 订单状态
    list_order_status: OcoOrderStatus,
    /// 用户自定义订单列表 ID
    list_client_order_id: String,
    /// 交易时间戳
    transaction_time: i64,
    /// 交易对
    symbol: String,
    /// 订单列表
    orders: Vec<OrderInfo>
}

// ==================== 账户信息响应结构 ====================

/// 账户信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct AccountInfo {
    /// Maker 佣金率（bps）
    maker_commission: i32,
    /// Taker 佣金率（bps）
    taker_commission: i32,
    /// 买方佣金率（bps）
    buyer_commission: i32,
    /// 卖方佣金率（bps）
    seller_commission: i32,
    /// 佣金费率
    commission_rates: CommissionRates,
    /// 是否可以交易
    can_trade: bool,
    /// 是否可以提现
    can_withdraw: bool,
    /// 是否可以充值
    can_deposit: bool,
    /// 订单限流计数器重置时间
    brokered: bool,
    /// 是否需要更新余额
    require_self_trade_prevention: bool,
    /// 阻止 SOR 交易
    prevent_sor: bool,
    /// 更新时间
    update_time: i64,
    /// 账户类型
    account_type: String,
    /// 余额列表
    balances: Vec<Balance>,
    /// 权限列表
    permissions: Vec<String>,
    /// UID
    uid: i64
}

/// 余额信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct Balance {
    /// 资产名称
    asset: String,
    /// 可用余额
    free: String,
    /// 冻结余额
    locked: String
}

/// 佣金费率
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct CommissionRates {
    /// Maker 标准佣金率
    maker: String,
    /// Taker 标准佣金率
    taker: String,
    /// Buyer 佣金率（保留字段）
    buyer: Option<String>,
    /// Seller 佣金率（保留字段）
    seller: Option<String>
}

/// 成交信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct TradeInfo {
    /// 交易对
    symbol: String,
    /// 成交 ID
    id: i64,
    /// 订单 ID
    order_id: i64,
    /// 订单列表 ID
    order_list_id: i64,
    /// 价格
    price: String,
    /// 数量
    qty: String,
    /// 成交金额
    quote_qty: String,
    /// 佣金
    commission: String,
    /// 佣金资产
    commission_asset: String,
    /// 成交时间
    time: i64,
    /// 是否是买方
    is_buyer: bool,
    /// 是否是挂单方
    is_maker: bool,
    /// 是否是最优成交
    is_best_match: bool
}

/// 未成交订单计数
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct UnfilledOrderCount {
    /// 速率限制类型
    rate_limit_type: String,
    /// 时间间隔
    interval: String,
    /// 时间间隔数量
    interval_num: i32,
    /// 限制值
    limit: i32,
    /// 当前计数
    count: i32
}

/// 阻止匹配信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct PreventedMatch {
    /// 交易对
    symbol: String,
    /// 阻止匹配 ID
    prevented_match_id: i64,
    /// Taker 订单 ID
    taker_order_id: i64,
    /// Maker 订单 ID
    maker_order_id: i64,
    /// 成交 ID
    trade_group_id: i64,
    /// 自成交保护模式
    self_trade_prevention_mode: SelfTradePreventionMode,
    /// 价格
    price: String,
    /// Maker 阻止数量
    maker_prevented_quantity: String,
    /// 交易时间戳
    transact_time: i64
}

/// 分配信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]

pub struct Allocation {
    /// 分配 ID
    id: i64,
    /// 交易对
    symbol: String,
    /// 分配类型
    allocation_type: String,
    /// 订单 ID
    order_id: i64,
    /// 订单列表 ID
    order_list_id: i64,
    /// 价格
    price: String,
    /// 数量
    qty: String,
    /// 成交金额
    quote_qty: String,
    /// 佣金
    commission: String,
    /// 佣金资产
    commission_asset: String,
    /// 时间戳
    time: i64,
    /// 是否是买方
    is_buyer: bool,
    /// 是否是挂单方
    is_maker: bool,
    /// 是否是分配方
    is_allocator: bool
}

// ==================== 行为接口定义 ====================

/// Spot Trading 行为接口
pub trait SpotTradeBehaviorV2: Handler<SpotTradeCmdAny, SpotTradeResAny, SpotCmdErrorAny> {}
