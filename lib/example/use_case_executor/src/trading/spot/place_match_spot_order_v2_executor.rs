use cmd_handler::command_use_case_def2::{
    ExecutionError, ExecutionResult, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    PlaceMatchSpotOrderV2Changes, PlaceMatchSpotOrderV2Error, PlaceMatchSpotOrderV2UseCase,
    PlaceOnlySpotOrderV2Cmd,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
};

pub fn execute_place_match_spot_order_v2(
    command: &PlaceOnlySpotOrderV2Cmd,
) -> Result<
    ExecutionResult<PlaceMatchSpotOrderV2Changes>,
    ExecutionError<PlaceMatchSpotOrderV2Error, DefaultSpotOrderV2PlaceOutboundError>,
> {
    execute_place_match_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOutbound)
}

pub fn execute_place_match_spot_order_v2_with_outbound<OB>(
    command: &PlaceOnlySpotOrderV2Cmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<PlaceMatchSpotOrderV2Changes>,
    ExecutionError<
        PlaceMatchSpotOrderV2Error,
        <OB as StateSink<PlaceMatchSpotOrderV2UseCase>>::Error,
    >,
>
where
    OB: StateSource<
            PlaceMatchSpotOrderV2UseCase,
            Error = <OB as StateSink<PlaceMatchSpotOrderV2UseCase>>::Error,
        > + StateSink<PlaceMatchSpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<PlaceMatchSpotOrderV2UseCase, OB, OB>(
        &PlaceMatchSpotOrderV2UseCase,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use cmd_handler::EntityReplayableEvent;
    use cmd_handler::command_use_case_def2::{StateSink, StateSource};
    use common_entity::Entity;
    use example_core_use_case::{
        ActivatePendingSpotOrderV2Input, Balance, BalanceLedgerOperation,
        PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd,
        PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType, SpotOrderSide, SpotOrderTif,
        SpotOrderType, SpotOrderV2,
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FakePlaceMatchSpotOrderV2OutboundError;

    impl std::fmt::Display for FakePlaceMatchSpotOrderV2OutboundError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "fake place-match spot order v2 outbound error")
        }
    }

    impl std::error::Error for FakePlaceMatchSpotOrderV2OutboundError {}

    #[derive(Debug, Default)]
    struct FakePlaceMatchSpotOrderV2Outbound;

    impl StateSource<PlaceMatchSpotOrderV2UseCase> for FakePlaceMatchSpotOrderV2Outbound {
        type Error = FakePlaceMatchSpotOrderV2OutboundError;

        fn load_given_state(
            &self,
            _request: &PlaceOnlySpotOrderV2Cmd,
        ) -> Result<PlaceMatchSpotOrderV2State, Self::Error> {
            Ok(PlaceMatchSpotOrderV2State {
                maker_orders: vec![sell_order(2, "seller", 100, 1)?],
                settlement_balances: vec![
                    Balance::new("buyer".to_string(), "USDT".to_string(), 101, 0, 1),
                    Balance::new("buyer".to_string(), "BTC".to_string(), 0, 0, 1),
                    Balance::new("seller".to_string(), "BTC".to_string(), 0, 1, 1),
                    Balance::new("seller".to_string(), "USDT".to_string(), 0, 1, 1),
                    Balance::new("fee".to_string(), "USDT".to_string(), 0, 0, 1),
                ],
                fee_account_id: "fee".to_string(),
            })
        }
    }

    impl StateSink<PlaceMatchSpotOrderV2UseCase> for FakePlaceMatchSpotOrderV2Outbound {
        type Error = FakePlaceMatchSpotOrderV2OutboundError;

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

    fn place_cmd() -> PlaceOnlySpotOrderV2Cmd {
        PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
            party_id: "buyer".to_string(),
            asset: 10_001,
            order_id: 1,
            symbol: "BTCUSDT".to_string(),
            is_buy: true,
            price: "100".to_string(),
            size: "1".to_string(),
            order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "ioc".to_string() },
            reduce_only: false,
            cloid: None,
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        })
    }

    fn sell_order(
        order_id: u64,
        account_id: &str,
        price: u64,
        qty: u64,
    ) -> Result<SpotOrderV2, FakePlaceMatchSpotOrderV2OutboundError> {
        let reservation = SpotOrderV2::principal_reservation(
            order_id,
            account_id,
            SpotOrderSide::Sell,
            qty,
            price,
            "BTC",
            "USDT",
        )
        .map_err(|_| FakePlaceMatchSpotOrderV2OutboundError)?;

        let mut order = SpotOrderV2::new_pending_limit(
            order_id,
            10_001,
            account_id.to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            qty,
            price,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
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
            .map_err(|_| FakePlaceMatchSpotOrderV2OutboundError)?;
        debug_assert_eq!(order.reservation, reservation);
        Ok(order)
    }

    #[test]
    fn execute_place_match_with_default_outbound_stops_at_load_state() {
        let result = execute_place_match_spot_order_v2(&place_cmd());

        assert_eq!(
            result,
            Err(ExecutionError::LoadState(DefaultSpotOrderV2PlaceOutboundError::StateUnavailable,)),
        );
    }

    #[test]
    fn execute_place_match_ioc_crosses_book_and_projects_events() {
        let result = execute_place_match_spot_order_v2_with_outbound(
            &place_cmd(),
            &FakePlaceMatchSpotOrderV2Outbound,
        )
        .expect("place-match spot order v2 should execute");

        let changes = result.changes;
        let PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
            created_taker_order,
            activation_changes,
            match_changes,
        } = &changes
        else {
            panic!("limit order should continue to matching");
        };
        assert_eq!(created_taker_order.order_id(), 1);
        assert_eq!(
            activation_changes.created_balance_ledger_entries.first().map(|entry| entry.operation),
            Some(BalanceLedgerOperation::Freeze)
        );
        assert_eq!(match_changes.created_trades().len(), 1);

        assert!(!result.events.is_empty());
        assert!(result.events[0].is_created());
        assert_eq!(result.events[0].entity_type, SpotOrderV2::entity_type());
        assert!(result.events.iter().skip(1).any(EntityReplayableEvent::is_created));
        assert!(result.events.iter().skip(1).any(EntityReplayableEvent::is_updated));
    }
}
