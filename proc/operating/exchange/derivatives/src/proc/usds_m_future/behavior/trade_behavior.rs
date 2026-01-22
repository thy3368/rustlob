// 参考 ## trade Endpoints /Users/hongyaotang/src/rustlob/design/other/binance_derivatives_api/usds-margined-futures/trade
// 定义所有trade 接口

use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};

// ============================================================================
// 枚举类型定义
// ============================================================================

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    BUY,
    SELL,
}

/// 持仓方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSide {
    /// 单向持仓模式
    BOTH,
    /// 多头（对冲模式）
    LONG,
    /// 空头（对冲模式）
    SHORT,
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    LIMIT,
    MARKET,
    STOP,
    STOP_MARKET,
    TAKE_PROFIT,
    TAKE_PROFIT_MARKET,
    TRAILING_STOP_MARKET,
}

/// 有效时间类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    /// 成交为止
    GTC,
    /// 立即成交或取消
    IOC,
    /// 全部成交或取消
    FOK,
    /// 只做Maker
    GTX,
    /// 指定时间前有效
    GTD,
}

/// 工作类型（触发价格类型）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkingType {
    MARK_PRICE,
    CONTRACT_PRICE,
}

/// 价格匹配模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceMatch {
    NONE,
    OPPONENT,
    OPPONENT_5,
    OPPONENT_10,
    OPPONENT_20,
    QUEUE,
    QUEUE_5,
    QUEUE_10,
    QUEUE_20,
}

/// 自成交防护模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfTradePreventionMode {
    /// 无防护
    NONE,
    /// 撤销吃单方
    EXPIRE_TAKER,
    /// 撤销挂单方
    EXPIRE_MAKER,
    /// 撤销双方
    EXPIRE_BOTH,
}

/// 订单响应类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewOrderRespType {
    /// 仅返回确认
    ACK,
    /// 返回完整结果
    RESULT,
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    NEW,
    PARTIALLY_FILLED,
    FILLED,
    CANCELED,
    REJECTED,
    EXPIRED,
}

/// 保证金类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginType {
    ISOLATED,
    CROSSED,
}

// ============================================================================
// Trade 命令枚举
// ============================================================================

/// USDS-M期货交易命令枚举
#[derive(Debug, Clone)]
pub enum UsdsMFutureTradeCmdAny {
    /// 下单 POST /fapi/v1/order
    /// Weight: 0 (IP), 1 (10s order), 1 (1m order)
    NewOrder(NewOrderCmd),

    /// 测试下单 POST /fapi/v1/order/test
    /// Weight: 1
    NewOrderTest(NewOrderTestCmd),

    /// 批量下单 POST /fapi/v1/batchOrders
    /// Weight: 5 (IP), 5 (10s order), 1 (1m order)
    PlaceMultipleOrders(PlaceMultipleOrdersCmd),

    /// 查询订单 GET /fapi/v1/order
    /// Weight: 1
    QueryOrder(QueryOrderCmd),

    /// 撤销订单 DELETE /fapi/v1/order
    /// Weight: 1
    CancelOrder(CancelOrderCmd),

    /// 修改订单 PUT /fapi/v1/order
    /// Weight: 1 (IP), 1 (10s order), 1 (1m order)
    ModifyOrder(ModifyOrderCmd),

    /// 批量撤销订单 DELETE /fapi/v1/batchOrders
    /// Weight: 1
    CancelMultipleOrders(CancelMultipleOrdersCmd),

    /// 批量修改订单 PUT /fapi/v1/batchOrders
    ModifyMultipleOrders(ModifyMultipleOrdersCmd),

    /// 撤销全部订单 DELETE /fapi/v1/allOpenOrders
    /// Weight: 1
    CancelAllOpenOrders(CancelAllOpenOrdersCmd),

    /// 自动撤销全部订单 POST /fapi/v1/countdownCancelAll
    AutoCancelAllOpenOrders(AutoCancelAllOpenOrdersCmd),

    /// 查询当前挂单 GET /fapi/v1/openOrders
    /// Weight: 1 (single), 40 (all)
    CurrentAllOpenOrders(CurrentAllOpenOrdersCmd),

    /// 查询当前某个订单 GET /fapi/v1/openOrder
    QueryCurrentOpenOrder(QueryCurrentOpenOrderCmd),

    /// 查询所有订单 GET /fapi/v1/allOrders
    /// Weight: 5
    AllOrders(AllOrdersCmd),

    /// 账户成交历史 GET /fapi/v1/userTrades
    /// Weight: 5
    AccountTradeList(AccountTradeListCmd),

    /// 查询订单修改历史 GET /fapi/v1/orderAmendment
    GetOrderModifyHistory(GetOrderModifyHistoryCmd),

    /// 调整杠杆倍数 POST /fapi/v1/leverage
    /// Weight: 1
    ChangeInitialLeverage(ChangeInitialLeverageCmd),

    /// 变换保证金模式 POST /fapi/v1/marginType
    /// Weight: 1
    ChangeMarginType(ChangeMarginTypeCmd),

    /// 调整逐仓保证金 POST /fapi/v1/positionMargin
    ModifyIsolatedPositionMargin(ModifyIsolatedPositionMarginCmd),

    /// 查询保证金变动历史 GET /fapi/v1/positionMargin/history
    /// Weight: 1
    GetPositionMarginChangeHistory(GetPositionMarginChangeHistoryCmd),

    /// 查询持仓信息 V2 GET /fapi/v2/positionRisk
    /// Weight: 5
    PositionInformationV2(PositionInformationV2Cmd),

    /// 查询持仓信息 V3 GET /fapi/v3/positionRisk
    /// Weight: 5
    PositionInformationV3(PositionInformationV3Cmd),

    /// 用户强平单历史 GET /fapi/v1/forceOrders
    /// Weight: 1 (single), 20 (all)
    UsersForceOrders(UsersForceOrdersCmd),

    /// 持仓ADL队列估算 GET /fapi/v1/adlQuantile
    /// Weight: 5
    PositionADLQuantileEstimation(PositionADLQuantileEstimationCmd),

    /// 切换持仓模式 POST /fapi/v1/positionSide/dual
    /// Weight: 1
    ChangePositionMode(ChangePositionModeCmd),

    /// 切换联合保证金模式 POST /fapi/v1/multiAssetsMargin
    /// Weight: 1
    ChangeMultiAssetsMode(ChangeMultiAssetsModeCmd),
}

// ============================================================================
// 1. 订单管理命令
// ============================================================================

/// 下单命令
/// POST /fapi/v1/order
/// Weight: 0 (IP), 1 (10s order), 1 (1m order)
#[derive(Debug, Clone)]
pub struct NewOrderCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 买卖方向
    pub side: OrderSide,
    /// 持仓方向（单向模式默认BOTH，对冲模式必填）
    pub position_side: Option<PositionSide>,
    /// 订单类型
    pub order_type: OrderType,
    /// 有效时间类型
    pub time_in_force: Option<TimeInForce>,
    /// 数量（不能与closePosition=true同时使用）
    pub quantity: Option<String>,
    /// 只减仓（默认false，对冲模式不可用）
    pub reduce_only: Option<bool>,
    /// 价格
    pub price: Option<String>,
    /// 客户端订单ID（自动生成，需符合规则）
    pub new_client_order_id: Option<String>,
    /// 止损/止盈触发价
    pub stop_price: Option<String>,
    /// 全平仓（仅STOP_MARKET/TAKE_PROFIT_MARKET可用）
    pub close_position: Option<bool>,
    /// 追踪止损激活价格
    pub activation_price: Option<String>,
    /// 追踪止损回调比率（0.1-5，1表示1%）
    pub callback_rate: Option<String>,
    /// 触发价格类型
    pub working_type: Option<WorkingType>,
    /// 条件单触发保护
    pub price_protect: Option<bool>,
    /// 订单响应类型
    pub new_order_resp_type: Option<NewOrderRespType>,
    /// 价格匹配模式
    pub price_match: Option<PriceMatch>,
    /// 自成交防护模式
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// GTD订单自动取消时间（毫秒时间戳）
    pub good_till_date: Option<i64>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 测试下单命令（验证参数但不实际下单）
/// POST /fapi/v1/order/test
/// Weight: 1
#[derive(Debug, Clone)]
pub struct NewOrderTestCmd {
    pub metadata: CMetadata,
    /// 与NewOrderCmd参数相同
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<String>,
    pub reduce_only: Option<bool>,
    pub price: Option<String>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<String>,
    pub close_position: Option<bool>,
    pub activation_price: Option<String>,
    pub callback_rate: Option<String>,
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<bool>,
    pub new_order_resp_type: Option<NewOrderRespType>,
    pub price_match: Option<PriceMatch>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    pub good_till_date: Option<i64>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 批量下单命令
/// POST /fapi/v1/batchOrders
/// Weight: 5 (IP), 5 (10s order), 1 (1m order)
/// 最大5个订单
#[derive(Debug, Clone)]
pub struct PlaceMultipleOrdersCmd {
    pub metadata: CMetadata,
    /// 订单列表（最多5个）
    pub batch_orders: Vec<BatchOrderItem>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 批量订单项
#[derive(Debug, Clone)]
pub struct BatchOrderItem {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: String,
    pub reduce_only: Option<bool>,
    pub price: Option<String>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<String>,
    pub activation_price: Option<String>,
    pub callback_rate: Option<String>,
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<bool>,
    pub new_order_resp_type: Option<NewOrderRespType>,
    pub price_match: Option<PriceMatch>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    pub good_till_date: Option<i64>,
}

/// 查询订单命令
/// GET /fapi/v1/order
/// Weight: 1
#[derive(Debug, Clone)]
pub struct QueryOrderCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 订单ID
    pub order_id: Option<i64>,
    /// 客户端订单ID
    pub orig_client_order_id: Option<String>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 撤销订单命令
/// DELETE /fapi/v1/order
/// Weight: 1
#[derive(Debug, Clone)]
pub struct CancelOrderCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 订单ID
    pub order_id: Option<i64>,
    /// 客户端订单ID
    pub orig_client_order_id: Option<String>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 修改订单命令（仅支持LIMIT订单）
/// PUT /fapi/v1/order
/// Weight: 1 (IP), 1 (10s order), 1 (1m order)
#[derive(Debug, Clone)]
pub struct ModifyOrderCmd {
    pub metadata: CMetadata,
    /// 订单ID
    pub order_id: Option<i64>,
    /// 客户端订单ID
    pub orig_client_order_id: Option<String>,
    /// 交易对
    pub symbol: String,
    /// 买卖方向
    pub side: OrderSide,
    /// 新数量（必填）
    pub quantity: String,
    /// 新价格（必填）
    pub price: String,
    /// 价格匹配模式
    pub price_match: Option<PriceMatch>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 批量撤销订单命令
/// DELETE /fapi/v1/batchOrders
/// Weight: 1
#[derive(Debug, Clone)]
pub struct CancelMultipleOrdersCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 订单ID列表（最多10个）
    pub order_id_list: Option<Vec<i64>>,
    /// 客户端订单ID列表（最多10个）
    pub orig_client_order_id_list: Option<Vec<String>>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 批量修改订单命令
/// PUT /fapi/v1/batchOrders
#[derive(Debug, Clone)]
pub struct ModifyMultipleOrdersCmd {
    pub metadata: CMetadata,
    /// 修改订单列表
    pub batch_orders: Vec<ModifyBatchOrderItem>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 批量修改订单项
#[derive(Debug, Clone)]
pub struct ModifyBatchOrderItem {
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: String,
    pub price: String,
    pub price_match: Option<PriceMatch>,
}

/// 撤销全部订单命令
/// DELETE /fapi/v1/allOpenOrders
/// Weight: 1
#[derive(Debug, Clone)]
pub struct CancelAllOpenOrdersCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 自动撤销全部订单命令（倒计时）
/// POST /fapi/v1/countdownCancelAll
#[derive(Debug, Clone)]
pub struct AutoCancelAllOpenOrdersCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 倒计时时间（毫秒，0表示取消倒计时）
    pub countdown_time: i64,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 查询当前挂单命令
/// GET /fapi/v1/openOrders
/// Weight: 1 (single symbol), 40 (all symbols)
#[derive(Debug, Clone)]
pub struct CurrentAllOpenOrdersCmd {
    pub metadata: CMetadata,
    /// 交易对（不填返回所有交易对）
    pub symbol: Option<String>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 查询当前某个订单命令
/// GET /fapi/v1/openOrder
#[derive(Debug, Clone)]
pub struct QueryCurrentOpenOrderCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 订单ID
    pub order_id: Option<i64>,
    /// 客户端订单ID
    pub orig_client_order_id: Option<String>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 查询所有订单命令（包括历史订单）
/// GET /fapi/v1/allOrders
/// Weight: 5
#[derive(Debug, Clone)]
pub struct AllOrdersCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 起始订单ID（返回>=该ID的订单）
    pub order_id: Option<i64>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量（默认500，最大1000）
    pub limit: Option<i32>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 账户成交历史命令
/// GET /fapi/v1/userTrades
/// Weight: 5
#[derive(Debug, Clone)]
pub struct AccountTradeListCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 起始成交ID
    pub from_id: Option<i64>,
    /// 限制数量（默认500，最大1000）
    pub limit: Option<i32>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 查询订单修改历史命令
/// GET /fapi/v1/orderAmendment
#[derive(Debug, Clone)]
pub struct GetOrderModifyHistoryCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 订单ID
    pub order_id: Option<i64>,
    /// 客户端订单ID
    pub orig_client_order_id: Option<String>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量（默认50，最大100）
    pub limit: Option<i32>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

// ============================================================================
// 2. 持仓和保证金管理命令
// ============================================================================

/// 调整杠杆倍数命令
/// POST /fapi/v1/leverage
/// Weight: 1
#[derive(Debug, Clone)]
pub struct ChangeInitialLeverageCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 目标杠杆倍数（1-125）
    pub leverage: i32,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 变换保证金模式命令
/// POST /fapi/v1/marginType
/// Weight: 1
#[derive(Debug, Clone)]
pub struct ChangeMarginTypeCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 保证金类型
    pub margin_type: MarginType,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 调整逐仓保证金命令
/// POST /fapi/v1/positionMargin
#[derive(Debug, Clone)]
pub struct ModifyIsolatedPositionMarginCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 持仓方向
    pub position_side: Option<PositionSide>,
    /// 保证金变动金额
    pub amount: String,
    /// 操作类型（1:增加，2:减少）
    pub margin_type: i32,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 查询保证金变动历史命令
/// GET /fapi/v1/positionMargin/history
/// Weight: 1
#[derive(Debug, Clone)]
pub struct GetPositionMarginChangeHistoryCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 操作类型（1:增加，2:减少）
    pub margin_type: Option<i32>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量（默认500）
    pub limit: Option<i32>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 查询持仓信息命令 V2
/// GET /fapi/v2/positionRisk
/// Weight: 5
#[derive(Debug, Clone)]
pub struct PositionInformationV2Cmd {
    pub metadata: CMetadata,
    /// 交易对（不填返回所有）
    pub symbol: Option<String>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 查询持仓信息命令 V3
/// GET /fapi/v3/positionRisk
/// Weight: 5
#[derive(Debug, Clone)]
pub struct PositionInformationV3Cmd {
    pub metadata: CMetadata,
    /// 交易对（不填返回所有有持仓或挂单的交易对）
    pub symbol: Option<String>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 用户强平单历史命令
/// GET /fapi/v1/forceOrders
/// Weight: 1 (single symbol), 20 (all symbols)
#[derive(Debug, Clone)]
pub struct UsersForceOrdersCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: Option<String>,
    /// 自动强平类型（LIQUIDATION/ADL）
    pub auto_close_type: Option<String>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量（默认50，最大100）
    pub limit: Option<i32>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 持仓ADL队列估算命令
/// GET /fapi/v1/adlQuantile
/// Weight: 5
#[derive(Debug, Clone)]
pub struct PositionADLQuantileEstimationCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: Option<String>,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

// ============================================================================
// 3. 账户配置命令
// ============================================================================

/// 切换持仓模式命令（单向/双向）
/// POST /fapi/v1/positionSide/dual
/// Weight: 1
#[derive(Debug, Clone)]
pub struct ChangePositionModeCmd {
    pub metadata: CMetadata,
    /// 双向持仓模式（true:对冲模式，false:单向模式）
    pub dual_side_position: bool,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

/// 切换联合保证金模式命令
/// POST /fapi/v1/multiAssetsMargin
/// Weight: 1
#[derive(Debug, Clone)]
pub struct ChangeMultiAssetsModeCmd {
    pub metadata: CMetadata,
    /// 联合保证金模式（true:启用，false:禁用）
    pub multi_assets_margin: bool,
    /// 接收窗口
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
}

// ============================================================================
// 响应类型定义
// ============================================================================

/// Trade 响应枚举
#[derive(Debug, Clone)]
pub enum UsdsMFutureTradeRes {
    /// 订单响应
    Order(OrderResponse),
    /// 批量订单响应
    BatchOrders(Vec<BatchOrderResponse>),
    /// 订单列表响应
    Orders(Vec<OrderResponse>),
    /// 成交列表响应
    Trades(Vec<TradeResponse>),
    /// 杠杆响应
    Leverage(LeverageResponse),
    /// 保证金类型响应
    MarginType(MarginTypeResponse),
    /// 保证金响应
    Margin(MarginResponse),
    /// 保证金历史响应
    MarginHistory(Vec<MarginHistoryResponse>),
    /// 持仓信息响应
    Positions(Vec<PositionResponse>),
    /// 强平单响应
    ForceOrders(Vec<ForceOrderResponse>),
    /// ADL队列响应
    ADLQuantile(Vec<ADLQuantileResponse>),
    /// 通用成功响应
    Success(SuccessResponse),
    /// 倒计时响应
    CountdownResponse(CountdownResponse),
}

/// 订单响应
#[derive(Debug, Clone)]
pub struct OrderResponse {
    pub order_id: i64,
    pub symbol: String,
    pub status: OrderStatus,
    pub client_order_id: String,
    pub price: String,
    pub avg_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub stop_price: Option<String>,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub orig_type: OrderType,
    pub price_match: Option<PriceMatch>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    pub good_till_date: i64,
    pub update_time: i64,
    // TRAILING_STOP_MARKET 特有字段
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
}

/// 批量订单响应（成功或错误）
#[derive(Debug, Clone)]
pub enum BatchOrderResponse {
    Success(OrderResponse),
    Error { code: i32, msg: String },
}

/// 成交响应
#[derive(Debug, Clone)]
pub struct TradeResponse {
    pub symbol: String,
    pub id: i64,
    pub order_id: i64,
    pub side: OrderSide,
    pub price: String,
    pub qty: String,
    pub realized_pnl: String,
    pub margin_asset: String,
    pub quote_qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub time: i64,
    pub position_side: PositionSide,
    pub buyer: bool,
    pub maker: bool,
}

/// 杠杆响应
#[derive(Debug, Clone)]
pub struct LeverageResponse {
    pub leverage: i32,
    pub max_notional_value: String,
    pub symbol: String,
}

/// 保证金类型响应
#[derive(Debug, Clone)]
pub struct MarginTypeResponse {
    pub code: i32,
    pub msg: String,
}

/// 保证金响应
#[derive(Debug, Clone)]
pub struct MarginResponse {
    pub amount: String,
    pub code: i32,
    pub msg: String,
    pub margin_type: i32,
}

/// 保证金历史响应
#[derive(Debug, Clone)]
pub struct MarginHistoryResponse {
    pub amount: String,
    pub asset: String,
    pub symbol: String,
    pub time: i64,
    pub margin_type: i32,
    pub position_side: PositionSide,
}

/// 持仓信息响应
#[derive(Debug, Clone)]
pub struct PositionResponse {
    pub symbol: String,
    pub position_side: PositionSide,
    pub position_amt: String,
    pub entry_price: String,
    pub break_even_price: String,
    pub mark_price: String,
    pub un_realized_profit: String,
    pub liquidation_price: String,
    pub isolated_margin: String,
    pub notional: String,
    pub margin_asset: String,
    pub isolated_wallet: String,
    pub initial_margin: String,
    pub maint_margin: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub adl: i32,
    pub bid_notional: String,
    pub ask_notional: String,
    pub update_time: i64,
}

/// 强平单响应
#[derive(Debug, Clone)]
pub struct ForceOrderResponse {
    pub order_id: i64,
    pub symbol: String,
    pub status: OrderStatus,
    pub client_order_id: String,
    pub price: String,
    pub avg_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_quote: String,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub stop_price: String,
    pub working_type: WorkingType,
    pub orig_type: OrderType,
    pub time: i64,
    pub update_time: i64,
}

/// ADL队列响应
#[derive(Debug, Clone)]
pub struct ADLQuantileResponse {
    pub symbol: String,
    pub adl_quantile: ADLQuantile,
}

/// ADL队列详情
#[derive(Debug, Clone)]
pub struct ADLQuantile {
    pub long: i32,
    pub short: i32,
    pub both: i32,
}

/// 通用成功响应
#[derive(Debug, Clone)]
pub struct SuccessResponse {
    pub code: i32,
    pub msg: String,
}

/// 倒计时响应
#[derive(Debug, Clone)]
pub struct CountdownResponse {
    pub symbol: String,
    pub countdown_time: i64,
}

// ============================================================================
// 错误类型定义
// ============================================================================

/// Trade 命令错误
#[derive(Debug, Clone)]
pub enum UsdsMFutureTradeCmdError {
    /// 参数验证错误
    InvalidParameter(String),
    /// 网络错误
    NetworkError(String),
    /// API错误
    ApiError { code: i32, msg: String },
    /// 签名错误
    SignatureError(String),
    /// 权限不足
    PermissionDenied(String),
    /// 未知错误
    Unknown(String),
}

// ============================================================================
// Trade 行为接口
// ============================================================================

/// USDS-M期货交易行为接口
pub trait UsdsMFutureTradeBehavior: Send + Sync {
    /// 处理交易命令
    fn handle(
        &mut self,
        cmd: UsdsMFutureTradeCmdAny,
    ) -> Result<CmdResp<UsdsMFutureTradeRes>, UsdsMFutureTradeCmdError>;
}
