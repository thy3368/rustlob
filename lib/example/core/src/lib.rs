pub mod entity;
pub mod use_case;

pub use entity::{
    Account, AccountId, AccountStatus, AssetId, Balance, BalanceError, BalanceLedgerEntry,
    BalanceLedgerEntryV2, BalanceLedgerOperation, BalanceLedgerReason, HyperliquidPerpAdlBatch,
    HyperliquidPerpAdlBatchStatus, HyperliquidPerpAdlDeleveragingRecord,
    HyperliquidPerpAdlExecution, HyperliquidPerpAdlExecutionStatus, HyperliquidPerpBookLevel,
    HyperliquidPerpFundingDirection, HyperliquidPerpFundingRateError, HyperliquidPerpFundingSample,
    HyperliquidPerpFundingSettlement, HyperliquidPerpInsuranceFundAllocation,
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationFill, HyperliquidPerpLiquidationStatus,
    HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode, HyperliquidPerpOrder,
    HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide, HyperliquidPerpOrderStatus,
    HyperliquidPerpOrderTimeInForce, HyperliquidPerpPosition, HyperliquidPerpPositionSide,
    HyperliquidPerpShortfall, HyperliquidPerpShortfallStatus, HyperliquidPerpTrade,
    MarginReservation, MarginSummary, MarketRules, MasterAccount, PerpAssetId,
    PerpClearinghouseState, Reservation, ReservationCloseReason, ReservationConsumed,
    ReservationCreated, ReservationError, ReservationKind, ReservationMarketKind,
    ReservationReleased, ReservationStatus, RiskState, SpotClearinghouseState,
    SpotConditionalOrder, SpotConditionalOrderStatus, SpotOrderExecution,
    SpotOrderFeeConsumeRequirement, SpotOrderFeeHoldRequirement, SpotOrderHoldAsset,
    SpotOrderHoldRequirement, SpotOrderReleaseReason, SpotOrderReleaseRequirement, SpotOrderSide,
    SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce, SpotOrderTriggerRole,
    SpotOrderV2, SpotOrderV2MatchError, SpotTrade, SpotTradeFeeRole, SubAccountProfile,
    SubAccountSnapshot, SubAccountSnapshotError, compute_hourly_funding_rate_e8,
    compute_impact_ask_price, compute_impact_bid_price, required_position_margin,
};
pub use use_case::{
    ACCOUNT_ENTITY_TYPE, AllocateHyperliquidPerpInsuranceFundChanges,
    AllocateHyperliquidPerpInsuranceFundCmd, AllocateHyperliquidPerpInsuranceFundError,
    AllocateHyperliquidPerpInsuranceFundState, AllocateHyperliquidPerpInsuranceFundUseCase,
    ApplyHyperliquidPerpLiquidationFillChanges, ApplyHyperliquidPerpLiquidationFillCmd,
    ApplyHyperliquidPerpLiquidationFillError, ApplyHyperliquidPerpLiquidationFillState,
    ApplyHyperliquidPerpLiquidationFillUseCase, CancelSpotOrderChanges, CancelSpotOrderCmd,
    CancelSpotOrderError, CancelSpotOrderState, CancelSpotOrderUseCase, CancelSpotOrderV2Cmd,
    CancelSpotOrderV2Lookup, CloseHyperliquidPerpLiquidationChanges,
    CloseHyperliquidPerpLiquidationCmd, CloseHyperliquidPerpLiquidationError,
    CloseHyperliquidPerpLiquidationState, CloseHyperliquidPerpLiquidationUseCase,
    CompleteHyperliquidPerpAdlExecutionChanges, CompleteHyperliquidPerpAdlExecutionCmd,
    CompleteHyperliquidPerpAdlExecutionError, CompleteHyperliquidPerpAdlExecutionState,
    CompleteHyperliquidPerpAdlExecutionUseCase, ConfirmHyperliquidPerpShortfallChanges,
    ConfirmHyperliquidPerpShortfallCmd, ConfirmHyperliquidPerpShortfallError,
    ConfirmHyperliquidPerpShortfallState, ConfirmHyperliquidPerpShortfallUseCase,
    DepositQuoteChanges, DepositQuoteCmd, DepositQuoteError, DepositQuoteState,
    DepositQuoteUseCase, ExecuteImmediateSpotOrderPipelineChanges,
    ExecuteImmediateSpotOrderPipelineCmd, ExecuteImmediateSpotOrderPipelineError,
    ExecuteImmediateSpotOrderPipelineState, ExecuteImmediateSpotOrderPipelineUseCase,
    ExecuteMatchedSpotTradeChanges, ExecuteMatchedSpotTradeCmd, ExecuteMatchedSpotTradeError,
    ExecuteMatchedSpotTradeState, ExecuteMatchedSpotTradeUseCase,
    HyperliquidPerpLiquidatablePositionAtPriceSnapshot,
    HyperliquidPerpLiquidatablePositionAtPriceView, HyperliquidPerpLiquidationCandidate,
    HyperliquidPerpLiquidationCloseAs, HyperliquidPerpOpenOrderView,
    HyperliquidPerpOrderDetailView, HyperliquidPerpRiskSnapshot, MatchHyperliquidPerpOrderChanges,
    MatchHyperliquidPerpOrderCmd, MatchHyperliquidPerpOrderError, MatchHyperliquidPerpOrderState,
    MatchHyperliquidPerpOrderUseCase, MatchSpotOrderChanges, MatchSpotOrderCmd,
    MatchSpotOrderError, MatchSpotOrderState, MatchSpotOrderUseCase, ORDER_ENTITY_TYPE,
    PlaceConditionalOrderCmd, PlaceConditionalOrderOutput, PlaceConditionalOrderState,
    PlaceConditionalOrderUseCase, PlaceHyperliquidPerpOrderChanges, PlaceHyperliquidPerpOrderCmd,
    PlaceHyperliquidPerpOrderError, PlaceHyperliquidPerpOrderExecution,
    PlaceHyperliquidPerpOrderState, PlaceHyperliquidPerpOrderUseCase, PlaceImmediateOrderChanges,
    PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceImmediateOrderState,
    PlaceImmediateOrderUseCase, PlaceOrderCmd, PlaceOrderError, PlaceOrderExecution,
    PlaceOrderSide, PlaceOrderState, PlaceOrderTimeInForce, PlaceOrderTriggerRole,
    PlaceOrderUseCase, PlaceSpotOrderV2Cmd, QueryHyperliquidPerpLiquidatablePositionsAtPrice,
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
    QueryHyperliquidPerpOrderDetailUseCase, SPOT_SETTLEMENT_GROUP_SPEC, SPOT_TRADING_GROUP_SPEC,
    SettleHyperliquidPerpFundingChanges, SettleHyperliquidPerpFundingCmd,
    SettleHyperliquidPerpFundingError, SettleHyperliquidPerpFundingState,
    SettleHyperliquidPerpFundingUseCase, SettleHyperliquidPerpTradeChanges,
    SettleHyperliquidPerpTradeCmd, SettleHyperliquidPerpTradeError,
    SettleHyperliquidPerpTradeState, SettleHyperliquidPerpTradeUseCase, SettleSpotTradeChanges,
    SettleSpotTradeCmd, SettleSpotTradeError, SettleSpotTradeState, SettleSpotTradeUseCase,
    SpotOrderV2AfterChanges, SpotOrderV2CaseChanges, SpotOrderV2Command, SpotOrderV2GivenState,
    SpotOrderV2UseCaseFamily, SpotOrderV2UseCaseFamilyError, StartHyperliquidPerpAdlBatchChanges,
    StartHyperliquidPerpAdlBatchCmd, StartHyperliquidPerpAdlBatchError,
    StartHyperliquidPerpAdlBatchState, StartHyperliquidPerpAdlBatchUseCase,
    StartHyperliquidPerpAdlExecutionChanges, StartHyperliquidPerpAdlExecutionCmd,
    StartHyperliquidPerpAdlExecutionError, StartHyperliquidPerpAdlExecutionState,
    StartHyperliquidPerpAdlExecutionUseCase, StartHyperliquidPerpLiquidationChanges,
    StartHyperliquidPerpLiquidationCmd, StartHyperliquidPerpLiquidationError,
    StartHyperliquidPerpLiquidationState, StartHyperliquidPerpLiquidationUseCase,
    UpdateHyperliquidPerpLeverageChanges, UpdateHyperliquidPerpLeverageCmd,
    UpdateHyperliquidPerpLeverageError, UpdateHyperliquidPerpLeverageState,
    UpdateHyperliquidPerpLeverageUseCase, WithdrawQuoteChanges, WithdrawQuoteCmd,
    WithdrawQuoteError, WithdrawQuoteState, WithdrawQuoteUseCase,
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
