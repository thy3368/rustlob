// 参考 ## wallet Endpoints  /Users/hongyaotang/src/rustlob/design/other/binance_wallet_docs 定义所有 wallet 接口

use base_types::cqrs::cqrs_types::{CMetadata, CmdResp};
use serde::{Deserialize, Serialize};
use immutable_derive::immutable;
use serde_json::Value;
// ============================================================================
// CAPITAL ENDPOINTS - 钱包 (充提币)
// ============================================================================

/// 获取所有币信息命令
/// GET /sapi/v1/capital/config/getall
/// Weight: 10
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetAllCoinsCmd {
     metadata: CMetadata,
     recv_window: Option<u64>,
}

/// 提币命令
/// POST /sapi/v1/capital/withdraw/apply
/// Weight: 900 (UID)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct WithdrawCmd {
     metadata: CMetadata,
     coin: String,
     address: String,
     amount: String,
     network: Option<String>,
     address_tag: Option<String>,
     withdraw_order_id: Option<String>,
     transaction_fee_flag: Option<bool>,
     name: Option<String>,
     wallet_type: Option<u32>,
}

/// 获取提币历史命令
/// GET /sapi/v1/capital/withdraw/hisrec
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetWithdrawHistoryCmd {
     metadata: CMetadata,
     coin: Option<String>,
     status: Option<u32>,
     start_time: Option<i64>,
     end_time: Option<i64>,
     offset: Option<u32>,
     limit: Option<u32>,
     tx_id: Option<String>,
}

/// 获取充值历史命令
/// GET /sapi/v1/capital/deposit/hisrec
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetDepositHistoryCmd {
     metadata: CMetadata,
     coin: Option<String>,
     status: Option<u32>,
     start_time: Option<i64>,
     end_time: Option<i64>,
     offset: Option<u32>,
     limit: Option<u32>,
     tx_id: Option<String>,
     include_source: Option<bool>,
}

/// 获取充值地址命令
/// GET /sapi/v1/capital/deposit/address
/// Weight: 10
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetDepositAddressCmd {
     metadata: CMetadata,
     coin: String,
     network: Option<String>,
     amount: Option<String>,
}

// ============================================================================
// ASSET ENDPOINTS - 资产
// ============================================================================

/// 上架资产详情命令
/// GET /sapi/v1/asset/assetDetail
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetAssetDetailCmd {
     metadata: CMetadata,
     asset: Option<String>,
}

/// 用户持仓命令
/// POST /sapi/v3/asset/getUserAsset
/// Weight: 5
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetUserAssetsCmd {
     metadata: CMetadata,
     asset: Option<String>,
     need_btc_valuation: Option<bool>,

}

/// 用户万向划转命令
/// POST /sapi/v1/asset/transfer
/// Weight: 900 (UID)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct UniversalTransferCmd {
     metadata: CMetadata,
     transfer_type: String,
     asset: String,
     amount: String,
     from_symbol: Option<String>,
     to_symbol: Option<String>,

}

/// 查询用户万向划转历史命令
/// GET /sapi/v1/asset/transfer
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetUniversalTransferHistoryCmd {
     metadata: CMetadata,
     transfer_type: String,
     start_time: Option<i64>,
     end_time: Option<i64>,
     current: Option<u32>,
     size: Option<u32>,
     from_symbol: Option<String>,
     to_symbol: Option<String>,

}

/// 小额资产转换命令
/// POST /sapi/v1/asset/dust
/// Weight: 10
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct DustTransferCmd {
     metadata: CMetadata,
     asset: Vec<String>,
     account_type: Option<String>,
     recv_window: Option<u64>,
}

/// 交易手续费率查询命令
/// GET /sapi/v1/asset/tradeFee
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetTradeFeeCmd {
     metadata: CMetadata,
     symbol: Option<String>,

}

/// 查询资金账户命令
/// POST /sapi/v1/asset/get-funding-asset
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetFundingAssetCmd {
     metadata: CMetadata,
     asset: Option<String>,
     need_btc_valuation: Option<bool>,

}

// ============================================================================
// ACCOUNT ENDPOINTS - 账户
// ============================================================================

/// 帐户信息命令
/// GET /sapi/v1/account/info
/// Weight: 1
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetAccountInfoCmd {
     metadata: CMetadata,

}

/// 查询每日资产快照命令
/// GET /sapi/v1/accountSnapshot
/// Weight: 2400
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct GetAccountSnapshotCmd {
     metadata: CMetadata,
     snapshot_type: String, // "SPOT", "MARGIN", "FUTURES"
     start_time: Option<i64>,
     end_time: Option<i64>,
     limit: Option<u32>,

}

// ============================================================================
// COMMAND ENUM
// ============================================================================

/// Wallet 命令枚举
#[derive(Debug, Clone)]
pub enum WalletCmdAny {
    // Capital
    GetAllCoins(GetAllCoinsCmd),
    Withdraw(WithdrawCmd),
    GetWithdrawHistory(GetWithdrawHistoryCmd),
    GetDepositHistory(GetDepositHistoryCmd),
    GetDepositAddress(GetDepositAddressCmd),

    // Asset
    GetAssetDetail(GetAssetDetailCmd),
    GetUserAssets(GetUserAssetsCmd),
    UniversalTransfer(UniversalTransferCmd),
    GetUniversalTransferHistory(GetUniversalTransferHistoryCmd),
    DustTransfer(DustTransferCmd),
    GetTradeFee(GetTradeFeeCmd),
    GetFundingAsset(GetFundingAssetCmd),

    // Account
    GetAccountInfo(GetAccountInfoCmd),
    GetAccountSnapshot(GetAccountSnapshotCmd),
}

// ============================================================================
// RESPONSE TYPES
// ============================================================================

/// 币种信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct CoinInfo {
    coin: String,
    deposit_all_enable: bool,
    withdraw_all_enable: bool,
    name: String,
    free: String,
    locked: String,
    freeze: String,
    withdrawing: String,
    ipoing: String,
    ipoable: String,
    storage: String,
    is_legal_money: bool,
    trading: bool,
    network_list: Vec<NetworkInfo>,
}

/// 网络信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct NetworkInfo {
    network: String,
    coin: String,
    withdraw_integer_multiple: String,
    is_default: bool,
    deposit_enable: bool,
    withdraw_enable: bool,
    deposit_desc: Option<String>,
    withdraw_desc: Option<String>,
    special_tips: Option<String>,
    special_withdraw_tips: Option<String>,
    name: String,
    reset_address_status: bool,
    address_regex: String,
    memo_regex: String,
    withdraw_fee: String,
    withdraw_min: String,
    withdraw_max: String,
    withdraw_internal_min: String,
    deposit_dust: String,
    min_confirm: u32,
    un_lock_confirm: u32,
    same_address: bool,
    withdraw_tag: bool,
    estimated_arrival_time: u32,
    busy: bool,
    contract_address_url: Option<String>,
    contract_address: Option<String>,
    denomination: Option<u64>,
}

/// 提币响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct WithdrawResponse {
    id: String,
}

/// 提币历史记录
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct WithdrawRecord {
    id: String,
    amount: String,
    coin: String,
    network: String,
    status: u32,
    address: String,
    address_tag: String,
    tx_id: String,
    insert_time: i64,
    complete_time: Option<i64>,
    transfer_type: u32,
    confirm_times: String,
    unlock_confirm: u32,
    wallet_type: u32,
    travel_rule_status: u32,
}

/// 充值历史记录
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct DepositRecord {
    id: String,
    amount: String,
    coin: String,
    network: String,
    status: u32,
    address: String,
    address_tag: String,
    tx_id: String,
    insert_time: i64,
    complete_time: Option<i64>,
    transfer_type: u32,
    confirm_times: String,
    unlock_confirm: u32,
    wallet_type: u32,
    travel_rule_status: u32,
    source_address: Option<String>,
}

/// 充值地址响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct DepositAddressResponse {
    address: String,
    coin: String,
    tag: String,
    url: String,
}

/// 资产详情
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct AssetDetail {
    min_withdraw_amount: String,
    deposit_status: bool,
    withdraw_fee: String,
    withdraw_status: bool,
    deposit_tip: Option<String>,
}

/// 用户资产
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct UserAsset {
    asset: String,
    free: String,
    locked: String,
    freeze: String,
    withdrawing: String,
    ipoable: String,
    btc_valuation: Option<String>,
}

/// 万向划转响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct UniversalTransferResponse {
    tran_id: u64,
}

/// 万向划转历史记录
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct UniversalTransferRecord {
    tran_id: u64,
    from_account_type: String,
    to_account_type: String,
    asset: String,
    amount: String,
    status: String,
    create_time_stamp: i64,
}

/// 小额资产转换响应
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct DustTransferResponse {
    total_service_charge: String,
    total_transfered: String,
    transfer_result: Vec<DustTransferResult>,
}

/// 小额资产转换结果
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct DustTransferResult {
    amount: String,
    from_asset: String,
    operate_time: i64,
    service_charge_amount: String,
    tran_id: u64,
    transfered_amount: String,
}

/// 交易手续费
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct TradeFee {
    symbol: String,
    maker_commission: String,
    taker_commission: String,
}

/// 资金账户资产
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct FundingAsset {
    asset: String,
    free: String,
    locked: String,
    freeze: String,
    withdrawing: String,
    btc_valuation: Option<String>,
}

/// 账户信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct AccountInfo {
    vip_level: u32,
    is_margin_enabled: bool,
    is_future_enabled: bool,
    is_options_enabled: bool,
    is_portfolio_margin_retail_enabled: bool,
}

/// 账户快照
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct AccountSnapshot {
    code: u32,
    msg: String,
    snapshot_vos: Vec<SnapshotVo>,
}

/// 快照数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[immutable]
pub struct SnapshotVo {
    data: Value,
    snapshot_type: String,
    update_time: i64,
}

// ============================================================================
// RESPONSE ENUM
// ============================================================================

/// Wallet 响应枚举
#[derive(Debug, Clone)]
pub enum WalletRes {
    // Capital
    GetAllCoins(Vec<CoinInfo>),
    Withdraw(WithdrawResponse),
    GetWithdrawHistory(Vec<WithdrawRecord>),
    GetDepositHistory(Vec<DepositRecord>),
    GetDepositAddress(DepositAddressResponse),

    // Asset
    GetAssetDetail(std::collections::HashMap<String, AssetDetail>),
    GetUserAssets(Vec<UserAsset>),
    UniversalTransfer(UniversalTransferResponse),
    GetUniversalTransferHistory(Vec<UniversalTransferRecord>),
    DustTransfer(DustTransferResponse),
    GetTradeFee(Vec<TradeFee>),
    GetFundingAsset(Vec<FundingAsset>),

    // Account
    GetAccountInfo(AccountInfo),
    GetAccountSnapshot(AccountSnapshot),
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Wallet 命令错误
#[derive(Debug, Clone)]
pub enum WalletCmdError {
    InvalidParameter(String),
    NetworkError(String),
    AuthenticationError(String),
    RateLimitError(String),
    ServerError(String),
    UnknownError(String),
}

// ============================================================================
// BEHAVIOR TRAIT
// ============================================================================

/// Wallet 行为接口
pub trait WalletBehavior: Send + Sync {
    /// 处理 Wallet 命令
    fn handle(&mut self, cmd: WalletCmdAny) -> Result<CmdResp<WalletRes>, WalletCmdError>;
}
