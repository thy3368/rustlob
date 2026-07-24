mod market_rules;
mod reservation;

pub mod hyperliquid_account;
pub mod perp;

pub mod spot;

pub mod account;

pub use account::account::{Account, AccountStatus};
pub use account::balance::{Balance, BalanceError};
pub use account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2 as BalanceLedgerEntry, BalanceLedgerEntryV2, BalanceLedgerEntryV2Error,
    BalanceLedgerOperation,
};
pub use account::balance_ledger_reason::BalanceLedgerReason;
pub use account::settlement_transfer_voucher::{
    SettlementKind, SettlementTransferLeg, SettlementTransferPurpose, SettlementTransferSummary,
    SettlementTransferVoucher,
};
pub use hyperliquid_account::{
    AccountId, AssetId, MarginSummary, MasterAccount, PerpAssetId, PerpAssetRiskRule,
    PerpClearinghouseState, PerpClearinghouseStateCalcError, PerpClearinghouseStateCalcInput,
    PerpCollateralSnapshot, PerpMarketMark, PerpPositionRiskSnapshot, PerpRiskPolicy, RiskState,
    SpotClearinghouseState, SubAccountProfile, SubAccountSnapshot, SubAccountSnapshotError,
};
pub use market_rules::MarketRules;
pub use perp::fund::hyperliquid_perp_funding_settlement::HyperliquidPerpFundingSettlement;
pub use perp::hyperliquid_perp_adl_batch::{
    HyperliquidPerpAdlBatch, HyperliquidPerpAdlBatchStatus,
};
pub use perp::hyperliquid_perp_adl_deleveraging_record::HyperliquidPerpAdlDeleveragingRecord;
pub use perp::hyperliquid_perp_adl_execution::{
    HyperliquidPerpAdlExecution, HyperliquidPerpAdlExecutionStatus,
};
pub use perp::hyperliquid_perp_funding_rate::{
    HyperliquidPerpBookLevel, HyperliquidPerpFundingRateError, HyperliquidPerpFundingSample,
    compute_hourly_funding_rate_e8, compute_impact_ask_price, compute_impact_bid_price,
};
pub use perp::hyperliquid_perp_insurance_fund_allocation::HyperliquidPerpInsuranceFundAllocation;
pub use perp::hyperliquid_perp_liquidation::{
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus,
    HyperliquidPerpLiquidationTriggerReason,
};
pub use perp::hyperliquid_perp_liquidation_fill::HyperliquidPerpLiquidationFill;
pub use perp::hyperliquid_perp_shortfall::{
    HyperliquidPerpShortfall, HyperliquidPerpShortfallStatus,
};
pub use perp::trade::hyperliquid_perp_leverage_setting::{
    HyperliquidPerpLeverageSetting, HyperliquidPerpLeverageSettingError,
};
pub use perp::trade::hyperliquid_perp_order::{
    HyperliquidPerpOrder, HyperliquidPerpOrderBehaviorError, HyperliquidPerpOrderExecution,
    HyperliquidPerpOrderSide, HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
    PlaceHyperliquidPerpOrderInput, PlaceHyperliquidPerpOrderIntent,
    PlaceHyperliquidPerpOrderOutcome,
};
pub use perp::trade::hyperliquid_perp_position::{
    HyperliquidPerpFundingDirection, HyperliquidPerpMarginMode, HyperliquidPerpPosition,
    HyperliquidPerpPositionError, HyperliquidPerpPositionLeverageOutcome,
    HyperliquidPerpPositionStatus, HyperliquidPerpPositionTradeOutcome, required_position_margin,
};
pub use perp::trade::hyperliquid_perp_trade::{
    HyperliquidPerpTrade, HyperliquidPerpTradePositionSettlement,
};
pub use reservation::{
    AssetReservation, MarginReservation, Reservation, ReservationCloseReason, ReservationError,
    ReservationKind, ReservationMarketKind, ReservationStatus,
};
use spot as spot_entity;
pub use spot::spot_trade::SpotTrade;
pub use spot_entity::spot_order_primitives::{
    SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
    SpotOrderTimeInForce, SpotOrderTriggerRole,
};
pub use spot_entity::spot_order_v2::{
    CancelSpotOrderV2Input, CancelSpotOrderV2Outcome, MatchSpotOrderV2Input,
    MatchSpotOrderV2Outcome, PlaceSpotOrderV2Input, PlaceSpotOrderV2Outcome,
    SpotOrderFeeConsumeRequirement, SpotOrderFeeHoldRequirement, SpotOrderHoldAsset,
    SpotOrderHoldRequirement, SpotOrderReleaseReason, SpotOrderReleaseRequirement, SpotOrderV2,
    SpotOrderV2BehaviorError, SpotOrderV2MatchError, SpotTradeFeeRole, TriggerSpotOrderV2Input,
};
