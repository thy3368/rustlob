use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::hyperliquid_ws::ClearinghouseStateWire;

/// 前向兼容保留字段。
///
/// 官方后续如果给对象新增字段，而我们当前还没有建模，就落到这里，
/// 避免反序列化因为未知字段失败。
pub type ExtraFields = BTreeMap<String, JsonValue>;

/// `/info` 请求体的最小探针。
///
/// 只先读取 `type`，用于把请求路由到具体 query handler。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoRequestTypeProbe {
    /// 官方 `/info` 请求类型名，如 `allMids`、`orderStatus`。
    #[serde(rename = "type")]
    pub type_: String,
}

/// `/info` 统一错误响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InfoErrorResponseWire {
    /// 固定错误状态，当前约定为 `"err"`。
    pub status: &'static str,
    /// 面向客户端的稳定错误文案。
    pub error: String,
}

/// `candleSnapshot.req` 的内层请求体。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CandleSnapshotReqWire {
    /// K 线所属交易标的，例如 `BTC`。
    pub coin: String,
    /// K 线周期，例如 `15m`、`1h`。
    pub interval: String,
    /// 查询起始时间，毫秒时间戳。
    #[serde(rename = "startTime")]
    pub start_time: u64,
    /// 查询结束时间，毫秒时间戳。
    #[serde(rename = "endTime")]
    pub end_time: u64,
}

/// Hyperliquid 订单标识。
///
/// 既支持数值型 `oid`，也支持字符串型 `cloid`。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum OidWire {
    /// 系统生成的数值订单 ID。
    Oid(u64),
    /// 客户端自定义订单 ID。
    Cloid(String),
}

/// `frontendOpenOrders`、`orderStatus`、`historicalOrders` 复用的前端订单视图。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrontendOrderWire {
    /// 订单所属币种/交易对简称。
    pub coin: String,
    /// 方向；官方样例里常见 `A` / `B`，分别表示 ask / bid 侧语义。
    pub side: String,
    /// 当前订单剩余数量，字符串保留官方精度。
    pub sz: String,
    /// 限价价格；市价单也可能回传参考价格字符串。
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    /// 系统订单 ID。
    pub oid: u64,
    /// 订单创建时间，毫秒时间戳。
    pub timestamp: u64,
    /// 是否为触发单。
    #[serde(rename = "isTrigger")]
    pub is_trigger: bool,
    /// 触发价格；非触发单通常为 `"0.0"`。
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    /// 触发条件文本，例如 `N/A`、止盈止损条件名。
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    /// 是否是仓位级 TP/SL 订单。
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    /// 是否只减仓，不允许增加仓位。
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    /// 订单类型文本，例如 `Limit`、`Market`。
    #[serde(rename = "orderType")]
    pub order_type: String,
    /// 原始下单数量，字符串保留官方精度。
    #[serde(rename = "origSz")]
    pub orig_sz: String,
    /// 子订单列表。
    ///
    /// 官方 shape 当前未稳定文档化，这里先保持原始 JSON 列表。
    #[serde(default)]
    pub children: Vec<JsonValue>,
    /// 客户端自定义订单 ID。
    ///
    /// 并非所有订单都有，所以保持可选。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
    /// Time in force 文本，例如 `FrontendMarket`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
}

/// `orderStatus.order` 与 `historicalOrders[]` 复用的订单状态包裹对象。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderStatusEnvelopeWire {
    /// 订单本体。
    pub order: FrontendOrderWire,
    /// 当前订单状态文本，例如 `filled`、`open`、`canceled`。
    pub status: String,
    /// 订单状态最后更新时间，毫秒时间戳。
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

/// `orderStatus.status` 的顶层判别值。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderLookupStatusWire {
    /// 查询命中了订单，响应中会带 `order` 详情。
    #[serde(rename = "order")]
    Order,
    /// 查询不到该 `oid` / `cloid`。
    #[serde(rename = "unknownOid")]
    UnknownOid,
}

/// `orderStatus` 的异形响应。
///
/// 官方有两种 shape：
/// 1. `{"status":"order","order":...}`
/// 2. `{"status":"unknownOid"}`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderStatusResponseWire {
    /// 成功查到订单。
    KnownOrder { status: OrderLookupStatusWire, order: OrderStatusEnvelopeWire },
    /// 查不到订单。
    UnknownOid { status: OrderLookupStatusWire },
}

/// 组合时序点 `(timestamp_ms, value_string)`。
///
/// 用于组合资金曲线、PnL 曲线等官方 tuple-list shape。
pub type PortfolioPointWire = (u64, String);

/// 组合周期切片 `(period_name, slice)`。
///
/// 例如 `("day", PortfolioSliceWire)`、`("week", PortfolioSliceWire)`。
pub type PortfolioPeriodWire = (String, PortfolioSliceWire);

/// `portfolio` 与 `vaultDetails.portfolio` 复用的单周期组合表现。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortfolioSliceWire {
    /// 账户净值曲线。
    ///
    /// 每个元素为 `(时间戳, 账户净值字符串)`。
    #[serde(rename = "accountValueHistory", default)]
    pub account_value_history: Vec<PortfolioPointWire>,
    /// 已实现/总 PnL 曲线。
    ///
    /// 每个元素为 `(时间戳, pnl 字符串)`。
    #[serde(rename = "pnlHistory", default)]
    pub pnl_history: Vec<PortfolioPointWire>,
    /// 该周期累计成交量，字符串保留官方精度。
    pub vlm: String,
    /// 官方未来可能追加的统计字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 借贷单边仓位。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowLendPositionWire {
    /// 该侧头寸的计息/记账基数。
    pub basis: String,
    /// 该侧当前估值。
    pub value: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 某个 token 的借贷状态。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowLendStateWire {
    /// 借入侧状态。
    pub borrow: BorrowLendPositionWire,
    /// 供应/出借侧状态。
    pub supply: BorrowLendPositionWire,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 现货余额项。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotBalanceWire {
    /// 币种符号，例如 `USDC`。
    pub coin: String,
    /// Hyperliquid 内部 token 编号。
    pub token: u32,
    /// 总余额。
    pub total: String,
    /// 已冻结/占用余额。
    pub hold: String,
    /// 入场名义价值或成本名义值。
    #[serde(rename = "entryNtl")]
    pub entry_ntl: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 子账户的现货状态。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotStateWire {
    /// 各币种现货余额列表。
    #[serde(default)]
    pub balances: Vec<SpotBalanceWire>,
    /// 官方未来可能追加的状态字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `subAccounts` 返回的单个子账户摘要。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubAccountWire {
    /// 子账户显示名。
    pub name: String,
    /// 子账户地址。
    #[serde(rename = "subAccountUser")]
    pub sub_account_user: String,
    /// 主账户地址。
    pub master: String,
    /// 子账户永续/清算所状态。
    #[serde(rename = "clearinghouseState")]
    pub clearinghouse_state: ClearinghouseStateWire,
    /// 子账户现货状态。
    #[serde(rename = "spotState")]
    pub spot_state: SpotStateWire,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 用户与某个 vault 的关系态。
///
/// 当前文档样例是 `null`，所以这里只保留前向兼容容器。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultFollowerStateWire {
    /// 官方未来如果补充 follower state 细节，统一落在这里。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `vaultDetails.followers[]` 中的跟投者摘要。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultFollowerWire {
    /// 跟投者地址。
    pub user: String,
    /// 当前在 vault 中的权益。
    #[serde(rename = "vaultEquity")]
    pub vault_equity: String,
    /// 当前跟投期间的 PnL。
    pub pnl: String,
    /// 历史累计 PnL。
    #[serde(rename = "allTimePnl")]
    pub all_time_pnl: String,
    /// 已连续跟投天数。
    #[serde(rename = "daysFollowing")]
    pub days_following: u64,
    /// 首次进入该 vault 的时间，毫秒时间戳。
    #[serde(rename = "vaultEntryTime")]
    pub vault_entry_time: u64,
    /// 锁仓到期时间，毫秒时间戳。
    #[serde(rename = "lockupUntil")]
    pub lockup_until: u64,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `vaultDetails.relationship.data` 的当前已知结构。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultRelationshipDataWire {
    /// 当当前 vault 是 parent 时，它管理的子地址列表。
    #[serde(rename = "childAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub child_addresses: Vec<String>,
    /// 官方未来可能追加的关系数据字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 用户与 vault 的关系描述。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultRelationshipWire {
    /// 关系类型，例如 `parent`。
    #[serde(rename = "type")]
    pub type_: String,
    /// 不同关系类型附带的数据。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<VaultRelationshipDataWire>,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `vaultDetails` 返回的 vault 明细。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultDetailsWire {
    /// Vault 名称。
    pub name: String,
    /// Vault 合约/账户地址。
    #[serde(rename = "vaultAddress")]
    pub vault_address: String,
    /// Leader 地址。
    pub leader: String,
    /// Vault 业务描述。
    pub description: String,
    /// 组合表现，保持官方 tuple-list shape。
    pub portfolio: Vec<PortfolioPeriodWire>,
    /// 年化收益率，官方当前返回 JSON number。
    pub apr: f64,
    /// 当前用户对该 vault 的 follower state；无数据时为 `null`。
    #[serde(rename = "followerState", default, skip_serializing_if = "Option::is_none")]
    pub follower_state: Option<VaultFollowerStateWire>,
    /// Leader 分润比例。
    #[serde(rename = "leaderFraction")]
    pub leader_fraction: f64,
    /// Leader 佣金档位/费率标识，当前样例为整数。
    #[serde(rename = "leaderCommission")]
    pub leader_commission: u64,
    /// 跟投者列表。
    pub followers: Vec<VaultFollowerWire>,
    /// 当前最多可分配金额。
    #[serde(rename = "maxDistributable")]
    pub max_distributable: f64,
    /// 当前最多可提取金额。
    #[serde(rename = "maxWithdrawable")]
    pub max_withdrawable: f64,
    /// Vault 是否已关闭。
    #[serde(rename = "isClosed")]
    pub is_closed: bool,
    /// 调用用户与该 vault 的关系。
    pub relationship: VaultRelationshipWire,
    /// 是否允许继续入金。
    #[serde(rename = "allowDeposits")]
    pub allow_deposits: bool,
    /// 提现时是否总是触发平仓。
    #[serde(rename = "alwaysCloseOnWithdraw")]
    pub always_close_on_withdraw: bool,
}

/// `referral.referredBy` 信息。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferralReferredByWire {
    /// 推荐人地址。
    pub referrer: String,
    /// 用户绑定的邀请码。
    pub code: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 推荐体系里某个 token 的累计奖励状态。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferrerRewardStateWire {
    /// 累计成交量。
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    /// 尚未领取的奖励。
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: String,
    /// 已领取的奖励。
    #[serde(rename = "claimedRewards")]
    pub claimed_rewards: String,
    /// Builder 奖励累计值。
    #[serde(rename = "builderRewards")]
    pub builder_rewards: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `referrerState.data.referralStates[]` 中的单个被推荐用户状态。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferralStateWire {
    /// 该被推荐用户贡献的累计成交量。
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    /// 自被推荐以来累计产生、且可参与奖励计算的手续费。
    #[serde(rename = "cumRewardedFeesSinceReferred")]
    pub cum_rewarded_fees_since_referred: String,
    /// 已经奖励给推荐人的累计手续费返佣。
    #[serde(rename = "cumFeesRewardedToReferrer")]
    pub cum_fees_rewarded_to_referrer: String,
    /// 加入推荐关系的时间，毫秒时间戳。
    #[serde(rename = "timeJoined")]
    pub time_joined: u64,
    /// 被推荐用户地址。
    pub user: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `referrerState.data` 主体。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferrerStateDataWire {
    /// 推荐码。
    pub code: String,
    /// 该推荐码名下的各用户状态。
    #[serde(rename = "referralStates", default)]
    pub referral_states: Vec<ReferralStateWire>,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 推荐人状态外层 envelope。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferrerStateEnvelopeWire {
    /// 推荐人当前阶段，例如 `ready`。
    pub stage: String,
    /// 阶段对应的附加数据。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ReferrerStateDataWire>,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `referral` 整体响应。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferralWire {
    /// 当前用户的上级推荐人信息；没有绑定时可能为 `null`。
    #[serde(rename = "referredBy", default, skip_serializing_if = "Option::is_none")]
    pub referred_by: Option<ReferralReferredByWire>,
    /// 用户累计成交量。
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    /// 用户未领取的推荐奖励。
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: String,
    /// 用户已领取的推荐奖励。
    #[serde(rename = "claimedRewards")]
    pub claimed_rewards: String,
    /// Builder 奖励累计值。
    #[serde(rename = "builderRewards")]
    pub builder_rewards: String,
    /// 某个 token 的奖励状态，保持官方二元 tuple shape `(token, state)`。
    #[serde(rename = "tokenToState")]
    pub token_to_state: (u32, ReferrerRewardStateWire),
    /// 当前用户作为推荐人的状态。
    #[serde(rename = "referrerState")]
    pub referrer_state: ReferrerStateEnvelopeWire,
    /// 奖励历史。
    ///
    /// 当前官方稳定字段不足，先保留原始 JSON 列表。
    #[serde(rename = "rewardHistory", default)]
    pub reward_history: Vec<JsonValue>,
}

/// `dailyUserVlm[]` 的单日成交量统计。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyUserVlmWire {
    /// 统计日期，格式通常为 `YYYY-MM-DD`。
    pub date: String,
    /// 用户 taker/cross 成交量。
    #[serde(rename = "userCross")]
    pub user_cross: String,
    /// 用户 maker/add 成交量。
    #[serde(rename = "userAdd")]
    pub user_add: String,
    /// 全站成交量。
    pub exchange: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// VIP 手续费档位。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VipTierWire {
    /// 进入该 VIP 档位所需的名义成交量门槛。
    #[serde(rename = "ntlCutoff")]
    pub ntl_cutoff: String,
    /// 永续/主市场 taker 费率。
    pub cross: String,
    /// 永续/主市场 maker 费率。
    pub add: String,
    /// 现货 taker 费率。
    #[serde(rename = "spotCross")]
    pub spot_cross: String,
    /// 现货 maker 费率。
    #[serde(rename = "spotAdd")]
    pub spot_add: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 做市商费率档位。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MmTierWire {
    /// 进入该 MM 档位所需的 maker 成交占比门槛。
    #[serde(rename = "makerFractionCutoff")]
    pub maker_fraction_cutoff: String,
    /// 该 MM 档位下的 maker 费率/返佣。
    pub add: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 手续费体系的分层集合。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeScheduleTiersWire {
    /// VIP 档位列表。
    #[serde(default)]
    pub vip: Vec<VipTierWire>,
    /// MM 档位列表。
    #[serde(default)]
    pub mm: Vec<MmTierWire>,
    /// 官方未来可能追加的 tier 分类。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 质押折扣档位。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakingDiscountTierWire {
    /// 质押量占最大供应量的基点占比。
    #[serde(rename = "bpsOfMaxSupply")]
    pub bps_of_max_supply: String,
    /// 对应折扣比例。
    pub discount: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 官方手续费计划表。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeScheduleWire {
    /// 基础永续/主市场 taker 费率。
    pub cross: String,
    /// 基础永续/主市场 maker 费率。
    pub add: String,
    /// 基础现货 taker 费率。
    #[serde(rename = "spotCross")]
    pub spot_cross: String,
    /// 基础现货 maker 费率。
    #[serde(rename = "spotAdd")]
    pub spot_add: String,
    /// 各档位费率表。
    pub tiers: FeeScheduleTiersWire,
    /// 推荐折扣比例。
    #[serde(rename = "referralDiscount")]
    pub referral_discount: String,
    /// 质押折扣档位列表。
    #[serde(rename = "stakingDiscountTiers", default)]
    pub staking_discount_tiers: Vec<StakingDiscountTierWire>,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 费率试用活动状态。
///
/// 当前样例是 `null`，因此只保留前向兼容容器。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserFeeTrialWire {
    /// 官方未来如果补充试用活动细节，统一落在这里。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 质押关系。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakingLinkWire {
    /// 关系类型，例如 `tradingUser`。
    #[serde(rename = "type")]
    pub type_: String,
    /// 与当前交易用户关联的质押用户地址。
    #[serde(rename = "stakingUser")]
    pub staking_user: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// 当前生效的质押折扣。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveStakingDiscountWire {
    /// 当前质押量占最大供应量的基点占比。
    #[serde(rename = "bpsOfMaxSupply")]
    pub bps_of_max_supply: String,
    /// 当前实际生效的折扣比例。
    pub discount: String,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `userFees` 整体响应。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserFeesWire {
    /// 逐日成交量统计。
    #[serde(rename = "dailyUserVlm", default)]
    pub daily_user_vlm: Vec<DailyUserVlmWire>,
    /// 官方费率计划表。
    #[serde(rename = "feeSchedule")]
    pub fee_schedule: FeeScheduleWire,
    /// 当前用户实际永续/主市场 taker 费率。
    #[serde(rename = "userCrossRate")]
    pub user_cross_rate: String,
    /// 当前用户实际永续/主市场 maker 费率。
    #[serde(rename = "userAddRate")]
    pub user_add_rate: String,
    /// 当前用户实际现货 taker 费率。
    #[serde(rename = "userSpotCrossRate")]
    pub user_spot_cross_rate: String,
    /// 当前用户实际现货 maker 费率。
    #[serde(rename = "userSpotAddRate")]
    pub user_spot_add_rate: String,
    /// 当前生效的推荐折扣比例。
    #[serde(rename = "activeReferralDiscount")]
    pub active_referral_discount: String,
    /// 当前费率试用活动状态。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trial: Option<UserFeeTrialWire>,
    /// 费率试用活动累计奖励。
    #[serde(rename = "feeTrialReward")]
    pub fee_trial_reward: String,
    /// 下一次可以参与试用活动的时间；没有限制时为 `null`。
    #[serde(
        rename = "nextTrialAvailableTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_trial_available_timestamp: Option<u64>,
    /// 质押关联关系。
    #[serde(rename = "stakingLink")]
    pub staking_link: StakingLinkWire,
    /// 当前生效的质押折扣。
    #[serde(rename = "activeStakingDiscount")]
    pub active_staking_discount: ActiveStakingDiscountWire,
}

/// `delegatorHistory.delta.delegate` 变体。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatorDelegateWire {
    /// 被委托的验证者地址。
    pub validator: String,
    /// 委托或撤委托数量。
    pub amount: String,
    /// 是否是撤委托动作。
    #[serde(rename = "isUndelegate")]
    pub is_undelegate: bool,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `delegatorHistory.delta`。
///
/// 当前只稳定建模 `delegate` 变体，其它未来动作通过 `extra` 承接。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatorDeltaWire {
    /// 委托/撤委托动作。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegate: Option<DelegatorDelegateWire>,
    /// 未来其它 delta 变体。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `delegatorHistory[]` 中的单条委托历史事件。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatorHistoryWire {
    /// 事件发生时间，毫秒时间戳。
    pub time: u64,
    /// 对应链上交易哈希。
    pub hash: String,
    /// 该次事件带来的委托状态变化。
    pub delta: DelegatorDeltaWire,
}

/// `userRole.data` 的异形数据体。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserRoleDataWire {
    /// 当 `role = "agent"` 时，关联的真实用户地址。
    Agent {
        /// Agent 归属的主用户地址。
        user: String,
        /// 官方未来可能追加的字段。
        #[serde(flatten, default)]
        extra: ExtraFields,
    },
    /// 当 `role = "subAccount"` 时，关联的 master 地址。
    SubAccount {
        /// 该子账户对应的主账户地址。
        master: String,
        /// 官方未来可能追加的字段。
        #[serde(flatten, default)]
        extra: ExtraFields,
    },
}

/// `userRole` 响应。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRoleResponseWire {
    /// 用户角色文本，例如 `user`、`agent`、`subAccount`。
    pub role: String,
    /// 某些角色附带的额外关系数据。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<UserRoleDataWire>,
}

/// `borrowLendUserState` 整体响应。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowLendUserStateWire {
    /// 各 token 的借贷状态，保持官方 tuple-list shape `(token, state)`。
    #[serde(rename = "tokenToState")]
    pub token_to_state: Vec<(u32, BorrowLendStateWire)>,
    /// 账户整体健康状态文本，例如 `healthy`。
    pub health: String,
    /// 健康因子；当前样例可能为 `null`，因此使用 `Option<String>`。
    #[serde(rename = "healthFactor", default, skip_serializing_if = "Option::is_none")]
    pub health_factor: Option<String>,
    /// 官方未来可能追加的字段。
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
