use cmd_handler::command_use_case_def2::{
    ExecutionError, ExecutionResult, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    MatchSpotOrderV3Changes, MatchSpotOrderV3Cmd, MatchSpotOrderV3Error, MatchSpotOrderV3UseCase,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
};

pub fn execute_match_spot_order_v3(
    command: &MatchSpotOrderV3Cmd,
) -> Result<
    ExecutionResult<MatchSpotOrderV3Changes>,
    ExecutionError<MatchSpotOrderV3Error, DefaultSpotOrderV2PlaceOutboundError>,
> {
    execute_match_spot_order_v3_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub fn execute_match_spot_order_v3_with_outbound<OB>(
    command: &MatchSpotOrderV3Cmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<MatchSpotOrderV3Changes>,
    ExecutionError<MatchSpotOrderV3Error, <OB as StateSink<MatchSpotOrderV3UseCase>>::Error>,
>
where
    OB: StateSource<
            MatchSpotOrderV3UseCase,
            Error = <OB as StateSink<MatchSpotOrderV3UseCase>>::Error,
        > + StateSink<MatchSpotOrderV3UseCase>,
{
    StateMachineExecutor.execute::<MatchSpotOrderV3UseCase, OB, OB>(
        &MatchSpotOrderV3UseCase,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use cmd_handler::EntityReplayableEvent;
    use cmd_handler::command_use_case_def2::{StateSink, StateSource};
    use example_core_use_case::{
        ActivatePendingSpotOrderV2Input, Balance, MatchSpotOrderV3Cmd, MatchSpotOrderV3State,
        MatchSpotOrderV3UseCase, SpotOrderSide, SpotOrderStatus, SpotOrderTif, SpotOrderType,
        SpotOrderV2,
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FakePlaceSpotOrderV2OutboundError;

    impl std::fmt::Display for FakePlaceSpotOrderV2OutboundError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "fake place spot order v2 outbound error")
        }
    }

    impl std::error::Error for FakePlaceSpotOrderV2OutboundError {}

    #[derive(Debug, Default)]
    struct FakePlaceSpotOrderV2Outbound;

    impl StateSource<MatchSpotOrderV3UseCase> for FakePlaceSpotOrderV2Outbound {
        type Error = FakePlaceSpotOrderV2OutboundError;

        fn load_given_state(
            &self,
            _request: &MatchSpotOrderV3Cmd,
        ) -> Result<MatchSpotOrderV3State, Self::Error> {
            Ok(MatchSpotOrderV3State {
                taker_order: buy_order("taker-buy", "buyer", 100, 2, SpotOrderTif::Ioc)?,
                maker_orders: vec![sell_order("maker-1", "seller", 100, 1)?],
                settlement_balances: vec![
                    Balance::new("buyer".to_string(), "USDT".to_string(), 1000, 201, 1),
                    Balance::new("buyer".to_string(), "BTC".to_string(), 0, 0, 1),
                    Balance::new("seller".to_string(), "BTC".to_string(), 0, 1, 1),
                    Balance::new("seller".to_string(), "USDT".to_string(), 0, 1, 1),
                    Balance::new("fee".to_string(), "USDT".to_string(), 0, 0, 1),
                ],
                base_asset_id: "BTC".to_string(),
                quote_asset_id: "USDT".to_string(),
                fee_account_id: "fee".to_string(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
            })
        }
    }

    impl StateSink<MatchSpotOrderV3UseCase> for FakePlaceSpotOrderV2Outbound {
        type Error = FakePlaceSpotOrderV2OutboundError;

        fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    fn place_cmd() -> MatchSpotOrderV3Cmd {
        MatchSpotOrderV3Cmd {
            party_id: "buyer".to_string(),
            asset: 10_001,
            order_id: "taker-buy".to_string(),
        }
    }

    fn buy_order(
        order_id: &str,
        account_id: &str,
        price: u64,
        qty: u64,
        tif: SpotOrderTif,
    ) -> Result<SpotOrderV2, FakePlaceSpotOrderV2OutboundError> {
        order(order_id, account_id, SpotOrderSide::Buy, price, qty, tif)
    }

    fn sell_order(
        order_id: &str,
        account_id: &str,
        price: u64,
        qty: u64,
    ) -> Result<SpotOrderV2, FakePlaceSpotOrderV2OutboundError> {
        order(order_id, account_id, SpotOrderSide::Sell, price, qty, SpotOrderTif::Gtc)
    }

    fn order(
        order_id: &str,
        account_id: &str,
        side: SpotOrderSide,
        price: u64,
        qty: u64,
        tif: SpotOrderTif,
    ) -> Result<SpotOrderV2, FakePlaceSpotOrderV2OutboundError> {
        let reservation = SpotOrderV2::principal_reservation(
            order_id, account_id, side, qty, price, "BTC", "USDT",
        )
        .map_err(|_| FakePlaceSpotOrderV2OutboundError)?;

        let mut order = SpotOrderV2::new_pending_limit(
            order_id.to_string(),
            10_001,
            account_id.to_string(),
            "BTCUSDT".to_string(),
            side,
            qty,
            price,
            SpotOrderType::Limit { tif },
            None,
            0,
            1,
        );
        order
            .activate_pending(ActivatePendingSpotOrderV2Input {
                base_asset_id: "BTC".to_string(),
                quote_asset_id: "USDT".to_string(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
                timestamp: 1,
            })
            .map_err(|_| FakePlaceSpotOrderV2OutboundError)?;
        debug_assert_eq!(order.reservation, reservation);
        Ok(order)
    }

    #[test]
    fn execute_place_with_default_outbound_stops_at_load_state() {
        let result = execute_match_spot_order_v3(&place_cmd());

        assert_eq!(
            result,
            Err(ExecutionError::LoadState(DefaultSpotOrderV2PlaceOutboundError::StateUnavailable,)),
        );
    }

    #[test]
    fn execute_place_ioc_crosses_book_and_projects_events() {
        let result =
            execute_match_spot_order_v3_with_outbound(&place_cmd(), &FakePlaceSpotOrderV2Outbound)
                .expect("place spot order v2 should execute");

        let changes = result.changes;
        assert_eq!(
            changes.taker_order_after().expect("taker should be updated").status(),
            SpotOrderStatus::Canceled,
        );
        assert_eq!(changes.updated_maker_orders().len(), 1);
        assert_eq!(changes.created_trades().len(), 1);
        assert!(changes.created_trades()[0].executed_at_ms > 0);
        assert!(!changes.updated_balances().is_empty());
        assert!(!changes.created_balance_ledger_entries().is_empty());
        assert!(changes.updated_balances().iter().any(|pair| pair.before != pair.after));

        assert!(!result.events.is_empty());
    }
}
