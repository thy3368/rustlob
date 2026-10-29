use common_entity::{
    ExecutionError, ExecutionResult, LogEntry, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    CancelSpotOrderV3Changes, CancelSpotOrderV3Cmd, CancelSpotOrderV3Error,
    CancelSpotOrderV3UseCase,
};

pub fn execute_cancel_spot_order_v3_with_outbound<OB>(
    command: &LogEntry<CancelSpotOrderV3Cmd>,
    outbound: &OB,
) -> Result<
    ExecutionResult<CancelSpotOrderV3Changes>,
    ExecutionError<CancelSpotOrderV3Error, <OB as StateSink<CancelSpotOrderV3UseCase>>::Error>,
>
where
    OB: StateSource<
            CancelSpotOrderV3UseCase,
            Error = <OB as StateSink<CancelSpotOrderV3UseCase>>::Error,
        > + StateSink<CancelSpotOrderV3UseCase>,
{
    StateMachineExecutor.execute::<CancelSpotOrderV3UseCase, OB, OB>(
        &CancelSpotOrderV3UseCase,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use common_entity::{EntityReplayableEvent, LogEntrySequencer, StateSink, StateSource};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::{CommonOrderExecution, CommonOrderTrigger, OrderBase};
    use example_core_entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
    use example_core_entity::spot::spot_order_v2::OrderGroupRelation;
    use example_core_entity::spot::spot_order_v3::SpotOrderV3Any;
    use example_core_use_case::{
        CancelSpotOrderV3Cmd, CancelSpotOrderV3State, CancelSpotOrderV3UseCase,
    };
    use rust_decimal::Decimal;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FakeOutboundError;

    impl std::fmt::Display for FakeOutboundError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "fake outbound error")
        }
    }

    impl std::error::Error for FakeOutboundError {}

    #[derive(Default)]
    struct FakeOutbound {
        persisted: Mutex<Vec<EntityReplayableEvent>>,
        replayed: Mutex<Vec<EntityReplayableEvent>>,
        published: Mutex<Vec<EntityReplayableEvent>>,
    }

    impl StateSource<CancelSpotOrderV3UseCase> for FakeOutbound {
        type Error = FakeOutboundError;

        fn load_given_state(
            &self,
            _cmd: &LogEntry<CancelSpotOrderV3Cmd>,
        ) -> Result<CancelSpotOrderV3State, Self::Error> {
            Ok(CancelSpotOrderV3State {
                order: SpotOrderV3Any::from_record(order(SpotOrderStatus::Open))
                    .map_err(|_| FakeOutboundError)?,
            })
        }
    }

    impl StateSink<CancelSpotOrderV3UseCase> for FakeOutbound {
        type Error = FakeOutboundError;

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.persisted.lock().map_err(|_| FakeOutboundError)?.extend_from_slice(events);
            Ok(())
        }

        fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.replayed.lock().map_err(|_| FakeOutboundError)?.extend_from_slice(events);
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.published.lock().map_err(|_| FakeOutboundError)?.extend_from_slice(events);
            Ok(())
        }
    }

    fn order(status: SpotOrderStatus) -> OrderBase {
        OrderBase {
            owner: base_types::UserId(7),
            order_id: 11,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side: OrderSide::Buy,
            limit_px: Decimal::from(100),
            size: Decimal::ONE,
            orig_size: Decimal::ONE,
            execution: CommonOrderExecution::Limit,
            tif: OrderTif::Gtc,
            trigger: CommonOrderTrigger::None,
            trigger_px: None,
            group_relation: OrderGroupRelation::Standalone,
            is_position_tpsl: false,
            reduce_only: false,
            status,
            buy_quote_fee_reserve_limit: Decimal::ZERO,
            buy_quote_fee_reserve_consumed: Decimal::ZERO,
            cloid: Some("client-order-11".to_string()),
            created_at: 1,
            updated_at: 1,
            version: 0,
        }
    }

    fn command(cloid: &str) -> LogEntry<CancelSpotOrderV3Cmd> {
        LogEntrySequencer::from_metadata(
            7,
            100,
            1,
            2,
            CancelSpotOrderV3Cmd { cloid: cloid.to_string() },
        )
    }

    #[test]
    fn cancel_executor_projects_and_sends_update_event() {
        let outbound = FakeOutbound::default();
        let result =
            execute_cancel_spot_order_v3_with_outbound(&command("client-order-11"), &outbound)
                .expect("cancel should execute");

        assert_eq!(result.changes.updated_order.before.order_base().status, SpotOrderStatus::Open);
        assert_eq!(
            result.changes.updated_order.after.order_base().status,
            SpotOrderStatus::Canceled
        );
        assert_eq!(result.events.len(), 1);
        assert_eq!(outbound.persisted.lock().expect("persist lock").len(), 1);
        assert_eq!(outbound.replayed.lock().expect("replay lock").len(), 1);
        assert_eq!(outbound.published.lock().expect("publish lock").len(), 1);
    }

    #[test]
    fn empty_cloid_fails_before_loading_state() {
        let outbound = FakeOutbound::default();
        let result = execute_cancel_spot_order_v3_with_outbound(&command(" "), &outbound);

        assert!(matches!(
            result,
            Err(ExecutionError::Business(CancelSpotOrderV3Error::EmptyCloid))
        ));
        assert!(outbound.persisted.lock().expect("persist lock").is_empty());
    }
}
