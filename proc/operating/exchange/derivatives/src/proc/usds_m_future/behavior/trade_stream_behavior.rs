// 参考 v1 stream Endpoints /Users/hongyaotang/src/rustlob/design/other/binance_derivatives_api/usds-margined-futures/v1/websocket-api 定义所有 v1 stream 接口

use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};

// ============================================================================
// 枚举类型定义（复用部分trade_behavior.rs的类型）
// ============================================================================

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrderSide {
    Buy,
    Sell,
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

/// 有效时间类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimeInForce {
    /// Good Till Cancel - 成交为止
    GTC,
    /// Immediate or Cancel - 无法立即成交的部分就撤销
    IOC,
    /// Fill or Kill - 无法全部立即成交就撤销
    FOK,
    /// Good Till Crossing - 无法成为挂单方就撤销
    GTX,
    /// Good Till Date - 在特定时间之前有效
    GTD,
}

/// 持仓方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PositionSide {
    /// 单向持仓
    Both,
    /// 多头
    Long,
    /// 空头
    Short,
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
    ExpiredInMatch,
}

/// 工作类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

/// 响应类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResponseType {
    Ack,
    Result,
}

/// 自成交防护模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelfTradePreventionMode {
    None,
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
}

/// 价格匹配模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// 保证金类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MarginType {
    Isolated,
    Crossed,
}

/// 比率类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RateLimitType {
    Orders,
    RequestWeight,
}

/// 比率间隔
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RateLimitInterval {
    Second,
    Minute,
    Day,
}

// ============================================================================
// WebSocket Trade Stream 命令枚举
// ============================================================================

/// USDS-M期货WebSocket交易流命令枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureTradeStreamCmdAny {
    /// 下单 order.place
    /// Request Weight: 0
    OrderPlace(OrderPlaceWsCmd),

    /// 撤销订单 order.cancel
    /// Request Weight: 1
    OrderCancel(OrderCancelWsCmd),

    /// 修改订单 order.modify (仅支持LIMIT订单)
    /// Request Weight: 1
    OrderModify(OrderModifyWsCmd),

    /// 查询订单 order.status
    /// Request Weight: 1
    OrderStatus(OrderStatusWsCmd),

    /// 查询持仓 account.position
    /// Request Weight: 5
    AccountPosition(AccountPositionWsCmd),
}

// ============================================================================
// WebSocket 下单命令
// ============================================================================

/// WebSocket下单命令
/// Method: order.place
/// Request Weight: 0
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderPlaceWsCmd {
    pub metadata: CMetadata,
    /// WebSocket请求ID
    pub request_id: String,
    /// 交易对
    pub symbol: String,
    /// 订单方向
    pub side: OrderSide,
    /// 持仓方向
    pub position_side: Option<PositionSide>,
    /// 订单类型
    pub order_type: OrderType,
    /// 有效时间类型
    pub time_in_force: Option<TimeInForce>,
    /// 数量
    pub quantity: Option<String>,
    /// 价格
    pub price: Option<String>,
    /// 用户自定义的订单号
    pub new_client_order_id: Option<String>,
    /// 止损价格
    pub stop_price: Option<String>,
    /// 激活价格，仅TRAILING_STOP_MARKET订单使用
    pub activation_price: Option<String>,
    /// 回调比率，仅TRAILING_STOP_MARKET订单使用
    pub callback_rate: Option<String>,
    /// 工作类型
    pub working_type: Option<WorkingType>,
    /// 价格保护
    pub price_protect: Option<bool>,
    /// 响应类型
    pub new_order_resp_type: Option<ResponseType>,
    /// 只减仓
    pub reduce_only: Option<bool>,
    /// 全平仓
    pub close_position: Option<bool>,
    /// 自成交防护模式
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    /// GTD订单自动取消时间
    pub good_till_date: Option<i64>,
    /// 接收窗口（微秒精度）
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
    /// 签名
    pub signature: String,
}

// ============================================================================
// WebSocket 撤销订单命令
// ============================================================================

/// WebSocket撤销订单命令
/// Method: order.cancel
/// Request Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderCancelWsCmd {
    pub metadata: CMetadata,
    /// WebSocket请求ID
    pub request_id: String,
    /// 交易对
    pub symbol: String,
    /// 系统订单号
    pub order_id: Option<i64>,
    /// 用户自定义的订单号
    pub orig_client_order_id: Option<String>,
    /// 接收窗口（微秒精度）
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
    /// 签名
    pub signature: String,
}

// ============================================================================
// WebSocket 修改订单命令
// ============================================================================

/// WebSocket修改订单命令（仅支持LIMIT订单）
/// Method: order.modify
/// Request Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderModifyWsCmd {
    pub metadata: CMetadata,
    /// WebSocket请求ID
    pub request_id: String,
    /// 交易对
    pub symbol: String,
    /// 订单方向
    pub side: OrderSide,
    /// 修改数量
    pub quantity: String,
    /// 修改价格
    pub price: String,
    /// 系统订单号
    pub order_id: Option<i64>,
    /// 用户自定义的订单号
    pub orig_client_order_id: Option<String>,
    /// 价格匹配模式
    pub price_match: Option<PriceMatchMode>,
    /// 接收窗口（微秒精度）
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
    /// 签名
    pub signature: String,
}

// ============================================================================
// WebSocket 查询订单命令
// ============================================================================

/// WebSocket查询订单命令
/// Method: order.status
/// Request Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderStatusWsCmd {
    pub metadata: CMetadata,
    /// WebSocket请求ID
    pub request_id: String,
    /// 交易对
    pub symbol: String,
    /// 系统订单号
    pub order_id: Option<i64>,
    /// 用户自定义的订单号
    pub orig_client_order_id: Option<String>,
    /// 接收窗口（微秒精度）
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
    /// 签名
    pub signature: String,
}

// ============================================================================
// WebSocket 查询持仓命令
// ============================================================================

/// WebSocket查询持仓命令
/// Method: account.position
/// Request Weight: 5
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountPositionWsCmd {
    pub metadata: CMetadata,
    /// WebSocket请求ID
    pub request_id: String,
    /// 交易对（可选，不填返回所有持仓）
    pub symbol: Option<String>,
    /// 接收窗口（微秒精度）
    pub recv_window: Option<i64>,
    /// 时间戳
    pub timestamp: i64,
    /// 签名
    pub signature: String,
}

// ============================================================================
// 响应类型定义
// ============================================================================

/// WebSocket Trade Stream 响应枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureTradeStreamRes {
    /// 下单响应
    OrderPlace(OrderPlaceWsResponse),
    /// 撤销订单响应
    OrderCancel(OrderCancelWsResponse),
    /// 修改订单响应
    OrderModify(OrderModifyWsResponse),
    /// 查询订单响应
    OrderStatus(OrderStatusWsResponse),
    /// 查询持仓响应
    AccountPosition(AccountPositionWsResponse),
}

/// 比率限制信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RateLimit {
    /// 比率限制类型
    pub rate_limit_type: RateLimitType,
    /// 间隔
    pub interval: RateLimitInterval,
    /// 间隔数量
    pub interval_num: i32,
    /// 限制值
    pub limit: i32,
    /// 当前使用量
    pub count: i32,
}

/// 下单响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderPlaceWsResponse {
    /// 请求ID
    pub id: String,
    /// 响应状态码（200表示成功）
    pub status: i32,
    /// 订单信息
    pub result: OrderInfo,
    /// 比率限制
    pub rate_limits: Vec<RateLimit>,
}

/// 订单信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderInfo {
    /// 客户端订单ID
    pub client_order_id: String,
    /// 累计成交数量
    pub cum_qty: String,
    /// 累计成交金额
    pub cum_quote: String,
    /// 执行类型
    pub exec_type: String,
    /// 订单ID
    pub order_id: i64,
    /// 平均价格
    pub avg_price: String,
    /// 原始数量
    pub orig_qty: String,
    /// 价格
    pub price: String,
    /// 只减仓
    pub reduce_only: bool,
    /// 订单方向
    pub side: OrderSide,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 订单状态
    pub status: OrderStatus,
    /// 止损价格
    pub stop_price: Option<String>,
    /// 全平仓
    pub close_position: bool,
    /// 交易对
    pub symbol: String,
    /// 有效时间类型
    pub time_in_force: TimeInForce,
    /// 订单类型
    pub order_type: OrderType,
    /// 原始订单类型
    pub orig_type: OrderType,
    /// 激活价格（TRAILING_STOP_MARKET）
    pub activate_price: Option<String>,
    /// 价格比率（TRAILING_STOP_MARKET）
    pub price_rate: Option<String>,
    /// 订单创建时间
    pub update_time: i64,
    /// 工作类型
    pub working_type: WorkingType,
    /// 价格保护
    pub price_protect: bool,
    /// 价格匹配模式
    pub price_match: Option<PriceMatchMode>,
    /// 自成交防护模式
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    /// GTD订单自动取消时间
    pub good_till_date: i64,
}

/// 撤销订单响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderCancelWsResponse {
    /// 请求ID
    pub id: String,
    /// 响应状态码（200表示成功）
    pub status: i32,
    /// 订单信息
    pub result: OrderInfo,
    /// 比率限制
    pub rate_limits: Vec<RateLimit>,
}

/// 修改订单响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderModifyWsResponse {
    /// 请求ID
    pub id: String,
    /// 响应状态码（200表示成功）
    pub status: i32,
    /// 订单信息
    pub result: OrderInfo,
    /// 比率限制
    pub rate_limits: Vec<RateLimit>,
}

/// 查询订单响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderStatusWsResponse {
    /// 请求ID
    pub id: String,
    /// 响应状态码（200表示成功）
    pub status: i32,
    /// 订单信息
    pub result: OrderInfo,
    /// 比率限制
    pub rate_limits: Vec<RateLimit>,
}

/// 持仓信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PositionInfo {
    /// 入场价格
    pub entry_price: String,
    /// 保证金类型
    pub margin_type: MarginType,
    /// 是否自动追加保证金
    pub is_auto_add_margin: bool,
    /// 逐仓保证金
    pub isolated_margin: String,
    /// 杠杆倍数
    pub leverage: String,
    /// 强平价格
    pub liquidation_price: String,
    /// 标记价格
    pub mark_price: String,
    /// 最大名义价值
    pub max_notional_value: String,
    /// 持仓数量
    pub position_amt: String,
    /// 名义价值
    pub notional: String,
    /// 逐仓钱包余额
    pub isolated_wallet: String,
    /// 交易对
    pub symbol: String,
    /// 未实现盈亏
    pub un_realized_profit: String,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 最近一次更新时间
    pub update_time: i64,
}

/// 查询持仓响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountPositionWsResponse {
    /// 请求ID
    pub id: String,
    /// 响应状态码（200表示成功）
    pub status: i32,
    /// 持仓列表
    pub result: Vec<PositionInfo>,
    /// 比率限制
    pub rate_limits: Vec<RateLimit>,
}

// ============================================================================
// 错误类型定义
// ============================================================================

/// WebSocket Trade Stream 命令错误
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureTradeStreamCmdError {
    /// 无效的交易对
    InvalidSymbol(String),
    /// 无效的订单ID
    InvalidOrderId(String),
    /// 订单不存在
    OrderNotFound(String),
    /// 无效的参数
    InvalidParameter(String),
    /// 签名验证失败
    SignatureError(String),
    /// API Key无效
    InvalidApiKey(String),
    /// 权限不足
    PermissionDenied(String),
    /// 余额不足
    InsufficientBalance(String),
    /// 订单类型不支持修改
    OrderTypeNotModifiable(String),
    /// WebSocket连接错误
    ConnectionError(String),
    /// 序列化/反序列化错误
    SerializationError(String),
    /// 比率限制超出
    RateLimitExceeded { limit_type: String, retry_after: i64 },
    /// API错误
    ApiError { code: i32, msg: String },
    /// 未知错误
    Unknown(String),
}

// ============================================================================
// WebSocket Trade Stream 行为接口
// ============================================================================

/// USDS-M期货WebSocket交易流行为接口
pub trait UsdsMFutureTradeStreamBehavior: Send + Sync {
    /// 处理WebSocket交易流命令
    fn handle(
        &mut self,
        cmd: UsdsMFutureTradeStreamCmdAny,
    ) -> Result<CmdResp<UsdsMFutureTradeStreamRes>, UsdsMFutureTradeStreamCmdError>;
}

// ============================================================================
// 辅助工具
// ============================================================================

/// WebSocket消息构建器
pub struct WsMessageBuilder;

impl WsMessageBuilder {
    /// 构建下单消息
    pub fn build_order_place(cmd: &OrderPlaceWsCmd) -> String {
        // 实际实现需要序列化为JSON格式
        format!(
            r#"{{"id":"{}","method":"order.place","params":{{"symbol":"{}","side":"{}","type":"{}","timestamp":{}}}}}"#,
            cmd.request_id,
            cmd.symbol,
            format!("{:?}", cmd.side).to_uppercase(),
            format!("{:?}", cmd.order_type).to_uppercase(),
            cmd.timestamp
        )
    }

    /// 构建撤销订单消息
    pub fn build_order_cancel(cmd: &OrderCancelWsCmd) -> String {
        format!(
            r#"{{"id":"{}","method":"order.cancel","params":{{"symbol":"{}","timestamp":{}}}}}"#,
            cmd.request_id, cmd.symbol, cmd.timestamp
        )
    }

    /// 构建修改订单消息
    pub fn build_order_modify(cmd: &OrderModifyWsCmd) -> String {
        format!(
            r#"{{"id":"{}","method":"order.modify","params":{{"symbol":"{}","side":"{}","quantity":"{}","price":"{}","timestamp":{}}}}}"#,
            cmd.request_id,
            cmd.symbol,
            format!("{:?}", cmd.side).to_uppercase(),
            cmd.quantity,
            cmd.price,
            cmd.timestamp
        )
    }

    /// 构建查询订单消息
    pub fn build_order_status(cmd: &OrderStatusWsCmd) -> String {
        format!(
            r#"{{"id":"{}","method":"order.status","params":{{"symbol":"{}","timestamp":{}}}}}"#,
            cmd.request_id, cmd.symbol, cmd.timestamp
        )
    }

    /// 构建查询持仓消息
    pub fn build_account_position(cmd: &AccountPositionWsCmd) -> String {
        if let Some(ref symbol) = cmd.symbol {
            format!(
                r#"{{"id":"{}","method":"account.position","params":{{"symbol":"{}","timestamp":{}}}}}"#,
                cmd.request_id, symbol, cmd.timestamp
            )
        } else {
            format!(
                r#"{{"id":"{}","method":"account.position","params":{{"timestamp":{}}}}}"#,
                cmd.request_id, cmd.timestamp
            )
        }
    }
}

/// WebSocket URL构建器
pub struct TradeStreamUrlBuilder;

impl TradeStreamUrlBuilder {
    /// 构建交易流WebSocket URL
    /// wss://fstream.binance.com/ws-fapi/v1
    pub fn build() -> String {
        "wss://fstream.binance.com/ws-fapi/v1".to_string()
    }

    /// 构建测试网交易流WebSocket URL
    /// wss://stream.binancefuture.com/ws-fapi/v1
    pub fn build_testnet() -> String {
        "wss://stream.binancefuture.com/ws-fapi/v1".to_string()
    }
}

/// 签名生成器接口
pub trait SignatureGenerator: Send + Sync {
    /// 生成HMAC SHA256签名
    /// 参数格式: symbol=BTCUSDT&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=9000&recvWindow=5000&timestamp=1591702613943
    fn generate(&self, query_string: &str, secret_key: &str) -> String;
}
