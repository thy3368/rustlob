use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Hyperliquid 地址，使用 42 字符十六进制字符串表示。
pub type HyperliquidAddress = String;
/// Hyperliquid 资产标识，永续可能是 BTC，现货可能是 @107。
pub type HyperliquidCoin = String;
/// 客户端自定义订单号，通常为 16-byte hex string。
pub type HyperliquidCloid = String;
/// 链上或撮合事件哈希。
pub type HyperliquidHash = String;
/// 毫秒时间戳。
pub type HyperliquidMillis = u64;
/// 交易所订单号。
pub type HyperliquidOid = u64;
/// token 索引编号。
pub type HyperliquidTokenIndex = u32;

/// Hyperliquid `/info` 请求联合体。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HyperliquidInfoQuery {
    /// 查询所有币种的 mid price。
    #[serde(rename = "allMids", rename_all = "camelCase")]
    AllMids(HyperliquidAllMidsQuery),
    /// 查询用户当前挂单。
    #[serde(rename = "openOrders", rename_all = "camelCase")]
    OpenOrders(HyperliquidUserQueryWithOptionalDex),
    /// 查询带前端扩展字段的当前挂单。
    #[serde(rename = "frontendOpenOrders", rename_all = "camelCase")]
    FrontendOpenOrders(HyperliquidUserQueryWithOptionalDex),
    /// 查询用户成交记录。
    #[serde(rename = "userFills", rename_all = "camelCase")]
    UserFills(HyperliquidUserFillsQuery),
    /// 按时间区间查询用户成交记录。
    #[serde(rename = "userFillsByTime", rename_all = "camelCase")]
    UserFillsByTime(HyperliquidUserFillsByTimeQuery),
    /// 查询用户频率限制状态。
    #[serde(rename = "userRateLimit", rename_all = "camelCase")]
    UserRateLimit(HyperliquidUserQuery),
    /// 按 OID 或 CLOID 查询订单状态。
    #[serde(rename = "orderStatus", rename_all = "camelCase")]
    OrderStatus(HyperliquidOrderStatusQuery),
    /// 查询 L2 深度快照。
    #[serde(rename = "l2Book", rename_all = "camelCase")]
    L2Book(HyperliquidL2BookQuery),
    /// 查询 K 线快照。
    #[serde(rename = "candleSnapshot", rename_all = "camelCase")]
    CandleSnapshot(HyperliquidCandleSnapshotQuery),
    /// 查询 builder 最大费率授权。
    #[serde(rename = "maxBuilderFee", rename_all = "camelCase")]
    MaxBuilderFee(HyperliquidMaxBuilderFeeQuery),
    /// 查询历史订单。
    #[serde(rename = "historicalOrders", rename_all = "camelCase")]
    HistoricalOrders(HyperliquidUserQuery),
    /// 查询 TWAP 切片成交。
    #[serde(rename = "userTwapSliceFills", rename_all = "camelCase")]
    UserTwapSliceFills(HyperliquidUserQuery),
    /// 查询子账户列表。
    #[serde(rename = "subAccounts", rename_all = "camelCase")]
    SubAccounts(HyperliquidUserQuery),
    /// 查询 vault 详情。
    #[serde(rename = "vaultDetails", rename_all = "camelCase")]
    VaultDetails(HyperliquidVaultDetailsQuery),
    /// 查询用户 vault 权益。
    #[serde(rename = "userVaultEquities", rename_all = "camelCase")]
    UserVaultEquities(HyperliquidUserQuery),
    /// 查询用户角色。
    #[serde(rename = "userRole", rename_all = "camelCase")]
    UserRole(HyperliquidUserQuery),
    /// 查询用户组合收益曲线。
    #[serde(rename = "portfolio", rename_all = "camelCase")]
    Portfolio(HyperliquidUserQuery),
    /// 查询用户推荐信息。
    #[serde(rename = "referral", rename_all = "camelCase")]
    Referral(HyperliquidUserQuery),
    /// 查询用户手续费档位。
    #[serde(rename = "userFees", rename_all = "camelCase")]
    UserFees(HyperliquidUserQuery),
    /// 查询用户委托质押。
    #[serde(rename = "delegations", rename_all = "camelCase")]
    Delegations(HyperliquidUserQuery),
    /// 查询用户质押汇总。
    #[serde(rename = "delegatorSummary", rename_all = "camelCase")]
    DelegatorSummary(HyperliquidUserQuery),
    /// 查询用户质押历史。
    #[serde(rename = "delegatorHistory", rename_all = "camelCase")]
    DelegatorHistory(HyperliquidUserQuery),
    /// 查询用户质押奖励。
    #[serde(rename = "delegatorRewards", rename_all = "camelCase")]
    DelegatorRewards(HyperliquidUserQuery),
    /// 查询 HIP-3 DEX abstraction 状态。
    #[serde(rename = "userDexAbstraction", rename_all = "camelCase")]
    UserDexAbstraction(HyperliquidUserQuery),
    /// 查询用户 abstraction 状态。
    #[serde(rename = "userAbstraction", rename_all = "camelCase")]
    UserAbstraction(HyperliquidUserQuery),
    /// 查询 aligned quote token 状态。
    #[serde(rename = "alignedQuoteTokenInfo", rename_all = "camelCase")]
    AlignedQuoteTokenInfo(HyperliquidTokenQuery),
    /// 查询 borrow/lend 用户状态。
    #[serde(rename = "borrowLendUserState", rename_all = "camelCase")]
    BorrowLendUserState(HyperliquidUserQuery),
    /// 查询 borrow/lend 单个 reserve 状态。
    #[serde(rename = "borrowLendReserveState", rename_all = "camelCase")]
    BorrowLendReserveState(HyperliquidTokenQuery),
    /// 查询全部 borrow/lend reserve 状态。
    #[serde(rename = "allBorrowLendReserveStates", rename_all = "camelCase")]
    AllBorrowLendReserveStates(HyperliquidEmptyQuery),
    /// 查询用户已批准的 builders。
    #[serde(rename = "approvedBuilders", rename_all = "camelCase")]
    ApprovedBuilders(HyperliquidUserQuery),
}

/// 空请求体，仅用于依赖 `type` 区分的接口。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HyperliquidEmptyQuery;

/// 仅包含 user 的通用查询结构。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidUserQuery {
    /// 用户地址。
    pub user: HyperliquidAddress,
}

/// 包含 user 和可选 dex 的通用查询结构。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidUserQueryWithOptionalDex {
    /// 用户地址。
    pub user: HyperliquidAddress,
    /// 可选 dex 名称；为空时表示默认 dex。
    pub dex: Option<String>,
}

/// 仅包含 token 的通用查询结构。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidTokenQuery {
    /// token 索引。
    pub token: HyperliquidTokenIndex,
}

/// 查询全部 mids 的请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidAllMidsQuery {
    /// 可选 dex 名称；为空时表示默认 dex。
    pub dex: Option<String>,
}

/// 查询用户成交的请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidUserFillsQuery {
    /// 用户地址。
    pub user: HyperliquidAddress,
    /// 是否按时间聚合部分成交。
    pub aggregate_by_time: Option<bool>,
}

/// 查询用户成交时间区间的请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidUserFillsByTimeQuery {
    /// 用户地址。
    pub user: HyperliquidAddress,
    /// 起始时间，毫秒，包含边界。
    pub start_time: HyperliquidMillis,
    /// 结束时间，毫秒，包含边界。
    pub end_time: Option<HyperliquidMillis>,
    /// 是否按时间聚合部分成交。
    pub aggregate_by_time: Option<bool>,
}

/// 订单状态查询请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidOrderStatusQuery {
    /// 用户地址。
    pub user: HyperliquidAddress,
    /// 订单标识，支持 OID 或 CLOID。
    pub oid: HyperliquidOrderStatusQueryOid,
}

/// L2 深度查询请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidL2BookQuery {
    /// 资产标识。
    pub coin: HyperliquidCoin,
    /// 可选有效数字聚合。
    pub n_sig_figs: Option<u8>,
    /// 可选尾数聚合；仅在 nSigFigs=5 时允许。
    pub mantissa: Option<u8>,
}

/// K 线快照查询请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidCandleSnapshotQuery {
    /// 嵌套请求对象。
    pub req: HyperliquidCandleSnapshotRequest,
}

/// Builder 最大费率查询请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidMaxBuilderFeeQuery {
    /// 用户地址。
    pub user: HyperliquidAddress,
    /// Builder 地址。
    pub builder: HyperliquidAddress,
}

/// Vault 详情查询请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidVaultDetailsQuery {
    /// Vault 地址。
    pub vault_address: HyperliquidAddress,
    /// 可选用户地址，用于返回与该用户相关的上下文。
    pub user: Option<HyperliquidAddress>,
}

/// 订单状态查询里的 OID 字段，既可能是整数 OID，也可能是十六进制 CLOID。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperliquidOrderStatusQueryOid {
    /// 交易所订单号。
    Oid(HyperliquidOid),
    /// 客户端订单号。
    Cloid(HyperliquidCloid),
}

/// K 线快照内部请求对象。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidCandleSnapshotRequest {
    /// 资产标识。
    pub coin: HyperliquidCoin,
    /// K 线周期。
    pub interval: HyperliquidCandleInterval,
    /// 起始时间。
    pub start_time: HyperliquidMillis,
    /// 结束时间。
    pub end_time: HyperliquidMillis,
}

/// Hyperliquid 支持的 K 线周期。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HyperliquidCandleInterval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "2h")]
    TwoHours,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "3d")]
    ThreeDays,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1M")]
    OneMonth,
}

/// 查询全部 mid price 的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidAllMidsResponse {
    /// key 为资产标识，value 为 mid price。
    pub mids: HashMap<HyperliquidCoin, Decimal>,
}

/// 查询当前挂单的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidOpenOrdersResponse {
    /// 当前挂单列表。
    pub orders: Vec<HyperliquidOpenOrder>,
}

/// 查询前端扩展挂单的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidFrontendOpenOrdersResponse {
    /// 当前挂单列表。
    pub orders: Vec<HyperliquidFrontendOpenOrder>,
}

/// 查询用户成交的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidUserFillsResponse {
    /// 成交记录列表。
    pub fills: Vec<HyperliquidUserFill>,
}

/// 按时间查询用户成交的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidUserFillsByTimeResponse {
    /// 成交记录列表。
    pub fills: Vec<HyperliquidUserFill>,
}

/// 查询 K 线快照的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidCandleSnapshotResponse {
    /// K 线列表。
    pub candles: Vec<HyperliquidCandle>,
}

/// 查询历史订单的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidHistoricalOrdersResponse {
    /// 历史订单列表。
    pub orders: Vec<HyperliquidHistoricalOrder>,
}

/// 查询 TWAP 切片成交的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidUserTwapSliceFillsResponse {
    /// TWAP 切片成交列表。
    pub fills: Vec<HyperliquidTwapSliceFill>,
}

/// 查询子账户的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidSubAccountsResponse {
    /// 子账户列表。
    pub sub_accounts: Vec<HyperliquidSubAccount>,
}

/// 查询用户 vault 权益的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidUserVaultEquitiesResponse {
    /// 用户在各 vault 中的权益。
    pub equities: Vec<HyperliquidUserVaultEquity>,
}

/// 查询用户组合收益曲线的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidPortfolioResponse {
    /// 不同周期的组合数据。
    pub periods: Vec<HyperliquidPortfolioPeriodEntry>,
}

/// 组合收益曲线的单个周期条目。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidPortfolioPeriodEntry {
    /// 周期名称。
    pub period: HyperliquidPortfolioPeriod,
    /// 周期数据。
    pub data: HyperliquidPortfolioPeriodData,
}

/// 查询委托质押的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidDelegationsResponse {
    /// 委托质押列表。
    pub delegations: Vec<HyperliquidDelegation>,
}

/// 查询质押历史的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidDelegatorHistoryResponse {
    /// 质押历史列表。
    pub history: Vec<HyperliquidDelegatorHistoryItem>,
}

/// 查询质押奖励的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidDelegatorRewardsResponse {
    /// 质押奖励列表。
    pub rewards: Vec<HyperliquidDelegatorReward>,
}

/// 查询全部 borrow/lend reserve 的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidAllBorrowLendReserveStatesResponse {
    /// 每个 token 对应的 reserve 状态。
    pub reserves: Vec<HyperliquidBorrowLendReserveStateEntry>,
}

/// borrow/lend reserve 状态条目。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidBorrowLendReserveStateEntry {
    /// token 索引。
    pub token: HyperliquidTokenIndex,
    /// reserve 状态。
    pub state: HyperliquidBorrowLendReserveState,
}

/// 查询已批准 builders 的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidApprovedBuildersResponse {
    /// Builder 地址列表。
    pub builders: Vec<HyperliquidAddress>,
}

/// `/info` 返回值联合体。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperliquidInfoResponse {
    /// 全部 mids。
    AllMids(HyperliquidAllMidsResponse),
    /// 当前挂单。
    OpenOrders(HyperliquidOpenOrdersResponse),
    /// 带前端扩展字段的当前挂单。
    FrontendOpenOrders(HyperliquidFrontendOpenOrdersResponse),
    /// 用户成交。
    UserFills(HyperliquidUserFillsResponse),
    /// 用户速率限制。
    UserRateLimit(HyperliquidUserRateLimit),
    /// 订单状态。
    OrderStatus(HyperliquidOrderStatusResponse),
    /// L2 深度快照。
    L2Book(HyperliquidL2Book),
    /// K 线快照。
    CandleSnapshot(HyperliquidCandleSnapshotResponse),
    /// Builder 最大费率。
    MaxBuilderFee(u16),
    /// 历史订单。
    HistoricalOrders(HyperliquidHistoricalOrdersResponse),
    /// TWAP 切片成交。
    UserTwapSliceFills(HyperliquidUserTwapSliceFillsResponse),
    /// 子账户列表。
    SubAccounts(HyperliquidSubAccountsResponse),
    /// Vault 详情。
    VaultDetails(Box<HyperliquidVaultDetails>),
    /// 用户 vault 权益。
    UserVaultEquities(HyperliquidUserVaultEquitiesResponse),
    /// 用户角色。
    UserRole(HyperliquidUserRole),
    /// 组合收益曲线。
    Portfolio(HyperliquidPortfolioResponse),
    /// 推荐信息。
    Referral(Box<HyperliquidReferral>),
    /// 手续费信息。
    UserFees(Box<HyperliquidUserFees>),
    /// 质押委托。
    Delegations(HyperliquidDelegationsResponse),
    /// 质押汇总。
    DelegatorSummary(HyperliquidDelegatorSummary),
    /// 质押历史。
    DelegatorHistory(HyperliquidDelegatorHistoryResponse),
    /// 质押奖励。
    DelegatorRewards(HyperliquidDelegatorRewardsResponse),
    /// HIP-3 DEX abstraction 开关。
    UserDexAbstraction(bool),
    /// 用户 abstraction 状态。
    UserAbstraction(HyperliquidUserAbstraction),
    /// aligned quote token 信息。
    AlignedQuoteTokenInfo(HyperliquidAlignedQuoteTokenInfo),
    /// borrow/lend 用户状态。
    BorrowLendUserState(HyperliquidBorrowLendUserState),
    /// borrow/lend 单个 reserve 状态。
    BorrowLendReserveState(HyperliquidBorrowLendReserveState),
    /// 全部 borrow/lend reserve 状态。
    AllBorrowLendReserveStates(HyperliquidAllBorrowLendReserveStatesResponse),
    /// 已批准 builders。
    ApprovedBuilders(HyperliquidApprovedBuildersResponse),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidOpenOrder {
    pub coin: HyperliquidCoin,
    pub limit_px: Decimal,
    pub oid: HyperliquidOid,
    pub side: HyperliquidSide,
    pub sz: Decimal,
    pub timestamp: HyperliquidMillis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidFrontendOpenOrder {
    pub coin: HyperliquidCoin,
    pub is_position_tpsl: bool,
    pub is_trigger: bool,
    pub limit_px: Decimal,
    pub oid: HyperliquidOid,
    pub order_type: String,
    pub orig_sz: Decimal,
    pub reduce_only: bool,
    pub side: HyperliquidSide,
    pub sz: Decimal,
    pub timestamp: HyperliquidMillis,
    pub trigger_condition: String,
    pub trigger_px: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HyperliquidSide {
    B,
    A,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidUserFill {
    pub closed_pnl: Decimal,
    pub coin: HyperliquidCoin,
    pub crossed: bool,
    pub dir: String,
    pub hash: HyperliquidHash,
    pub oid: HyperliquidOid,
    pub px: Decimal,
    pub side: HyperliquidSide,
    pub start_position: Decimal,
    pub sz: Decimal,
    pub time: HyperliquidMillis,
    pub fee: Decimal,
    pub fee_token: String,
    pub builder_fee: Option<Decimal>,
    pub tid: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidUserRateLimit {
    pub cum_vlm: Decimal,
    pub n_requests_used: u64,
    pub n_requests_cap: u64,
    pub n_requests_surplus: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum HyperliquidOrderStatusResponse {
    #[serde(rename = "order", rename_all = "camelCase")]
    Order { order: HyperliquidOrderStatusData },
    #[serde(rename = "unknownOid")]
    UnknownOid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidOrderStatusData {
    pub order: HyperliquidFrontendOpenOrderWithChildren,
    pub status: HyperliquidHistoricalOrderStatus,
    pub status_timestamp: HyperliquidMillis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidFrontendOpenOrderWithChildren {
    pub coin: HyperliquidCoin,
    pub side: HyperliquidSide,
    pub limit_px: Decimal,
    pub sz: Decimal,
    pub oid: HyperliquidOid,
    pub timestamp: HyperliquidMillis,
    pub trigger_condition: String,
    pub is_trigger: bool,
    pub trigger_px: Decimal,
    pub children: Vec<HyperliquidFrontendOpenOrderWithChildren>,
    pub is_position_tpsl: bool,
    pub reduce_only: bool,
    pub order_type: String,
    pub orig_sz: Decimal,
    pub tif: Option<String>,
    pub cloid: Option<HyperliquidCloid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HyperliquidHistoricalOrderStatus {
    Open,
    Filled,
    Canceled,
    Triggered,
    Rejected,
    MarginCanceled,
    VaultWithdrawalCanceled,
    OpenInterestCapCanceled,
    SelfTradeCanceled,
    ReduceOnlyCanceled,
    SiblingFilledCanceled,
    DelistedCanceled,
    LiquidatedCanceled,
    ScheduledCancel,
    TickRejected,
    MinTradeNtlRejected,
    PerpMarginRejected,
    ReduceOnlyRejected,
    BadAloPxRejected,
    IocCancelRejected,
    BadTriggerPxRejected,
    MarketOrderNoLiquidityRejected,
    PositionIncreaseAtOpenInterestCapRejected,
    PositionFlipAtOpenInterestCapRejected,
    TooAggressiveAtOpenInterestCapRejected,
    OpenInterestIncreaseRejected,
    InsufficientSpotBalanceRejected,
    OracleRejected,
    PerpMaxPositionRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidL2Book {
    pub coin: HyperliquidCoin,
    pub time: HyperliquidMillis,
    pub levels: [Vec<HyperliquidL2Level>; 2],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidL2Level {
    pub px: Decimal,
    pub sz: Decimal,
    pub n: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidCandle {
    #[serde(rename = "T")]
    pub close_time: HyperliquidMillis,
    pub c: Decimal,
    pub h: Decimal,
    pub i: HyperliquidCandleInterval,
    pub l: Decimal,
    pub n: u64,
    pub o: Decimal,
    pub s: HyperliquidCoin,
    pub t: HyperliquidMillis,
    pub v: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidHistoricalOrder {
    pub order: HyperliquidFrontendOpenOrderWithChildren,
    pub status: HyperliquidHistoricalOrderStatus,
    pub status_timestamp: HyperliquidMillis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidTwapSliceFill {
    pub fill: HyperliquidUserFill,
    pub twap_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidSubAccount {
    pub name: String,
    pub sub_account_user: HyperliquidAddress,
    pub master: HyperliquidAddress,
    pub clearinghouse_state: HyperliquidClearinghouseState,
    pub spot_state: HyperliquidSpotState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidClearinghouseState {
    pub margin_summary: HyperliquidMarginSummary,
    pub cross_margin_summary: HyperliquidMarginSummary,
    pub cross_maintenance_margin_used: Decimal,
    pub withdrawable: Decimal,
    pub asset_positions: Vec<serde_json::Value>,
    pub time: HyperliquidMillis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidMarginSummary {
    pub account_value: Decimal,
    pub total_ntl_pos: Decimal,
    pub total_raw_usd: Decimal,
    pub total_margin_used: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidSpotState {
    pub balances: Vec<HyperliquidSpotBalance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidSpotBalance {
    pub coin: HyperliquidCoin,
    pub token: HyperliquidTokenIndex,
    pub total: Decimal,
    pub hold: Decimal,
    pub entry_ntl: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidVaultDetails {
    pub name: String,
    pub vault_address: HyperliquidAddress,
    pub leader: HyperliquidAddress,
    pub description: String,
    pub portfolio: HyperliquidPortfolioResponse,
    pub apr: Decimal,
    pub follower_state: Option<serde_json::Value>,
    pub leader_fraction: Decimal,
    pub leader_commission: Decimal,
    pub followers: Vec<HyperliquidVaultFollower>,
    pub max_distributable: Decimal,
    pub max_withdrawable: Decimal,
    pub is_closed: bool,
    pub relationship: HyperliquidVaultRelationship,
    pub allow_deposits: bool,
    pub always_close_on_withdraw: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidVaultFollower {
    pub user: HyperliquidAddress,
    pub vault_equity: Decimal,
    pub pnl: Decimal,
    pub all_time_pnl: Decimal,
    pub days_following: u64,
    pub vault_entry_time: HyperliquidMillis,
    pub lockup_until: HyperliquidMillis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum HyperliquidVaultRelationship {
    #[serde(rename = "parent", rename_all = "camelCase")]
    Parent { child_addresses: Vec<HyperliquidAddress> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidUserVaultEquity {
    pub vault_address: HyperliquidAddress,
    pub equity: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "role", content = "data")]
pub enum HyperliquidUserRole {
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "agent", rename_all = "camelCase")]
    Agent { user: HyperliquidAddress },
    #[serde(rename = "vault")]
    Vault,
    #[serde(rename = "subAccount", rename_all = "camelCase")]
    SubAccount { master: HyperliquidAddress },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HyperliquidPortfolioPeriod {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "allTime")]
    AllTime,
    #[serde(rename = "perpDay")]
    PerpDay,
    #[serde(rename = "perpWeek")]
    PerpWeek,
    #[serde(rename = "perpMonth")]
    PerpMonth,
    #[serde(rename = "perpAllTime")]
    PerpAllTime,
}

/// 账户价值历史中的一个采样点。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidTimeDecimalPoint {
    /// 时间戳。
    pub time: HyperliquidMillis,
    /// 数值。
    pub value: Decimal,
}

/// 推荐返佣状态中的 token 条目。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidReferralTokenStateEntry {
    /// token 索引。
    pub token: HyperliquidTokenIndex,
    /// token 对应状态。
    pub state: HyperliquidReferralTokenState,
}

/// 单日应付金额条目。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidDailyAmountOwedEntry {
    /// 日期字符串。
    pub date: String,
    /// 应付金额。
    pub amount: Decimal,
}

/// borrow/lend 用户状态中的 token 条目。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidBorrowLendTokenStateEntry {
    /// token 索引。
    pub token: HyperliquidTokenIndex,
    /// token 对应借贷状态。
    pub state: HyperliquidBorrowLendTokenState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidPortfolioPeriodData {
    pub account_value_history: Vec<HyperliquidTimeDecimalPoint>,
    pub pnl_history: Vec<HyperliquidTimeDecimalPoint>,
    pub vlm: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidReferral {
    pub referred_by: HyperliquidReferredBy,
    pub cum_vlm: Decimal,
    pub unclaimed_rewards: Decimal,
    pub claimed_rewards: Decimal,
    pub builder_rewards: Decimal,
    pub token_to_state: Vec<HyperliquidReferralTokenStateEntry>,
    pub referrer_state: HyperliquidReferrerState,
    pub reward_history: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidReferredBy {
    pub referrer: HyperliquidAddress,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidReferralTokenState {
    pub cum_vlm: Decimal,
    pub unclaimed_rewards: Decimal,
    pub claimed_rewards: Decimal,
    pub builder_rewards: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "stage", content = "data")]
pub enum HyperliquidReferrerState {
    #[serde(rename = "ready")]
    Ready(HyperliquidReferrerData),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidReferrerData {
    pub code: String,
    pub referral_states: Vec<HyperliquidReferralState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidReferralState {
    pub cum_vlm: Decimal,
    pub cum_rewarded_fees_since_referred: Decimal,
    pub cum_fees_rewarded_to_referrer: Decimal,
    pub time_joined: HyperliquidMillis,
    pub user: HyperliquidAddress,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidUserFees {
    pub daily_user_vlm: Vec<HyperliquidDailyUserVolume>,
    pub fee_schedule: HyperliquidFeeSchedule,
    pub user_cross_rate: Decimal,
    pub user_add_rate: Decimal,
    pub user_spot_cross_rate: Decimal,
    pub user_spot_add_rate: Decimal,
    pub active_referral_discount: Decimal,
    pub trial: Option<serde_json::Value>,
    pub fee_trial_reward: Decimal,
    pub next_trial_available_timestamp: Option<HyperliquidMillis>,
    pub staking_link: HyperliquidStakingLink,
    pub active_staking_discount: HyperliquidStakingDiscount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidDailyUserVolume {
    pub date: String,
    pub user_cross: Decimal,
    pub user_add: Decimal,
    pub exchange: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidFeeSchedule {
    pub cross: Decimal,
    pub add: Decimal,
    pub spot_cross: Decimal,
    pub spot_add: Decimal,
    pub tiers: HyperliquidFeeTiers,
    pub referral_discount: Decimal,
    pub staking_discount_tiers: Vec<HyperliquidStakingDiscount>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidFeeTiers {
    pub vip: Vec<HyperliquidVipFeeTier>,
    pub mm: Vec<HyperliquidMarketMakerFeeTier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidVipFeeTier {
    pub ntl_cutoff: Decimal,
    pub cross: Decimal,
    pub add: Decimal,
    pub spot_cross: Decimal,
    pub spot_add: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidMarketMakerFeeTier {
    pub maker_fraction_cutoff: Decimal,
    pub add: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidStakingDiscount {
    pub bps_of_max_supply: Decimal,
    pub discount: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum HyperliquidStakingLink {
    TradingUser { staking_user: HyperliquidAddress },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidDelegation {
    pub validator: HyperliquidAddress,
    pub amount: Decimal,
    pub locked_until_timestamp: HyperliquidMillis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidDelegatorSummary {
    pub delegated: Decimal,
    pub undelegated: Decimal,
    pub total_pending_withdrawal: Decimal,
    pub n_pending_withdrawals: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidDelegatorHistoryItem {
    pub time: HyperliquidMillis,
    pub hash: HyperliquidHash,
    pub delta: HyperliquidDelegatorHistoryDelta,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HyperliquidDelegatorHistoryDelta {
    Delegate(HyperliquidDelegateDelta),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidDelegateDelta {
    pub validator: HyperliquidAddress,
    pub amount: Decimal,
    pub is_undelegate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidDelegatorReward {
    pub time: HyperliquidMillis,
    pub source: String,
    pub total_amount: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HyperliquidUserAbstraction {
    #[serde(rename = "unifiedAccount")]
    UnifiedAccount,
    #[serde(rename = "portfolioMargin")]
    PortfolioMargin,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "dexAbstraction")]
    DexAbstraction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidAlignedQuoteTokenInfo {
    pub is_aligned: bool,
    pub first_aligned_time: HyperliquidMillis,
    pub evm_minted_supply: Decimal,
    pub daily_amount_owed: Vec<HyperliquidDailyAmountOwedEntry>,
    pub predicted_rate: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidBorrowLendUserState {
    pub token_to_state: Vec<HyperliquidBorrowLendTokenStateEntry>,
    pub health: String,
    pub health_factor: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidBorrowLendTokenState {
    pub borrow: HyperliquidBorrowLendPosition,
    pub supply: HyperliquidBorrowLendPosition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidBorrowLendPosition {
    pub basis: Decimal,
    pub value: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidBorrowLendReserveState {
    pub borrow_yearly_rate: Decimal,
    pub supply_yearly_rate: Decimal,
    pub balance: Decimal,
    pub utilization: Decimal,
    pub oracle_px: Decimal,
    pub ltv: Decimal,
    pub total_supplied: Decimal,
    pub total_borrowed: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperliquidInfoError {
    Message { error: String },
    Status { status: String },
}
