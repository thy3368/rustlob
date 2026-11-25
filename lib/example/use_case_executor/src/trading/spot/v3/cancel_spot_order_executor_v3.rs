use common_entity::{
    ExecutionErrorV3, ExecutionResultV3, LogEntry, StateSinkV3, StateSourceV3, UseCaseExecutorV3,
};
use example_core_use_case::pipe_line::cancel_spot_order::{
    CancelSpotOrderFsmV3Changes, CancelSpotOrderFsmV3Cmd, CancelSpotOrderFsmV3Error,
    CancelSpotOrderFsmV3UseCase,
};

/// 使用共享 V3 runtime 执行 Cancel Spot Order，并复用 outbound 完成状态读写与事件副作用。
pub fn execute_cancel_spot_order_v3_with_outbound<OB>(
    command: &LogEntry<CancelSpotOrderFsmV3Cmd>,
    outbound: &OB,
) -> Result<
    ExecutionResultV3<CancelSpotOrderFsmV3Changes>,
    ExecutionErrorV3<
        CancelSpotOrderFsmV3Error,
        <OB as StateSourceV3<CancelSpotOrderFsmV3UseCase>>::Error,
        <OB as StateSinkV3>::Error,
    >,
>
where
    OB: StateSourceV3<CancelSpotOrderFsmV3UseCase> + StateSinkV3,
{
    UseCaseExecutorV3.execute::<CancelSpotOrderFsmV3UseCase, OB, OB>(
        &CancelSpotOrderFsmV3UseCase,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use common_entity::{
        ExecutionErrorV3, LogEntrySequencer, ReplayableDomainEvent, StateSinkV3, StateSourceV3,
    };
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::identifiers::OrderId;
    use example_core_entity::common::{OrderBase, OrderTrigger, PriceType};
    use example_core_entity::entity::common::order_primitives::{
        OrderSide, OrderTif, SpotOrderStatus,
    };
    use example_core_entity::spot::spot_order_v7::SpotOrderV7;
    use example_core_entity::{BalanceV2, OrderGroupRelation};
    use example_core_use_case::pipe_line::cancel_spot_order::{
        CancelSpotOrderFsmV3Cmd, CancelSpotOrderFsmV3State, CancelSpotOrderFsmV3UseCase,
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
        calls: Mutex<Vec<&'static str>>,
        persisted: Mutex<Vec<ReplayableDomainEvent>>,
        replayed: Mutex<Vec<ReplayableDomainEvent>>,
        published: Mutex<Vec<ReplayableDomainEvent>>,
    }

    impl StateSourceV3<CancelSpotOrderFsmV3UseCase> for FakeOutbound {
        type Error = FakeOutboundError;

        fn load_given_state(
            &self,
            _cmd: &LogEntry<CancelSpotOrderFsmV3Cmd>,
        ) -> Result<CancelSpotOrderFsmV3State, Self::Error> {
            self.calls
                .lock()
                .map_err(|_| FakeOutboundError)?
                .push("load");
            Ok(CancelSpotOrderFsmV3State {
                order: SpotOrderV7::from_record(order(SpotOrderStatus::Open))
                    .map_err(|_| FakeOutboundError)?,
                balance: BalanceV2::new(
                    common_entity::stable_string_entity_id("trader-1"),
                    7,
                    u64::from(Coin::USDT.index()),
                    Decimal::ZERO,
                    Decimal::from(100),
                    0,
                    0,
                ),
            })
        }
    }

    impl StateSinkV3 for FakeOutbound {
        type Error = FakeOutboundError;

        fn persist(&self, events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
            self.record("persist", events, &self.persisted)
        }

        fn replay(&self, events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
            self.record("replay", events, &self.replayed)
        }

        fn publish(&self, events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
            self.record("publish", events, &self.published)
        }
    }

    impl FakeOutbound {
        fn record(
            &self,
            call: &'static str,
            events: &[&ReplayableDomainEvent],
            destination: &Mutex<Vec<ReplayableDomainEvent>>,
        ) -> Result<(), FakeOutboundError> {
            self.calls.lock().map_err(|_| FakeOutboundError)?.push(call);
            destination
                .lock()
                .map_err(|_| FakeOutboundError)?
                .extend(events.iter().map(|event| (*event).clone()));
            Ok(())
        }
    }

    fn order(status: SpotOrderStatus) -> OrderBase {
        OrderBase {
            owner: common_entity::stable_string_entity_id("trader-1").into(),
            order_id: 11,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side: OrderSide::Buy,
            limit_px: Decimal::from(100),
            size: Decimal::ONE,
            orig_size: Decimal::ONE,
            execution: PriceType::Limit,
            tif: OrderTif::Gtc,
            trigger: OrderTrigger::None,
            trigger_px: None,
            group_relation: OrderGroupRelation::Standalone,
            is_position_tpsl: false,
            reduce_only: false,
            status,
            buy_quote_fee_reserve_frozen: Decimal::from(100),
            buy_quote_fee_reserve_paid: Decimal::ZERO,
            cloid: Some("client-order-11".to_string()),
            created_at: 1,
            updated_at: 1,
            lsn: 0,
        }
    }

    fn command(order_id: OrderId) -> LogEntry<CancelSpotOrderFsmV3Cmd> {
        LogEntrySequencer::from_metadata(
            7,
            100,
            1,
            2,
            "trader-1".to_owned(),
            CancelSpotOrderFsmV3Cmd { order_id },
        )
    }

    #[test]
    fn cancel_by_cloid_releases_balance_and_sends_order_event_first() {
        let outbound = FakeOutbound::default();
        let result = execute_cancel_spot_order_v3_with_outbound(
            &command(OrderId::Cloid("client-order-11".into())),
            &outbound,
        )
        .expect("cancel should execute");

        assert_eq!(
            result.state_changed.order.after().status(),
            SpotOrderStatus::Canceled
        );
        let balance = result
            .state_changed
            .balance
            .as_ref()
            .expect("frozen balance should be released");
        assert_eq!(balance.after().frozen(), Decimal::ZERO);
        assert_eq!(balance.after().available(), Decimal::from(100));

        let persisted = outbound.persisted.lock().expect("persist lock");
        let replayed = outbound.replayed.lock().expect("replay lock");
        let published = outbound.published.lock().expect("publish lock");
        assert_eq!(persisted.len(), 2);
        assert_eq!(persisted[0].entity_type, 16);
        assert_eq!(persisted[1].entity_type, 12);
        assert_eq!(*replayed, *persisted);
        assert_eq!(*published, *persisted);
        assert_eq!(
            *outbound.calls.lock().expect("calls lock"),
            ["load", "persist", "replay", "publish"]
        );
    }

    #[test]
    fn empty_cloid_fails_before_loading_state() {
        let outbound = FakeOutbound::default();
        let result = execute_cancel_spot_order_v3_with_outbound(
            &command(OrderId::Cloid(" ".into())),
            &outbound,
        );

        assert!(matches!(
            result,
            Err(ExecutionErrorV3::Business(
                CancelSpotOrderFsmV3Error::EmptyCloid
            ))
        ));
        assert!(outbound.calls.lock().expect("calls lock").is_empty());
        assert!(outbound.persisted.lock().expect("persist lock").is_empty());
        assert!(outbound.replayed.lock().expect("replay lock").is_empty());
        assert!(outbound.published.lock().expect("publish lock").is_empty());
    }

    #[test]
    fn numeric_oid_executes() {
        let outbound = FakeOutbound::default();
        let result =
            execute_cancel_spot_order_v3_with_outbound(&command(OrderId::Oid(11)), &outbound);

        assert!(result.is_ok());
    }
}
