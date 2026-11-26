use std::collections::BTreeMap;

use serde::Serialize;

use super::InfoRequestV2;
use super::wire::InfoRequestV2 as Request;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum InfoResponseV2 {
    AllMids(BTreeMap<String, String>),
    OpenOrders(Vec<OpenOrderResponseV2>),
    FrontendOpenOrders(Vec<FrontendOrderResponseV2>),
    UserFills(Vec<UserFillResponseV2>),
    UserFillsByTime(Vec<UserFillResponseV2>),
    UserRateLimit(UserRateLimitResponseV2),
    OrderStatus(OrderStatusResponseV2),
    L2Book(L2BookResponseV2),
    CandleSnapshot(Vec<CandleResponseV2>),
    MaxBuilderFee(u64),
    HistoricalOrders(Vec<HistoricalOrderResponseV2>),
    UserTwapSliceFills(Vec<UserTwapSliceFillResponseV2>),
    SubAccounts(Vec<SubAccountResponseV2>),
    VaultDetails(VaultDetailsResponseV2),
    UserVaultEquities(Vec<UserVaultEquityResponseV2>),
    UserRole(UserRoleResponseV2),
    Portfolio(Vec<PortfolioPeriodResponseV2>),
    Referral(ReferralResponseV2),
    UserFees(UserFeesResponseV2),
    Delegations(Vec<DelegationResponseV2>),
    DelegatorSummary(DelegatorSummaryResponseV2),
    DelegatorHistory(Vec<DelegatorHistoryResponseV2>),
    DelegatorRewards(Vec<DelegatorRewardResponseV2>),
    UserDexAbstraction(bool),
    UserAbstraction(String),
    AlignedQuoteTokenInfo(AlignedQuoteTokenInfoResponseV2),
    BorrowLendUserState(BorrowLendUserStateResponseV2),
    BorrowLendReserveState(BorrowLendReserveStateResponseV2),
    AllBorrowLendReserveStates(Vec<(u32, BorrowLendReserveStateResponseV2)>),
    ApprovedBuilders(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OpenOrderResponseV2 {
    pub coin: String,
    pub side: String,
    pub sz: String,
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    pub oid: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrontendOrderResponseV2 {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserFillResponseV2 {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub time: u64,
    #[serde(rename = "startPosition", skip_serializing_if = "Option::is_none")]
    pub start_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    #[serde(rename = "closedPnl", skip_serializing_if = "Option::is_none")]
    pub closed_pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<u64>,
    pub crossed: bool,
    pub fee: String,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_fee: Option<String>,
    pub tid: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserRateLimitResponseV2 {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    #[serde(rename = "nRequestsUsed")]
    pub n_requests_used: u64,
    #[serde(rename = "nRequestsCap")]
    pub n_requests_cap: u64,
    #[serde(rename = "nRequestsSurplus")]
    pub n_requests_surplus: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum OrderStatusResponseV2 {
    KnownOrder {
        status: OrderStatusKindV2,
        order: OrderStatusEnvelopeV2,
    },
    UnknownOid {
        status: OrderStatusKindV2,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum OrderStatusKindV2 {
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "unknownOid")]
    UnknownOid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderStatusEnvelopeV2 {
    pub order: FrontendOrderResponseV2,
    pub status: String,
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct L2BookResponseV2 {
    pub coin: String,
    pub time: u64,
    pub levels: (Vec<L2LevelResponseV2>, Vec<L2LevelResponseV2>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct L2LevelResponseV2 {
    pub px: String,
    pub sz: String,
    pub n: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CandleResponseV2 {
    pub t: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    pub s: String,
    pub i: String,
    pub o: String,
    pub c: String,
    pub h: String,
    pub l: String,
    pub v: String,
    pub n: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HistoricalOrderResponseV2 {
    pub order: FrontendOrderResponseV2,
    pub status: String,
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserTwapSliceFillResponseV2 {
    pub fill: UserTwapFillV2,
    #[serde(rename = "twapId")]
    pub twap_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserTwapFillV2 {
    #[serde(rename = "closedPnl")]
    pub closed_pnl: String,
    pub coin: String,
    pub crossed: bool,
    pub dir: String,
    pub hash: String,
    pub oid: u64,
    pub px: String,
    pub side: String,
    #[serde(rename = "startPosition")]
    pub start_position: String,
    pub sz: String,
    pub time: u64,
    pub fee: String,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
    pub tid: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubAccountResponseV2 {
    pub name: String,
    #[serde(rename = "subAccountUser")]
    pub sub_account_user: String,
    pub master: String,
    #[serde(rename = "clearinghouseState")]
    pub clearinghouse_state: ClearinghouseStateResponseV2,
    #[serde(rename = "spotState")]
    pub spot_state: SpotStateResponseV2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClearinghouseStateResponseV2 {
    #[serde(rename = "assetPositions")]
    pub asset_positions: Vec<AssetPositionResponseV2>,
    pub withdrawable: String,
    #[serde(rename = "marginSummary")]
    pub margin_summary: MarginSummaryResponseV2,
    #[serde(rename = "crossMarginSummary")]
    pub cross_margin_summary: MarginSummaryResponseV2,
    #[serde(rename = "crossMaintenanceMarginUsed")]
    pub cross_maintenance_margin_used: String,
    pub time: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AssetPositionResponseV2 {
    pub position: AssetPositionDetailResponseV2,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AssetPositionDetailResponseV2 {
    pub coin: String,
    #[serde(rename = "entryPx")]
    pub entry_px: Option<String>,
    pub leverage: LeverageResponseV2,
    #[serde(rename = "liquidationPx")]
    pub liquidation_px: Option<String>,
    #[serde(rename = "marginUsed")]
    pub margin_used: String,
    #[serde(rename = "positionValue")]
    pub position_value: String,
    #[serde(rename = "returnOnEquity")]
    pub return_on_equity: String,
    pub szi: String,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LeverageResponseV2 {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_usd: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MarginSummaryResponseV2 {
    #[serde(rename = "accountValue")]
    pub account_value: String,
    #[serde(rename = "totalNtlPos")]
    pub total_ntl_pos: String,
    #[serde(rename = "totalRawUsd")]
    pub total_raw_usd: String,
    #[serde(rename = "totalMarginUsed")]
    pub total_margin_used: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SpotStateResponseV2 {
    pub balances: Vec<SpotBalanceResponseV2>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SpotBalanceResponseV2 {
    pub coin: String,
    pub token: u32,
    pub total: String,
    pub hold: String,
    #[serde(rename = "entryNtl")]
    pub entry_ntl: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserVaultEquityResponseV2 {
    #[serde(rename = "vaultAddress")]
    pub vault_address: String,
    #[serde(rename = "equity")]
    pub equity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserRoleResponseV2 {
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PortfolioSliceResponseV2 {
    #[serde(rename = "accountValueHistory")]
    pub account_value_history: Vec<(u64, String)>,
    #[serde(rename = "pnlHistory")]
    pub pnl_history: Vec<(u64, String)>,
    pub vlm: String,
}

pub type PortfolioPeriodResponseV2 = (String, PortfolioSliceResponseV2);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferralResponseV2 {
    #[serde(rename = "referredBy", skip_serializing_if = "Option::is_none")]
    pub referred_by: Option<ReferralReferredByV2>,
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: String,
    #[serde(rename = "claimedRewards")]
    pub claimed_rewards: String,
    #[serde(rename = "builderRewards")]
    pub builder_rewards: String,
    #[serde(rename = "tokenToState")]
    pub token_to_state: (u32, ReferrerRewardStateV2),
    #[serde(rename = "referrerState")]
    pub referrer_state: ReferrerStateV2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferralReferredByV2 {
    pub referrer: String,
    pub code: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferrerRewardStateV2 {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    #[serde(rename = "unclaimedRewards")]
    pub unclaimed_rewards: String,
    #[serde(rename = "claimedRewards")]
    pub claimed_rewards: String,
    #[serde(rename = "builderRewards")]
    pub builder_rewards: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferrerStateV2 {
    pub stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ReferrerStateDataV2>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferrerStateDataV2 {
    pub code: String,
    #[serde(rename = "referralStates")]
    pub referral_states: Vec<ReferralStateV2>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferralStateV2 {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    #[serde(rename = "cumRewardedFeesSinceReferred")]
    pub cum_rewarded_fees_since_referred: String,
    #[serde(rename = "cumFeesRewardedToReferrer")]
    pub cum_fees_rewarded_to_referrer: String,
    #[serde(rename = "timeJoined")]
    pub time_joined: u64,
    pub user: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserFeesResponseV2 {
    #[serde(rename = "dailyUserVlm")]
    pub daily_user_vlm: Vec<DailyUserVlmV2>,
    #[serde(rename = "feeSchedule")]
    pub fee_schedule: FeeScheduleV2,
    #[serde(rename = "userCrossRate")]
    pub user_cross_rate: String,
    #[serde(rename = "userAddRate")]
    pub user_add_rate: String,
    #[serde(rename = "userSpotCrossRate")]
    pub user_spot_cross_rate: String,
    #[serde(rename = "userSpotAddRate")]
    pub user_spot_add_rate: String,
    #[serde(rename = "activeStakingDiscount")]
    pub active_staking_discount: Option<StakingDiscountV2>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DailyUserVlmV2 {
    pub date: String,
    #[serde(rename = "userCross")]
    pub user_cross: String,
    #[serde(rename = "userAdd")]
    pub user_add: String,
    pub exchange: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FeeScheduleV2 {
    pub cross: String,
    pub add: String,
    #[serde(rename = "spotCross")]
    pub spot_cross: String,
    #[serde(rename = "spotAdd")]
    pub spot_add: String,
    pub tiers: FeeTiersV2,
    #[serde(rename = "referralDiscount")]
    pub referral_discount: String,
    #[serde(rename = "stakingDiscountTiers")]
    pub staking_discount_tiers: Vec<StakingDiscountV2>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FeeTiersV2 {
    pub vip: Vec<VipTierV2>,
    pub mm: Vec<MmTierV2>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VipTierV2 {
    #[serde(rename = "ntlCutoff")]
    pub ntl_cutoff: String,
    pub cross: String,
    pub add: String,
    #[serde(rename = "spotCross")]
    pub spot_cross: String,
    #[serde(rename = "spotAdd")]
    pub spot_add: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MmTierV2 {
    #[serde(rename = "makerFractionCutoff")]
    pub maker_fraction_cutoff: String,
    pub add: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StakingDiscountV2 {
    #[serde(rename = "bpsOfMaxSupply")]
    pub bps_of_max_supply: String,
    pub discount: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DelegationResponseV2 {
    pub validator: String,
    pub amount: String,
    #[serde(rename = "lockedUntilTimestamp")]
    pub locked_until_timestamp: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DelegatorSummaryResponseV2 {
    pub delegated: String,
    pub undelegated: String,
    #[serde(rename = "totalPendingWithdrawal")]
    pub total_pending_withdrawal: String,
    #[serde(rename = "nPendingWithdrawals")]
    pub n_pending_withdrawals: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DelegatorHistoryResponseV2 {
    pub time: u64,
    pub hash: String,
    pub delta: DelegatorDeltaV2,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DelegatorDeltaV2 {
    pub delegate: Option<String>,
    pub undelegate: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DelegatorRewardResponseV2 {
    pub validator: String,
    pub amount: String,
    pub time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AlignedQuoteTokenInfoResponseV2 {
    #[serde(rename = "isAligned")]
    pub is_aligned: bool,
    #[serde(rename = "firstAlignedTime")]
    pub first_aligned_time: u64,
    #[serde(rename = "evmMintedSupply")]
    pub evm_minted_supply: String,
    #[serde(rename = "dailyAmountOwed")]
    pub daily_amount_owed: Vec<(String, String)>,
    #[serde(rename = "predictedRate")]
    pub predicted_rate: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BorrowLendPositionResponseV2 {
    pub basis: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BorrowLendStateResponseV2 {
    pub borrow: BorrowLendPositionResponseV2,
    pub supply: BorrowLendPositionResponseV2,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BorrowLendUserStateResponseV2 {
    #[serde(rename = "tokenToState")]
    pub token_to_state: Vec<(u32, BorrowLendStateResponseV2)>,
    pub health: String,
    #[serde(rename = "healthFactor", skip_serializing_if = "Option::is_none")]
    pub health_factor: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BorrowLendReserveStateResponseV2 {
    #[serde(rename = "borrowYearlyRate")]
    pub borrow_yearly_rate: String,
    #[serde(rename = "supplyYearlyRate")]
    pub supply_yearly_rate: String,
    pub balance: String,
    pub utilization: String,
    #[serde(rename = "oraclePx")]
    pub oracle_px: String,
    pub ltv: String,
    #[serde(rename = "totalSupplied")]
    pub total_supplied: String,
    #[serde(rename = "totalBorrowed")]
    pub total_borrowed: String,
}

impl InfoResponseV2 {
    pub(crate) fn for_request(request: &InfoRequestV2) -> Self {
        match request {
            Request::AllMids(_) => Self::AllMids(BTreeMap::from([
                ("APE".into(), "4.33245".into()),
                ("ARB".into(), "1.21695".into()),
            ])),
            Request::OpenOrders(_) => Self::OpenOrders(vec![OpenOrderResponseV2 {
                coin: "BTC".into(),
                side: "A".into(),
                sz: "0.0".into(),
                limit_px: "29792.0".into(),
                oid: 91490942,
                timestamp: 1681247412573,
            }]),
            Request::FrontendOpenOrders(_) => Self::FrontendOpenOrders(vec![frontend_order()]),
            Request::UserFills(_) => Self::UserFills(vec![fill()]),
            Request::UserFillsByTime(_) => Self::UserFillsByTime(vec![fill()]),
            Request::UserRateLimit(_) => Self::UserRateLimit(UserRateLimitResponseV2 {
                cum_vlm: "2854574.593578".into(),
                n_requests_used: 2890,
                n_requests_cap: 2864574,
                n_requests_surplus: 0,
            }),
            Request::OrderStatus(_) => Self::OrderStatus(OrderStatusResponseV2::UnknownOid {
                status: OrderStatusKindV2::UnknownOid,
            }),
            Request::L2Book(_) => Self::L2Book(L2BookResponseV2 {
                coin: "BTC".into(),
                time: 1754450974231,
                levels: (
                    vec![
                        L2LevelResponseV2 {
                            px: "113377.0".into(),
                            sz: "7.6699".into(),
                            n: 17,
                        },
                        L2LevelResponseV2 {
                            px: "113376.0".into(),
                            sz: "4.13714".into(),
                            n: 8,
                        },
                    ],
                    vec![L2LevelResponseV2 {
                        px: "113397.0".into(),
                        sz: "0.11543".into(),
                        n: 3,
                    }],
                ),
            }),
            Request::CandleSnapshot(_) => Self::CandleSnapshot(vec![CandleResponseV2 {
                t: 1681923600000,
                close_time: 1681924499999,
                s: "BTC".into(),
                i: "15m".into(),
                o: "29295.0".into(),
                c: "29258.0".into(),
                h: "29309.0".into(),
                l: "29250.0".into(),
                v: "0.98639".into(),
                n: 189,
            }]),
            Request::MaxBuilderFee(_) => Self::MaxBuilderFee(1),
            Request::HistoricalOrders(_) => {
                Self::HistoricalOrders(vec![HistoricalOrderResponseV2 {
                    order: frontend_order(),
                    status: "open".into(),
                    status_timestamp: 1681247412573,
                }])
            }
            Request::UserTwapSliceFills(_) => {
                Self::UserTwapSliceFills(vec![UserTwapSliceFillResponseV2 {
                    fill: UserTwapFillV2 {
                        closed_pnl: "0.0".into(),
                        coin: "AVAX".into(),
                        crossed: true,
                        dir: "Open Long".into(),
                        hash: "0x0000000000000000000000000000000000000000000000000000000000000000"
                            .into(),
                        oid: 90542681,
                        px: "18.435".into(),
                        side: "B".into(),
                        start_position: "26.86".into(),
                        sz: "93.53".into(),
                        time: 1681222254710,
                        fee: "0.01".into(),
                        fee_token: "USDC".into(),
                        tid: 118906512037719,
                    },
                    twap_id: 3156,
                }])
            }
            Request::SubAccounts(_) => Self::SubAccounts(vec![SubAccountResponseV2 {
                name: "sub-account".into(),
                sub_account_user: "0x0000000000000000000000000000000000000000".into(),
                master: "0x0000000000000000000000000000000000000000".into(),
                clearinghouse_state: ClearinghouseStateResponseV2 {
                    asset_positions: vec![],
                    withdrawable: "0.0".into(),
                    margin_summary: margin_summary(),
                    cross_margin_summary: margin_summary(),
                    cross_maintenance_margin_used: "0.0".into(),
                    time: 0,
                },
                spot_state: SpotStateResponseV2 { balances: vec![] },
            }]),
            Request::VaultDetails(_) => Self::VaultDetails(VaultDetailsResponseV2 {
                name: "Vault".into(),
                vault_address: "0x0000000000000000000000000000000000000000".into(),
                leader: "0x0000000000000000000000000000000000000000".into(),
                description: "".into(),
                portfolio: vec![],
                apr: 0.0,
                follower_state: None,
                leader_fraction: 0.0,
                leader_commission: 0,
                followers: vec![],
                max_distributable: 0.0,
                max_withdrawable: 0.0,
                is_closed: false,
                relationship: VaultRelationshipV2 {
                    type_: "none".into(),
                    data: None,
                },
                allow_deposits: true,
                always_close_on_withdraw: false,
            }),
            Request::UserVaultEquities(_) => {
                Self::UserVaultEquities(vec![UserVaultEquityResponseV2 {
                    vault_address: "0x0000000000000000000000000000000000000000".into(),
                    equity: "0.0".into(),
                }])
            }
            Request::UserRole(_) => Self::UserRole(UserRoleResponseV2 {
                role: "user".into(),
            }),
            Request::Portfolio(_) => Self::Portfolio(vec![
                (
                    "day".into(),
                    PortfolioSliceResponseV2 {
                        account_value_history: vec![
                            (1741886630493, "0.0".into()),
                            (1741895270493, "0.0".into()),
                        ],
                        pnl_history: vec![
                            (1741886630493, "0.0".into()),
                            (1741895270493, "0.0".into()),
                        ],
                        vlm: "0.0".into(),
                    },
                ),
                (
                    "week".into(),
                    PortfolioSliceResponseV2 {
                        account_value_history: vec![],
                        pnl_history: vec![],
                        vlm: "0.0".into(),
                    },
                ),
            ]),
            Request::Referral(_) => Self::Referral(ReferralResponseV2 {
                referred_by: None,
                cum_vlm: "0.0".into(),
                unclaimed_rewards: "0.0".into(),
                claimed_rewards: "0.0".into(),
                builder_rewards: "0.0".into(),
                token_to_state: (
                    0,
                    ReferrerRewardStateV2 {
                        cum_vlm: "0.0".into(),
                        unclaimed_rewards: "0.0".into(),
                        claimed_rewards: "0.0".into(),
                        builder_rewards: "0.0".into(),
                    },
                ),
                referrer_state: ReferrerStateV2 {
                    stage: "ready".into(),
                    data: None,
                },
            }),
            Request::UserFees(_) => Self::UserFees(UserFeesResponseV2 {
                daily_user_vlm: vec![],
                fee_schedule: FeeScheduleV2 {
                    cross: "0.00035".into(),
                    add: "0.0001".into(),
                    spot_cross: "0.0007".into(),
                    spot_add: "0.0004".into(),
                    tiers: FeeTiersV2 {
                        vip: vec![],
                        mm: vec![],
                    },
                    referral_discount: "0.0".into(),
                    staking_discount_tiers: vec![],
                },
                user_cross_rate: "0.00035".into(),
                user_add_rate: "0.0001".into(),
                user_spot_cross_rate: "0.0007".into(),
                user_spot_add_rate: "0.0004".into(),
                active_staking_discount: None,
            }),
            Request::Delegations(_) => Self::Delegations(vec![DelegationResponseV2 {
                validator: "0x0000000000000000000000000000000000000000".into(),
                amount: "0.0".into(),
                locked_until_timestamp: 0,
            }]),
            Request::DelegatorSummary(_) => Self::DelegatorSummary(DelegatorSummaryResponseV2 {
                delegated: "12060.16529862".into(),
                undelegated: "0.0".into(),
                total_pending_withdrawal: "0.0".into(),
                n_pending_withdrawals: 0,
            }),
            Request::DelegatorHistory(_) => {
                Self::DelegatorHistory(vec![DelegatorHistoryResponseV2 {
                    time: 0,
                    hash: "0x0000000000000000000000000000000000000000000000000000000000000000"
                        .into(),
                    delta: DelegatorDeltaV2 {
                        delegate: None,
                        undelegate: None,
                    },
                }])
            }
            Request::DelegatorRewards(_) => {
                Self::DelegatorRewards(vec![DelegatorRewardResponseV2 {
                    validator: "0x0000000000000000000000000000000000000000".into(),
                    amount: "0.0".into(),
                    time: 0,
                }])
            }
            Request::UserDexAbstraction(_) => Self::UserDexAbstraction(true),
            Request::UserAbstraction(_) => Self::UserAbstraction("unifiedAccount".into()),
            Request::AlignedQuoteTokenInfo(_) => {
                Self::AlignedQuoteTokenInfo(AlignedQuoteTokenInfoResponseV2 {
                    is_aligned: true,
                    first_aligned_time: 1758949452538,
                    evm_minted_supply: "0.0".into(),
                    daily_amount_owed: vec![("2025-10-04".into(), "0.0".into())],
                    predicted_rate: "0.0".into(),
                })
            }
            Request::BorrowLendUserState(_) => {
                Self::BorrowLendUserState(BorrowLendUserStateResponseV2 {
                    token_to_state: vec![(
                        0,
                        BorrowLendStateResponseV2 {
                            borrow: BorrowLendPositionResponseV2 {
                                basis: "0.0".into(),
                                value: "0.0".into(),
                            },
                            supply: BorrowLendPositionResponseV2 {
                                basis: "44.69295862".into(),
                                value: "44.69692314".into(),
                            },
                        },
                    )],
                    health: "healthy".into(),
                    health_factor: None,
                })
            }
            Request::BorrowLendReserveState(_) => Self::BorrowLendReserveState(reserve_state()),
            Request::AllBorrowLendReserveStates(_) => {
                Self::AllBorrowLendReserveStates(vec![(0, reserve_state())])
            }
            Request::ApprovedBuilders(_) => {
                Self::ApprovedBuilders(vec!["0x476fa87b4d3818f437f38f1263bee508d7672d82".into()])
            }
            Request::Unknown { .. } => unreachable!("unknown requests are rejected by validation"),
        }
    }
}

fn frontend_order() -> FrontendOrderResponseV2 {
    FrontendOrderResponseV2 {
        coin: "BTC".into(),
        side: "A".into(),
        sz: "5.0".into(),
        limit_px: "29792.0".into(),
        oid: 91490942,
        timestamp: 1681247412573,
        is_trigger: false,
        trigger_px: "0.0".into(),
        trigger_condition: "N/A".into(),
        is_position_tpsl: false,
        reduce_only: false,
        order_type: "Limit".into(),
        orig_sz: "5.0".into(),
        cloid: None,
    }
}
fn margin_summary() -> MarginSummaryResponseV2 {
    MarginSummaryResponseV2 {
        account_value: "0.0".into(),
        total_ntl_pos: "0.0".into(),
        total_raw_usd: "0.0".into(),
        total_margin_used: "0.0".into(),
    }
}
fn fill() -> UserFillResponseV2 {
    UserFillResponseV2 {
        coin: "AVAX".into(),
        px: "18.435".into(),
        sz: "93.53".into(),
        side: "B".into(),
        time: 1681222254710,
        start_position: Some("26.86".into()),
        dir: Some("Open Long".into()),
        closed_pnl: Some("0.0".into()),
        hash: Some("0xa166e3fa63c25663024b03f2e0da011a00307e4017465df020210d3d432e7cb8".into()),
        oid: Some(90542681),
        crossed: false,
        fee: "0.01".into(),
        fee_token: "USDC".into(),
        builder_fee: Some("0.01".into()),
        tid: 118906512037719,
    }
}
fn reserve_state() -> BorrowLendReserveStateResponseV2 {
    BorrowLendReserveStateResponseV2 {
        borrow_yearly_rate: "0.05".into(),
        supply_yearly_rate: "0.0008245002".into(),
        balance: "3245939.4732256099".into(),
        utilization: "0.018322226".into(),
        oracle_px: "1.0".into(),
        ltv: "0.0".into(),
        total_supplied: "3306509.7335290499".into(),
        total_borrowed: "60582.61869494".into(),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VaultDetailsResponseV2 {
    pub name: String,
    #[serde(rename = "vaultAddress")]
    pub vault_address: String,
    pub leader: String,
    pub description: String,
    pub portfolio: Vec<PortfolioPeriodResponseV2>,
    pub apr: f64,
    #[serde(rename = "followerState", skip_serializing_if = "Option::is_none")]
    pub follower_state: Option<VaultFollowerStateV2>,
    #[serde(rename = "leaderFraction")]
    pub leader_fraction: f64,
    #[serde(rename = "leaderCommission")]
    pub leader_commission: u64,
    pub followers: Vec<VaultFollowerV2>,
    #[serde(rename = "maxDistributable")]
    pub max_distributable: f64,
    #[serde(rename = "maxWithdrawable")]
    pub max_withdrawable: f64,
    #[serde(rename = "isClosed")]
    pub is_closed: bool,
    pub relationship: VaultRelationshipV2,
    #[serde(rename = "allowDeposits")]
    pub allow_deposits: bool,
    #[serde(rename = "alwaysCloseOnWithdraw")]
    pub always_close_on_withdraw: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VaultFollowerStateV2 {}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VaultFollowerV2 {
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
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VaultRelationshipV2 {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<VaultRelationshipDataV2>,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VaultRelationshipDataV2 {
    #[serde(rename = "childAddresses")]
    pub child_addresses: Vec<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn typed_fixtures_serialize_with_official_field_names_and_nonempty_shapes() {
        let fill = serde_json::to_value(InfoResponseV2::UserFills(vec![fill()])).unwrap();
        assert_eq!(fill[0]["startPosition"], "26.86");
        assert!(fill[0].get("start_position").is_none());
        let order =
            serde_json::to_value(InfoResponseV2::FrontendOpenOrders(vec![frontend_order()]))
                .unwrap();
        assert_eq!(order[0]["limitPx"], "29792.0");
        let book = serde_json::to_value(InfoResponseV2::L2Book(L2BookResponseV2 {
            coin: "BTC".into(),
            time: 1,
            levels: (vec![], vec![]),
        }))
        .unwrap();
        assert!(book.get("levels").is_some());
        assert_eq!(
            serde_json::to_value(OrderStatusResponseV2::UnknownOid {
                status: OrderStatusKindV2::UnknownOid
            })
            .unwrap(),
            json!({"status":"unknownOid"})
        );
        assert_eq!(
            serde_json::to_value(InfoResponseV2::MaxBuilderFee(1)).unwrap(),
            json!(1)
        );
    }
}
