pub mod entity;
pub mod use_case;

pub use entity::{
    Account, AccountId, AccountStatus, AssetId, Balance, BalanceError, BalanceLedgerEntry,
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error, BalanceLedgerOperation, BalanceLedgerReason,
    CancelSpotOrderV2Input, CancelSpotOrderV2Outcome, HyperliquidPerpAdlBatch,
    HyperliquidPerpAdlBatchStatus, HyperliquidPerpAdlDeleveragingRecord,
    HyperliquidPerpAdlExecution, HyperliquidPerpAdlExecutionStatus, HyperliquidPerpBookLevel,
    HyperliquidPerpFundingDirection, HyperliquidPerpFundingRateError, HyperliquidPerpFundingSample,
    HyperliquidPerpFundingSettlement, HyperliquidPerpInsuranceFundAllocation,
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationFill, HyperliquidPerpLiquidationStatus,
    HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode, HyperliquidPerpOrder,
    HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide, HyperliquidPerpOrderStatus,
    HyperliquidPerpOrderTimeInForce, HyperliquidPerpPosition, HyperliquidPerpPositionSide,
    HyperliquidPerpShortfall, HyperliquidPerpShortfallStatus, HyperliquidPerpTrade,
    MarginReservation, MarginSummary, MarketRules, MasterAccount, MatchSpotOrderV2Input,
    MatchSpotOrderV2Outcome, PerpAssetId, PerpClearinghouseState, PlaceSpotOrderV2Input,
    PlaceSpotOrderV2Outcome, Reservation, ReservationCloseReason, ReservationError,
    ReservationKind, ReservationMarketKind, ReservationStatus, RiskState, SpotClearinghouseState,
    SpotOrderExecution, SpotOrderFeeConsumeRequirement, SpotOrderFeeHoldRequirement,
    SpotOrderHoldAsset, SpotOrderHoldRequirement, SpotOrderReleaseReason,
    SpotOrderReleaseRequirement, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
    SpotOrderTimeInForce, SpotOrderTriggerRole, SpotOrderV2, SpotOrderV2BehaviorError,
    SpotOrderV2MatchError, SpotTrade, SpotTradeFeeRole, SubAccountProfile, SubAccountSnapshot,
    SubAccountSnapshotError, TriggerSpotOrderV2Input, compute_hourly_funding_rate_e8,
    compute_impact_ask_price, compute_impact_bid_price, required_position_margin,
};
pub use use_case::{
    ACCOUNT_ENTITY_TYPE, AllocateHyperliquidPerpInsuranceFundChanges,
    AllocateHyperliquidPerpInsuranceFundCmd, AllocateHyperliquidPerpInsuranceFundError,
    AllocateHyperliquidPerpInsuranceFundState, AllocateHyperliquidPerpInsuranceFundUseCase,
    ApplyHyperliquidPerpLiquidationFillChanges, ApplyHyperliquidPerpLiquidationFillCmd,
    ApplyHyperliquidPerpLiquidationFillError, ApplyHyperliquidPerpLiquidationFillState,
    ApplyHyperliquidPerpLiquidationFillUseCase, CancelSpotOrderV2AfterChangesV3,
    CancelSpotOrderV2ChangesV3, CancelSpotOrderV2CmdV3, CancelSpotOrderV2LookupV3,
    CloseHyperliquidPerpLiquidationChanges, CloseHyperliquidPerpLiquidationCmd,
    CloseHyperliquidPerpLiquidationError, CloseHyperliquidPerpLiquidationState,
    CloseHyperliquidPerpLiquidationUseCase, CompleteHyperliquidPerpAdlExecutionChanges,
    CompleteHyperliquidPerpAdlExecutionCmd, CompleteHyperliquidPerpAdlExecutionError,
    CompleteHyperliquidPerpAdlExecutionState, CompleteHyperliquidPerpAdlExecutionUseCase,
    ConfirmHyperliquidPerpShortfallChanges, ConfirmHyperliquidPerpShortfallCmd,
    ConfirmHyperliquidPerpShortfallError, ConfirmHyperliquidPerpShortfallState,
    ConfirmHyperliquidPerpShortfallUseCase, DepositQuoteChanges, DepositQuoteCmd,
    DepositQuoteError, DepositQuoteState, DepositQuoteUseCase,
    HyperliquidPerpLiquidatablePositionAtPriceSnapshot,
    HyperliquidPerpLiquidatablePositionAtPriceView, HyperliquidPerpLiquidationCandidate,
    HyperliquidPerpLiquidationCloseAs, HyperliquidPerpOpenOrderView,
    HyperliquidPerpOrderDetailView, HyperliquidPerpRiskSnapshot, MatchHyperliquidPerpOrderChanges,
    MatchHyperliquidPerpOrderCmd, MatchHyperliquidPerpOrderError, MatchHyperliquidPerpOrderState,
    MatchHyperliquidPerpOrderUseCase, ORDER_ENTITY_TYPE, PlaceHyperliquidPerpOrderChanges,
    PlaceHyperliquidPerpOrderCmd, PlaceHyperliquidPerpOrderError,
    PlaceHyperliquidPerpOrderExecution, PlaceHyperliquidPerpOrderState,
    PlaceHyperliquidPerpOrderUseCase, PlaceOrderError, PlaceOrderExecution, PlaceOrderSide,
    PlaceOrderTimeInForce, PlaceOrderTriggerRole, PlaceSpotOrderV2AfterChangesV3,
    PlaceSpotOrderV2ChangesV3, PlaceSpotOrderV2CmdV3, PlaceSpotOrderV2TakerTemplateContextV3,
    PlaceTriggerPendingSpotOrderV2AfterChangesV3, PlaceTriggerPendingSpotOrderV2ChangesV3,
    PlaceTriggerPendingSpotOrderV2CmdV3, PlaceTriggerPendingSpotOrderV2TemplateContextV3,
    QueryHyperliquidPerpLiquidatablePositionsAtPrice,
    QueryHyperliquidPerpLiquidatablePositionsAtPriceError,
    QueryHyperliquidPerpLiquidatablePositionsAtPriceReadModel,
    QueryHyperliquidPerpLiquidatablePositionsAtPriceUseCase,
    QueryHyperliquidPerpLiquidatablePositionsAtPriceView,
    QueryHyperliquidPerpLiquidationCandidates, QueryHyperliquidPerpLiquidationCandidatesError,
    QueryHyperliquidPerpLiquidationCandidatesReadModel,
    QueryHyperliquidPerpLiquidationCandidatesUseCase, QueryHyperliquidPerpOpenOrders,
    QueryHyperliquidPerpOpenOrdersError, QueryHyperliquidPerpOpenOrdersReadModel,
    QueryHyperliquidPerpOpenOrdersUseCase, QueryHyperliquidPerpOrderDetail,
    QueryHyperliquidPerpOrderDetailError, QueryHyperliquidPerpOrderDetailReadModel,
    QueryHyperliquidPerpOrderDetailUseCase, SettleHyperliquidPerpFundingChanges,
    SettleHyperliquidPerpFundingCmd, SettleHyperliquidPerpFundingError,
    SettleHyperliquidPerpFundingState, SettleHyperliquidPerpFundingUseCase,
    SettleHyperliquidPerpTradeChanges, SettleHyperliquidPerpTradeCmd,
    SettleHyperliquidPerpTradeError, SettleHyperliquidPerpTradeState,
    SettleHyperliquidPerpTradeUseCase, SpotOrderV2AfterChangesV3, SpotOrderV2CaseChangesV3,
    SpotOrderV2CommandV3, SpotOrderV2GivenStateV3, SpotOrderV2UseCaseFamilyV3,
    SpotOrderV2UseCaseFamilyV3Error, StartHyperliquidPerpAdlBatchChanges,
    StartHyperliquidPerpAdlBatchCmd, StartHyperliquidPerpAdlBatchError,
    StartHyperliquidPerpAdlBatchState, StartHyperliquidPerpAdlBatchUseCase,
    StartHyperliquidPerpAdlExecutionChanges, StartHyperliquidPerpAdlExecutionCmd,
    StartHyperliquidPerpAdlExecutionError, StartHyperliquidPerpAdlExecutionState,
    StartHyperliquidPerpAdlExecutionUseCase, StartHyperliquidPerpLiquidationChanges,
    StartHyperliquidPerpLiquidationCmd, StartHyperliquidPerpLiquidationError,
    StartHyperliquidPerpLiquidationState, StartHyperliquidPerpLiquidationUseCase,
    TriggerSpotOrderV2AfterChangesV3, TriggerSpotOrderV2ChangesV3, TriggerSpotOrderV2CmdV3,
    UpdateHyperliquidPerpLeverageChanges, UpdateHyperliquidPerpLeverageCmd,
    UpdateHyperliquidPerpLeverageError, UpdateHyperliquidPerpLeverageState,
    UpdateHyperliquidPerpLeverageUseCase, WithdrawQuoteChanges, WithdrawQuoteCmd,
    WithdrawQuoteError, WithdrawQuoteState, WithdrawQuoteUseCase,
    build_place_spot_order_v2_taker_template_v3,
    build_place_trigger_pending_spot_order_v2_template_v3,
};

#[cfg(test)]
mod naming_tests {
    use std::any::type_name;

    use super::*;

    #[test]
    fn liquidation_chain_names_follow_mi_conventions() {
        let names = [
            type_name::<HyperliquidPerpLiquidationFill>(),
            type_name::<HyperliquidPerpShortfall>(),
            type_name::<HyperliquidPerpInsuranceFundAllocation>(),
            type_name::<HyperliquidPerpAdlBatch>(),
            type_name::<HyperliquidPerpAdlExecution>(),
            type_name::<HyperliquidPerpAdlDeleveragingRecord>(),
        ];

        for name in names {
            assert!(!name.contains("Event"));
            assert!(!name.contains("Process"));
            assert!(!name.contains("Handler"));
            assert!(!name.contains("Step"));
            assert!(!name.contains("Closure"));
        }
    }
}
