mod market_rules;
mod reservation;

pub mod hyperliquid_account;
pub mod perp;

pub mod spot;

pub mod account;

pub use account::account::{Account, AccountStatus};
pub use account::balance::{Balance, BalanceError};
pub use account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2 as BalanceLedgerEntry, BalanceLedgerEntryV2, BalanceLedgerOperation,
};
pub use account::balance_ledger_reason::BalanceLedgerReason;
pub use account::settlement_transfer_voucher::{
    SettlementKind, SettlementTransferLeg, SettlementTransferPurpose, SettlementTransferSummary,
    SettlementTransferVoucher,
};
pub use hyperliquid_account::{
    AccountId, AssetId, MarginSummary, MasterAccount, PerpAssetId, PerpClearinghouseState,
    RiskState, SpotClearinghouseState, SubAccountProfile, SubAccountSnapshot,
    SubAccountSnapshotError,
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
pub use perp::trade::hyperliquid_perp_leverage_setting::HyperliquidPerpLeverageSetting;
pub use perp::trade::hyperliquid_perp_order::{
    HyperliquidPerpOrder, HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide,
    HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
};
pub use perp::trade::hyperliquid_perp_position::{
    HyperliquidPerpFundingDirection, HyperliquidPerpMarginMode, HyperliquidPerpPosition,
    HyperliquidPerpPositionSide, required_position_margin,
};
pub use perp::trade::hyperliquid_perp_trade::HyperliquidPerpTrade;
pub use reservation::{
    AssetReservation, MarginReservation, Reservation, ReservationCloseReason, ReservationConsumed,
    ReservationCreated, ReservationError, ReservationKind, ReservationMarketKind,
    ReservationReleased, ReservationStatus,
};
pub(crate) use spot::spot_order::SpotOrderMatchError;
#[cfg(test)]
pub(crate) use spot::spot_order::spot_order_scenarios::{
    ActiveSpotOrderScenario, active_spot_order_scenario_strategy,
};
pub use spot::spot_order::{
    SpotConditionalOrder, SpotConditionalOrderStatus, SpotOrder, SpotOrderExecution, SpotOrderSide,
    SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce, SpotOrderTriggerRole,
};
pub use spot::spot_order_v2::{
    SpotOrderFeeConsumeRequirement, SpotOrderFeeHoldRequirement, SpotOrderHoldAsset,
    SpotOrderHoldRequirement, SpotOrderReleaseReason, SpotOrderReleaseRequirement, SpotOrderV2,
    SpotOrderV2MatchError, SpotTradeFeeRole,
};
pub use spot::spot_trade::SpotTrade;
