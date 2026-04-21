
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Hyperliquid 用户抽象模式。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HyperliquidUserAbstraction {
    /// 统一账户模式。
    #[serde(rename = "unifiedAccount")]
    UnifiedAccount,
    /// 组合保证金模式。
    #[serde(rename = "portfolioMargin")]
    PortfolioMargin,
    /// 禁用抽象模式。
    #[serde(rename = "disabled")]
    Disabled,
    /// 默认模式。
    #[serde(rename = "default")]
    Default,
    /// DEX abstraction 模式。
    #[serde(rename = "dexAbstraction")]
    DexAbstraction,
}

/// Hyperliquid 订单有效期策略。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HyperliquidTif {
    /// ALO: 只做 Maker，若会立即成交则拒绝。
    Alo,
    /// IOC: 立即成交可成交部分，其余取消。
    Ioc,
    /// GTC: 挂单直到成交或手动取消。
    Gtc,
}

/// Hyperliquid 条件单触发方向。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HyperliquidTriggerKind {
    /// 止盈触发。
    TakeProfit,
    /// 止损触发。
    StopLoss,
}

/// Hyperliquid 条件单参数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidTrigger {
    /// 触发价格，金融场景必须避免使用 f64。
    pub trigger_px: Decimal,
    /// 触发后是否按市价执行。
    pub is_market: bool,
    /// 触发方向。
    pub kind: HyperliquidTriggerKind,
}

/// Hyperliquid 单笔订单类型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HyperliquidOrderRequest {
    /// 限价单。
    Limit { tif: HyperliquidTif },
    /// 条件触发单。
    Trigger(HyperliquidTrigger),
}

/// Builder 手续费配置。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidBuilderFee {
    /// Builder 地址。
    pub builder: String,
    /// 费率，单位为 0.1 bps。
    pub fee_tenths_of_bps: u16,
}

/// 单笔下单请求。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPlaceOrderCmd {
    /// 资产编号；现货使用 10000 + spot index，永续使用 meta universe index。
    pub asset: u32,
    /// 是否买单。
    pub is_buy: bool,
    /// 委托价格。
    pub price: Decimal,
    /// 委托数量。
    pub size: Decimal,
    /// 是否只减仓。
    pub reduce_only: bool,
    /// 订单类型。
    pub order_type: HyperliquidOrderRequest,
    /// 客户端自定义订单号，128-bit hex string。
    pub cloid: Option<String>,
}

/// 批量下单动作。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPlaceOrdersCmd {
    /// 批量订单列表。
    pub orders: Vec<HyperliquidPlaceOrderCmd>,
    /// 分组模式。
    pub grouping: String,
    /// 可选 builder 费率配置。
    pub builder: Option<HyperliquidBuilderFee>,
}

/// 按订单号撤单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidCancelOrderCmd {
    /// 资产编号。
    pub asset: u32,
    /// 订单号 OID。
    pub order_id: u64,
}

/// 批量按订单号撤单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidCancelOrdersCmd {
    /// 待撤单列表。
    pub cancels: Vec<HyperliquidCancelOrderCmd>,
}

/// 按客户端订单号撤单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidCancelByCloidCmd {
    /// 资产编号。
    pub asset: u32,
    /// 客户端订单号。
    pub cloid: String,
}

/// 批量按客户端订单号撤单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidCancelByCloidsCmd {
    /// 待撤单列表。
    pub cancels: Vec<HyperliquidCancelByCloidCmd>,
}

/// Dead Man's Switch 定时全撤。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidScheduleCancelCmd {
    /// 到达该毫秒时间戳后触发全撤。
    pub cancel_at_ms: u64,
}

/// 改单目标，可以按 OID 或 CLOID 定位。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HyperliquidModifyTarget {
    /// 按交易所订单号改单。
    OrderId(u64),
    /// 按客户端订单号改单。
    Cloid(String),
}

/// 单笔改单请求。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidModifyOrderCmd {
    /// 资产编号。
    pub asset: u32,
    /// 改单目标。
    pub target: HyperliquidModifyTarget,
    /// 修改后的新订单参数。
    pub order: HyperliquidPlaceOrderCmd,
}

/// 批量改单动作。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidModifyOrdersCmd {
    /// 待改单列表。
    pub modifies: Vec<HyperliquidModifyOrderCmd>,
}

/// 更新杠杆。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidUpdateLeverageCmd {
    /// 资产编号。
    pub asset: u32,
    /// 目标杠杆倍数。
    pub leverage: u32,
    /// 是否使用 cross 模式；false 表示 isolated。
    pub is_cross: bool,
}

/// 按金额调整 isolated margin。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidUpdateIsolatedMarginCmd {
    /// 资产编号。
    pub asset: u32,
    /// 预留字段；文档要求该值存在，但在当前版本暂无实际效果。
    pub is_buy: bool,
    /// 保证金增减金额，使用 6 位小数的整数表示。
    pub ntli: i64,
}

/// 按目标杠杆调整 isolated margin。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidUpdateIsolatedMarginByLeverageCmd {
    /// 资产编号。
    pub asset: u32,
    /// 目标杠杆倍数。
    pub leverage: Decimal,
}

/// Core USDC 转账。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidCoreUsdcTransferCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 目标地址。
    pub destination: String,
    /// 转账金额。
    pub amount: Decimal,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// Core 现货资产转账。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidCoreSpotTransferCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 目标地址。
    pub destination: String,
    /// token 标识，格式通常为 name:id。
    pub token: String,
    /// 转账数量。
    pub amount: Decimal,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// 提现请求。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidWithdrawCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 提现金额。
    pub amount: Decimal,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
    /// 提现目标地址。
    pub destination: String,
}

/// spot 与 perp 钱包之间的 USDC 划转。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidUsdClassTransferCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 划转金额。
    pub amount: Decimal,
    /// 可选子账户地址。
    pub subaccount: Option<String>,
    /// true 表示转入 perp，false 表示转回 spot。
    pub to_perp: bool,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// 泛化资产转账。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidSendAssetCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 目标地址。
    pub destination: String,
    /// 源 DEX；默认 perp 可用空字符串表示。
    pub source_dex: String,
    /// 目标 DEX；现货可用 "spot"。
    pub destination_dex: String,
    /// token 标识。
    pub token: String,
    /// 转账数量。
    pub amount: Decimal,
    /// 来源子账户地址；无则为空字符串或 None。
    pub from_subaccount: Option<String>,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// 发送到 EVM 并附带数据。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidSendToEvmCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// token 标识。
    pub token: String,
    /// 转账金额。
    pub amount: Decimal,
    /// 源 DEX 名称。
    pub source_dex: String,
    /// 目标接收者，采用 addressEncoding 指定格式。
    pub destination_recipient: String,
    /// 地址编码格式，hex 或 base58。
    pub address_encoding: String,
    /// 目标链 ID。
    pub destination_chain_id: u32,
    /// gas limit。
    pub gas_limit: u64,
    /// 附带数据负载。
    pub data: Vec<u8>,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// spot 与 staking 间转账。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidStakingTransferCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 转账数量，单位为 wei。
    pub wei: u128,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// 质押委托或撤销委托。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidDelegateCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 验证者地址或标识。
    pub validator: String,
    /// true 表示 undelegate。
    pub is_undelegate: bool,
    /// 委托数量，单位为 wei。
    pub wei: u128,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// Vault 存取款。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidVaultTransferCmd {
    /// Vault 地址。
    pub vault_address: String,
    /// true 表示 deposit，false 表示 withdraw。
    pub is_deposit: bool,
    /// 金额，文档字段名为 usd。
    pub usd: Decimal,
}

/// 批准 API Wallet / Agent Wallet。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidApproveAgentCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// Agent 地址。
    pub agent_address: String,
    /// 可选名称。
    pub agent_name: Option<String>,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// 批准 Builder 最大费率。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidApproveBuilderFeeCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 最大费率，文档格式为百分比字符串，如 0.001%。
    pub max_fee_rate: String,
    /// Builder 地址。
    pub builder: String,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// TWAP 下单。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidTwapOrderCmd {
    /// 资产编号。
    pub asset: u32,
    /// 是否买入。
    pub is_buy: bool,
    /// 总下单数量。
    pub size: Decimal,
    /// 是否只减仓。
    pub reduce_only: bool,
    /// 执行时长，单位分钟。
    pub minutes: u32,
    /// 是否随机化切片。
    pub randomize: bool,
}

/// 取消运行中的 TWAP。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidCancelTwapCmd {
    /// TWAP 标识。
    pub twap_id: u64,
}

/// 使待处理 nonce 失效。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidInvalidatePendingNonceCmd {
    /// 需要失效的 nonce。
    pub nonce: u64,
}

/// 预留附加动作额度。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidReserveAdditionalActionsCmd {
    /// 文档字段名为 weight。
    pub weight: u16,
}

/// 设置用户抽象模式。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidSetUserAbstractionCmd {
    /// Hyperliquid 链环境，如 Mainnet / Testnet。
    pub hyperliquid_chain: String,
    /// 签名所用链 ID，十六进制字符串。
    pub signature_chain_id: String,
    /// 用户或子账户地址。
    pub user: String,
    /// 抽象模式。
    pub abstraction: HyperliquidUserAbstraction,
    /// 动作内时间戳，必须与外层 nonce 一致。
    pub time: u64,
}

/// Agent 设置用户抽象模式。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidSetAgentUserAbstractionCmd {
    /// 抽象模式。
    pub abstraction: HyperliquidUserAbstraction,
}

/// Validator 对无风险利率投票。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidValidatorL1VoteCmd {
    /// 文档字段名为 riskFreeRate。
    pub risk_free_rate: Decimal,
}

/// 领取奖励。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidClaimRewardsCmd;

/// Hyperliquid 交易所动作枚举。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HyperliquidAction {
    /// 批量下单。
    PlaceOrders(HyperliquidPlaceOrdersCmd),
    /// 按 OID 批量撤单。
    CancelOrders(HyperliquidCancelOrdersCmd),
    /// 按 CLOID 批量撤单。
    CancelByCloids(HyperliquidCancelByCloidsCmd),
    /// 定时全撤。
    ScheduleCancel(HyperliquidScheduleCancelCmd),
    /// 批量改单。
    ModifyOrders(HyperliquidModifyOrdersCmd),
    /// 更新杠杆。
    UpdateLeverage(HyperliquidUpdateLeverageCmd),
    /// 按金额调整 isolated margin。
    UpdateIsolatedMargin(HyperliquidUpdateIsolatedMarginCmd),
    /// 按目标杠杆调整 isolated margin。
    UpdateIsolatedMarginByLeverage(HyperliquidUpdateIsolatedMarginByLeverageCmd),
    /// Core USDC 转账。
    CoreUsdcTransfer(HyperliquidCoreUsdcTransferCmd),
    /// Core 现货资产转账。
    CoreSpotTransfer(HyperliquidCoreSpotTransferCmd),
    /// 提现请求。
    Withdraw(HyperliquidWithdrawCmd),
    /// spot/perp USDC 划转。
    UsdClassTransfer(HyperliquidUsdClassTransferCmd),
    /// 泛化资产转账。
    SendAsset(HyperliquidSendAssetCmd),
    /// 转到 EVM 并附带数据。
    SendToEvm(HyperliquidSendToEvmCmd),
    /// 转入 staking。
    DepositIntoStaking(HyperliquidStakingTransferCmd),
    /// 从 staking 转出。
    WithdrawFromStaking(HyperliquidStakingTransferCmd),
    /// 委托或撤销委托。
    Delegate(HyperliquidDelegateCmd),
    /// Vault 存取款。
    VaultTransfer(HyperliquidVaultTransferCmd),
    /// 批准 Agent Wallet。
    ApproveAgent(HyperliquidApproveAgentCmd),
    /// 批准 Builder 费率。
    ApproveBuilderFee(HyperliquidApproveBuilderFeeCmd),
    /// TWAP 下单。
    TwapOrder(HyperliquidTwapOrderCmd),
    /// 撤销 TWAP。
    CancelTwap(HyperliquidCancelTwapCmd),
    /// 预留附加动作额度。
    ReserveAdditionalActions(HyperliquidReserveAdditionalActionsCmd),
    /// 使 nonce 失效。
    InvalidatePendingNonce(HyperliquidInvalidatePendingNonceCmd),
    /// 设置用户抽象模式。
    SetUserAbstraction(HyperliquidSetUserAbstractionCmd),
    /// Agent 设置用户抽象模式。
    SetAgentUserAbstraction(HyperliquidSetAgentUserAbstractionCmd),
    /// Validator 利率投票。
    ValidatorL1Vote(HyperliquidValidatorL1VoteCmd),
    /// 领取奖励。
    ClaimRewards(HyperliquidClaimRewardsCmd),
}

/// Hyperliquid 命令信封。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidCommandEnvelope {
    /// 用户侧 nonce，通常使用毫秒时间戳。
    pub nonce: u64,
    /// 代签场景下的 vault / subaccount 地址。
    pub vault_address: Option<String>,
    /// 可选过期时间，毫秒时间戳。
    pub expires_after_ms: Option<u64>,
    /// 具体动作。
    pub action: HyperliquidAction,
}

/// Hyperliquid 单笔订单执行状态。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HyperliquidOrderStatusType {
    /// 订单进入挂单簿。
    Resting,
    /// 订单已立即成交。
    Filled,
    /// 请求被拒绝。
    Error,
}

/// 挂单成功时的返回信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidRestingOrderStatus {
    /// 交易所订单号。
    pub oid: u64,
}

/// 成交成功时的返回信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidFilledOrderStatus {
    /// 总成交数量。
    pub total_sz: Decimal,
    /// 平均成交价格。
    pub avg_px: Decimal,
    /// 交易所订单号。
    pub oid: u64,
}

/// 错误返回信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidErrorStatus {
    /// 错误消息。
    pub error: String,
}

/// 单笔订单返回状态。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperliquidOrderStatus {
    /// 订单已挂单。
    Resting { resting: HyperliquidRestingOrderStatus },
    /// 订单已成交。
    Filled { filled: HyperliquidFilledOrderStatus },
    /// 订单处理失败。
    Error(HyperliquidErrorStatus),
}

/// 批量下单或批量改单的 data 字段。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidOrderStatusesData {
    /// 每笔订单的处理结果。
    pub statuses: Vec<HyperliquidOrderStatus>,
}

/// 批量撤单或按 cloid 撤单的单项结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperliquidCancelStatus {
    /// 成功时直接返回 success 字符串。
    Success(String),
    /// 失败时返回 error 对象。
    Error(HyperliquidErrorStatus),
}

/// 批量撤单 data 字段。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidCancelStatusesData {
    /// 各撤单项结果。
    pub statuses: Vec<HyperliquidCancelStatus>,
}

/// 定时全撤返回。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidScheduleCancelData {
    /// 文档当前未提供明确返回样例，因此仅作为保留定义，不纳入 ResponseData。
    pub status: Option<String>,
}

/// 通用成功响应，占位用于 default 类型接口。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperliquidDefaultResponse;

/// TWAP 运行中的返回信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidTwapRunningStatus {
    /// TWAP 标识。
    pub twap_id: u64,
}

/// TWAP 状态。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperliquidTwapStatus {
    /// TWAP 已开始运行。
    Running { running: HyperliquidTwapRunningStatus },
    /// TWAP 请求失败。
    Error(HyperliquidErrorStatus),
    /// TWAP 取消成功时直接返回 success 字符串。
    Success(String),
}

/// TWAP 下单返回。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidTwapOrderData {
    /// 状态对象。
    pub status: HyperliquidTwapStatus,
}

/// TWAP 撤单返回。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidTwapCancelData {
    /// 状态对象或 success 字符串。
    pub status: HyperliquidTwapStatus,
}

/// Hyperliquid exchange 接口 data 联合体。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperliquidResponseData {
    /// 下单或改单返回订单状态列表。
    OrderStatuses(HyperliquidOrderStatusesData),
    /// 撤单或按 cloid 撤单返回状态列表。
    CancelStatuses(HyperliquidCancelStatusesData),
    /// TWAP 下单返回。
    TwapOrder(HyperliquidTwapOrderData),
    /// TWAP 撤单返回。
    TwapCancel(HyperliquidTwapCancelData),
}

/// Hyperliquid 内层 response 包装。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidTypedResponse {
    /// 动作类型，文档示例包括 order / cancel / twapOrder / twapCancel / default。
    pub r#type: String,
    /// 动作返回数据；default 类型时不存在。
    pub data: Option<HyperliquidResponseData>,
}

/// Hyperliquid 标准成功响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidSuccessResponse {
    /// 响应状态，通常为 ok。
    pub status: String,
    /// 具体返回数据。
    pub response: HyperliquidTypedResponse,
}

/// Hyperliquid 标准错误响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidErrorResponse {
    /// 错误消息。
    pub error: String,
}

/// Hyperliquid exchange endpoint 响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperliquidResponse {
    /// 成功响应。
    Success(HyperliquidSuccessResponse),
    /// 错误响应。
    Error(HyperliquidErrorResponse),
}
