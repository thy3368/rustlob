use cmd_handler::command_use_case_def2::{
    ExecutionError, ExecutionResult, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{SpotBlockChanges, SpotBlockCmd, SpotBlockError, SpotBlockUseCase};
use example_outbound_adapter::{DefaultSpotBlockOutbound, DefaultSpotBlockOutboundError};

pub fn execute_spot_block(
    command: &SpotBlockCmd,
) -> Result<
    ExecutionResult<SpotBlockChanges>,
    ExecutionError<SpotBlockError, DefaultSpotBlockOutboundError>,
> {
    execute_spot_block_with_outbound(command, &DefaultSpotBlockOutbound)
}

pub fn execute_spot_block_with_outbound<OB>(
    command: &SpotBlockCmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<SpotBlockChanges>,
    ExecutionError<SpotBlockError, <OB as StateSink<SpotBlockUseCase>>::Error>,
>
where
    OB: StateSource<SpotBlockUseCase, Error = <OB as StateSink<SpotBlockUseCase>>::Error>
        + StateSink<SpotBlockUseCase>,
{
    StateMachineExecutor.execute::<SpotBlockUseCase, OB, OB>(
        &SpotBlockUseCase,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::sync::{Arc, Mutex};

    use cmd_handler::EntityReplayableEvent;
    use example_core_use_case::{
        ActivatePendingSpotOrderV2Input, Balance, CancelSpotOrderV2Cmd, CancelSpotOrderV2Lookup,
        ModifySpotOrderV2Cmd, ModifySpotOrderV2Error, ModifySpotOrderV2OrderType, OrderId,
        SpotBlockAppliedChanges, SpotBlockCommand, SpotBlockItemError, SpotBlockItemResult,
        SpotBlockState, SpotOrderSide, SpotOrderTif, SpotOrderType, SpotOrderV2,
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FakeSpotBlockOutboundError;

    impl fmt::Display for FakeSpotBlockOutboundError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("fake spot block outbound error")
        }
    }

    impl std::error::Error for FakeSpotBlockOutboundError {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SinkPhase {
        Persist,
        Replay,
        Publish,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ObservedSinkCall {
        phase: SinkPhase,
        events: Vec<EntityReplayableEvent>,
    }

    #[derive(Debug, Default)]
    struct FakeSpotBlockOutbound {
        sink_calls: Arc<Mutex<Vec<ObservedSinkCall>>>,
    }

    impl FakeSpotBlockOutbound {
        fn sink_calls(&self) -> Vec<ObservedSinkCall> {
            self.sink_calls.lock().map(|calls| calls.clone()).unwrap_or_default()
        }

        fn record(
            &self,
            phase: SinkPhase,
            events: &[EntityReplayableEvent],
        ) -> Result<(), FakeSpotBlockOutboundError> {
            self.sink_calls
                .lock()
                .map_err(|_| FakeSpotBlockOutboundError)?
                .push(ObservedSinkCall { phase, events: events.to_vec() });
            Ok(())
        }
    }

    impl StateSource<SpotBlockUseCase> for FakeSpotBlockOutbound {
        type Error = FakeSpotBlockOutboundError;

        fn load_given_state(&self, _cmd: &SpotBlockCmd) -> Result<SpotBlockState, Self::Error> {
            let order = buy_order()?;
            let frozen = order
                .reservation
                .remaining_amount
                .checked_add(order.fee_reservation.remaining_amount)
                .ok_or(FakeSpotBlockOutboundError)?;

            Ok(SpotBlockState {
                orders: vec![order],
                balances: vec![Balance::new(
                    "buyer".to_owned(),
                    "USDT".to_owned(),
                    100_000,
                    frozen,
                    1,
                )],
                base_asset_id: "BTC".to_owned(),
                quote_asset_id: "USDT".to_owned(),
                fee_account_id: "fee".to_owned(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
            })
        }
    }

    impl StateSink<SpotBlockUseCase> for FakeSpotBlockOutbound {
        type Error = FakeSpotBlockOutboundError;

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record(SinkPhase::Persist, events)
        }

        fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record(SinkPhase::Replay, events)
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record(SinkPhase::Publish, events)
        }
    }

    fn buy_order() -> Result<SpotOrderV2, FakeSpotBlockOutboundError> {
        let mut order = SpotOrderV2::new_pending_limit(
            "order-1".to_owned(),
            10_001,
            "buyer".to_owned(),
            "BTCUSDT".to_owned(),
            SpotOrderSide::Buy,
            2,
            10_000,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            Some("original-cloid".to_owned()),
            0,
            1,
        );
        order
            .activate_pending(ActivatePendingSpotOrderV2Input {
                base_asset_id: "BTC".to_owned(),
                quote_asset_id: "USDT".to_owned(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
                timestamp: 1,
            })
            .map_err(|_| FakeSpotBlockOutboundError)?;
        Ok(order)
    }

    fn cancel_command() -> SpotBlockCommand {
        SpotBlockCommand::Cancel(CancelSpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            lookup: CancelSpotOrderV2Lookup::OrderId("order-1".to_owned()),
        })
    }

    fn modify_command() -> SpotBlockCommand {
        SpotBlockCommand::Modify(ModifySpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            order_id: OrderId::OrderId("order-1".to_owned()),
            is_buy: true,
            price: "12000".to_owned(),
            size: "3".to_owned(),
            order_type: ModifySpotOrderV2OrderType::Limit { tif: "gtc".to_owned() },
            cloid: None,
        })
    }

    fn valid_command() -> SpotBlockCmd {
        SpotBlockCmd { commands: vec![cancel_command()] }
    }

    #[test]
    fn execute_spot_block_with_default_outbound_stops_at_load_state() {
        let result = execute_spot_block(&valid_command());

        assert_eq!(
            result,
            Err(ExecutionError::LoadState(DefaultSpotBlockOutboundError::StateUnavailable,)),
        );
    }

    #[test]
    fn execute_mixed_spot_block_projects_events_and_runs_all_sink_phases() {
        let command = SpotBlockCmd { commands: vec![cancel_command(), modify_command()] };
        let outbound = FakeSpotBlockOutbound::default();

        let result = match execute_spot_block_with_outbound(&command, &outbound) {
            Ok(result) => result,
            Err(error) => {
                panic!("mixed spot block should execute with item-level rejection: {error:?}")
            }
        };

        assert_eq!(result.changes.item_results.len(), 2);
        assert!(matches!(
            &result.changes.item_results[0],
            SpotBlockItemResult::Applied { changes: SpotBlockAppliedChanges::Cancel(_), .. }
        ));
        assert!(matches!(
            &result.changes.item_results[1],
            SpotBlockItemResult::Rejected {
                error: SpotBlockItemError::Modify(ModifySpotOrderV2Error::OrderNotModifiable),
                ..
            }
        ));
        assert!(!result.events.is_empty());

        let sink_calls = outbound.sink_calls();
        assert_eq!(sink_calls.len(), 3);
        assert_eq!(sink_calls[0].phase, SinkPhase::Persist);
        assert_eq!(sink_calls[1].phase, SinkPhase::Replay);
        assert_eq!(sink_calls[2].phase, SinkPhase::Publish);
        assert!(sink_calls.iter().all(|call| call.events == result.events));
    }
}
