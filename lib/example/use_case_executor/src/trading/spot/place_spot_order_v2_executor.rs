use cmd_handler::command_use_case_def2::{
    ExecutionResult, MiFamilyExecutionError, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    PlaceSpotOrderV2Changes, PlaceSpotOrderV2Cmd, PlaceSpotOrderV2Error, PlaceSpotOrderV2UseCase,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
};

pub fn execute_place_spot_order_v2(
    command: &PlaceSpotOrderV2Cmd,
) -> Result<
    ExecutionResult<PlaceSpotOrderV2Changes>,
    MiFamilyExecutionError<PlaceSpotOrderV2Error, DefaultSpotOrderV2PlaceOutboundError>,
> {
    execute_place_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub fn execute_place_spot_order_v2_with_outbound<OB>(
    command: &PlaceSpotOrderV2Cmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<PlaceSpotOrderV2Changes>,
    MiFamilyExecutionError<
        PlaceSpotOrderV2Error,
        <OB as StateSink<PlaceSpotOrderV2UseCase>>::Error,
    >,
>
where
    OB: StateSource<
            PlaceSpotOrderV2UseCase,
            Error = <OB as StateSink<PlaceSpotOrderV2UseCase>>::Error,
        > + StateSink<PlaceSpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<PlaceSpotOrderV2UseCase, OB, OB>(
        &PlaceSpotOrderV2UseCase,
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
        Balance, PlaceSpotOrderV2Cmd, PlaceSpotOrderV2State, PlaceSpotOrderV2UseCase,
        SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderTimeInForce, SpotOrderV2,
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

    impl StateSource<PlaceSpotOrderV2UseCase> for FakePlaceSpotOrderV2Outbound {
        type Error = FakePlaceSpotOrderV2OutboundError;

        fn load_given_state(
            &self,
            _request: &PlaceSpotOrderV2Cmd,
        ) -> Result<PlaceSpotOrderV2State, Self::Error> {
            Ok(PlaceSpotOrderV2State {
                order_id: "taker-buy".to_string(),
                symbol: "BTCUSDT".to_string(),
                maker_orders: vec![sell_order("maker-1", "seller", 100, 1)?],
                settlement_balances: vec![
                    Balance::new("buyer".to_string(), "USDT".to_string(), 1200, 1, 1),
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

    impl StateSink<PlaceSpotOrderV2UseCase> for FakePlaceSpotOrderV2Outbound {
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

    fn place_cmd() -> PlaceSpotOrderV2Cmd {
        PlaceSpotOrderV2Cmd {
            party_id: "buyer".to_string(),
            asset: 10_001,
            is_buy: true,
            price: "100".to_string(),
            size: "2".to_string(),
            tif: "ioc".to_string(),
            cloid: None,
        }
    }

    fn sell_order(
        order_id: &str,
        account_id: &str,
        price: u64,
        qty: u64,
    ) -> Result<SpotOrderV2, FakePlaceSpotOrderV2OutboundError> {
        let reservation = SpotOrderV2::principal_reservation(
            order_id,
            account_id,
            SpotOrderSide::Sell,
            qty,
            price,
            "BTC",
            "USDT",
        )
        .map_err(|_| FakePlaceSpotOrderV2OutboundError)?;

        Ok(SpotOrderV2::new(
            order_id.to_string(),
            10_001,
            Some(price),
            account_id.to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            qty,
            0,
            SpotOrderStatus::Open,
            None,
            reservation,
            None,
            1,
        ))
    }

    #[test]
    fn execute_place_with_default_outbound_stops_at_load_state() {
        let result = execute_place_spot_order_v2(&place_cmd());

        assert_eq!(
            result,
            Err(MiFamilyExecutionError::LoadState(
                DefaultSpotOrderV2PlaceOutboundError::StateUnavailable,
            )),
        );
    }

    #[test]
    fn execute_place_ioc_crosses_book_and_projects_events() {
        let result =
            execute_place_spot_order_v2_with_outbound(&place_cmd(), &FakePlaceSpotOrderV2Outbound)
                .expect("place spot order v2 should execute");

        let changes = result.changes;
        assert_eq!(changes.updated_taker_order.after.status(), SpotOrderStatus::Canceled);
        assert_eq!(changes.updated_maker_orders.len(), 1);
        assert_eq!(changes.created_trades.len(), 1);
        assert!(!changes.updated_balances.is_empty());
        assert!(!changes.created_balance_ledger_entries.is_empty());
        assert!(changes.updated_balances.iter().any(|pair| pair.before != pair.after));

        assert!(!result.events.is_empty());
    }
}
