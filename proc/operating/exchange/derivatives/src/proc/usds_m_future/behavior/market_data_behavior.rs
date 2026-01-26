// 参考 ## marketdata Endpoints
// /Users/hongyaotang/src/rustlob/design/other/binance_derivatives_api/
// usds-margined-futures/market-data/rest-api 定义所有 market data 接口

use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};

// ============================================================================
// 枚举类型定义
// ============================================================================

/// K线时间间隔
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KlineInterval {
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
    MO1
}

/// 合约类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContractType {
    /// 永续合约
    PERPETUAL,
    /// 当季合约
    CURRENT_QUARTER,
    /// 次季合约
    NEXT_QUARTER
}

/// 持续合约类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContinuousContractType {
    /// 永续
    PERPETUAL,
    /// 当季
    CURRENT_QUARTER,
    /// 次季
    NEXT_QUARTER
}

/// 统计周期类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PeriodType {
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
    /// 12小时
    H12,
    /// 1天
    D1
}

// ============================================================================
// Market Data 命令枚举
// ============================================================================

/// USDS-M期货市场数据命令枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureMarketDataCmdAny {
    // ========== 基础信息 ==========
    /// 测试连接 GET /fapi/v1/ping
    /// Weight: 1
    TestConnectivity(TestConnectivityCmd),

    /// 检查服务器时间 GET /fapi/v1/time
    /// Weight: 1
    CheckServerTime(CheckServerTimeCmd),

    /// 交易所信息 GET /fapi/v1/exchangeInfo
    /// Weight: 1
    ExchangeInformation(ExchangeInformationCmd),

    // ========== 市场数据 ==========
    /// 订单簿 GET /fapi/v1/depth
    /// Weight: 2-20 (根据limit)
    OrderBook(OrderBookCmd),

    /// 最近成交 GET /fapi/v1/trades
    /// Weight: 5
    RecentTradesList(RecentTradesListCmd),

    /// 历史成交 GET /fapi/v1/historicalTrades
    /// Weight: 20
    OldTradesLookup(OldTradesLookupCmd),

    /// 归集成交 GET /fapi/v1/aggTrades
    /// Weight: 20
    CompressedAggregateTradesList(CompressedAggregateTradesListCmd),

    // ========== K线数据 ==========
    /// K线数据 GET /fapi/v1/klines
    /// Weight: 1-10 (根据limit)
    KlineCandlestickData(KlineCandlestickDataCmd),

    /// 持续合约K线 GET /fapi/v1/continuousKlines
    /// Weight: 1-10 (根据limit)
    ContinuousContractKlineCandlestickData(ContinuousContractKlineCandlestickDataCmd),

    /// 指数价格K线 GET /fapi/v1/indexPriceKlines
    /// Weight: 1-10 (根据limit)
    IndexPriceKlineCandlestickData(IndexPriceKlineCandlestickDataCmd),

    /// 标记价格K线 GET /fapi/v1/markPriceKlines
    /// Weight: 1-10 (根据limit)
    MarkPriceKlineCandlestickData(MarkPriceKlineCandlestickDataCmd),

    /// 溢价指数K线 GET /fapi/v1/premiumIndexKlines
    /// Weight: 1-10 (根据limit)
    PremiumIndexKlineData(PremiumIndexKlineDataCmd),

    // ========== 标记价格和资金费率 ==========
    /// 标记价格 GET /fapi/v1/premiumIndex
    /// Weight: 1
    MarkPrice(MarkPriceCmd),

    /// 资金费率历史 GET /fapi/v1/fundingRate
    /// Weight: 1
    GetFundingRateHistory(GetFundingRateHistoryCmd),

    /// 资金费率信息 GET /fapi/v1/fundingInfo
    /// Weight: 1
    GetFundingRateInfo(GetFundingRateInfoCmd),

    // ========== 行情数据 ==========
    /// 24小时价格变动 GET /fapi/v1/ticker/24hr
    /// Weight: 1 (single), 40 (all)
    Ticker24hr(Ticker24hrCmd),

    /// 最新价格 GET /fapi/v1/ticker/price
    /// Weight: 1 (single), 2 (all)
    SymbolPriceTicker(SymbolPriceTickerCmd),

    /// 最新价格V2 GET /fapi/v2/ticker/price
    /// Weight: 1 (single), 2 (all)
    SymbolPriceTickerV2(SymbolPriceTickerV2Cmd),

    /// 最优挂单 GET /fapi/v1/ticker/bookTicker
    /// Weight: 1 (single), 2 (all)
    SymbolOrderBookTicker(SymbolOrderBookTickerCmd),

    // ========== 持仓量 ==========
    /// 持仓量 GET /fapi/v1/openInterest
    /// Weight: 1
    OpenInterest(OpenInterestCmd),

    /// 持仓量统计 GET /futures/data/openInterestHist
    /// Weight: 1
    OpenInterestStatistics(OpenInterestStatisticsCmd),

    // ========== 大户持仓 ==========
    /// 大户账户数多空比 GET /futures/data/topLongShortAccountRatio
    /// Weight: 1
    TopLongShortAccountRatio(TopLongShortAccountRatioCmd),

    /// 大户持仓量多空比 GET /futures/data/topLongShortPositionRatio
    /// Weight: 1
    TopTraderLongShortRatio(TopTraderLongShortRatioCmd),

    /// 多空持仓人数比 GET /futures/data/globalLongShortAccountRatio
    /// Weight: 1
    LongShortRatio(LongShortRatioCmd),

    // ========== 主动买卖量 ==========
    /// 合约主动买卖量 GET /futures/data/takerlongshortRatio
    /// Weight: 1
    TakerBuySellVolume(TakerBuySellVolumeCmd),

    // ========== 其他 ==========
    /// 历史BLVT NAV K线 GET /fapi/v1/lvtKlines
    HistoricalBLVTNAVKlineCandlestick(HistoricalBLVTNAVKlineCandlestickCmd),

    /// 复合指数信息 GET /fapi/v1/constituents
    /// Weight: 2
    CompositeIndexSymbolInformation(CompositeIndexSymbolInformationCmd),

    /// 指数成分 GET /fapi/v1/constituents
    IndexConstituents(IndexConstituentsCmd),

    /// 多资产模式资产指数 GET /fapi/v1/assetIndex
    /// Weight: 10
    MultiAssetsModeAssetIndex(MultiAssetsModeAssetIndexCmd),

    /// 交割价格 GET /fapi/v1/deliveryPrice
    /// Weight: 1
    DeliveryPrice(DeliveryPriceCmd)
}

// ============================================================================
// 1. 基础信息命令
// ============================================================================

/// 测试连接命令
/// GET /fapi/v1/ping
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestConnectivityCmd {
    pub metadata: CMetadata
}

/// 检查服务器时间命令
/// GET /fapi/v1/time
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CheckServerTimeCmd {
    pub metadata: CMetadata
}

/// 交易所信息命令
/// GET /fapi/v1/exchangeInfo
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExchangeInformationCmd {
    pub metadata: CMetadata
}

// ============================================================================
// 2. 市场数据命令
// ============================================================================

/// 订单簿命令
/// GET /fapi/v1/depth
/// Weight: 2-20 (根据limit: 5/10/20/50=2, 100=5, 500=10, 1000=20)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderBookCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 深度限制 [5, 10, 20, 50, 100, 500, 1000] (默认500)
    pub limit: Option<i32>
}

/// 最近成交命令
/// GET /fapi/v1/trades
/// Weight: 5
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RecentTradesListCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 限制数量 (默认500, 最大1000)
    pub limit: Option<i32>
}

/// 历史成交命令
/// GET /fapi/v1/historicalTrades
/// Weight: 20
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OldTradesLookupCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 限制数量 (默认500, 最大1000)
    pub limit: Option<i32>,
    /// 从哪个tradeId开始返回
    pub from_id: Option<i64>
}

/// 归集成交命令
/// GET /fapi/v1/aggTrades
/// Weight: 20
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompressedAggregateTradesListCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 从哪个aggTradeId开始返回
    pub from_id: Option<i64>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量 (默认500, 最大1000)
    pub limit: Option<i32>
}

// ============================================================================
// 3. K线数据命令
// ============================================================================

/// K线数据命令
/// GET /fapi/v1/klines
/// Weight: 1-10 (根据limit: [1,100)=1, [100,500)=2, [500,1000]=5, >1000=10)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KlineCandlestickDataCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 时间间隔
    pub interval: KlineInterval,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量 (默认500, 最大1500)
    pub limit: Option<i32>
}

/// 持续合约K线命令
/// GET /fapi/v1/continuousKlines
/// Weight: 1-10 (根据limit)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContinuousContractKlineCandlestickDataCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub pair: String,
    /// 合约类型
    pub contract_type: ContinuousContractType,
    /// 时间间隔
    pub interval: KlineInterval,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量 (默认500, 最大1500)
    pub limit: Option<i32>
}

/// 指数价格K线命令
/// GET /fapi/v1/indexPriceKlines
/// Weight: 1-10 (根据limit)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexPriceKlineCandlestickDataCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub pair: String,
    /// 时间间隔
    pub interval: KlineInterval,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量 (默认500, 最大1500)
    pub limit: Option<i32>
}

/// 标记价格K线命令
/// GET /fapi/v1/markPriceKlines
/// Weight: 1-10 (根据limit)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkPriceKlineCandlestickDataCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 时间间隔
    pub interval: KlineInterval,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量 (默认500, 最大1500)
    pub limit: Option<i32>
}

/// 溢价指数K线命令
/// GET /fapi/v1/premiumIndexKlines
/// Weight: 1-10 (根据limit)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PremiumIndexKlineDataCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 时间间隔
    pub interval: KlineInterval,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量 (默认500, 最大1500)
    pub limit: Option<i32>
}

// ============================================================================
// 4. 标记价格和资金费率命令
// ============================================================================

/// 标记价格命令
/// GET /fapi/v1/premiumIndex
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkPriceCmd {
    pub metadata: CMetadata,
    /// 交易对 (不填返回所有)
    pub symbol: Option<String>
}

/// 资金费率历史命令
/// GET /fapi/v1/fundingRate
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetFundingRateHistoryCmd {
    pub metadata: CMetadata,
    /// 交易对 (不填返回所有)
    pub symbol: Option<String>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量 (默认100, 最大1000)
    pub limit: Option<i32>
}

/// 资金费率信息命令
/// GET /fapi/v1/fundingInfo
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GetFundingRateInfoCmd {
    pub metadata: CMetadata
}

// ============================================================================
// 5. 行情数据命令
// ============================================================================

/// 24小时价格变动命令
/// GET /fapi/v1/ticker/24hr
/// Weight: 1 (single), 40 (all)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ticker24hrCmd {
    pub metadata: CMetadata,
    /// 交易对 (不填返回所有)
    pub symbol: Option<String>
}

/// 最新价格命令
/// GET /fapi/v1/ticker/price
/// Weight: 1 (single), 2 (all)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolPriceTickerCmd {
    pub metadata: CMetadata,
    /// 交易对 (不填返回所有)
    pub symbol: Option<String>
}

/// 最新价格V2命令
/// GET /fapi/v2/ticker/price
/// Weight: 1 (single), 2 (all)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolPriceTickerV2Cmd {
    pub metadata: CMetadata,
    /// 交易对 (不填返回所有)
    pub symbol: Option<String>
}

/// 最优挂单命令
/// GET /fapi/v1/ticker/bookTicker
/// Weight: 1 (single), 2 (all)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolOrderBookTickerCmd {
    pub metadata: CMetadata,
    /// 交易对 (不填返回所有)
    pub symbol: Option<String>
}

// ============================================================================
// 6. 持仓量命令
// ============================================================================

/// 持仓量命令
/// GET /fapi/v1/openInterest
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenInterestCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String
}

/// 持仓量统计命令
/// GET /futures/data/openInterestHist
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenInterestStatisticsCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 周期类型
    pub period: PeriodType,
    /// 限制数量 (默认30, 最大500)
    pub limit: Option<i32>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>
}

// ============================================================================
// 7. 大户持仓命令
// ============================================================================

/// 大户账户数多空比命令
/// GET /futures/data/topLongShortAccountRatio
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopLongShortAccountRatioCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 周期类型
    pub period: PeriodType,
    /// 限制数量 (默认30, 最大500)
    pub limit: Option<i32>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>
}

/// 大户持仓量多空比命令
/// GET /futures/data/topLongShortPositionRatio
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopTraderLongShortRatioCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 周期类型
    pub period: PeriodType,
    /// 限制数量 (默认30, 最大500)
    pub limit: Option<i32>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>
}

/// 多空持仓人数比命令
/// GET /futures/data/globalLongShortAccountRatio
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LongShortRatioCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 周期类型
    pub period: PeriodType,
    /// 限制数量 (默认30, 最大500)
    pub limit: Option<i32>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>
}

// ============================================================================
// 8. 主动买卖量命令
// ============================================================================

/// 合约主动买卖量命令
/// GET /futures/data/takerlongshortRatio
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TakerBuySellVolumeCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 周期类型
    pub period: PeriodType,
    /// 限制数量 (默认30, 最大500)
    pub limit: Option<i32>,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>
}

// ============================================================================
// 9. 其他命令
// ============================================================================

/// 历史BLVT NAV K线命令
/// GET /fapi/v1/lvtKlines
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HistoricalBLVTNAVKlineCandlestickCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 时间间隔
    pub interval: KlineInterval,
    /// 起始时间
    pub start_time: Option<i64>,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 限制数量 (默认500, 最大1000)
    pub limit: Option<i32>
}

/// 复合指数信息命令
/// GET /fapi/v1/constituents
/// Weight: 2
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompositeIndexSymbolInformationCmd {
    pub metadata: CMetadata,
    /// 交易对 (不填返回所有)
    pub symbol: Option<String>
}

/// 指数成分命令
/// GET /fapi/v1/constituents
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexConstituentsCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String
}

/// 多资产模式资产指数命令
/// GET /fapi/v1/assetIndex
/// Weight: 10
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultiAssetsModeAssetIndexCmd {
    pub metadata: CMetadata,
    /// 资产 (不填返回所有)
    pub symbol: Option<String>
}

/// 交割价格命令
/// GET /fapi/v1/deliveryPrice
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeliveryPriceCmd {
    pub metadata: CMetadata,
    /// 交易对 (不填返回所有)
    pub symbol: Option<String>
}

// ============================================================================
// 响应类型定义
// ============================================================================

/// Market Data 响应枚举
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureMarketDataRes {
    /// Ping响应
    Ping,
    /// 服务器时间响应
    ServerTime(ServerTimeResponse),
    /// 交易所信息响应
    ExchangeInfo(ExchangeInfoResponse),
    /// 订单簿响应
    OrderBook(OrderBookResponse),
    /// 成交列表响应
    Trades(Vec<TradeResponse>),
    /// 归集成交列表响应
    AggTrades(Vec<AggTradeResponse>),
    /// K线数据响应
    Klines(Vec<KlineResponse>),
    /// 标记价格响应
    MarkPrice(MarkPriceResponseData),
    /// 资金费率历史响应
    FundingRateHistory(Vec<FundingRateResponse>),
    /// 资金费率信息响应
    FundingRateInfo(Vec<FundingRateInfoResponse>),
    /// 24小时行情响应
    Ticker24hr(Ticker24hrResponseData),
    /// 价格响应
    Price(PriceResponseData),
    /// 最优挂单响应
    BookTicker(BookTickerResponseData),
    /// 持仓量响应
    OpenInterest(OpenInterestResponse),
    /// 持仓量统计响应
    OpenInterestStatistics(Vec<OpenInterestStatisticsResponse>),
    /// 多空比响应
    LongShortRatio(Vec<LongShortRatioResponse>),
    /// 主动买卖量响应
    TakerBuySellVolume(Vec<TakerBuySellVolumeResponse>),
    /// BLVT NAV K线响应
    BLVTKlines(Vec<KlineResponse>),
    /// 复合指数信息响应
    CompositeIndex(Vec<CompositeIndexResponse>),
    /// 资产指数响应
    AssetIndex(Vec<AssetIndexResponse>),
    /// 交割价格响应
    DeliveryPrice(Vec<DeliveryPriceResponse>)
}

/// 服务器时间响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ServerTimeResponse {
    pub server_time: i64
}

/// 交易所信息响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExchangeInfoResponse {
    pub timezone: String,
    pub server_time: i64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    pub assets: Vec<AssetInfo>,
    pub symbols: Vec<SymbolInfo>
}

/// 限流规则
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: i32,
    pub limit: i32
}

/// 交易所过滤器
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExchangeFilter {
    pub filter_type: String
}

/// 资产信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetInfo {
    pub asset: String,
    pub margin_available: bool,
    pub auto_asset_exchange: Option<String>
}

/// 交易对信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolInfo {
    pub symbol: String,
    pub pair: String,
    pub contract_type: String,
    pub delivery_date: i64,
    pub onboard_date: i64,
    pub status: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: String,
    pub price_precision: i32,
    pub quantity_precision: i32,
    pub base_asset_precision: i32,
    pub quote_precision: i32,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: i32,
    pub trigger_protect: String,
    pub liquidation_fee: String,
    pub market_take_bound: String,
    pub filters: Vec<SymbolFilter>,
    pub order_types: Vec<String>,
    pub time_in_force: Vec<String>
}

/// 交易对过滤器
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolFilter {
    pub filter_type: String
    // 其他字段根据filter_type动态解析
}

/// 订单簿响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderBookResponse {
    pub last_update_id: i64,
    pub message_output_time: i64,
    pub transaction_time: i64,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>
}

/// 价格档位
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceLevel {
    pub price: String,
    pub quantity: String
}

/// 成交响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TradeResponse {
    pub id: i64,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub time: i64,
    pub is_buyer_maker: bool
}

/// 归集成交响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AggTradeResponse {
    pub agg_trade_id: i64,
    pub price: String,
    pub quantity: String,
    pub first_trade_id: i64,
    pub last_trade_id: i64,
    pub timestamp: i64,
    pub is_buyer_maker: bool
}

/// K线响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KlineResponse {
    pub open_time: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: i64,
    pub quote_asset_volume: String,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
    pub ignore: String
}

/// 标记价格响应数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MarkPriceResponseData {
    Single(MarkPriceResponse),
    Multiple(Vec<MarkPriceResponse>)
}

/// 标记价格响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkPriceResponse {
    pub symbol: String,
    pub mark_price: String,
    pub index_price: String,
    pub estimated_settle_price: String,
    pub last_funding_rate: String,
    pub interest_rate: String,
    pub next_funding_time: i64,
    pub time: i64
}

/// 资金费率响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FundingRateResponse {
    pub symbol: String,
    pub funding_time: i64,
    pub funding_rate: String,
    pub mark_price: String
}

/// 资金费率信息响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FundingRateInfoResponse {
    pub symbol: String,
    pub adjusted_funding_rate_cap: String,
    pub adjusted_funding_rate_floor: String,
    pub funding_interval_hours: i32
}

/// 24小时行情响应数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ticker24hrResponseData {
    Single(Ticker24hrResponse),
    Multiple(Vec<Ticker24hrResponse>)
}

/// 24小时行情响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ticker24hrResponse {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub last_price: String,
    pub last_qty: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: i64
}

/// 价格响应数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PriceResponseData {
    Single(PriceResponse),
    Multiple(Vec<PriceResponse>)
}

/// 价格响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceResponse {
    pub symbol: String,
    pub price: String,
    pub time: Option<i64>
}

/// 最优挂单响应数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BookTickerResponseData {
    Single(BookTickerResponse),
    Multiple(Vec<BookTickerResponse>)
}

/// 最优挂单响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BookTickerResponse {
    pub symbol: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub time: i64
}

/// 持仓量响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenInterestResponse {
    pub open_interest: String,
    pub symbol: String,
    pub time: i64
}

/// 持仓量统计响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenInterestStatisticsResponse {
    pub symbol: String,
    pub sum_open_interest: String,
    pub sum_open_interest_value: String,
    pub timestamp: i64
}

/// 多空比响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LongShortRatioResponse {
    pub symbol: String,
    pub long_short_ratio: String,
    pub long_account: String,
    pub short_account: String,
    pub timestamp: i64
}

/// 主动买卖量响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TakerBuySellVolumeResponse {
    pub buy_sell_ratio: String,
    pub buy_vol: String,
    pub sell_vol: String,
    pub timestamp: i64
}

/// 复合指数响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompositeIndexResponse {
    pub symbol: String,
    pub time: i64,
    pub component: String,
    pub base_asset_list: Vec<BaseAssetInfo>
}

/// 基础资产信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseAssetInfo {
    pub base_asset: String,
    pub quote_asset: String,
    pub weight_in_quantity: String,
    pub weight_in_percentage: String
}

/// 资产指数响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetIndexResponse {
    pub symbol: String,
    pub time: i64,
    pub index: String,
    pub bid_buffer: String,
    pub ask_buffer: String,
    pub bid_rate: String,
    pub ask_rate: String,
    pub auto_exchange_bid_buffer: String,
    pub auto_exchange_ask_buffer: String,
    pub auto_exchange_bid_rate: String,
    pub auto_exchange_ask_rate: String
}

/// 交割价格响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeliveryPriceResponse {
    pub symbol: String,
    pub delivery_price: String,
    pub delivery_time: i64
}

// ============================================================================
// 错误类型定义
// ============================================================================

/// Market Data 命令错误
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsdsMFutureMarketDataCmdError {
    /// 参数验证错误
    InvalidParameter(String),
    /// 网络错误
    NetworkError(String),
    /// API错误
    ApiError { code: i32, msg: String },
    /// 限流错误
    RateLimitExceeded(String),
    /// 未知错误
    Unknown(String)
}

// ============================================================================
// Market Data 行为接口
// ============================================================================

/// USDS-M期货市场数据行为接口
pub trait UsdsMFutureMarketDataBehavior: Send + Sync {
    /// 处理市场数据命令
    fn handle(
        &mut self, cmd: UsdsMFutureMarketDataCmdAny
    ) -> Result<CmdResp<UsdsMFutureMarketDataRes>, UsdsMFutureMarketDataCmdError>;
}
