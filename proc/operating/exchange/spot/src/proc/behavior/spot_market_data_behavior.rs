// 参考 Market Data endpoints 在 /Users/hongyaotang/src/rustlob/design/other/binance-spot-api-docs/rest-api.md 定义所有 market data 接口

use crate::proc::behavior::spot_trade_behavior::{CMetadata, CmdResp, SpotCmdError};

/// Market Data 命令枚举
#[derive(Debug, Clone)]
pub enum SpotMarketDataCmdAny {
    /// 订单簿 GET /api/v3/depth
    /// Weight: 5-250 (根据limit调整)
    OrderBook(OrderBookCmd),

    /// 最近成交 GET /api/v3/trades
    /// Weight: 25
    RecentTrades(RecentTradesCmd),

    /// 历史成交 GET /api/v3/historicalTrades
    /// Weight: 25
    HistoricalTrades(HistoricalTradesCmd),

    /// 聚合成交 GET /api/v3/aggTrades
    /// Weight: 4
    AggTrades(AggTradesCmd),

    /// K线数据 GET /api/v3/klines
    /// Weight: 2
    Klines(KlinesCmd),

    /// UI K线数据 GET /api/v3/uiKlines
    /// Weight: 2
    UIKlines(UIKlinesCmd),

    /// 当前平均价格 GET /api/v3/avgPrice
    /// Weight: 2
    AvgPrice(AvgPriceCmd),

    /// 24小时价格变动统计 GET /api/v3/ticker/24hr
    /// Weight: 2-80 (根据symbol数量)
    Ticker24hr(Ticker24hrCmd),

    /// 交易日价格变动 GET /api/v3/ticker/tradingDay
    /// Weight: 4 per symbol (max 200)
    TradingDayTicker(TradingDayTickerCmd),

    /// 最新价格 GET /api/v3/ticker/price
    /// Weight: 2-4
    SymbolPriceTicker(SymbolPriceTickerCmd),

    /// 最优挂单 GET /api/v3/ticker/bookTicker
    /// Weight: 2-4
    BookTicker(BookTickerCmd),

    /// 滚动窗口价格变动统计 GET /api/v3/ticker
    /// Weight: 4 per symbol (max 200)
    RollingWindowTicker(RollingWindowTickerCmd),
}

/// 订单簿查询命令
/// GET /api/v3/depth
/// Weight: 根据limit调整 (1-100: 5, 101-500: 25, 501-1000: 50, 1001-5000: 250)
/// Data Source: Memory
#[derive(Debug, Clone)]
pub struct OrderBookCmd {
    pub metadata: CMetadata,
    /// 交易对（必填）
    pub symbol: String,
    /// 深度限制，默认 100，最大 5000
    pub limit: Option<i32>,
    /// 交易状态过滤（TRADING, HALT, BREAK）
    pub symbol_status: Option<String>,
}

/// 最近成交查询命令
/// GET /api/v3/trades
/// Weight: 25
/// Data Source: Memory
#[derive(Debug, Clone)]
pub struct RecentTradesCmd {
    pub metadata: CMetadata,
    /// 交易对（必填）
    pub symbol: String,
    /// 限制数量，默认 500，最大 1000
    pub limit: Option<i32>,
}

/// 历史成交查询命令
/// GET /api/v3/historicalTrades
/// Weight: 25
/// Data Source: Database
#[derive(Debug, Clone)]
pub struct HistoricalTradesCmd {
    pub metadata: CMetadata,
    /// 交易对（必填）
    pub symbol: String,
    /// 限制数量，默认 500，最大 1000
    pub limit: Option<i32>,
    /// 从该成交ID开始，默认返回最近的成交
    pub from_id: Option<i64>,
}

/// 聚合成交查询命令
/// GET /api/v3/aggTrades
/// Weight: 4
/// Data Source: Database
#[derive(Debug, Clone)]
pub struct AggTradesCmd {
    pub metadata: CMetadata,
    /// 交易对（必填）
    pub symbol: String,
    /// 从该ID开始（包含）
    pub from_id: Option<i64>,
    /// 开始时间（毫秒，包含）
    pub start_time: Option<i64>,
    /// 结束时间（毫秒，包含）
    pub end_time: Option<i64>,
    /// 限制数量，默认 500，最大 1000
    pub limit: Option<i32>,
}

/// K线数据查询命令
/// GET /api/v3/klines
/// Weight: 2
/// Data Source: Database
#[derive(Debug, Clone)]
pub struct KlinesCmd {
    pub metadata: CMetadata,
    /// 交易对（必填）
    pub symbol: String,
    /// 时间间隔（必填）：1s, 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M
    pub interval: String,
    /// 开始时间（毫秒）
    pub start_time: Option<i64>,
    /// 结束时间（毫秒）
    pub end_time: Option<i64>,
    /// 时区，默认 0 (UTC)，范围 [-12:00 到 +14:00]
    pub time_zone: Option<String>,
    /// 限制数量，默认 500，最大 1000
    pub limit: Option<i32>,
}

/// UI K线数据查询命令
/// GET /api/v3/uiKlines
/// Weight: 2
/// Data Source: Database
/// 返回优化后的K线数据，适合图表展示
#[derive(Debug, Clone)]
pub struct UIKlinesCmd {
    pub metadata: CMetadata,
    /// 交易对（必填）
    pub symbol: String,
    /// 时间间隔（必填）
    pub interval: String,
    /// 开始时间（毫秒）
    pub start_time: Option<i64>,
    /// 结束时间（毫秒）
    pub end_time: Option<i64>,
    /// 时区，默认 0 (UTC)
    pub time_zone: Option<String>,
    /// 限制数量，默认 500，最大 1000
    pub limit: Option<i32>,
}

/// 当前平均价格查询命令
/// GET /api/v3/avgPrice
/// Weight: 2
/// Data Source: Memory
#[derive(Debug, Clone)]
pub struct AvgPriceCmd {
    pub metadata: CMetadata,
    /// 交易对（必填）
    pub symbol: String,
}

/// 24小时价格变动统计查询命令
/// GET /api/v3/ticker/24hr
/// Weight: 2 (单symbol) / 40 (21-100 symbols) / 80 (>100 或全部)
/// Data Source: Memory
#[derive(Debug, Clone)]
pub struct Ticker24hrCmd {
    pub metadata: CMetadata,
    /// 单个交易对（与 symbols 互斥）
    pub symbol: Option<String>,
    /// 多个交易对数组（与 symbol 互斥）
    pub symbols: Option<Vec<String>>,
    /// 响应类型：FULL 或 MINI，默认 FULL
    pub ticker_type: Option<String>,
    /// 交易状态过滤（TRADING, HALT, BREAK）
    pub symbol_status: Option<String>,
}

/// 交易日价格变动查询命令
/// GET /api/v3/ticker/tradingDay
/// Weight: 4 per symbol (max 200 when >50 symbols)
/// Data Source: Database
#[derive(Debug, Clone)]
pub struct TradingDayTickerCmd {
    pub metadata: CMetadata,
    /// 单个交易对（与 symbols 互斥，二选一）
    pub symbol: Option<String>,
    /// 多个交易对数组（与 symbol 互斥，最大 100）
    pub symbols: Option<Vec<String>>,
    /// 时区，默认 0 (UTC)
    pub time_zone: Option<String>,
    /// 响应类型：FULL 或 MINI，默认 FULL
    pub ticker_type: Option<String>,
    /// 交易状态过滤（TRADING, HALT, BREAK）
    pub symbol_status: Option<String>,
}

/// 最新价格查询命令
/// GET /api/v3/ticker/price
/// Weight: 2 (单symbol) / 4 (多symbol或全部)
/// Data Source: Memory
#[derive(Debug, Clone)]
pub struct SymbolPriceTickerCmd {
    pub metadata: CMetadata,
    /// 单个交易对（与 symbols 互斥）
    pub symbol: Option<String>,
    /// 多个交易对数组（与 symbol 互斥）
    pub symbols: Option<Vec<String>>,
    /// 交易状态过滤（TRADING, HALT, BREAK）
    pub symbol_status: Option<String>,
}

/// 最优挂单查询命令
/// GET /api/v3/ticker/bookTicker
/// Weight: 2 (单symbol) / 4 (多symbol或全部)
/// Data Source: Memory
#[derive(Debug, Clone)]
pub struct BookTickerCmd {
    pub metadata: CMetadata,
    /// 单个交易对（与 symbols 互斥）
    pub symbol: Option<String>,
    /// 多个交易对数组（与 symbol 互斥）
    pub symbols: Option<Vec<String>>,
    /// 交易状态过滤（TRADING, HALT, BREAK）
    pub symbol_status: Option<String>,
}

/// 滚动窗口价格变动统计查询命令
/// GET /api/v3/ticker
/// Weight: 4 per symbol (max 200 when >50 symbols)
/// Data Source: Database
#[derive(Debug, Clone)]
pub struct RollingWindowTickerCmd {
    pub metadata: CMetadata,
    /// 单个交易对（与 symbols 互斥，二选一）
    pub symbol: Option<String>,
    /// 多个交易对数组（与 symbol 互斥，最大 100）
    pub symbols: Option<Vec<String>>,
    /// 窗口大小：1m-59m (分钟), 1h-23h (小时), 1d-7d (天)，默认 1d
    pub window_size: Option<String>,
    /// 响应类型：FULL 或 MINI，默认 FULL
    pub ticker_type: Option<String>,
    /// 交易状态过滤（TRADING, HALT, BREAK）
    pub symbol_status: Option<String>,
}

/// Market Data 响应枚举
#[derive(Debug, Clone)]
pub enum SpotMarketDataRes {
    /// 订单簿响应
    OrderBook(OrderBookData),

    /// 成交列表响应
    Trades(Vec<TradeData>),

    /// 聚合成交列表响应
    AggTrades(Vec<AggTradeData>),

    /// K线数据响应
    Klines(Vec<KlineData>),

    /// 平均价格响应
    AvgPrice(AvgPriceData),

    /// 24小时Ticker响应（单个）
    Ticker24hr(Ticker24hrData),

    /// 24小时Ticker响应（数组）
    Ticker24hrList(Vec<Ticker24hrData>),

    /// 交易日Ticker响应（单个）
    TradingDayTicker(TradingDayTickerData),

    /// 交易日Ticker响应（数组）
    TradingDayTickerList(Vec<TradingDayTickerData>),

    /// 价格Ticker响应（单个）
    PriceTicker(PriceTickerData),

    /// 价格Ticker响应（数组）
    PriceTickerList(Vec<PriceTickerData>),

    /// 订单簿Ticker响应（单个）
    BookTicker(BookTickerData),

    /// 订单簿Ticker响应（数组）
    BookTickerList(Vec<BookTickerData>),

    /// 滚动窗口Ticker响应（单个）
    RollingWindowTicker(RollingWindowTickerData),

    /// 滚动窗口Ticker响应（数组）
    RollingWindowTickerList(Vec<RollingWindowTickerData>),
}

/// 订单簿数据
#[derive(Debug, Clone)]
pub struct OrderBookData {
    /// 最后更新ID
    pub last_update_id: i64,
    /// 买单列表 [[价格, 数量], ...]
    pub bids: Vec<(String, String)>,
    /// 卖单列表 [[价格, 数量], ...]
    pub asks: Vec<(String, String)>,
}

/// 成交数据
#[derive(Debug, Clone)]
pub struct TradeData {
    /// 成交ID
    pub id: i64,
    /// 价格
    pub price: String,
    /// 数量
    pub qty: String,
    /// 成交额
    pub quote_qty: String,
    /// 成交时间
    pub time: i64,
    /// 是否为买方maker
    pub is_buyer_maker: bool,
    /// 是否为最佳匹配
    pub is_best_match: bool,
}

/// 聚合成交数据
#[derive(Debug, Clone)]
pub struct AggTradeData {
    /// 聚合成交ID
    pub agg_trade_id: i64,
    /// 价格
    pub price: String,
    /// 数量
    pub quantity: String,
    /// 第一个成交ID
    pub first_trade_id: i64,
    /// 最后一个成交ID
    pub last_trade_id: i64,
    /// 成交时间
    pub timestamp: i64,
    /// 是否为买方maker
    pub is_buyer_maker: bool,
    /// 是否为最佳匹配
    pub is_best_match: bool,
}

/// K线数据
#[derive(Debug, Clone)]
pub struct KlineData {
    /// 开盘时间
    pub open_time: i64,
    /// 开盘价
    pub open: String,
    /// 最高价
    pub high: String,
    /// 最低价
    pub low: String,
    /// 收盘价
    pub close: String,
    /// 成交量
    pub volume: String,
    /// 收盘时间
    pub close_time: i64,
    /// 成交额
    pub quote_asset_volume: String,
    /// 成交笔数
    pub number_of_trades: i32,
    /// Taker买入成交量
    pub taker_buy_base_asset_volume: String,
    /// Taker买入成交额
    pub taker_buy_quote_asset_volume: String,
}

/// 平均价格数据
#[derive(Debug, Clone)]
pub struct AvgPriceData {
    /// 平均价格间隔（分钟）
    pub mins: i32,
    /// 平均价格
    pub price: String,
    /// 最后成交时间
    pub close_time: i64,
}

/// 24小时Ticker数据（完整）
#[derive(Debug, Clone)]
pub struct Ticker24hrData {
    /// 交易对
    pub symbol: String,
    /// 价格变动
    pub price_change: String,
    /// 价格变动百分比
    pub price_change_percent: String,
    /// 加权平均价
    pub weighted_avg_price: String,
    /// 前一收盘价
    pub prev_close_price: Option<String>,
    /// 最新价
    pub last_price: String,
    /// 最新成交量
    pub last_qty: Option<String>,
    /// 买一价
    pub bid_price: Option<String>,
    /// 买一量
    pub bid_qty: Option<String>,
    /// 卖一价
    pub ask_price: Option<String>,
    /// 卖一量
    pub ask_qty: Option<String>,
    /// 开盘价
    pub open_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 成交量
    pub volume: String,
    /// 成交额
    pub quote_volume: String,
    /// 开盘时间
    pub open_time: i64,
    /// 收盘时间
    pub close_time: i64,
    /// 第一笔成交ID
    pub first_id: Option<i64>,
    /// 最后一笔成交ID
    pub last_id: Option<i64>,
    /// 成交笔数
    pub count: i64,
}

/// 交易日Ticker数据
#[derive(Debug, Clone)]
pub struct TradingDayTickerData {
    /// 交易对
    pub symbol: String,
    /// 价格变动
    pub price_change: Option<String>,
    /// 价格变动百分比
    pub price_change_percent: Option<String>,
    /// 加权平均价
    pub weighted_avg_price: Option<String>,
    /// 开盘价
    pub open_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 最新价
    pub last_price: String,
    /// 成交量
    pub volume: String,
    /// 成交额
    pub quote_volume: String,
    /// 开盘时间
    pub open_time: i64,
    /// 收盘时间
    pub close_time: i64,
    /// 第一笔成交ID
    pub first_id: i64,
    /// 最后一笔成交ID
    pub last_id: i64,
    /// 成交笔数
    pub count: i64,
}

/// 价格Ticker数据
#[derive(Debug, Clone)]
pub struct PriceTickerData {
    /// 交易对
    pub symbol: String,
    /// 价格
    pub price: String,
}

/// 订单簿Ticker数据
#[derive(Debug, Clone)]
pub struct BookTickerData {
    /// 交易对
    pub symbol: String,
    /// 买一价
    pub bid_price: String,
    /// 买一量
    pub bid_qty: String,
    /// 卖一价
    pub ask_price: String,
    /// 卖一量
    pub ask_qty: String,
}

/// 滚动窗口Ticker数据
#[derive(Debug, Clone)]
pub struct RollingWindowTickerData {
    /// 交易对
    pub symbol: String,
    /// 价格变动
    pub price_change: String,
    /// 价格变动百分比
    pub price_change_percent: String,
    /// 加权平均价
    pub weighted_avg_price: String,
    /// 开盘价
    pub open_price: String,
    /// 最高价
    pub high_price: String,
    /// 最低价
    pub low_price: String,
    /// 最新价
    pub last_price: String,
    /// 成交量
    pub volume: String,
    /// 成交额
    pub quote_volume: String,
    /// 开盘时间
    pub open_time: i64,
    /// 收盘时间
    pub close_time: i64,
    /// 第一笔成交ID
    pub first_id: i64,
    /// 最后一笔成交ID
    pub last_id: i64,
    /// 成交笔数
    pub count: i64,
}

/// Market Data 行为接口
pub trait SpotMarketDataBehavior: Send + Sync {
    /// 处理 Market Data 命令
    fn handle(&mut self, cmd: SpotMarketDataCmdAny) -> Result<CmdResp<SpotMarketDataRes>, SpotCmdError>;
}
