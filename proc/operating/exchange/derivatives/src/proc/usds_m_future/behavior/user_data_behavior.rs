// 参考 ## user data  Endpoints /Users/hongyaotang/src/rustlob/design/other/binance_derivatives_api/usds-margined-futures/account/rest-api 定义所有 user data 接口

use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};

/// User Data 命令枚举
#[derive(Debug, Clone)]
pub enum UsdsMFutureUserDataCmdAny {
    /// 账户信息查询 V3 - GET /fapi/v3/account
    /// Weight: 5
    AccountInfoV3(AccountInfoV3Cmd),

    /// 账户信息查询 V2 - GET /fapi/v2/account
    /// Weight: 5
    AccountInfoV2(AccountInfoV2Cmd),

    /// 账户余额查询 V3 - GET /fapi/v3/balance
    /// Weight: 5
    AccountBalanceV3(AccountBalanceV3Cmd),

    /// 账户余额查询 V2 - GET /fapi/v2/balance
    /// Weight: 5
    AccountBalanceV2(AccountBalanceV2Cmd),

    /// 获取当前持仓模式 - GET /fapi/v1/positionSide/dual
    /// Weight: 30
    GetPositionMode(GetPositionModeCmd),

    /// 变更持仓模式 - POST /fapi/v1/positionSide/dual
    /// Weight: 1
    ChangePositionMode(ChangePositionModeCmd),

    /// 获取多资产模式 - GET /fapi/v1/multiAssetsMargin
    /// Weight: 30
    GetMultiAssetsMode(GetMultiAssetsModeCmd),

    /// 变更多资产模式 - POST /fapi/v1/multiAssetsMargin
    /// Weight: 1
    ChangeMultiAssetsMode(ChangeMultiAssetsModeCmd),

    /// 用户佣金率 - GET /fapi/v1/commissionRate
    /// Weight: 20
    CommissionRate(CommissionRateCmd),

    /// 杠杆分层标准 - GET /fapi/v1/leverageBracket
    /// Weight: 1
    LeverageBracket(LeverageBracketCmd),

    /// 收入历史 - GET /fapi/v1/income
    /// Weight: 30
    IncomeHistory(IncomeHistoryCmd),

    /// 账户配置 - GET /fapi/v1/accountConfig
    /// Weight: 5
    AccountConfig(AccountConfigCmd),

    /// 交易规则指标 - GET /fapi/v1/apiTradingStatus
    /// Weight: 10
    TradingStatus(TradingStatusCmd),

    /// 账户成交历史 - GET /fapi/v1/userTrades
    /// Weight: 20
    UserTrades(UserTradesCmd),

    /// BNB抵扣状态 - GET /fapi/v1/feeBurn
    /// Weight: 1
    GetBNBBurnStatus(GetBNBBurnStatusCmd),

    /// 设置BNB抵扣 - POST /fapi/v1/feeBurn
    /// Weight: 1
    SetBNBBurn(SetBNBBurnCmd),

    /// 账户转账 - POST /fapi/v1/futures/transfer
    /// Weight: 1
    FuturesTransfer(FuturesTransferCmd),

    /// 转账历史 - GET /fapi/v1/futures/transfer
    /// Weight: 5
    TransferHistory(TransferHistoryCmd),

    /// 获取订单历史下载ID - GET /fapi/v1/order/asyn
    /// Weight: 5
    GetOrderDownloadId(GetOrderDownloadIdCmd),

    /// 获取订单下载链接 - GET /fapi/v1/order/asyn/id
    /// Weight: 5
    GetOrderDownloadLink(GetOrderDownloadLinkCmd),

    /// 获取成交历史下载ID - GET /fapi/v1/trade/asyn
    /// Weight: 5
    GetTradeDownloadId(GetTradeDownloadIdCmd),

    /// 获取成交下载链接 - GET /fapi/v1/trade/asyn/id
    /// Weight: 5
    GetTradeDownloadLink(GetTradeDownloadLinkCmd),

    /// 获取交易历史下载ID - GET /fapi/v1/income/asyn
    /// Weight: 5
    GetIncomeDownloadId(GetIncomeDownloadIdCmd),

    /// 获取交易历史下载链接 - GET /fapi/v1/income/asyn/id
    /// Weight: 5
    GetIncomeDownloadLink(GetIncomeDownloadLinkCmd),

    /// 查询限流规则 - GET /fapi/v1/rateLimit/order
    /// Weight: 20
    RateLimit(RateLimitCmd),

    /// 交易对配置 - GET /fapi/v1/symbolConfig
    /// Weight: 1
    SymbolConfig(SymbolConfigCmd),
}

// ==================== 账户信息 V3 ====================

/// 账户信息查询命令 V3
/// GET /fapi/v3/account
/// Weight: 5
/// 获取当前账户信息，单资产/多资产模式显示不同数据
#[derive(Debug, Clone)]
pub struct AccountInfoV3Cmd {
    pub metadata: CMetadata,
    /// 接收窗口（毫秒），不超过 60000
    pub recv_window: Option<i64>,
    /// 时间戳（毫秒）
    pub timestamp: i64,
}

/// 账户信息响应 V3
#[derive(Debug, Clone)]
pub struct AccountInfoV3Res {
    /// 总初始保证金（单资产模式仅USDT，多资产模式为USD价值）
    pub total_initial_margin: String,
    /// 总维持保证金
    pub total_maint_margin: String,
    /// 总钱包余额
    pub total_wallet_balance: String,
    /// 总未实现盈亏
    pub total_unrealized_profit: String,
    /// 总保证金余额
    pub total_margin_balance: String,
    /// 总持仓初始保证金
    pub total_position_initial_margin: String,
    /// 总挂单初始保证金
    pub total_open_order_initial_margin: String,
    /// 总全仓钱包余额
    pub total_cross_wallet_balance: String,
    /// 总全仓未实现盈亏
    pub total_cross_un_pnl: String,
    /// 可用余额
    pub available_balance: String,
    /// 最大可转出金额
    pub max_withdraw_amount: String,
    /// 资产列表
    pub assets: Vec<AccountAsset>,
    /// 持仓列表
    pub positions: Vec<AccountPosition>,
}

/// 账户资产信息
#[derive(Debug, Clone)]
pub struct AccountAsset {
    /// 资产名称
    pub asset: String,
    /// 钱包余额
    pub wallet_balance: String,
    /// 未实现盈亏
    pub unrealized_profit: String,
    /// 保证金余额
    pub margin_balance: String,
    /// 维持保证金
    pub maint_margin: String,
    /// 初始保证金
    pub initial_margin: String,
    /// 持仓初始保证金
    pub position_initial_margin: String,
    /// 挂单初始保证金
    pub open_order_initial_margin: String,
    /// 全仓钱包余额
    pub cross_wallet_balance: String,
    /// 全仓未实现盈亏
    pub cross_un_pnl: String,
    /// 可用余额
    pub available_balance: String,
    /// 最大可转出金额
    pub max_withdraw_amount: String,
    /// 是否可用作多资产模式保证金（仅多资产模式）
    pub margin_available: Option<bool>,
    /// 更新时间
    pub update_time: i64,
}

/// 账户持仓信息
#[derive(Debug, Clone)]
pub struct AccountPosition {
    /// 交易对
    pub symbol: String,
    /// 持仓方向（BOTH/LONG/SHORT）
    pub position_side: String,
    /// 持仓数量
    pub position_amt: String,
    /// 未实现盈亏
    pub unrealized_profit: String,
    /// 逐仓保证金
    pub isolated_margin: String,
    /// 名义价值
    pub notional: String,
    /// 逐仓钱包
    pub isolated_wallet: String,
    /// 初始保证金
    pub initial_margin: String,
    /// 维持保证金
    pub maint_margin: String,
    /// 更新时间
    pub update_time: i64,
}

// ==================== 账户信息 V2 ====================

/// 账户信息查询命令 V2
/// GET /fapi/v2/account
/// Weight: 5
#[derive(Debug, Clone)]
pub struct AccountInfoV2Cmd {
    pub metadata: CMetadata,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 账户余额 V3 ====================

/// 账户余额查询命令 V3
/// GET /fapi/v3/balance
/// Weight: 5
#[derive(Debug, Clone)]
pub struct AccountBalanceV3Cmd {
    pub metadata: CMetadata,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 账户余额 V2 ====================

/// 账户余额查询命令 V2
/// GET /fapi/v2/balance
/// Weight: 5
/// 查询账户余额信息
#[derive(Debug, Clone)]
pub struct AccountBalanceV2Cmd {
    pub metadata: CMetadata,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 账户余额响应 V2
#[derive(Debug, Clone)]
pub struct AccountBalanceV2Res {
    pub balances: Vec<BalanceInfo>,
}

/// 余额信息
#[derive(Debug, Clone)]
pub struct BalanceInfo {
    /// 账户别名
    pub account_alias: String,
    /// 资产名称
    pub asset: String,
    /// 钱包余额
    pub balance: String,
    /// 全仓钱包余额
    pub cross_wallet_balance: String,
    /// 全仓未实现盈亏
    pub cross_un_pnl: String,
    /// 可用余额
    pub available_balance: String,
    /// 最大可转出金额
    pub max_withdraw_amount: String,
    /// 是否可用作多资产模式保证金
    pub margin_available: bool,
    /// 更新时间
    pub update_time: i64,
}

// ==================== 持仓模式 ====================

/// 获取当前持仓模式
/// GET /fapi/v1/positionSide/dual
/// Weight: 30
#[derive(Debug, Clone)]
pub struct GetPositionModeCmd {
    pub metadata: CMetadata,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 持仓模式响应
#[derive(Debug, Clone)]
pub struct PositionModeRes {
    /// true: 双向持仓模式; false: 单向持仓模式
    pub dual_side_position: bool,
}

/// 变更持仓模式
/// POST /fapi/v1/positionSide/dual
/// Weight: 1
#[derive(Debug, Clone)]
pub struct ChangePositionModeCmd {
    pub metadata: CMetadata,
    /// true: 双向持仓模式; false: 单向持仓模式
    pub dual_side_position: bool,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 多资产模式 ====================

/// 获取多资产模式
/// GET /fapi/v1/multiAssetsMargin
/// Weight: 30
#[derive(Debug, Clone)]
pub struct GetMultiAssetsModeCmd {
    pub metadata: CMetadata,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 多资产模式响应
#[derive(Debug, Clone)]
pub struct MultiAssetsModeRes {
    /// true: 多资产模式开启; false: 多资产模式关闭
    pub multi_assets_margin: bool,
}

/// 变更多资产模式
/// POST /fapi/v1/multiAssetsMargin
/// Weight: 1
#[derive(Debug, Clone)]
pub struct ChangeMultiAssetsModeCmd {
    pub metadata: CMetadata,
    /// true: 开启; false: 关闭
    pub multi_assets_margin: bool,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 佣金率 ====================

/// 用户佣金率查询
/// GET /fapi/v1/commissionRate
/// Weight: 20
#[derive(Debug, Clone)]
pub struct CommissionRateCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 佣金率响应
#[derive(Debug, Clone)]
pub struct CommissionRateRes {
    /// 交易对
    pub symbol: String,
    /// Maker佣金率
    pub maker_commission_rate: String,
    /// Taker佣金率
    pub taker_commission_rate: String,
}

// ==================== 杠杆分层标准 ====================

/// 杠杆分层标准查询
/// GET /fapi/v1/leverageBracket
/// Weight: 1
#[derive(Debug, Clone)]
pub struct LeverageBracketCmd {
    pub metadata: CMetadata,
    /// 交易对（可选，不填查询所有）
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 杠杆分层响应
#[derive(Debug, Clone)]
pub struct LeverageBracketRes {
    pub brackets: Vec<SymbolLeverageBracket>,
}

/// 交易对杠杆分层
#[derive(Debug, Clone)]
pub struct SymbolLeverageBracket {
    /// 交易对
    pub symbol: String,
    /// 杠杆档位列表
    pub brackets: Vec<LeverageBracket>,
}

/// 杠杆档位
#[derive(Debug, Clone)]
pub struct LeverageBracket {
    /// 档位
    pub bracket: i32,
    /// 该档位的初始杠杆倍数
    pub initial_leverage: i32,
    /// 该档位名义价值上限
    pub notional_cap: String,
    /// 该档位名义价值下限
    pub notional_floor: String,
    /// 维持保证金率
    pub maint_margin_ratio: String,
    /// 速算数
    pub cum: String,
}

// ==================== 收入历史 ====================

/// 收入历史查询
/// GET /fapi/v1/income
/// Weight: 30
#[derive(Debug, Clone)]
pub struct IncomeHistoryCmd {
    pub metadata: CMetadata,
    /// 交易对（可选）
    pub symbol: Option<String>,
    /// 收入类型（可选）
    pub income_type: Option<String>,
    /// 起始时间（毫秒）
    pub start_time: Option<i64>,
    /// 结束时间（毫秒）
    pub end_time: Option<i64>,
    /// 返回数量限制，默认100，最大1000
    pub limit: Option<i32>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 收入记录
#[derive(Debug, Clone)]
pub struct IncomeRecord {
    /// 交易对
    pub symbol: String,
    /// 收入类型
    pub income_type: String,
    /// 收入金额
    pub income: String,
    /// 资产
    pub asset: String,
    /// 信息
    pub info: String,
    /// 时间
    pub time: i64,
    /// 交易ID
    pub tran_id: String,
    /// 交易ID
    pub trade_id: String,
}

// ==================== 账户配置 ====================

/// 账户配置查询
/// GET /fapi/v1/accountConfig
/// Weight: 5
#[derive(Debug, Clone)]
pub struct AccountConfigCmd {
    pub metadata: CMetadata,
    /// 交易对（可选）
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 交易规则指标 ====================

/// 交易规则指标查询
/// GET /fapi/v1/apiTradingStatus
/// Weight: 10
#[derive(Debug, Clone)]
pub struct TradingStatusCmd {
    pub metadata: CMetadata,
    /// 交易对（可选）
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 账户成交历史 ====================

/// 账户成交历史查询
/// GET /fapi/v1/userTrades
/// Weight: 20
#[derive(Debug, Clone)]
pub struct UserTradesCmd {
    pub metadata: CMetadata,
    /// 交易对
    pub symbol: String,
    /// 起始时间（毫秒）
    pub start_time: Option<i64>,
    /// 结束时间（毫秒）
    pub end_time: Option<i64>,
    /// 从哪个TradeId开始
    pub from_id: Option<i64>,
    /// 返回数量限制，默认500，最大1000
    pub limit: Option<i32>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== BNB抵扣 ====================

/// 获取BNB抵扣状态
/// GET /fapi/v1/feeBurn
/// Weight: 1
#[derive(Debug, Clone)]
pub struct GetBNBBurnStatusCmd {
    pub metadata: CMetadata,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// BNB抵扣状态响应
#[derive(Debug, Clone)]
pub struct BNBBurnStatusRes {
    /// 是否开启BNB抵扣
    pub fee_burn: bool,
}

/// 设置BNB抵扣
/// POST /fapi/v1/feeBurn
/// Weight: 1
#[derive(Debug, Clone)]
pub struct SetBNBBurnCmd {
    pub metadata: CMetadata,
    /// 是否开启BNB抵扣
    pub fee_burn: bool,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 账户转账 ====================

/// 账户转账
/// POST /fapi/v1/futures/transfer
/// Weight: 1
#[derive(Debug, Clone)]
pub struct FuturesTransferCmd {
    pub metadata: CMetadata,
    /// 资产类型
    pub asset: String,
    /// 金额
    pub amount: String,
    /// 转账类型（1:现货到USDT合约, 2:USDT合约到现货, 3:现货到币本位合约, 4:币本位合约到现货）
    pub transfer_type: i32,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 转账响应
#[derive(Debug, Clone)]
pub struct FuturesTransferRes {
    /// 转账ID
    pub tran_id: i64,
}

/// 转账历史查询
/// GET /fapi/v1/futures/transfer
/// Weight: 5
#[derive(Debug, Clone)]
pub struct TransferHistoryCmd {
    pub metadata: CMetadata,
    /// 资产类型
    pub asset: String,
    /// 起始时间（毫秒）
    pub start_time: i64,
    /// 结束时间（毫秒）
    pub end_time: Option<i64>,
    /// 当前页，默认1
    pub current: Option<i32>,
    /// 每页大小，默认10，最大100
    pub size: Option<i32>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 历史数据下载 ====================

/// 获取订单历史下载ID
/// GET /fapi/v1/order/asyn
/// Weight: 5
#[derive(Debug, Clone)]
pub struct GetOrderDownloadIdCmd {
    pub metadata: CMetadata,
    /// 起始时间（毫秒）
    pub start_time: i64,
    /// 结束时间（毫秒）
    pub end_time: i64,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 下载ID响应
#[derive(Debug, Clone)]
pub struct DownloadIdRes {
    /// 平均生成时间（毫秒）
    pub avg_cost_timestamp: String,
    /// 下载ID
    pub download_id: String,
}

/// 获取订单下载链接
/// GET /fapi/v1/order/asyn/id
/// Weight: 5
#[derive(Debug, Clone)]
pub struct GetOrderDownloadLinkCmd {
    pub metadata: CMetadata,
    /// 下载ID
    pub download_id: String,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 下载链接响应
#[derive(Debug, Clone)]
pub struct DownloadLinkRes {
    /// 下载链接
    pub download_url: String,
    /// 状态
    pub status: String,
    /// 过期时间（毫秒）
    pub expire_timestamp: i64,
}

/// 获取成交历史下载ID
/// GET /fapi/v1/trade/asyn
/// Weight: 5
#[derive(Debug, Clone)]
pub struct GetTradeDownloadIdCmd {
    pub metadata: CMetadata,
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 获取成交下载链接
/// GET /fapi/v1/trade/asyn/id
/// Weight: 5
#[derive(Debug, Clone)]
pub struct GetTradeDownloadLinkCmd {
    pub metadata: CMetadata,
    pub download_id: String,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 获取交易历史下载ID
/// GET /fapi/v1/income/asyn
/// Weight: 5
#[derive(Debug, Clone)]
pub struct GetIncomeDownloadIdCmd {
    pub metadata: CMetadata,
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

/// 获取交易历史下载链接
/// GET /fapi/v1/income/asyn/id
/// Weight: 5
#[derive(Debug, Clone)]
pub struct GetIncomeDownloadLinkCmd {
    pub metadata: CMetadata,
    pub download_id: String,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 限流规则 ====================

/// 查询限流规则
/// GET /fapi/v1/rateLimit/order
/// Weight: 20
#[derive(Debug, Clone)]
pub struct RateLimitCmd {
    pub metadata: CMetadata,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 交易对配置 ====================

/// 交易对配置查询
/// GET /fapi/v1/symbolConfig
/// Weight: 1
#[derive(Debug, Clone)]
pub struct SymbolConfigCmd {
    pub metadata: CMetadata,
    /// 交易对（可选）
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
    pub timestamp: i64,
}

// ==================== 响应枚举 ====================

/// User Data 响应枚举
#[derive(Debug, Clone)]
pub enum UsdsMFutureUserDataRes {
    /// 账户信息 V3
    AccountInfoV3(AccountInfoV3Res),
    /// 账户余额 V2
    AccountBalanceV2(AccountBalanceV2Res),
    /// 持仓模式
    PositionMode(PositionModeRes),
    /// 多资产模式
    MultiAssetsMode(MultiAssetsModeRes),
    /// 佣金率
    CommissionRate(CommissionRateRes),
    /// 杠杆分层
    LeverageBracket(LeverageBracketRes),
    /// 收入历史
    IncomeHistory(Vec<IncomeRecord>),
    /// BNB抵扣状态
    BNBBurnStatus(BNBBurnStatusRes),
    /// 转账响应
    FuturesTransfer(FuturesTransferRes),
    /// 下载ID
    DownloadId(DownloadIdRes),
    /// 下载链接
    DownloadLink(DownloadLinkRes),
    /// 通用成功响应
    Success,
}

// ==================== 错误类型 ====================

/// User Data 命令错误
#[derive(Debug, Clone)]
pub enum UsdsMFutureUserDataError {
    /// 无效参数
    InvalidParameter(String),
    /// API错误
    ApiError { code: i32, msg: String },
    /// 网络错误
    NetworkError(String),
    /// 未授权
    Unauthorized,
    /// 其他错误
    Other(String),
}

// ==================== 行为接口 ====================

/// User Data 行为接口
pub trait UsdsMFutureUserDataBehavior: Send + Sync {
    /// 处理 User Data 命令
    fn handle(
        &mut self,
        cmd: UsdsMFutureUserDataCmdAny,
    ) -> Result<CmdResp<UsdsMFutureUserDataRes>, UsdsMFutureUserDataError>;
}
