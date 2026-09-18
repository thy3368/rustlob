use cmd_handler::command_use_case_def2::{
    ExecutionResult, MiFamilyExecutionError, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    MatchSpotOrderV2Changes, MatchSpotOrderV2Cmd, MatchSpotOrderV2Error, OpenMatchSpotOrderV2UseCase,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
};

pub fn execute_place_spot_order_v2(
    command: &MatchSpotOrderV2Cmd,
) -> Result<
    ExecutionResult<MatchSpotOrderV2Changes>,
    MiFamilyExecutionError<MatchSpotOrderV2Error, DefaultSpotOrderV2PlaceOutboundError>,
> {
    execute_place_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub fn execute_place_spot_order_v2_with_outbound<OB>(
    command: &MatchSpotOrderV2Cmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<MatchSpotOrderV2Changes>,
    MiFamilyExecutionError<
        MatchSpotOrderV2Error,
        <OB as StateSink<OpenMatchSpotOrderV2UseCase>>::Error,
    >,
>
where
    OB: StateSource<
        OpenMatchSpotOrderV2UseCase,
            Error = <OB as StateSink<OpenMatchSpotOrderV2UseCase>>::Error,
        > + StateSink<OpenMatchSpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<OpenMatchSpotOrderV2UseCase, OB, OB>(
        &OpenMatchSpotOrderV2UseCase,
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
        Balance, MatchSpotOrderV2Cmd, MatchSpotOrderV2State, OpenMatchSpotOrderV2UseCase,
        SpotOrderSide, SpotOrderStatus, SpotOrderTif, SpotOrderType, SpotOrderV2,
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

    impl StateSource<OpenMatchSpotOrderV2UseCase> for FakePlaceSpotOrderV2Outbound {
        type Error = FakePlaceSpotOrderV2OutboundError;

        fn load_given_state(
            &self,
            _request: &MatchSpotOrderV2Cmd,
        ) -> Result<MatchSpotOrderV2State, Self::Error> {
            Ok(MatchSpotOrderV2State {
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

    impl StateSink<OpenMatchSpotOrderV2UseCase> for FakePlaceSpotOrderV2Outbound {
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

    fn place_cmd() -> MatchSpotOrderV2Cmd {
        MatchSpotOrderV2Cmd {
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

        Ok(SpotOrderV2::new(
            order_id.to_string(),
            10_001,
            Some(price),
            account_id.to_string(),
            "BTCUSDT".to_string(),
            side,
            price,
            SpotOrderType::Limit { tif },
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
        assert_eq!(
            changes.taker_order_after().expect("taker should be updated").status(),
            SpotOrderStatus::Canceled,
        );
        assert_eq!(changes.updated_maker_orders.len(), 1);
        assert_eq!(changes.created_trades.len(), 1);
        assert!(!changes.updated_balances.is_empty());
        assert!(!changes.created_balance_ledger_entries.is_empty());
        assert!(changes.updated_balances.iter().any(|pair| pair.before != pair.after));

        assert!(!result.events.is_empty());
    }
}
