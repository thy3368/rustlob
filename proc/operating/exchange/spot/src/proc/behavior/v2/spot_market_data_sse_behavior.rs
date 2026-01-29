// 参考 /Users/hongyaotang/src/rustlob/design/other/binance-spot-api-docs/
// web-socket-streams.md 定义所有 market data 接口

use entity_derive::Entity;
use crate::proc::behavior::spot_trade_behavior::{CMetadata, CmdResp, SpotCmdErrorAny};

/// Market Data Stream 消息枚举
#[derive(Debug, Clone, serde::Serialize)]

// #[entity(id = "order_id")]

pub enum SpotMarketDataStreamAny {
    /// 归集交易流 <symbol>@aggTrade
    AggregateTrade(AggregateTradeStream),
    /// 逐笔交易流 <symbol>@v1
    Trade(TradeStream),
    /// K线流 (UTC) <symbol>@kline_<interval>
    Kline(KlineStream),
    /// K线流 (带时区偏移) <symbol>@kline_<interval>@+08:00
    KlineWithTimezone(KlineStream),
    /// 单交易对 Mini Ticker <symbol>@miniTicker
    MiniTicker(MiniTickerStream),
    /// 全市场 Mini Ticker !miniTicker@arr
    AllMiniTickers(Vec<MiniTickerStream>),
    /// 单交易对 Ticker <symbol>@ticker
    Ticker(TickerStream),
    /// 单交易对滚动窗口统计 <symbol>@ticker_<window_size>
    RollingWindowStats(RollingWindowStatsStream),
    /// 全市场滚动窗口统计 !ticker_<window_size>@arr
    AllRollingWindowStats(Vec<RollingWindowStatsStream>),
    /// 单交易对最优挂单 <symbol>@bookTicker
    BookTicker(BookTickerStream),
    /// 平均价格 <symbol>@avgPrice
    AveragePrice(AveragePriceStream),
    /// 部分深度流 <symbol>@depth<levels> or <symbol>@depth<levels>@100ms
    PartialDepth(PartialDepthStream),
    /// 增量深度流 <symbol>@depth or <symbol>@depth@100ms
    DiffDepth(DiffDepthStream)
}

/// 归集交易流数据
/// Stream: <symbol>@aggTrade
/// Update Speed: Real-time
#[derive(Debug, Clone, serde::Serialize)]
pub struct AggregateTradeStream {
    /// 事件类型 "aggTrade"
    pub event_type: String,
    /// 事件时间 (毫秒)
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 归集交易ID
    pub agg_trade_id: i64,
    /// 价格
    pub price: String,
    /// 数量
    pub quantity: String,
    /// 首个交易ID
    pub first_trade_id: i64,
    /// 最后交易ID
    pub last_trade_id: i64,
    /// 交易时间 (毫秒)
    pub trade_time: i64,
    /// 买方是否为做市方
    pub is_buyer_maker: bool,
    /// 忽略字段
    pub ignore: bool
}

/// 逐笔交易流数据
/// Stream: <symbol>@v1
/// Update Speed: Real-time
#[derive(Debug, Clone, serde::Serialize)]
pub struct TradeStream {
    /// 事件类型 "v1"
    pub event_type: String,
    /// 事件时间 (毫秒)
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 交易ID
    pub trade_id: i64,
    /// 价格
    pub price: String,
    /// 数量
    pub quantity: String,
    /// 交易时间 (毫秒)
    pub trade_time: i64,
    /// 买方是否为做市方
    pub is_buyer_maker: bool,
    /// 忽略字段
    pub ignore: bool
}

/// K线间隔枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KlineInterval {
    /// 1秒
    S1,
    /// 1分钟
    M1,
    /// 3分钟
    M3,
    /// 5分钟
    M5,
    /// 15分钟
    M15,
    /// 30分钟
    M30,
    /// 1小时
    H1,
    /// 2小时
    H2,
    /// 4小时
    H4,
    /// 6小时
    H6,
    /// 8小时
    H8,
    /// 12小时
    H12,
    /// 1天
    D1,
    /// 3天
    D3,
    /// 1周
    W1,
    /// 1月
    Mon1
}

impl KlineInterval {
    /// 转换为字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            KlineInterval::S1 => "1s",
            KlineInterval::M1 => "1m",
            KlineInterval::M3 => "3m",
            KlineInterval::M5 => "5m",
            KlineInterval::M15 => "15m",
            KlineInterval::M30 => "30m",
            KlineInterval::H1 => "1h",
            KlineInterval::H2 => "2h",
            KlineInterval::H4 => "4h",
            KlineInterval::H6 => "6h",
            KlineInterval::H8 => "8h",
            KlineInterval::H12 => "12h",
            KlineInterval::D1 => "1d",
            KlineInterval::D3 => "3d",
            KlineInterval::W1 => "1w",
            KlineInterval::Mon1 => "1M"
        }
    }
}

/// K线数据
#[derive(Debug, Clone, serde::Serialize)]
pub struct KlineData {
    /// K线开始时间 (毫秒)
    pub start_time: i64,
    /// K线关闭时间 (毫秒)
    pub close_time: i64,
    /// 交易对
    pub symbol: String,
    /// 间隔
    pub interval: String,
    /// 首个交易ID
    pub first_trade_id: i64,
    /// 最后交易ID
    pub last_trade_id: i64,
    /// 开盘价
    pub open_price: String,
    /// 收盘价
    pub close_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 成交量 (基础资产)
    pub base_volume: String,
    /// 成交笔数
    pub number_of_trades: i64,
    /// K线是否完结
    pub is_closed: bool,
    /// 成交额 (计价资产)
    pub quote_volume: String,
    /// 主动买入成交量 (基础资产)
    pub taker_buy_base_volume: String,
    /// 主动买入成交额 (计价资产)
    pub taker_buy_quote_volume: String,
    /// 忽略字段
    pub ignore: String
}

/// K线流数据
/// Stream: <symbol>@kline_<interval> or <symbol>@kline_<interval>@+08:00
/// Update Speed: 1000ms for 1s, 2000ms for others
#[derive(Debug, Clone, serde::Serialize)]
pub struct KlineStream {
    /// 事件类型 "kline"
    pub event_type: String,
    /// 事件时间 (毫秒)
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// K线数据
    pub kline: KlineData
}

/// Mini Ticker 流数据
/// Stream: <symbol>@miniTicker
/// Update Speed: 1000ms
#[derive(Debug, Clone, serde::Serialize)]
pub struct MiniTickerStream {
    /// 事件类型 "24hrMiniTicker"
    pub event_type: String,
    /// 事件时间 (毫秒)
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
    /// 成交量 (基础资产)
    pub base_volume: String,
    /// 成交额 (计价资产)
    pub quote_volume: String
}

/// 24小时 Ticker 流数据
/// Stream: <symbol>@ticker
/// Update Speed: 1000ms
#[derive(Debug, Clone, serde::Serialize)]
pub struct TickerStream {
    /// 事件类型 "24hrTicker"
    pub event_type: String,
    /// 事件时间 (毫秒)
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 价格变化
    pub price_change: String,
    /// 价格变化百分比
    pub price_change_percent: String,
    /// 加权平均价
    pub weighted_avg_price: String,
    /// 前一个收盘价
    pub prev_close_price: String,
    /// 最新价格
    pub last_price: String,
    /// 最新成交量
    pub last_quantity: String,
    /// 最优买价
    pub best_bid_price: String,
    /// 最优买量
    pub best_bid_quantity: String,
    /// 最优卖价
    pub best_ask_price: String,
    /// 最优卖量
    pub best_ask_quantity: String,
    /// 开盘价
    pub open_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 成交量 (基础资产)
    pub base_volume: String,
    /// 成交额 (计价资产)
    pub quote_volume: String,
    /// 统计开始时间 (毫秒)
    pub open_time: i64,
    /// 统计结束时间 (毫秒)
    pub close_time: i64,
    /// 首个交易ID
    pub first_trade_id: i64,
    /// 最后交易ID
    pub last_trade_id: i64,
    /// 成交笔数
    pub number_of_trades: i64
}

/// 滚动窗口统计流数据
/// Stream: <symbol>@ticker_<window_size>
/// Window Sizes: 1h, 4h, 1d
/// Update Speed: 1000ms
#[derive(Debug, Clone, serde::Serialize)]
pub struct RollingWindowStatsStream {
    /// 事件类型 "1hTicker", "4hTicker", "1dTicker"
    pub event_type: String,
    /// 事件时间 (毫秒)
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 价格变化
    pub price_change: String,
    /// 价格变化百分比
    pub price_change_percent: String,
    /// 开盘价
    pub open_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 最新价格
    pub last_price: String,
    /// 加权平均价
    pub weighted_avg_price: String,
    /// 成交量 (基础资产)
    pub base_volume: String,
    /// 成交额 (计价资产)
    pub quote_volume: String,
    /// 统计开始时间 (毫秒)
    pub open_time: i64,
    /// 统计结束时间 (毫秒)
    pub close_time: i64,
    /// 首个交易ID
    pub first_trade_id: i64,
    /// 最后交易ID
    pub last_trade_id: i64,
    /// 成交笔数
    pub number_of_trades: i64
}

/// 滚动窗口大小枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSize {
    /// 1小时
    H1,
    /// 4小时
    H4,
    /// 1天
    D1
}

impl WindowSize {
    /// 转换为字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            WindowSize::H1 => "1h",
            WindowSize::H4 => "4h",
            WindowSize::D1 => "1d"
        }
    }
}

/// 最优挂单流数据
/// Stream: <symbol>@bookTicker
/// Update Speed: Real-time
#[derive(Debug, Clone, serde::Serialize)]
pub struct BookTickerStream {
    /// 订单簿更新ID
    pub update_id: i64,
    /// 交易对
    pub symbol: String,
    /// 最优买价
    pub best_bid_price: String,
    /// 最优买量
    pub best_bid_quantity: String,
    /// 最优卖价
    pub best_ask_price: String,
    /// 最优卖量
    pub best_ask_quantity: String
}

/// 平均价格流数据
/// Stream: <symbol>@avgPrice
/// Update Speed: 1000ms
#[derive(Debug, Clone, serde::Serialize)]
pub struct AveragePriceStream {
    /// 事件类型 "avgPrice"
    pub event_type: String,
    /// 事件时间 (毫秒)
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 平均价格间隔
    pub interval: String,
    /// 平均价格
    pub avg_price: String,
    /// 最后交易时间 (毫秒)
    pub last_trade_time: i64
}

/// 价格档位
#[derive(Debug, Clone, serde::Serialize)]
pub struct PriceLevel {
    /// 价格
    pub price: String,
    /// 数量
    pub quantity: String
}

/// 部分深度流数据
/// Stream: <symbol>@depth<levels> or <symbol>@depth<levels>@100ms
/// Levels: 5, 10, 20
/// Update Speed: 1000ms or 100ms
#[derive(Debug, Clone, serde::Serialize)]
pub struct PartialDepthStream {
    /// 最后更新ID
    pub last_update_id: i64,
    /// 买单深度
    pub bids: Vec<PriceLevel>,
    /// 卖单深度
    pub asks: Vec<PriceLevel>
}

/// 增量深度流数据
/// Stream: <symbol>@depth or <symbol>@depth@100ms
/// Update Speed: 1000ms or 100ms
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiffDepthStream {
    /// 事件类型 "depthUpdate"
    pub event_type: String,
    /// 事件时间 (毫秒)
    pub event_time: i64,
    /// 交易对
    pub symbol: String,
    /// 首个更新ID
    pub first_update_id: i64,
    /// 最后更新ID
    pub last_update_id: i64,
    /// 买单更新
    pub bids: Vec<PriceLevel>,
    /// 卖单更新
    pub asks: Vec<PriceLevel>
}

/// 深度档位枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthLevel {
    /// 5档
    Level5,
    /// 10档
    Level10,
    /// 20档
    Level20
}

impl DepthLevel {
    /// 转换为数字
    pub fn as_number(&self) -> u8 {
        match self {
            DepthLevel::Level5 => 5,
            DepthLevel::Level10 => 10,
            DepthLevel::Level20 => 20
        }
    }
}

/// 更新速度枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateSpeed {
    /// 1000毫秒
    Ms1000,
    /// 100毫秒
    Ms100
}

impl UpdateSpeed {
    /// 转换为字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            UpdateSpeed::Ms1000 => "1000ms",
            UpdateSpeed::Ms100 => "100ms"
        }
    }
}

/// WebSocket 订阅/取消订阅请求
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SubscriptionRequest {
    /// 方法: SUBSCRIBE, UNSUBSCRIBE, LIST_SUBSCRIPTIONS, SET_PROPERTY,
    /// GET_PROPERTY
    pub method: String,
    /// 参数列表
    pub params: Vec<String>,
    /// 请求ID (可选)
    pub id: Option<SubscriptionId>
}

/// 订阅ID类型
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SubscriptionId {
    /// 整数ID
    Integer(i64),
    /// 字符串ID (最大36字符)
    String(String),
    /// 空ID
    Null
}

/// WebSocket 订阅响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptionResponse {
    /// 结果 (null表示成功)
    pub result: Option<SubscriptionResult>,
    /// 请求ID
    pub id: Option<SubscriptionId>
}

/// 订阅结果类型
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SubscriptionResult {
    /// 成功 (null)
    Success,
    /// 订阅列表
    Subscriptions(Vec<String>),
    /// 属性值
    Property(bool),
    /// 错误
    Error(SubscriptionError)
}

/// 订阅错误
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptionError {
    /// 错误码
    pub code: i32,
    /// 错误消息
    pub msg: String,
    /// 请求ID
    pub id: Option<SubscriptionId>
}

/// Market Data Stream 订阅命令
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MarketDataSubscriptionCmdAny {
    /// 订阅流
    Subscribe {
        /// 元数据
        metadata: CMetadata,
        /// 流名称列表
        streams: Vec<String>
    },
    /// 取消订阅流
    Unsubscribe {
        /// 元数据
        metadata: CMetadata,
        /// 流名称列表
        streams: Vec<String>
    },
    /// 列出当前订阅
    ListSubscriptions {
        /// 元数据
        metadata: CMetadata
    },
    /// 设置属性
    SetProperty {
        /// 元数据
        metadata: CMetadata,
        /// 属性名
        property: String,
        /// 属性值
        value: bool
    },
    /// 获取属性
    GetProperty {
        /// 元数据
        metadata: CMetadata,
        /// 属性名
        property: String
    }
}

/// 组合流包装
#[derive(Debug, Clone, serde::Serialize)]
pub struct CombinedStreamWrapper {
    /// 流名称
    pub stream: String,
    /// 原始数据
    pub data: SpotMarketDataStreamAny
}

// ============================================================================
// User Data Stream 相关类型 (从原有代码保留)
// ============================================================================

/// 佣金费率
#[derive(Debug, Clone, serde::Serialize)]
pub struct CommissionRates {
    /// Maker 佣金费率
    pub maker: String,
    /// Taker 佣金费率
    pub taker: String,
    /// 买方佣金费率
    pub buyer: String,
    /// 卖方佣金费率
    pub seller: String
}

/// 余额信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct Balance {
    /// 资产名称
    pub asset: String,
    /// 可用余额
    pub free: String,
    /// 锁定余额
    pub locked: String
}

/// User Data 命令枚举
#[derive(Debug, Clone, serde::Serialize)]
pub enum SpotUserDataCmdAny {
    /// 账户信息查询 GET /api/v3/account
    /// Weight: 20
    Account(AccountCmd)
}

/// 账户信息查询命令
/// GET /api/v3/account
/// Weight: 20
/// Data Source: Memory => Database
#[derive(Debug, Clone, serde::Serialize)]
pub struct AccountCmd {
    pub metadata: CMetadata,
    /// 仅返回非零余额，默认 false
    pub omit_zero_balances: Option<bool>,
    /// 接收窗口（微秒精度），不超过 60000
    pub recv_window: Option<f64>,
    /// 时间戳
    pub timestamp: i64
}

/// User Data 响应枚举
#[derive(Debug, Clone, serde::Serialize)]
pub enum SpotUserDataRes {
    /// 账户信息响应
    Account(AccountInfo)
}

/// 账户信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct AccountInfo {
    /// Maker 佣金
    pub maker_commission: i32,
    /// Taker 佣金
    pub taker_commission: i32,
    /// 买方佣金
    pub buyer_commission: i32,
    /// 卖方佣金
    pub seller_commission: i32,
    /// 佣金费率
    pub commission_rates: CommissionRates,
    /// 可交易
    pub can_trade: bool,
    /// 可提现
    pub can_withdraw: bool,
    /// 可充值
    pub can_deposit: bool,
    /// 经纪账户
    pub brokered: bool,
    /// 需要自成交防护
    pub require_self_trade_prevention: bool,
    /// 阻止 SOR
    pub prevent_sor: bool,
    /// 更新时间
    pub update_time: i64,
    /// 账户类型
    pub account_type: String,
    /// 余额列表
    pub balances: Vec<Balance>,
    /// 权限列表
    pub permissions: Vec<String>,
    /// 用户ID
    pub uid: i64
}

/// Market Data Stream 行为接口
pub trait SpotMarketDataSubscriptionBehavior: Send + Sync {
    /// 处理订阅命令
    fn handle_subscription(
        &mut self, cmd: MarketDataSubscriptionCmdAny
    ) -> Result<CmdResp<SubscriptionResponse>, SpotCmdErrorAny>;
}


pub trait SpotMarketDataPublishBehavior: Send + Sync {
    fn generate_stream_message(&self) -> SpotMarketDataStreamAny;
}
