// 参考 ## market data stream Endpoints /Users/hongyaotang/src/rustlob/design/other/binance_derivatives_api/usds-margined-futures/websocket-market-streams 定义所有 market data stream 接口

use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};

// ============================================================================
// 枚举类型定义
// ============================================================================

/// WebSocket更新速度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UpdateSpeed {
    /// 100毫秒
    Ms100,
    /// 250毫秒
    Ms250,
    /// 500毫秒
    Ms500,
    /// 1秒
    S1,
    /// 3秒
    S3,
}

/// 深度档位级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DepthLevel {
    /// 5档
    Level5,
    /// 10档
    Level10,
    /// 20档
    Level20,
}

/// K线时间间隔（复用market_data_behavior.rs中的定义）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KlineInterval {
    M1,  // 1分钟
    M3,  // 3分钟
    M5,  // 5分钟
    M15, // 15分钟
    M30, // 30分钟
    H1,  // 1小时
    H2,  // 2小时
    H4,  // 4小时
    H6,  // 6小时
    H8,  // 8小时
    H12, // 12小时
    D1,  // 1天
    D3,  // 3天
    W1,  // 1周
    MO1, // 1月
}

/// 持续合约类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContractType {
    Perpetual,
    CurrentQuarter,
    NextQuarter,
}

/// WebSocket方法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WsMethod {
    /// 订阅流
    Subscribe,
    /// 取消订阅
    Unsubscribe,
    /// 列出订阅
    ListSubscriptions,
    /// 设置属性
    SetProperty,
    /// 获取属性
    GetProperty,
}

// ============================================================================
// Market Data Stream 命令枚举
// ============================================================================

/// USDS-M期货市场数据流命令枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureMarketDataStreamCmdAny {
    // ========== 订阅管理 ==========
    /// 订阅流
    Subscribe(SubscribeCmd),

    /// 取消订阅流
    Unsubscribe(UnsubscribeCmd),

    /// 列出当前订阅
    ListSubscriptions(ListSubscriptionsCmd),

    /// 设置属性
    SetProperty(SetPropertyCmd),

    /// 获取属性
    GetProperty(GetPropertyCmd),

    // ========== 流事件（接收） ==========
    /// 归集成交流事件
    AggTradeStreamEvent(AggTradeStreamEvent),

    /// 标记价格流事件
    MarkPriceStreamEvent(MarkPriceStreamEvent),

    /// 全市场标记价格流事件
    MarkPriceAllMarketStreamEvent(MarkPriceAllMarketStreamEvent),

    /// K线流事件
    KlineStreamEvent(KlineStreamEvent),

    /// 持续合约K线流事件
    ContinuousContractKlineStreamEvent(ContinuousContractKlineStreamEvent),

    /// 个股Ticker流事件
    IndividualSymbolTickerStreamEvent(IndividualSymbolTickerStreamEvent),

    /// 全市场Ticker流事件
    AllMarketTickersStreamEvent(AllMarketTickersStreamEvent),

    /// 个股MiniTicker流事件
    IndividualSymbolMiniTickerStreamEvent(IndividualSymbolMiniTickerStreamEvent),

    /// 全市场MiniTicker流事件
    AllMarketMiniTickersStreamEvent(AllMarketMiniTickersStreamEvent),

    /// 个股最优挂单流事件
    IndividualSymbolBookTickerStreamEvent(IndividualSymbolBookTickerStreamEvent),

    /// 全市场最优挂单流事件
    AllBookTickersStreamEvent(AllBookTickersStreamEvent),

    /// 强平订单流事件
    LiquidationOrderStreamEvent(LiquidationOrderStreamEvent),

    /// 全市场强平订单流事件
    AllMarketLiquidationOrderStreamEvent(AllMarketLiquidationOrderStreamEvent),

    /// 局部深度流事件
    PartialBookDepthStreamEvent(PartialBookDepthStreamEvent),

    /// 增量深度流事件
    DiffBookDepthStreamEvent(DiffBookDepthStreamEvent),

    /// 复合指数流事件
    CompositeIndexStreamEvent(CompositeIndexStreamEvent),

    /// 合约信息流事件
    ContractInfoStreamEvent(ContractInfoStreamEvent),

    /// 多资产模式资产指数流事件
    AssetIndexStreamEvent(AssetIndexStreamEvent),
}

// ============================================================================
// 订阅管理命令
// ============================================================================

/// 订阅流命令
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscribeCmd {
    pub metadata: CMetadata,
    /// 订阅的流名称列表
    pub streams: Vec<String>,
    /// 请求ID
    pub id: u64,
}

/// 取消订阅流命令
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnsubscribeCmd {
    pub metadata: CMetadata,
    /// 取消订阅的流名称列表
    pub streams: Vec<String>,
    /// 请求ID
    pub id: u64,
}

/// 列出订阅命令
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListSubscriptionsCmd {
    pub metadata: CMetadata,
    /// 请求ID
    pub id: u64,
}

/// 设置属性命令
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPropertyCmd {
    pub metadata: CMetadata,
    /// 属性名称（如"combined"）
    pub property: String,
    /// 属性值
    pub value: bool,
    /// 请求ID
    pub id: u64,
}

/// 获取属性命令
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyCmd {
    pub metadata: CMetadata,
    /// 属性名称
    pub property: String,
    /// 请求ID
    pub id: u64,
}

// ============================================================================
// 流事件定义（接收的推送数据）
// ============================================================================

/// 归集成交流事件
/// Stream: <symbol>@aggTrade
/// Update Speed: 100ms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AggTradeStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 归集成交ID
    pub agg_trade_id: i64,
    /// 价格
    pub price: String,
    /// 数量
    pub quantity: String,
    /// 第一笔成交ID
    pub first_trade_id: i64,
    /// 最后一笔成交ID
    pub last_trade_id: i64,
    /// 成交时间
    pub trade_time: i64,
    /// 买方是否为做市方
    pub is_buyer_maker: bool,
}

/// 标记价格流事件
/// Stream: <symbol>@markPrice 或 <symbol>@markPrice@1s
/// Update Speed: 3s 或 1s
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkPriceStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 标记价格
    pub mark_price: String,
    /// 指数价格
    pub index_price: String,
    /// 预估结算价
    pub estimated_settle_price: String,
    /// 资金费率
    pub funding_rate: String,
    /// 下次资金费时间
    pub next_funding_time: i64,
}

/// 全市场标记价格流事件
/// Stream: !markPrice@arr 或 !markPrice@arr@1s
/// Update Speed: 3s 或 1s
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkPriceAllMarketStreamEvent {
    pub metadata: CMetadata,
    /// 标记价格列表
    pub data: Vec<MarkPriceData>,
}

/// 标记价格数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkPriceData {
    pub event_type: String,
    pub event_time: i64,
    pub symbol: String,
    pub mark_price: String,
    pub index_price: String,
    pub estimated_settle_price: String,
    pub funding_rate: String,
    pub next_funding_time: i64,
}

/// K线流事件
/// Stream: <symbol>@kline_<interval>
/// Update Speed: 250ms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KlineStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// K线数据
    pub kline: KlineData,
}

/// K线数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KlineData {
    /// K线开始时间
    pub start_time: i64,
    /// K线结束时间
    pub close_time: i64,
    /// 交易对
    pub symbol: String,
    /// 时间间隔
    pub interval: String,
    /// 第一笔成交ID
    pub first_trade_id: i64,
    /// 最后一笔成交ID
    pub last_trade_id: i64,
    /// 开盘价
    pub open: String,
    /// 收盘价
    pub close: String,
    /// 最高价
    pub high: String,
    /// 最低价
    pub low: String,
    /// 成交量
    pub volume: String,
    /// 成交笔数
    pub number_of_trades: i64,
    /// K线是否完结
    pub is_closed: bool,
    /// 成交额
    pub quote_volume: String,
    /// 主动买入成交量
    pub taker_buy_volume: String,
    /// 主动买入成交额
    pub taker_buy_quote_volume: String,
    /// 忽略
    pub ignore: String,
}

/// 持续合约K线流事件
/// Stream: <pair>_<contractType>@continuousKline_<interval>
/// Update Speed: 250ms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContinuousContractKlineStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub pair: String,
    /// 合约类型
    pub contract_type: String,
    /// K线数据
    pub kline: KlineData,
}

/// 个股Ticker流事件
/// Stream: <symbol>@ticker
/// Update Speed: 500ms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndividualSymbolTickerStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 24小时价格变化
    pub price_change: String,
    /// 24小时价格变化百分比
    pub price_change_percent: String,
    /// 加权平均价
    pub weighted_avg_price: String,
    /// 最新价格
    pub last_price: String,
    /// 最新数量
    pub last_quantity: String,
    /// 开盘价
    pub open_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 成交量
    pub total_volume: String,
    /// 成交额
    pub total_quote_volume: String,
    /// 统计开始时间
    pub open_time: i64,
    /// 统计结束时间
    pub close_time: i64,
    /// 第一笔成交ID
    pub first_trade_id: i64,
    /// 最后一笔成交ID
    pub last_trade_id: i64,
    /// 成交笔数
    pub total_trades: i64,
}

/// 全市场Ticker流事件
/// Stream: !ticker@arr
/// Update Speed: 500ms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllMarketTickersStreamEvent {
    pub metadata: CMetadata,
    /// Ticker列表
    pub data: Vec<TickerData>,
}

/// Ticker数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TickerData {
    pub event_type: String,
    pub event_time: i64,
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub last_price: String,
    pub last_quantity: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub total_volume: String,
    pub total_quote_volume: String,
    pub open_time: i64,
    pub close_time: i64,
    pub first_trade_id: i64,
    pub last_trade_id: i64,
    pub total_trades: i64,
}

/// 个股MiniTicker流事件
/// Stream: <symbol>@miniTicker
/// Update Speed: 500ms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndividualSymbolMiniTickerStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 收盘价
    pub close_price: String,
    /// 开盘价
    pub open_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 成交量
    pub total_volume: String,
    /// 成交额
    pub total_quote_volume: String,
}

/// 全市场MiniTicker流事件
/// Stream: !miniTicker@arr
/// Update Speed: 1s
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllMarketMiniTickersStreamEvent {
    pub metadata: CMetadata,
    /// MiniTicker列表
    pub data: Vec<MiniTickerData>,
}

/// MiniTicker数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MiniTickerData {
    pub event_type: String,
    pub event_time: i64,
    pub symbol: String,
    pub close_price: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub total_volume: String,
    pub total_quote_volume: String,
}

/// 个股最优挂单流事件
/// Stream: <symbol>@bookTicker
/// Update Speed: Real-time
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndividualSymbolBookTickerStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 订单簿更新ID
    pub update_id: i64,
    /// 交易对
    pub symbol: String,
    /// 最优买单价
    pub best_bid_price: String,
    /// 最优买单量
    pub best_bid_quantity: String,
    /// 最优卖单价
    pub best_ask_price: String,
    /// 最优卖单量
    pub best_ask_quantity: String,
    /// 事务时间
    pub transaction_time: i64,
    /// 事件时间
    pub event_time: i64,
}

/// 全市场最优挂单流事件
/// Stream: !bookTicker
/// Update Speed: Real-time
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllBookTickersStreamEvent {
    pub metadata: CMetadata,
    /// BookTicker数据
    pub data: BookTickerData,
}

/// BookTicker数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BookTickerData {
    pub event_type: String,
    pub update_id: i64,
    pub symbol: String,
    pub best_bid_price: String,
    pub best_bid_quantity: String,
    pub best_ask_price: String,
    pub best_ask_quantity: String,
    pub transaction_time: i64,
    pub event_time: i64,
}

/// 强平订单流事件
/// Stream: <symbol>@forceOrder
/// Update Speed: Real-time
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidationOrderStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 强平订单信息
    pub order: LiquidationOrderData,
}

/// 强平订单数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidationOrderData {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub time_in_force: String,
    pub original_quantity: String,
    pub price: String,
    pub average_price: String,
    pub order_status: String,
    pub last_filled_quantity: String,
    pub filled_accumulated_quantity: String,
    pub trade_time: i64,
}

/// 全市场强平订单流事件
/// Stream: !forceOrder@arr
/// Update Speed: Real-time
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllMarketLiquidationOrderStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 强平订单信息
    pub order: LiquidationOrderData,
}

/// 局部深度流事件
/// Stream: <symbol>@depth<levels> 或 <symbol>@depth<levels>@500ms 或 <symbol>@depth<levels>@100ms
/// Update Speed: 250ms, 500ms, 100ms
/// Levels: 5, 10, 20
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartialBookDepthStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易时间
    pub transaction_time: i64,
    /// 交易对
    pub symbol: String,
    /// 最后更新ID
    pub last_update_id: i64,
    /// 买单
    pub bids: Vec<PriceLevel>,
    /// 卖单
    pub asks: Vec<PriceLevel>,
}

/// 增量深度流事件
/// Stream: <symbol>@depth 或 <symbol>@depth@500ms 或 <symbol>@depth@100ms
/// Update Speed: 250ms, 500ms, 100ms
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DiffBookDepthStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易时间
    pub transaction_time: i64,
    /// 交易对
    pub symbol: String,
    /// 从上次推送至今新增的第一个update Id
    pub first_update_id: i64,
    /// 从上次推送至今新增的最后一个update Id
    pub final_update_id: i64,
    /// 上次推送的最后一个update Id
    pub previous_final_update_id: i64,
    /// 买单更新
    pub bids: Vec<PriceLevel>,
    /// 卖单更新
    pub asks: Vec<PriceLevel>,
}

/// 价格档位
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLevel {
    pub price: String,
    pub quantity: String,
}

/// 复合指数流事件
/// Stream: <symbol>@compositeIndex
/// Update Speed: 1s
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompositeIndexStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 价格
    pub price: String,
    /// 成分
    pub composition: Vec<CompositeIndexComponent>,
}

/// 复合指数成分
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompositeIndexComponent {
    pub base_asset: String,
    pub quote_asset: String,
    pub weight_in_quantity: String,
    pub weight_in_percentage: String,
}

/// 合约信息流事件
/// Stream: !contractInfo
/// Update Speed: 3s
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContractInfoStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 交易对对
    pub pair: String,
    /// 合约类型
    pub contract_type: String,
    /// 交割时间
    pub delivery_date: i64,
    /// 上线时间
    pub onboard_date: i64,
    /// 合约状态
    pub contract_status: String,
    /// 合约大小
    pub contract_size: i64,
}

/// 多资产模式资产指数流事件
/// Stream: <symbol>@assetIndex 或 <symbol>@assetIndex@1s
/// Update Speed: 3s 或 1s
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetIndexStreamEvent {
    pub metadata: CMetadata,
    /// 事件类型
    pub event_type: String,
    /// 事件时间
    pub event_time: i64,
    /// 资产
    pub asset: String,
    /// 资产指数价格
    pub asset_index_price: String,
    /// 买方缓冲
    pub bid_buffer: String,
    /// 卖方缓冲
    pub ask_buffer: String,
    /// 买方汇率
    pub bid_rate: String,
    /// 卖方汇率
    pub ask_rate: String,
    /// 自动兑换买方缓冲
    pub auto_exchange_bid_buffer: String,
    /// 自动兑换卖方缓冲
    pub auto_exchange_ask_buffer: String,
    /// 自动兑换买方汇率
    pub auto_exchange_bid_rate: String,
    /// 自动兑换卖方汇率
    pub auto_exchange_ask_rate: String,
}

// ============================================================================
// 响应类型定义
// ============================================================================

/// Market Data Stream 响应枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureMarketDataStreamRes {
    /// 订阅响应
    SubscribeResponse(SubscribeResponse),
    /// 取消订阅响应
    UnsubscribeResponse(UnsubscribeResponse),
    /// 列出订阅响应
    ListSubscriptionsResponse(ListSubscriptionsResponse),
    /// 设置属性响应
    SetPropertyResponse(SetPropertyResponse),
    /// 获取属性响应
    GetPropertyResponse(GetPropertyResponse),
    /// 流事件已处理
    StreamEventProcessed,
}

/// 订阅响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscribeResponse {
    pub result: Option<String>,
    pub id: u64,
}

/// 取消订阅响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnsubscribeResponse {
    pub result: Option<String>,
    pub id: u64,
}

/// 列出订阅响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListSubscriptionsResponse {
    pub result: Vec<String>,
    pub id: u64,
}

/// 设置属性响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetPropertyResponse {
    pub result: Option<String>,
    pub id: u64,
}

/// 获取属性响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetPropertyResponse {
    pub result: bool,
    pub id: u64,
}

// ============================================================================
// 错误类型定义
// ============================================================================

/// Market Data Stream 命令错误
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureMarketDataStreamCmdError {
    /// 未知属性
    UnknownProperty { code: i32, msg: String },
    /// 无效值类型
    InvalidValueType { code: i32, msg: String },
    /// 无效请求
    InvalidRequest { code: i32, msg: String },
    /// 无效JSON
    InvalidJson { code: i32, msg: String },
    /// WebSocket连接错误
    ConnectionError(String),
    /// 序列化/反序列化错误
    SerializationError(String),
    /// 未知错误
    Unknown(String),
}

// ============================================================================
// Market Data Stream 行为接口
// ============================================================================

/// USDS-M期货市场数据流行为接口
pub trait UsdsMFutureMarketDataSSEBehavior: Send + Sync {
    /// 处理市场数据流命令
    fn handle(
        &mut self,
        cmd: UsdsMFutureMarketDataStreamCmdAny,
    ) -> Result<CmdResp<UsdsMFutureMarketDataStreamRes>, UsdsMFutureMarketDataStreamCmdError>;
}

// ============================================================================
// 流名称构建器（辅助工具）
// ============================================================================

/// 流名称构建器 - 用于生成标准的流名称
pub struct StreamNameBuilder;

impl StreamNameBuilder {
    /// 归集成交流: <symbol>@aggTrade
    pub fn agg_trade(symbol: &str) -> String {
        format!("{}@aggTrade", symbol.to_lowercase())
    }

    /// 标记价格流: <symbol>@markPrice 或 <symbol>@markPrice@1s
    pub fn mark_price(symbol: &str, fast_update: bool) -> String {
        if fast_update {
            format!("{}@markPrice@1s", symbol.to_lowercase())
        } else {
            format!("{}@markPrice", symbol.to_lowercase())
        }
    }

    /// 全市场标记价格流: !markPrice@arr 或 !markPrice@arr@1s
    pub fn mark_price_all(fast_update: bool) -> String {
        if fast_update { "!markPrice@arr@1s".to_string() } else { "!markPrice@arr".to_string() }
    }

    /// K线流: <symbol>@kline_<interval>
    pub fn kline(symbol: &str, interval: &str) -> String {
        format!("{}@kline_{}", symbol.to_lowercase(), interval)
    }

    /// 持续合约K线流: <pair>_<contractType>@continuousKline_<interval>
    pub fn continuous_kline(pair: &str, contract_type: &str, interval: &str) -> String {
        format!("{}_{}_@continuousKline_{}", pair.to_lowercase(), contract_type, interval)
    }

    /// 个股Ticker流: <symbol>@ticker
    pub fn ticker(symbol: &str) -> String {
        format!("{}@ticker", symbol.to_lowercase())
    }

    /// 全市场Ticker流: !ticker@arr
    pub fn ticker_all() -> String {
        "!ticker@arr".to_string()
    }

    /// 个股MiniTicker流: <symbol>@miniTicker
    pub fn mini_ticker(symbol: &str) -> String {
        format!("{}@miniTicker", symbol.to_lowercase())
    }

    /// 全市场MiniTicker流: !miniTicker@arr
    pub fn mini_ticker_all() -> String {
        "!miniTicker@arr".to_string()
    }

    /// 个股最优挂单流: <symbol>@bookTicker
    pub fn book_ticker(symbol: &str) -> String {
        format!("{}@bookTicker", symbol.to_lowercase())
    }

    /// 全市场最优挂单流: !bookTicker
    pub fn book_ticker_all() -> String {
        "!bookTicker".to_string()
    }

    /// 强平订单流: <symbol>@forceOrder
    pub fn force_order(symbol: &str) -> String {
        format!("{}@forceOrder", symbol.to_lowercase())
    }

    /// 全市场强平订单流: !forceOrder@arr
    pub fn force_order_all() -> String {
        "!forceOrder@arr".to_string()
    }

    /// 局部深度流: <symbol>@depth<levels> 或 <symbol>@depth<levels>@<speed>
    pub fn partial_depth(symbol: &str, levels: u8, speed: Option<&str>) -> String {
        if let Some(s) = speed {
            format!("{}@depth{}@{}", symbol.to_lowercase(), levels, s)
        } else {
            format!("{}@depth{}", symbol.to_lowercase(), levels)
        }
    }

    /// 增量深度流: <symbol>@depth 或 <symbol>@depth@<speed>
    pub fn diff_depth(symbol: &str, speed: Option<&str>) -> String {
        if let Some(s) = speed {
            format!("{}@depth@{}", symbol.to_lowercase(), s)
        } else {
            format!("{}@depth", symbol.to_lowercase())
        }
    }

    /// 复合指数流: <symbol>@compositeIndex
    pub fn composite_index(symbol: &str) -> String {
        format!("{}@compositeIndex", symbol.to_lowercase())
    }

    /// 合约信息流: !contractInfo
    pub fn contract_info() -> String {
        "!contractInfo".to_string()
    }

    /// 资产指数流: <symbol>@assetIndex 或 <symbol>@assetIndex@1s
    pub fn asset_index(symbol: &str, fast_update: bool) -> String {
        if fast_update {
            format!("{}@assetIndex@1s", symbol.to_lowercase())
        } else {
            format!("{}@assetIndex", symbol.to_lowercase())
        }
    }
}
