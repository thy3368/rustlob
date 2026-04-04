// =============================================================================
// 使用场景说明
// =============================================================================
// 本模块定义期权交易（Option Trade）的命令接口，用于处理期权交易相关的业务逻辑。
//
// 使用场景：
// 1. 期权下单：投资者买入或卖出期权合约
//    - 单笔下单：NewOrder - 提交单个限价单
//    - 批量下单：BatchOrders - 最多同时提交 5 个订单
//
// 2. 期权撤单：取消未成交的期权订单
//    - 单笔撤单：CancelOrder - 取消指定订单
//    - 批量撤单：BatchCancelOrders - 批量取消订单
//    - 按标的撤单：CancelAllByUnderlying - 取消某个标的（如 BTCUSDT）下的所有期权订单
//    - 按交易对撤单：CancelAllOnSymbol - 取消指定期权交易对的所有订单
//
// 3. 期权查询：查询订单、持仓、成交等历史数据
//    - 查询订单：QueryOrder - 查询单个订单详情
//    - 查询挂单：QueryOpenOrders - 查询当前未成交的挂单
//    - 查询历史：QueryOrderHistory - 查询已结束的订单（5天内）
//    - 查询成交：QueryUserTrades - 查询账户成交记录
//    - 查询持仓：QueryPosition - 查询当前持仓
//    - 查询行权：QueryExerciseRecord - 查询行权记录
//
// 4. 交易类型：
//    - 限价单（Limit）：只有当价格达到指定价格时才成交
//    - 只挂单（Post Only）：如果会立即成交则撤单，确保只挂单
//    - 仅减仓（Reduce Only）：只减少持仓，不增加持仓
//    - 做市商保护（MMP）：防止自成交
//
// 5. 订单参数说明：
//    - symbol：期权交易对，格式如 BTC-200730-9000-C（BTC-到期日-行权价-期权类型）
//    - underlying：标的资产，如 BTCUSDT
//    - side：订单方向，Buy（买入）或 Sell（卖出）
//    - optionSide：期权类型，Call（看涨期权）或 Put（看跌期权）
//    - timeInForce：有效方式，GTC（成交为止）、IOC（立即成交或取消）、FOK（全部成交或取消）
//
// 典型工作流：
// 1. 用户下单 -> NewOrder -> 创建期权订单 -> 返回订单ID
// 2. 订单成交 -> 持仓增加 -> QueryPosition 可查询持仓
// 3. 用户平仓 -> 卖出期权 -> 持仓减少
// 4. 到期行权 -> 行权记录 -> QueryExerciseRecord 可查询
//
// 参考 Binance Options API:
// /Users/hongyaotang/src/rustlob/design/other/binance_derivatives_api/option/trade
// =============================================================================

// 参考 Binance Options API
// 定义所有 Option Trade 接口;用中文注释

use base_types::cqrs::cqrs_types::CMetadata;
use base_types::exchange::spot::spot_types::TimeInForce;
use base_types::handler::handler::Handler;
use base_types::{OrderSide, Price, Quantity, Timestamp};
use immutable_derive::immutable;

// use crate::proc::behavior::spot_trade_behavior::{CMetadata, SpotCmdErrorAny};

/// Option Trade 命令枚举 - 包含所有期权交易端点
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionTradeCmdAny {
    /// 新建订单 POST /eapi/v1/order
    /// Weight: 1
    /// Data Source: Matching Engine
    NewOrder(NewOrderCmd),

    /// 批量下单 POST /eapi/v1/batchOrders
    /// Weight: 5
    /// Data Source: Matching Engine
    BatchOrders(BatchOrdersCmd),

    /// 取消订单 DELETE /eapi/v1/order
    /// Weight: 1
    /// Data Source: Matching Engine
    CancelOrder(CancelOrderCmd),

    /// 批量取消订单 DELETE /eapi/v1/batchOrders
    /// Weight: 1
    /// Data Source: Matching Engine
    BatchCancelOrders(BatchCancelOrdersCmd),

    /// 取消所有标的物下的期权订单 DELETE /eapi/v1/allOpenOrdersByUnderlying
    /// Weight: 1
    /// Data Source: Matching Engine
    CancelAllByUnderlying(CancelAllByUnderlyingCmd),

    /// 取消指定交易对的所有订单 DELETE /eapi/v1/allOpenOrders
    /// Weight: 1
    /// Data Source: Matching Engine
    CancelAllOnSymbol(CancelAllOnSymbolCmd),

    /// 查询单个订单 GET /eapi/v1/order
    /// Weight: 1
    /// Data Source: Memory => Database
    QueryOrder(QueryOrderCmd),

    /// 查询当前挂单 GET /eapi/v1/openOrders
    /// Weight: 1 (单交易对) / 40 (所有交易对)
    /// Data Source: Memory => Database
    QueryOpenOrders(QueryOpenOrdersCmd),

    /// 查询订单历史 GET /eapi/v1/historyOrders
    /// Weight: 3
    /// Data Source: Database
    QueryOrderHistory(QueryOrderHistoryCmd),

    /// 查询账户成交历史 GET /eapi/v1/userTrades
    /// Weight: 5
    /// Data Source: Memory => Database
    QueryUserTrades(QueryUserTradesCmd),

    /// 查询持仓信息 GET /eapi/v1/position
    /// Weight: 5
    /// Data Source: Memory => Database
    QueryPosition(QueryPositionCmd),

    /// 查询行权记录 GET /eapi/v1/exerciseRecord
    /// Weight: 5
    /// Data Source: Database
    QueryExerciseRecord(QueryExerciseRecordCmd),
}

/// 期权订单响应类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionOrderRespType {
    /// 仅返回确认信息
    ACK,
    /// 返回订单结果
    RESULT,
}

/// 期权订单方向（买入/卖出）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionSide {
    /// 买入
    Buy,
    /// 卖出
    Sell,
}

/// 期权类型（看涨/看跌）
///
/// ## 使用场景
///
/// ### Call（看涨期权）
/// - 买入看涨期权：预期标的资产价格上涨
///   - 付出权利金，获得在到期日以行权价买入标的资产的权利
///   - 潜在收益无限（标的上涨时），最大损失为权利金
/// - 卖出看涨期权：预期标的资产价格不涨或下跌
///   - 收取权利金，承担在到期日以行权价卖出标的资产的义务
///   - 最大收益为权利金，潜在损失无限
///
/// ### Put（看跌期权）
/// - 买入看跌期权：预期标的资产价格下跌
///   - 付出权利金，获得在到期日以行权价卖出标的资产的权利
///   - 潜在收益有限（最大为行权价 - 权利金），最大损失为权利金
/// - 卖出看跌期权：预期标的资产价格不跌或上涨 todo
///   - 收取权利金，承担在到期日以行权价买入标的资产的义务
///   - 最大收益为权利金，潜在损失有限（最大损失 = 行权价 - 权利金）
///
/// ## 实际应用
/// - 杠杆投机：用较少的权利金获得更大的价格波动敞口
/// - 风险对冲：为现货持仓提供价格下跌保护（买入 Put）
/// - 收入增强：卖出期权收取权利金（需承担对应义务）
/// - 组合策略：如牛市价差、熊市价差、保护性看跌等
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionType {
    /// 看涨期权
    Call,
    /// 看跌期权
    Put,
}

/// 期权订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionOrderStatus {
    /// 已接受
    Accepted,
    /// 部分成交
    PartiallyFilled,
    /// 全部成交
    Filled,
    /// 已取消
    Cancelled,
    /// 已拒绝
    Rejected,
}

/// 订单类型（期权仅支持 LIMIT）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionOrderType {
    /// 限价单
    Limit,
}

// ==================== 订单命令定义 ====================

/// 新建订单命令
/// POST /eapi/v1/order
/// Weight: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct NewOrderCmd {
    metadata: CMetadata,
    /// 期权交易对，如 BTC-200730-9000-C
    symbol: String,
    /// 订单方向
    side: OptionSide,
    /// 订单类型（仅支持 LIMIT）
    order_type: OptionOrderType,
    /// 订单数量
    quantity: Quantity,
    /// 订单价格
    price: Option<Price>,
    /// 有效方式（默认 GTC）
    time_in_force: Option<TimeInForce>,
    /// 是否仅减仓（默认 false）
    reduce_only: Option<bool>,
    /// 是否只挂单（默认 false）
    post_only: Option<bool>,
    /// 订单响应类型
    new_order_resp_type: Option<OptionOrderRespType>,
    /// 用户自定义订单 ID（不能与挂单中的重复）
    client_order_id: Option<String>,
    /// 是否为做市商保护订单
    is_mmp: Option<bool>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 批量下单命令
/// POST /eapi/v1/batchOrders
/// Weight: 5
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct BatchOrdersCmd {
    metadata: CMetadata,
    /// 订单列表（最多 5 个订单）
    orders: Vec<BatchOrderItem>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 批量订单项
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct BatchOrderItem {
    /// 期权交易对
    symbol: String,
    /// 订单方向
    side: OptionSide,
    /// 订单类型
    order_type: OptionOrderType,
    /// 订单数量
    quantity: Quantity,
    /// 订单价格
    price: Option<Price>,
    /// 有效方式
    time_in_force: Option<TimeInForce>,
    /// 是否仅减仓
    reduce_only: Option<bool>,
    /// 是否只挂单
    post_only: Option<bool>,
    /// 订单响应类型
    new_order_resp_type: Option<OptionOrderRespType>,
    /// 用户自定义订单 ID
    client_order_id: Option<String>,
    /// 是否为做市商保护订单
    is_mmp: Option<bool>,
}

/// 取消订单命令
/// DELETE /eapi/v1/order
/// Weight: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct CancelOrderCmd {
    metadata: CMetadata,
    /// 期权交易对
    symbol: String,
    /// 订单 ID（orderId 和 clientOrderId 至少提供一个）
    order_id: Option<u64>,
    /// 用户自定义订单 ID
    client_order_id: Option<String>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 批量取消订单命令
/// DELETE /eapi/v1/batchOrders
/// Weight: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct BatchCancelOrdersCmd {
    metadata: CMetadata,
    /// 期权交易对
    symbol: String,
    /// 订单 ID 列表
    order_ids: Option<Vec<u64>>,
    /// 用户自定义订单 ID 列表
    client_order_ids: Option<Vec<String>>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 取消所有标的物下的期权订单命令
/// DELETE /eapi/v1/allOpenOrdersByUnderlying
/// Weight: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct CancelAllByUnderlyingCmd {
    metadata: CMetadata,
    /// 标的物，如 BTCUSDT
    underlying: String,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 取消指定交易对的所有订单命令
/// DELETE /eapi/v1/allOpenOrders
/// Weight: 1
/// Data Source: Matching Engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct CancelAllOnSymbolCmd {
    metadata: CMetadata,
    /// 期权交易对
    symbol: String,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 查询单个订单命令
/// GET /eapi/v1/order
/// Weight: 1
/// Data Source: Memory => Database
/// 注意：已取消/拒绝且无成交且创建时间超过 3 天的订单无法查询
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct QueryOrderCmd {
    metadata: CMetadata,
    /// 期权交易对
    symbol: String,
    /// 订单 ID
    order_id: Option<u64>,
    /// 用户自定义订单 ID
    client_order_id: Option<String>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 查询当前挂单命令
/// GET /eapi/v1/openOrders
/// Weight: 1 (单交易对) / 40 (所有交易对)
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct QueryOpenOrdersCmd {
    metadata: CMetadata,
    /// 期权交易对（可选，不传则返回所有交易对的挂单）
    symbol: Option<String>,
    /// 起始订单 ID（返回 orderId >= orderId 的订单）
    order_id: Option<u64>,
    /// 起始时间
    start_time: Option<Timestamp>,
    /// 结束时间
    end_time: Option<Timestamp>,
    /// 返回数量限制（默认 100，最大 1000）
    limit: Option<i32>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 查询订单历史命令
/// GET /eapi/v1/historyOrders
/// Weight: 3
/// Data Source: Database
/// 查询 5 天内已结束的订单（状态: CANCELLED FILLED REJECTED）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct QueryOrderHistoryCmd {
    metadata: CMetadata,
    /// 期权交易对
    symbol: String,
    /// 起始订单 ID
    order_id: Option<u64>,
    /// 起始时间
    start_time: Option<Timestamp>,
    /// 结束时间
    end_time: Option<Timestamp>,
    /// 返回数量限制（默认 100，最大 1000）
    limit: Option<i32>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 查询账户成交历史命令
/// GET /eapi/v1/userTrades
/// Weight: 5
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct QueryUserTradesCmd {
    metadata: CMetadata,
    /// 期权交易对（可选）
    symbol: Option<String>,
    /// 起始成交 ID
    from_id: Option<u64>,
    /// 起始时间
    start_time: Option<Timestamp>,
    /// 结束时间
    end_time: Option<Timestamp>,
    /// 返回数量限制（默认 100，最大 1000）
    limit: Option<i32>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 查询持仓信息命令
/// GET /eapi/v1/position
/// Weight: 5
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct QueryPositionCmd {
    metadata: CMetadata,
    /// 期权交易对（可选）
    symbol: Option<String>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

/// 查询行权记录命令
/// GET /eapi/v1/exerciseRecord
/// Weight: 5
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct QueryExerciseRecordCmd {
    metadata: CMetadata,
    /// 期权交易对（可选）
    symbol: Option<String>,
    /// 起始时间
    start_time: Option<Timestamp>,
    /// 结束时间
    end_time: Option<Timestamp>,
    /// 返回数量限制（默认 1000，最大 1000）
    limit: Option<i32>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: Timestamp,
}

// ==================== 响应类型定义 ====================

/// Option Trade 响应枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionTradeResAny {
    /// 新建订单响应（ACK 模式）
    NewOrderAck(NewOrderAck),
    /// 新建订单响应（RESULT 模式）
    NewOrderResult(NewOrderResult),
    /// 批量下单响应
    BatchOrders(Vec<BatchOrderItemRes>),
    /// 取消订单响应
    CancelOrder(CancelOrderRes),
    /// 批量取消订单响应
    BatchCancelOrders(Vec<CancelOrderRes>),
    /// 取消所有标的物下订单响应
    CancelAllByUnderlying(CancelAllByUnderlyingRes),
    /// 取消指定交易对所有订单响应
    CancelAllOnSymbol(CancelAllOnSymbolRes),
    /// 查询订单响应
    QueryOrder(QueryOrderRes),
    /// 查询当前挂单响应
    QueryOpenOrders(Vec<QueryOrderRes>),
    /// 查询订单历史响应
    QueryOrderHistory(Vec<QueryOrderRes>),
    /// 查询账户成交历史响应
    QueryUserTrades(Vec<UserTradeRes>),
    /// 查询持仓信息响应
    QueryPosition(Vec<PositionRes>),
    /// 查询行权记录响应
    QueryExerciseRecord(Vec<ExerciseRecordRes>),
}

// ==================== 响应结构 ====================

/// 新建订单 ACK 响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct NewOrderAck {
    /// 订单 ID
    order_id: u64,
    /// 期权交易对
    symbol: String,
    /// 订单价格
    price: Price,
    /// 订单数量
    quantity: Quantity,
    /// 订单方向
    side: OptionSide,
    /// 订单类型
    order_type: OptionOrderType,
    /// 订单创建时间
    create_time: Timestamp,
    /// 是否仅减仓
    reduce_only: bool,
    /// 是否只挂单
    post_only: bool,
    /// 是否为做市商保护订单
    mmp: bool,
}

/// 新建订单 RESULT 响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct NewOrderResult {
    /// 订单 ID
    order_id: u64,
    /// 期权交易对
    symbol: String,
    /// 订单价格
    price: Price,
    /// 订单数量
    quantity: Quantity,
    /// 已成交数量
    executed_qty: Quantity,
    /// 手续费
    fee: Quantity,
    /// 订单方向
    side: OptionSide,
    /// 订单类型
    order_type: OptionOrderType,
    /// 有效方式
    time_in_force: TimeInForce,
    /// 是否仅减仓
    reduce_only: bool,
    /// 是否只挂单
    post_only: bool,
    /// 订单创建时间
    create_time: Timestamp,
    /// 订单更新时间
    update_time: Timestamp,
    /// 订单状态
    status: OptionOrderStatus,
    /// 成交均价
    avg_price: Price,
    /// 用户自定义订单 ID
    client_order_id: String,
    /// 价格精度
    price_scale: i32,
    /// 数量精度
    quantity_scale: i32,
    /// 期权类型（看涨/看跌）
    option_side: OptionType,
    /// 报价资产
    quote_asset: String,
    /// 是否为做市商保护订单
    mmp: bool,
}

/// 批量订单项响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct BatchOrderItemRes {
    /// 订单 ID
    order_id: u64,
    /// 期权交易对
    symbol: String,
    /// 订单价格
    price: Price,
    /// 订单数量
    quantity: Quantity,
    /// 订单方向
    side: OptionSide,
    /// 订单类型
    order_type: OptionOrderType,
    /// 是否仅减仓
    reduce_only: bool,
    /// 是否只挂单
    post_only: bool,
    /// 用户自定义订单 ID
    client_order_id: String,
    /// 是否为做市商保护订单
    mmp: bool,
}

/// 取消订单响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct CancelOrderRes {
    /// 订单 ID
    order_id: u64,
    /// 期权交易对
    symbol: String,
    /// 订单价格
    price: Price,
    /// 订单数量
    quantity: Quantity,
    /// 已成交数量
    executed_qty: Quantity,
    /// 手续费
    fee: Quantity,
    /// 订单方向
    side: OptionSide,
    /// 订单类型
    order_type: OptionOrderType,
    /// 有效方式
    time_in_force: TimeInForce,
    /// 是否仅减仓
    reduce_only: bool,
    /// 是否只挂单
    post_only: bool,
    /// 订单创建时间
    create_time: Timestamp,
    /// 订单更新时间
    update_time: Timestamp,
    /// 订单状态
    status: OptionOrderStatus,
    /// 成交均价
    avg_price: Price,
    /// 订单来源
    source: String,
    /// 用户自定义订单 ID
    client_order_id: String,
    /// 价格精度
    price_scale: i32,
    /// 数量精度
    quantity_scale: i32,
    /// 期权类型
    option_side: OptionType,
    /// 报价资产
    quote_asset: String,
    /// 是否为做市商保护订单
    mmp: bool,
}

/// 取消所有标的物下订单响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct CancelAllByUnderlyingRes {
    /// 响应码
    code: i32,
    /// 响应消息
    msg: String,
    /// 取消的订单数量
    data: i32,
}

/// 取消指定交易对所有订单响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct CancelAllOnSymbolRes {
    /// 响应码
    code: i32,
    /// 响应消息
    msg: String,
}

/// 查询订单响应（通用）
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct QueryOrderRes {
    /// 订单 ID
    order_id: u64,
    /// 期权交易对
    symbol: String,
    /// 订单价格
    price: Price,
    /// 订单数量
    quantity: Quantity,
    /// 已成交数量
    executed_qty: Quantity,
    /// 手续费
    fee: Quantity,
    /// 订单方向
    side: OptionSide,
    /// 订单类型
    order_type: OptionOrderType,
    /// 有效方式
    time_in_force: TimeInForce,
    /// 是否仅减仓
    reduce_only: bool,
    /// 是否只挂单
    post_only: bool,
    /// 订单创建时间
    create_time: Timestamp,
    /// 订单更新时间
    update_time: Timestamp,
    /// 订单状态
    status: OptionOrderStatus,
    /// 成交均价
    avg_price: Price,
    /// 订单来源
    source: String,
    /// 用户自定义订单 ID
    client_order_id: String,
    /// 价格精度
    price_scale: i32,
    /// 数量精度
    quantity_scale: i32,
    /// 期权类型
    option_side: OptionType,
    /// 报价资产
    quote_asset: String,
    /// 是否为做市商保护订单
    mmp: bool,
}

/// 成交记录
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct UserTradeRes {
    /// 唯一 ID
    id: u64,
    /// 成交 ID
    trade_id: u64,
    /// 订单 ID
    order_id: u64,
    /// 期权交易对
    symbol: String,
    /// 成交价格
    price: Price,
    /// 成交数量
    quantity: Quantity,
    /// 手续费
    fee: Quantity,
    /// 已实现盈亏
    realized_profit: Quantity,
    /// 订单方向
    side: OptionSide,
    /// 订单类型
    order_type: OptionOrderType,
    /// 波动率
    volatility: String,
    /// 流动性方向（TAKER 或 MAKER）
    liquidity: String,
    /// 报价资产
    quote_asset: String,
    /// 成交时间
    time: Timestamp,
    /// 价格精度
    price_scale: i32,
    /// 数量精度
    quantity_scale: i32,
    /// 期权类型
    option_side: OptionType,
}

/// 持仓信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct PositionRes {
    /// 开仓均价
    entry_price: Price,
    /// 期权交易对
    symbol: String,
    /// 持仓方向
    side: String,
    /// 持仓数量（正数代表多头，负数代表空头）
    quantity: Quantity,
    /// 可平仓数量
    reducible_qty: Quantity,
    /// 当前市值
    mark_value: Quantity,
    /// 收益率
    ror: Quantity,
    /// 未实现盈亏
    unrealized_pnl: Quantity,
    /// 标记价格
    mark_price: Price,
    /// 行权价格
    strike_price: Price,
    /// 持仓成本
    position_cost: Quantity,
    /// 到期时间
    expiry_date: Timestamp,
    /// 价格精度
    price_scale: i32,
    /// 数量精度
    quantity_scale: i32,
    /// 期权类型
    option_side: OptionType,
    /// 报价资产
    quote_asset: String,
}

/// 行权记录
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct ExerciseRecordRes {
    /// 记录 ID
    id: String,
    /// 币种
    currency: String,
    /// 期权交易对
    symbol: String,
    /// 行权价格
    exercise_price: Price,
    /// 标记价格
    mark_price: Price,
    /// 行权数量
    quantity: Quantity,
    /// 行权收益
    amount: Quantity,
    /// 手续费
    fee: Quantity,
    /// 创建时间
    create_date: Timestamp,
    /// 价格精度
    price_scale: i32,
    /// 数量精度
    quantity_scale: i32,
    /// 期权类型
    option_side: OptionType,
    /// 持仓方向
    position_side: String,
    /// 报价资产
    quote_asset: String,
}

// ==================== 行为接口定义 ====================

/// Option Trade 行为接口
pub trait OptionTradeBehavior:
    Handler<OptionTradeCmdAny, OptionTradeResAny, SpotCmdErrorAny>
{
}
