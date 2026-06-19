use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::hyperliquid_ws::ClearinghouseStateWire;

pub type ExtraFields = BTreeMap<String, JsonValue>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoRequestTypeProbe {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InfoErrorResponseWire {
    pub status: &'static str,
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CandleSnapshotReqWire {
    pub coin: String,
    pub interval: String,
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde(rename = "endTime")]
    pub end_time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum OidWire {
    Oid(u64),
    Cloid(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrontendOrderWire {
    pub coin: String,
    pub side: String,
    pub sz: String,
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    pub oid: u64,
    pub timestamp: u64,
    #[serde(rename = "isTrigger")]
    pub is_trigger: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "origSz")]
    pub orig_sz: String,
    #[serde(default)]
    pub children: Vec<JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderStatusEnvelopeWire {
    pub order: FrontendOrderWire,
    pub status: String,
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderLookupStatusWire {
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "unknownOid")]
    UnknownOid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderStatusResponseWire {
    KnownOrder {
        status: OrderLookupStatusWire,
        order: OrderStatusEnvelopeWire,
    },
    UnknownOid {
        status: OrderLookupStatusWire,
    },
}

pub type PortfolioPointWire = (u64, String);
pub type PortfolioPeriodWire = (String, PortfolioSliceWire);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortfolioSliceWire {
    #[serde(rename = "accountValueHistory", default)]
    pub account_value_history: Vec<PortfolioPointWire>,
    #[serde(rename = "pnlHistory", default)]
    pub pnl_history: Vec<PortfolioPointWire>,
    pub vlm: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowLendPositionWire {
    pub basis: String,
    pub value: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowLendStateWire {
    pub borrow: BorrowLendPositionWire,
    pub supply: BorrowLendPositionWire,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotBalanceWire {
    pub coin: String,
    pub token: u32,
    pub total: String,
    pub hold: String,
    #[serde(rename = "entryNtl")]
    pub entry_ntl: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotStateWire {
    #[serde(default)]
    pub balances: Vec<SpotBalanceWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubAccountWire {
    pub name: String,
    #[serde(rename = "subAccountUser")]
    pub sub_account_user: String,
    pub master: String,
    #[serde(rename = "clearinghouseState")]
    pub clearinghouse_state: ClearinghouseStateWire,
    #[serde(rename = "spotState")]
    pub spot_state: SpotStateWire,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultFollowerStateWire {
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultFollowerWire {
    pub user: String,
    #[serde(rename = "vaultEquity")]
    pub vault_equity: String,
    pub pnl: String,
    #[serde(rename = "allTimePnl")]
    pub all_time_pnl: String,
    #[serde(rename = "daysFollowing")]
    pub days_following: u64,
    #[serde(rename = "vaultEntryTime")]
    pub vault_entry_time: u64,
    #[serde(rename = "lockupUntil")]
    pub lockup_until: u64,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultRelationshipDataWire {
    #[serde(rename = "childAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub child_addresses: Vec<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultRelationshipWire {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<VaultRelationshipDataWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaultDetailsWire {
    pub name: String,
    #[serde(rename = "vaultAddress")]
    pub vault_address: String,
    pub leader: String,
    pub description: String,
    pub portfolio: Vec<PortfolioPeriodWire>,
    pub apr: f64,
    #[serde(rename = "followerState", default, skip_serializing_if = "Option::is_none")]
    pub follower_state: Option<VaultFollowerStateWire>,
    #[serde(rename = "leaderFraction")]
    pub leader_fraction: f64,
    #[serde(rename = "leaderCommission")]
    pub leader_commission: u64,
    pub followers: Vec<VaultFollowerWire>,
    #[serde(rename = "maxDistributable")]
    pub max_distributable: f64,
    #[serde(rename = "maxWithdrawable")]
    pub max_withdrawable: f64,
    #[serde(rename = "isClosed")]
    pub is_closed: bool,
    pub relationship: VaultRelationshipWire,
    #[serde(rename = "allowDeposits")]
    pub allow_deposits: bool,
    #[serde(rename = "alwaysCloseOnWithdraw")]
    pub always_close_on_withdraw: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferralReferredByWire {
    pub referrer: String,
    pub code: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferrerRewardStateWire {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: String,
    #[serde(rename = "claimedRewards")]
    pub claimed_rewards: String,
    #[serde(rename = "builderRewards")]
    pub builder_rewards: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferralStateWire {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    #[serde(rename = "cumRewardedFeesSinceReferred")]
    pub cum_rewarded_fees_since_referred: String,
    #[serde(rename = "cumFeesRewardedToReferrer")]
    pub cum_fees_rewarded_to_referrer: String,
    #[serde(rename = "timeJoined")]
    pub time_joined: u64,
    pub user: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferrerStateDataWire {
    pub code: String,
    #[serde(rename = "referralStates", default)]
    pub referral_states: Vec<ReferralStateWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferrerStateEnvelopeWire {
    pub stage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ReferrerStateDataWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferralWire {
    #[serde(rename = "referredBy", default, skip_serializing_if = "Option::is_none")]
    pub referred_by: Option<ReferralReferredByWire>,
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: String,
    #[serde(rename = "claimedRewards")]
    pub claimed_rewards: String,
    #[serde(rename = "builderRewards")]
    pub builder_rewards: String,
    #[serde(rename = "tokenToState")]
    pub token_to_state: (u32, ReferrerRewardStateWire),
    #[serde(rename = "referrerState")]
    pub referrer_state: ReferrerStateEnvelopeWire,
    #[serde(rename = "rewardHistory", default)]
    pub reward_history: Vec<JsonValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyUserVlmWire {
    pub date: String,
    #[serde(rename = "userCross")]
    pub user_cross: String,
    #[serde(rename = "userAdd")]
    pub user_add: String,
    pub exchange: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VipTierWire {
    #[serde(rename = "ntlCutoff")]
    pub ntl_cutoff: String,
    pub cross: String,
    pub add: String,
    #[serde(rename = "spotCross")]
    pub spot_cross: String,
    #[serde(rename = "spotAdd")]
    pub spot_add: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MmTierWire {
    #[serde(rename = "makerFractionCutoff")]
    pub maker_fraction_cutoff: String,
    pub add: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeScheduleTiersWire {
    #[serde(default)]
    pub vip: Vec<VipTierWire>,
    #[serde(default)]
    pub mm: Vec<MmTierWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakingDiscountTierWire {
    #[serde(rename = "bpsOfMaxSupply")]
    pub bps_of_max_supply: String,
    pub discount: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeScheduleWire {
    pub cross: String,
    pub add: String,
    #[serde(rename = "spotCross")]
    pub spot_cross: String,
    #[serde(rename = "spotAdd")]
    pub spot_add: String,
    pub tiers: FeeScheduleTiersWire,
    #[serde(rename = "referralDiscount")]
    pub referral_discount: String,
    #[serde(rename = "stakingDiscountTiers", default)]
    pub staking_discount_tiers: Vec<StakingDiscountTierWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserFeeTrialWire {
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakingLinkWire {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "stakingUser")]
    pub staking_user: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveStakingDiscountWire {
    #[serde(rename = "bpsOfMaxSupply")]
    pub bps_of_max_supply: String,
    pub discount: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserFeesWire {
    #[serde(rename = "dailyUserVlm", default)]
    pub daily_user_vlm: Vec<DailyUserVlmWire>,
    #[serde(rename = "feeSchedule")]
    pub fee_schedule: FeeScheduleWire,
    #[serde(rename = "userCrossRate")]
    pub user_cross_rate: String,
    #[serde(rename = "userAddRate")]
    pub user_add_rate: String,
    #[serde(rename = "userSpotCrossRate")]
    pub user_spot_cross_rate: String,
    #[serde(rename = "userSpotAddRate")]
    pub user_spot_add_rate: String,
    #[serde(rename = "activeReferralDiscount")]
    pub active_referral_discount: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trial: Option<UserFeeTrialWire>,
    #[serde(rename = "feeTrialReward")]
    pub fee_trial_reward: String,
    #[serde(rename = "nextTrialAvailableTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub next_trial_available_timestamp: Option<u64>,
    #[serde(rename = "stakingLink")]
    pub staking_link: StakingLinkWire,
    #[serde(rename = "activeStakingDiscount")]
    pub active_staking_discount: ActiveStakingDiscountWire,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatorDelegateWire {
    pub validator: String,
    pub amount: String,
    #[serde(rename = "isUndelegate")]
    pub is_undelegate: bool,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatorDeltaWire {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegate: Option<DelegatorDelegateWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DelegatorHistoryWire {
    pub time: u64,
    pub hash: String,
    pub delta: DelegatorDeltaWire,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserRoleDataWire {
    Agent {
        user: String,
        #[serde(flatten, default)]
        extra: ExtraFields,
    },
    SubAccount {
        master: String,
        #[serde(flatten, default)]
        extra: ExtraFields,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRoleResponseWire {
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<UserRoleDataWire>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowLendUserStateWire {
    #[serde(rename = "tokenToState")]
    pub token_to_state: Vec<(u32, BorrowLendStateWire)>,
    pub health: String,
    #[serde(rename = "healthFactor", default, skip_serializing_if = "Option::is_none")]
    pub health_factor: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
