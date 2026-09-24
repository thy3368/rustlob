use cmd_handler::command_use_case_def2::{
    ExecutionError, ExecutionResult, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    ModifySpotOrderV2Changes, ModifySpotOrderV2Cmd, ModifySpotOrderV2Error,
    ModifySpotOrderV2UseCase,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2ModifyOutbound, DefaultSpotOrderV2ModifyOutboundError,
};

pub fn execute_modify_spot_order_v2(
    command: &ModifySpotOrderV2Cmd,
) -> Result<
    ExecutionResult<ModifySpotOrderV2Changes>,
    ExecutionError<ModifySpotOrderV2Error, DefaultSpotOrderV2ModifyOutboundError>,
> {
    execute_modify_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2ModifyOutbound)
}

pub fn execute_modify_spot_order_v2_with_outbound<OB>(
    command: &ModifySpotOrderV2Cmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<ModifySpotOrderV2Changes>,
    ExecutionError<ModifySpotOrderV2Error, <OB as StateSink<ModifySpotOrderV2UseCase>>::Error>,
>
where
    OB: StateSource<
            ModifySpotOrderV2UseCase,
            Error = <OB as StateSink<ModifySpotOrderV2UseCase>>::Error,
        > + StateSink<ModifySpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<ModifySpotOrderV2UseCase, OB, OB>(
        &ModifySpotOrderV2UseCase,
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
        ActivatePendingSpotOrderV2Input, Balance, ModifySpotOrderV2OrderType,
        ModifySpotOrderV2State, OrderId, SpotOrderSide, SpotOrderTif, SpotOrderType, SpotOrderV2,
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FakeModifyOutboundError;

    impl fmt::Display for FakeModifyOutboundError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("fake modify outbound error")
        }
    }

    impl std::error::Error for FakeModifyOutboundError {}

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
    struct FakeModifyOutbound {
        sink_calls: Arc<Mutex<Vec<ObservedSinkCall>>>,
    }

    impl FakeModifyOutbound {
        fn sink_calls(&self) -> Vec<ObservedSinkCall> {
            self.sink_calls.lock().map(|calls| calls.clone()).unwrap_or_default()
        }
    }

    impl StateSource<ModifySpotOrderV2UseCase> for FakeModifyOutbound {
        type Error = FakeModifyOutboundError;

        fn load_given_state(
            &self,
            _cmd: &ModifySpotOrderV2Cmd,
        ) -> Result<ModifySpotOrderV2State, Self::Error> {
            let mut order = SpotOrderV2::new_pending_limit(
                1,
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
                .map_err(|_| FakeModifyOutboundError)?;
            let frozen = order
                .reservation
                .remaining_amount
                .checked_add(order.fee_reservation.remaining_amount)
                .ok_or(FakeModifyOutboundError)?;

            Ok(ModifySpotOrderV2State {
                order,
                balances: vec![Balance::new(
                    "buyer".to_owned(),
                    "USDT".to_owned(),
                    100_000,
                    frozen,
                    1,
                )],
                base_asset_id: "BTC".to_owned(),
                quote_asset_id: "USDT".to_owned(),
                maker_fee_bps: 5,
                taker_fee_bps: 10,
            })
        }
    }

    impl StateSink<ModifySpotOrderV2UseCase> for FakeModifyOutbound {
        type Error = FakeModifyOutboundError;

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

    impl FakeModifyOutbound {
        fn record(
            &self,
            phase: SinkPhase,
            events: &[EntityReplayableEvent],
        ) -> Result<(), FakeModifyOutboundError> {
            self.sink_calls
                .lock()
                .map_err(|_| FakeModifyOutboundError)?
                .push(ObservedSinkCall { phase, events: events.to_vec() });
            Ok(())
        }
    }

    fn valid_command() -> ModifySpotOrderV2Cmd {
        ModifySpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            order_id: OrderId::Oid(1),
            is_buy: true,
            price: "12000".to_owned(),
            size: "3".to_owned(),
            order_type: ModifySpotOrderV2OrderType::Limit { tif: "gtc".to_owned() },
            cloid: None,
        }
    }

    #[test]
    fn execute_modify_with_default_outbound_stops_at_load_state() {
        let result = execute_modify_spot_order_v2(&valid_command());

        assert_eq!(
            result,
            Err(
                ExecutionError::LoadState(DefaultSpotOrderV2ModifyOutboundError::StateUnavailable,)
            ),
        );
    }

    #[test]
    fn execute_modify_open_buy_limit_order_projects_events_and_runs_all_sink_phases() {
        let outbound = FakeModifyOutbound::default();
        let result = execute_modify_spot_order_v2_with_outbound(&valid_command(), &outbound)
            .expect("open buy limit order modification should execute");

        let changes = &result.changes;
        assert_eq!(changes.updated_order.before.order_id(), 1);
        assert_eq!(changes.updated_order.after.order_id(), 1);
        assert_eq!(changes.updated_order.after.version, changes.updated_order.before.version + 1);
        assert_eq!(changes.updated_order.after.order_price(), 12_000);
        assert_eq!(changes.updated_order.after.qty(), 3);
        assert_eq!(changes.updated_order.after.client_order_id.as_deref(), Some("original-cloid"));
        assert_eq!(changes.created_balance_ledger_entries.len(), 2);
        assert!(changes.created_balance_ledger_entries.iter().all(|entry| entry.amount > 0));
        assert!(!changes.updated_balances.is_empty());

        assert_eq!(result.events.len(), 5);
        assert!(result.events[0].is_updated());
        assert!(result.events[1].is_updated());
        assert!(result.events[2].is_updated());
        assert!(result.events[3].is_created());
        assert!(result.events[4].is_created());
        assert!(result.events.iter().all(|event| event.new_version == event.old_version + 1));

        let sink_calls = outbound.sink_calls();
        assert_eq!(sink_calls.len(), 3);
        assert_eq!(sink_calls[0].phase, SinkPhase::Persist);
        assert_eq!(sink_calls[1].phase, SinkPhase::Replay);
        assert_eq!(sink_calls[2].phase, SinkPhase::Publish);
        assert!(sink_calls.iter().all(|call| call.events == result.events));
    }
}
