// Binance Option WebSocket Market Streams
// Base URL: wss://nbstream.binance.com/eoptions/
// 根据 /Users/hongyaotang/src/rustlob/design/other/binance_derivatives_api/option/websocket-market-streams 文档定义

// use crate::proc::behavior::spot_trade_behavior::{CMetadata, CmdResp, SpotCmdErrorAny};

// ============================================================================
// Market Data Stream Events (响应事件)
// ============================================================================

use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};

/// Market Data Stream 事件枚举
#[derive(Debug, Clone)]
pub enum OptionMarketDataEvent {
    /// 标记价格事件
    MarkPrice(Vec<MarkPriceEvent>),
    /// 部分订单簿深度事件
    Depth(DepthEvent),
    /// 持仓量事件
    OpenInterest(Vec<OpenInterestEvent>),
    /// 新交易对上线事件
    NewSymbol(NewSymbolEvent),
    /// 24小时行情事件
    Ticker24hr(Ticker24hrEvent),
    /// 24小时行情（按标的和到期日）事件
    Ticker24hrByExpiry(Vec<Ticker24hrEvent>),
    /// 实时成交事件
    Trade(TradeEvent),
    /// 指数价格事件
    IndexPrice(IndexPriceEvent),
    /// K线事件
    Kline(KlineEvent),
}

/// 标记价格事件
/// Stream: <underlyingAsset>@markPrice
/// Update Speed: 1000ms
#[derive(Debug, Clone)]
pub struct MarkPriceEvent {
    /// Event Type
    pub e: String,
    /// Event time
    pub event_time: i64,
    /// Symbol
    pub symbol: String,
    /// Option mark price
    pub mark_price: String,
}

/// 部分订单簿深度事件
/// Stream: <symbol>@depth<levels> OR <symbol>@depth<levels>@100ms OR <symbol>@depth<levels>@1000ms
/// Levels: 10, 20, 50, 100
/// Update Speed: 100ms, 500ms(default), 1000ms
#[derive(Debug, Clone)]
pub struct DepthEvent {
    /// Event type
    pub e: String,
    /// Event time
    pub event_time: i64,
    /// Transaction time
    pub transaction_time: i64,
    /// Option symbol
    pub symbol: String,
    /// Update id in event
    pub update_id: u64,
    /// Previous update id (same as update_id)
    pub prev_update_id: u64,
    /// Buy orders (bids) - [price, quantity]
    pub bids: Vec<[String; 2]>,
    /// Sell orders (asks) - [price, quantity]
    pub asks: Vec<[String; 2]>,
}

/// 持仓量事件
/// Stream: <underlyingAsset>@openInterest@<expirationDate>
/// Update Speed: 60s
#[derive(Debug, Clone)]
pub struct OpenInterestEvent {
    /// Event type
    pub e: String,
    /// Event time
    pub event_time: i64,
    /// Option symbol
    pub symbol: String,
    /// Open interest in contracts
    pub open_interest_contracts: String,
    /// Open interest in USDT
    pub open_interest_usdt: String,
}

/// 新交易对上线事件
/// Stream: option_pair
/// Update Speed: 50ms
#[derive(Debug, Clone)]
pub struct NewSymbolEvent {
    /// Event type
    pub e: String,
    /// Event time
    pub event_time: i64,
    /// Underlying index of the contract
    pub underlying: String,
    /// Quotation asset
    pub quote_asset: String,
    /// Trading pair name
    pub symbol: String,
    /// Conversion ratio
    pub unit: i32,
    /// Minimum trade volume of the underlying asset
    pub min_qty: String,
    /// Option type (CALL/PUT)
    pub option_type: String,
    /// Strike price
    pub strike_price: String,
    /// Expiration time
    pub expiration_date: i64,
}

/// 24小时行情事件
/// Stream: <symbol>@ticker OR <underlyingAsset>@ticker@<expirationDate>
/// Update Speed: 1000ms
#[derive(Debug, Clone)]
pub struct Ticker24hrEvent {
    /// Event type
    pub e: String,
    /// Event time
    pub event_time: i64,
    /// Transaction time
    pub transaction_time: i64,
    /// Option symbol
    pub symbol: String,
    /// 24-hour opening price
    pub open: String,
    /// Highest price
    pub high: String,
    /// Lowest price
    pub low: String,
    /// Latest price
    pub close: String,
    /// Trading volume (in contracts)
    pub volume: String,
    /// Trade amount (in quote asset)
    pub amount: String,
    /// Price change percent
    pub price_change_percent: String,
    /// Price change
    pub price_change: String,
    /// Volume of last completed trade (in contracts)
    pub last_qty: String,
    /// First trade ID
    pub first_trade_id: String,
    /// Last trade ID
    pub last_trade_id: String,
    /// Number of trades
    pub trade_count: i64,
    /// The best buy price
    pub best_bid_price: String,
    /// The best sell price
    pub best_ask_price: String,
    /// The best buy quantity
    pub best_bid_qty: String,
    /// The best sell quantity
    pub best_ask_qty: String,
    /// Buy implied volatility
    pub bid_iv: String,
    /// Sell implied volatility
    pub ask_iv: String,
    /// Delta
    pub delta: String,
    /// Theta
    pub theta: String,
    /// Gamma
    pub gamma: String,
    /// Vega
    pub vega: String,
    /// Implied volatility
    pub implied_volatility: String,
    /// Mark price
    pub mark_price: String,
    /// Buy maximum price
    pub high_price_limit: String,
    /// Sell minimum price
    pub low_price_limit: String,
    /// Estimated strike price (return estimated strike price half hour before exercise)
    pub estimated_strike_price: String,
}

/// 实时成交事件
/// Stream: <symbol>@trade OR <underlyingAsset>@trade
/// Update Speed: 50ms
#[derive(Debug, Clone)]
pub struct TradeEvent {
    /// Event type
    pub e: String,
    /// Event time
    pub event_time: i64,
    /// Option trading symbol
    pub symbol: String,
    /// Trade ID
    pub trade_id: i64,
    /// Price
    pub price: String,
    /// Quantity
    pub quantity: String,
    /// Buy order ID
    pub buyer_order_id: i64,
    /// Sell order ID
    pub seller_order_id: i64,
    /// Trade completed time
    pub trade_time: i64,
    /// Direction
    pub side: String,
    /// Trade type enum: "MARKET" for Orderbook trading, "BLOCK" for Block trade
    pub trade_type: String,
}

/// 指数价格事件
/// Stream: <symbol>@index
/// Update Speed: 1000ms
#[derive(Debug, Clone)]
pub struct IndexPriceEvent {
    /// Event type
    pub e: String,
    /// Event time
    pub event_time: i64,
    /// Underlying symbol
    pub symbol: String,
    /// Index price
    pub index_price: String,
}

/// K线事件
/// Stream: <symbol>@kline_<interval>
/// Intervals: 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 12h, 1d, 3d, 1w
/// Update Speed: 1000ms
#[derive(Debug, Clone)]
pub struct KlineEvent {
    /// Event type
    pub e: String,
    /// Event time
    pub event_time: i64,
    /// Option trading symbol
    pub symbol: String,
    /// Kline data
    pub kline: KlineData,
}

/// K线数据
#[derive(Debug, Clone)]
pub struct KlineData {
    /// Kline start time
    pub start_time: i64,
    /// Kline end time
    pub end_time: i64,
    /// Option trading symbol
    pub symbol: String,
    /// Candle period
    pub interval: String,
    /// First trade ID
    pub first_trade_id: i64,
    /// Last trade ID
    pub last_trade_id: i64,
    /// Open
    pub open: String,
    /// Close
    pub close: String,
    /// High
    pub high: String,
    /// Low
    pub low: String,
    /// Volume (in contracts)
    pub volume: String,
    /// Number of trades
    pub trade_count: i64,
    /// Current candle has been completed Y/N
    pub is_closed: bool,
    /// Completed trade amount (in quote asset)
    pub quote_volume: String,
    /// Taker completed trade volume (in contracts)
    pub taker_buy_volume: String,
    /// Taker trade amount (in quote asset)
    pub taker_buy_quote_volume: String,
}

// ============================================================================
// Subscription Commands (订阅命令)
// ============================================================================

/// Market Data Stream 订阅命令枚举
#[derive(Debug, Clone)]
pub enum OptionMarketDataCmd {
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
}

/// 订阅命令
/// Method: SUBSCRIBE
#[derive(Debug, Clone)]
pub struct SubscribeCmd {
    pub metadata: CMetadata,
    /// Stream names to subscribe
    pub streams: Vec<String>,
    /// Request ID
    pub id: u64,
}

/// 取消订阅命令
/// Method: UNSUBSCRIBE
#[derive(Debug, Clone)]
pub struct UnsubscribeCmd {
    pub metadata: CMetadata,
    /// Stream names to unsubscribe
    pub streams: Vec<String>,
    /// Request ID
    pub id: u64,
}

/// 列出订阅命令
/// Method: LIST_SUBSCRIPTIONS
#[derive(Debug, Clone)]
pub struct ListSubscriptionsCmd {
    pub metadata: CMetadata,
    /// Request ID
    pub id: u64,
}

/// 设置属性命令
/// Method: SET_PROPERTY
/// Currently only supports "combined" property
#[derive(Debug, Clone)]
pub struct SetPropertyCmd {
    pub metadata: CMetadata,
    /// Property name (currently only "combined")
    pub property: String,
    /// Property value
    pub value: bool,
    /// Request ID
    pub id: u64,
}

/// 获取属性命令
/// Method: GET_PROPERTY
#[derive(Debug, Clone)]
pub struct GetPropertyCmd {
    pub metadata: CMetadata,
    /// Property name (currently only "combined")
    pub property: String,
    /// Request ID
    pub id: u64,
}

// ============================================================================
// Stream Subscription Helpers (流订阅辅助)
// ============================================================================

/// Stream 类型枚举
#[derive(Debug, Clone)]
pub enum StreamType {
    /// 标记价格流: <underlyingAsset>@markPrice
    MarkPrice { underlying_asset: String },
    /// 部分订单簿深度流: <symbol>@depth<levels>[@<speed>]
    Depth {
        symbol: String,
        levels: DepthLevel,
        speed: Option<UpdateSpeed>,
    },
    /// 持仓量流: <underlyingAsset>@openInterest@<expirationDate>
    OpenInterest {
        underlying_asset: String,
        expiration_date: String,
    },
    /// 新交易对流: option_pair
    NewSymbol,
    /// 24小时行情流: <symbol>@ticker
    Ticker { symbol: String },
    /// 24小时行情流（按到期日）: <underlyingAsset>@ticker@<expirationDate>
    TickerByExpiry {
        underlying_asset: String,
        expiration_date: String,
    },
    /// 实时成交流: <symbol>@trade OR <underlyingAsset>@trade
    Trade { identifier: String },
    /// 指数价格流: <symbol>@index
    IndexPrice { symbol: String },
    /// K线流: <symbol>@kline_<interval>
    Kline { symbol: String, interval: String },
}

/// 订单簿深度档位
#[derive(Debug, Clone, Copy)]
pub enum DepthLevel {
    Level10 = 10,
    Level20 = 20,
    Level50 = 50,
    Level100 = 100,
}

/// 更新速度
#[derive(Debug, Clone, Copy)]
pub enum UpdateSpeed {
    Ms100,
    Ms500,
    Ms1000,
}

impl StreamType {
    /// 转换为 stream name 字符串
    pub fn to_stream_name(&self) -> String {
        match self {
            StreamType::MarkPrice { underlying_asset } => {
                format!("{}@markPrice", underlying_asset)
            }
            StreamType::Depth {
                symbol,
                levels,
                speed,
            } => {
                let level_num = *levels as u32;
                match speed {
                    Some(UpdateSpeed::Ms100) => format!("{}@depth{}@100ms", symbol, level_num),
                    Some(UpdateSpeed::Ms1000) => format!("{}@depth{}@1000ms", symbol, level_num),
                    Some(UpdateSpeed::Ms500) | None => format!("{}@depth{}", symbol, level_num),
                }
            }
            StreamType::OpenInterest {
                underlying_asset,
                expiration_date,
            } => {
                format!("{}@openInterest@{}", underlying_asset, expiration_date)
            }
            StreamType::NewSymbol => "option_pair".to_string(),
            StreamType::Ticker { symbol } => format!("{}@ticker", symbol),
            StreamType::TickerByExpiry {
                underlying_asset,
                expiration_date,
            } => {
                format!("{}@ticker@{}", underlying_asset, expiration_date)
            }
            StreamType::Trade { identifier } => format!("{}@trade", identifier),
            StreamType::IndexPrice { symbol } => format!("{}@index", symbol),
            StreamType::Kline { symbol, interval } => format!("{}@kline_{}", symbol, interval),
        }
    }
}

// ============================================================================
// Response Types (响应类型)
// ============================================================================

/// Market Data Stream 响应枚举
#[derive(Debug, Clone)]
pub enum OptionMarketDataRes {
    /// 订阅成功响应
    SubscribeSuccess { id: u64 },
    /// 取消订阅成功响应
    UnsubscribeSuccess { id: u64 },
    /// 列出订阅响应
    ListSubscriptionsSuccess { id: u64, streams: Vec<String> },
    /// 设置属性成功响应
    SetPropertySuccess { id: u64 },
    /// 获取属性成功响应
    GetPropertySuccess { id: u64, value: bool },
    /// 市场数据事件
    MarketDataEvent(OptionMarketDataEvent),
}

// ============================================================================
// Behavior Trait (行为接口)
// ============================================================================

/// Option Market Data Stream 行为接口
pub trait OptionMarketDataStreamBehavior: Send + Sync {
    /// 处理 Market Data Stream 命令
    fn handle(
        &mut self,
        cmd: OptionMarketDataCmd,
    ) -> Result<CmdResp<OptionMarketDataRes>, SpotCmdErrorAny>;

    /// 处理接收到的市场数据事件
    fn on_event(&mut self, event: OptionMarketDataEvent);
}

// ============================================================================
// Connection Configuration (连接配置)
// ============================================================================

/// WebSocket 连接配置
#[derive(Debug, Clone)]
pub struct OptionMarketDataStreamConfig {
    /// Base URL: wss://nbstream.binance.com/eoptions/
    pub base_url: String,
    /// 连接类型
    pub connection_type: ConnectionType,
}

/// 连接类型
#[derive(Debug, Clone)]
pub enum ConnectionType {
    /// 单一原始流: /ws/<streamName>
    RawStream { stream_name: String },
    /// 组合流: /stream?streams=<streamName1>/<streamName2>/<streamName3>
    CombinedStream { stream_names: Vec<String> },
}

impl Default for OptionMarketDataStreamConfig {
    fn default() -> Self {
        Self {
            base_url: "wss://nbstream.binance.com/eoptions".to_string(),
            connection_type: ConnectionType::CombinedStream {
                stream_names: Vec::new(),
            },
        }
    }
}

impl OptionMarketDataStreamConfig {
    /// 构建 WebSocket URL
    pub fn build_url(&self) -> String {
        match &self.connection_type {
            ConnectionType::RawStream { stream_name } => {
                format!("{}/ws/{}", self.base_url, stream_name)
            }
            ConnectionType::CombinedStream { stream_names } => {
                let streams = stream_names.join("/");
                format!("{}/stream?streams={}", self.base_url, streams)
            }
        }
    }
}

// ============================================================================
// Connection Constraints (连接约束)
// ============================================================================

/// WebSocket 连接约束常量
pub mod connection_constraints {
    /// 连接有效期（24小时）
    pub const CONNECTION_VALIDITY_HOURS: u64 = 24;

    /// Ping 帧发送间隔（5分钟）
    pub const PING_INTERVAL_MINUTES: u64 = 5;

    /// Pong 超时时间（15分钟）
    pub const PONG_TIMEOUT_MINUTES: u64 = 15;

    /// 每秒最大消息数
    pub const MAX_MESSAGES_PER_SECOND: u32 = 10;

    /// 单连接最大流数量
    pub const MAX_STREAMS_PER_CONNECTION: usize = 200;
}
