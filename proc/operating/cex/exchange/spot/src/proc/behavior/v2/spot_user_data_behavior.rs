// 参考 ## Account Endpoints / Account information (USER_DATA)
// /Users/hongyaotang/src/rustlob/design/other/binance-spot-api-docs/rest-api.md
// 定义所有 user data 接口

use base_types::handler::handler::Handler;

use crate::proc::behavior::spot_trade_behavior::{CMetadata, SpotCmdErrorAny};

/// User Data 命令枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpotUserDataCmdAny {
    /// 账户信息查询 GET /api/v3/account
    /// Weight: 20
    Account(AccountCmd),

    /// 查询订单 GET /api/v3/order
    /// Weight: 4
    QueryOrder(QueryOrderCmd),

    /// 当前挂单 GET /api/v3/openOrders
    /// Weight: 6 (单交易对) / 80 (所有交易对)
    CurrentOpenOrders(CurrentOpenOrdersCmd),

    /// 所有订单 GET /api/v3/allOrders
    /// Weight: 20
    AllOrders(AllOrdersCmd),

    /// 查询订单列表 GET /api/v3/orderList
    /// Weight: 4
    QueryOrderList(QueryOrderListCmd),

    /// 查询所有订单列表 GET /api/v3/allOrderList
    /// Weight: 20
    QueryAllOrderList(QueryAllOrderListCmd),

    /// 查询当前挂单列表 GET /api/v3/openOrderList
    /// Weight: 6
    QueryOpenOrderList(QueryOpenOrderListCmd),

    /// 账户成交历史 GET /api/v3/myTrades
    /// Weight: 20 (无orderId) / 5 (有orderId)
    MyTrades(MyTradesCmd),

    /// 查询未完成订单计数 GET /api/v3/rateLimit/order
    /// Weight: 40
    QueryUnfilledOrderCount(QueryUnfilledOrderCountCmd),

    /// 查询被阻止的匹配 GET /api/v3/myPreventedMatches
    /// Weight: 2 (preventedMatchId) / 20 (orderId)
    QueryPreventedMatches(QueryPreventedMatchesCmd),

    /// 查询分配记录 GET /api/v3/myAllocations
    /// Weight: 20
    QueryAllocations(QueryAllocationsCmd),

    /// 查询佣金费率 GET /api/v3/account/commission
    /// Weight: 20
    QueryCommissionRates(QueryCommissionRatesCmd),
}

/// 账户信息查询命令
/// GET /api/v3/account
/// Weight: 20
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountCmd {
    metadata: CMetadata,
    /// 仅返回非零余额，默认 false
    omit_zero_balances: Option<bool>,
    /// 接收窗口（微秒精度），不超过 60000
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 查询订单命令
/// GET /api/v3/order
/// Weight: 4
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryOrderCmd {
    metadata: CMetadata,
    /// 交易对
    symbol: String,
    /// 订单ID（与 orig_client_order_id 二选一）
    order_id: Option<i64>,
    /// 客户端订单ID（与 order_id 二选一）
    orig_client_order_id: Option<String>,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 当前挂单查询命令
/// GET /api/v3/openOrders
/// Weight: 6 (单交易对) / 80 (所有交易对)
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CurrentOpenOrdersCmd {
    metadata: CMetadata,
    /// 交易对（可选，不传则返回所有交易对）
    symbol: Option<String>,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 所有订单查询命令
/// GET /api/v3/allOrders
/// Weight: 20
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllOrdersCmd {
    metadata: CMetadata,
    /// 交易对（必填）
    symbol: String,
    /// 订单ID（获取 >= 该ID的订单）
    order_id: Option<i64>,
    /// 开始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 限制数量，默认 500，最大 1000
    limit: Option<i32>,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 查询订单列表命令
/// GET /api/v3/orderList
/// Weight: 4
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryOrderListCmd {
    metadata: CMetadata,
    /// 订单列表ID（与 orig_client_order_id 二选一）
    order_list_id: Option<i64>,
    /// 客户端订单列表ID（与 order_list_id 二选一）
    orig_client_order_id: Option<String>,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 查询所有订单列表命令
/// GET /api/v3/allOrderList
/// Weight: 20
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryAllOrderListCmd {
    metadata: CMetadata,
    /// 从该ID开始（与时间参数互斥）
    from_id: Option<i64>,
    /// 开始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 限制数量，默认 500，最大 1000
    limit: Option<i32>,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 查询当前挂单列表命令
/// GET /api/v3/openOrderList
/// Weight: 6
/// Data Source: Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryOpenOrderListCmd {
    metadata: CMetadata,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 账户成交历史查询命令
/// GET /api/v3/myTrades
/// Weight: 20 (无orderId) / 5 (有orderId)
/// Data Source: Memory => Database
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MyTradesCmd {
    metadata: CMetadata,
    /// 交易对（必填）
    symbol: String,
    /// 订单ID（与symbol组合使用）
    order_id: Option<i64>,
    /// 开始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 从该成交ID开始
    from_id: Option<i64>,
    /// 限制数量，默认 500，最大 1000
    limit: Option<i32>,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 查询未完成订单计数命令
/// GET /api/v3/rateLimit/order
/// Weight: 40
/// Data Source: Memory
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryUnfilledOrderCountCmd {
    metadata: CMetadata,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 查询被阻止的匹配命令
/// GET /api/v3/myPreventedMatches
/// Weight: 2 (symbol无效或preventedMatchId) / 20 (orderId)
/// Data Source: Database
///
/// 支持的参数组合：
/// - symbol + preventedMatchId
/// - symbol + orderId
/// - symbol + orderId + fromPreventedMatchId
/// - symbol + orderId + fromPreventedMatchId + limit
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryPreventedMatchesCmd {
    metadata: CMetadata,
    /// 交易对（必填）
    symbol: String,
    /// 被阻止的匹配ID
    prevented_match_id: Option<i64>,
    /// 订单ID
    order_id: Option<i64>,
    /// 从该被阻止匹配ID开始
    from_prevented_match_id: Option<i64>,
    /// 限制数量，默认 500，最大 1000
    limit: Option<i32>,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 查询分配记录命令
/// GET /api/v3/myAllocations
/// Weight: 20
/// Data Source: Database
///
/// 支持的参数组合：
/// - symbol
/// - symbol + startTime
/// - symbol + endTime
/// - symbol + startTime + endTime
/// - symbol + fromAllocationId
/// - symbol + orderId
/// - symbol + orderId + fromAllocationId
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryAllocationsCmd {
    metadata: CMetadata,
    /// 交易对（必填）
    symbol: String,
    /// 开始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 从该分配ID开始
    from_allocation_id: Option<i32>,
    /// 限制数量，默认 500，最大 1000
    limit: Option<i32>,
    /// 订单ID
    order_id: Option<i64>,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// 查询佣金费率命令
/// GET /api/v3/account/commission
/// Weight: 20
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryCommissionRatesCmd {
    metadata: CMetadata,
    /// 交易对（必填）
    symbol: String,
    /// 接收窗口
    recv_window: Option<u64>,
    /// 时间戳
    timestamp: i64,
}

/// User Data 响应枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpotUserDataResAny {
    /// 账户信息响应
    Account(AccountInfo),

    /// 订单信息响应
    Order(OrderInfo),

    /// 订单列表响应
    Orders(Vec<OrderInfo>),

    /// 订单列表信息响应
    OrderList(OrderListInfo),

    /// 订单列表数组响应
    OrderLists(Vec<OrderListInfo>),

    /// 成交历史响应
    Trades(Vec<TradeInfo>),

    /// 未完成订单计数响应
    UnfilledOrderCount(Vec<RateLimitInfo>),

    /// 被阻止的匹配响应
    PreventedMatches(Vec<PreventedMatch>),

    /// 分配记录响应
    Allocations(Vec<AllocationInfo>),

    /// 佣金费率响应
    CommissionRates(CommissionRates),
}

/// 账户信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountInfo {
    /// Maker 佣金
    maker_commission: i32,
    /// Taker 佣金
    taker_commission: i32,
    /// 买方佣金
    buyer_commission: i32,
    /// 卖方佣金
    seller_commission: i32,
    /// 佣金费率
    commission_rates: CommissionRates,
    /// 可交易
    can_trade: bool,
    /// 可提现
    can_withdraw: bool,
    /// 可充值
    can_deposit: bool,
    /// 经纪账户
    brokered: bool,
    /// 需要自成交防护
    require_self_trade_prevention: bool,
    /// 阻止 SOR
    prevent_sor: bool,
    /// 更新时间
    update_time: i64,
    /// 账户类型
    account_type: String,
    /// 余额列表
    balances: Vec<Balance>,
    /// 权限列表
    permissions: Vec<String>,
    /// 用户ID
    uid: i64,
}

/// 佣金费率
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommissionRates {
    /// Maker 费率
    maker: String,
    /// Taker 费率
    taker: String,
    /// 买方费率
    buyer: String,
    /// 卖方费率
    seller: String,
}

/// 余额信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Balance {
    /// 资产名称
    asset: String,
    /// 可用余额
    free: String,
    /// 锁定余额
    locked: String,
}

/// 订单信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderInfo {
    /// 交易对
    symbol: String,
    /// 订单ID
    order_id: i64,
    /// 订单列表ID（-1表示不属于订单列表）
    order_list_id: i64,
    /// 客户端订单ID
    client_order_id: String,
    /// 价格
    price: String,
    /// 原始数量
    orig_qty: String,
    /// 已执行数量
    executed_qty: String,
    /// 累计成交金额
    cummulative_quote_qty: String,
    /// 订单状态
    status: String,
    /// 有效期类型
    time_in_force: String,
    /// 订单类型
    order_type: String,
    /// 买卖方向
    side: String,
    /// 止损价格
    stop_price: Option<String>,
    /// 冰山数量
    iceberg_qty: Option<String>,
    /// 订单创建时间
    time: i64,
    /// 订单更新时间
    update_time: i64,
    /// 是否工作中
    is_working: bool,
    /// 工作时间
    working_time: i64,
    /// 原始报价订单数量
    orig_quote_order_qty: String,
    /// 自成交防护模式
    self_trade_prevention_mode: String,
}

/// 订单列表信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderListInfo {
    /// 订单列表ID
    order_list_id: i64,
    /// 联动类型
    contingency_type: String,
    /// 列表状态类型
    list_status_type: String,
    /// 列表订单状态
    list_order_status: String,
    /// 列表客户端订单ID
    list_client_order_id: String,
    /// 交易时间
    transaction_time: i64,
    /// 交易对
    symbol: String,
    /// 订单列表
    orders: Vec<OrderListOrder>,
}

/// 订单列表中的订单
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderListOrder {
    /// 交易对
    symbol: String,
    /// 订单ID
    order_id: i64,
    /// 客户端订单ID
    client_order_id: String,
}

/// 成交信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TradeInfo {
    /// 交易对
    symbol: String,
    /// 成交ID
    id: i64,
    /// 订单ID
    order_id: i64,
    /// 订单列表ID
    order_list_id: i64,
    /// 成交价格
    price: String,
    /// 成交数量
    qty: String,
    /// 成交金额
    quote_qty: String,
    /// 佣金
    commission: String,
    /// 佣金资产
    commission_asset: String,
    /// 成交时间
    time: i64,
    /// 是否为买方
    is_buyer: bool,
    /// 是否为挂单方
    is_maker: bool,
    /// 是否为最佳匹配
    is_best_match: bool,
}

/// 速率限制信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RateLimitInfo {
    /// 速率限制类型
    rate_limit_type: String,
    /// 时间间隔
    interval: String,
    /// 间隔数量
    interval_num: i32,
    /// 限制值
    limit: i32,
    /// 当前计数
    count: i32,
}

/// 被阻止的匹配信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PreventedMatch {
    /// 交易对
    symbol: String,
    /// 被阻止的匹配ID
    prevented_match_id: i64,
    /// Taker 订单ID
    taker_order_id: i64,
    /// Maker 交易对
    maker_symbol: String,
    /// Maker 订单ID
    maker_order_id: i64,
    /// 交易组ID
    trade_group_id: i64,
    /// 自成交防护模式
    self_trade_prevention_mode: String,
    /// 价格
    price: String,
    /// Maker 被阻止的数量
    maker_prevented_quantity: String,
    /// 交易时间
    transact_time: i64,
}

/// 分配信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllocationInfo {
    /// 交易对
    symbol: String,
    /// 分配ID
    allocation_id: i64,
    /// 分配类型
    allocation_type: String,
    /// 订单ID
    order_id: i64,
    /// 订单列表ID
    order_list_id: i64,
    /// 价格
    price: String,
    /// 数量
    qty: String,
    /// 金额
    quote_qty: String,
    /// 佣金
    commission: String,
    /// 佣金资产
    commission_asset: String,
    /// 时间
    time: i64,
    /// 是否为买方
    is_buyer: bool,
    /// 是否为挂单方
    is_maker: bool,
    /// 是否为分配者
    is_allocator: bool,
}

/// User Data 行为接口
pub trait SpotUserDataBehavior:
    Send + Sync + Handler<SpotUserDataCmdAny, SpotUserDataResAny, SpotCmdErrorAny>
{
}
