use cmd_handler::command_use_case_def2::{
    ExecutionResult, MiFamilyExecutionError, StateMachineExecutor, StateSink, StateSource,
};
use example_core_use_case::{
    PlaceOnlySpotOrderV2Changes, PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2Error,
    PlaceOnlySpotOrderV2UseCase,
};
use example_outbound_adapter::{
    DefaultSpotOrderV2PlaceOnlyOutbound, DefaultSpotOrderV2PlaceOnlyOutboundError,
};

pub fn execute_place_only_spot_order_v2(
    command: &PlaceOnlySpotOrderV2Cmd,
) -> Result<
    ExecutionResult<PlaceOnlySpotOrderV2Changes>,
    MiFamilyExecutionError<PlaceOnlySpotOrderV2Error, DefaultSpotOrderV2PlaceOnlyOutboundError>,
> {
    execute_place_only_spot_order_v2_with_outbound(command, &DefaultSpotOrderV2PlaceOnlyOutbound)
}

pub fn execute_place_only_spot_order_v2_with_outbound<OB>(
    command: &PlaceOnlySpotOrderV2Cmd,
    outbound: &OB,
) -> Result<
    ExecutionResult<PlaceOnlySpotOrderV2Changes>,
    MiFamilyExecutionError<
        PlaceOnlySpotOrderV2Error,
        <OB as StateSink<PlaceOnlySpotOrderV2UseCase>>::Error,
    >,
>
where
    OB: StateSource<
            PlaceOnlySpotOrderV2UseCase,
            Error = <OB as StateSink<PlaceOnlySpotOrderV2UseCase>>::Error,
        > + StateSink<PlaceOnlySpotOrderV2UseCase>,
{
    StateMachineExecutor.execute::<PlaceOnlySpotOrderV2UseCase, OB, OB>(
        &PlaceOnlySpotOrderV2UseCase,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use cmd_handler::EntityReplayableEvent;
    use cmd_handler::command_use_case_def2::{StateSink, StateSource};
    use common_entity::Entity;
    use example_core_use_case::{
        PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType,
        PlaceOnlySpotOrderV2State, PlaceOnlySpotOrderV2UseCase, SpotOrderGroupRelation,
        SpotOrderStatus, SpotOrderV2,
    };

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FakePlaceOnlySpotOrderV2OutboundError;

    impl std::fmt::Display for FakePlaceOnlySpotOrderV2OutboundError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "fake place-only spot order v2 outbound error")
        }
    }

    impl std::error::Error for FakePlaceOnlySpotOrderV2OutboundError {}

    #[derive(Debug, Default)]
    struct FakePlaceOnlySpotOrderV2Outbound {
        persisted: Mutex<Vec<Vec<EntityReplayableEvent>>>,
        replayed: Mutex<Vec<Vec<EntityReplayableEvent>>>,
        published: Mutex<Vec<Vec<EntityReplayableEvent>>>,
    }

    impl StateSource<PlaceOnlySpotOrderV2UseCase> for FakePlaceOnlySpotOrderV2Outbound {
        type Error = FakePlaceOnlySpotOrderV2OutboundError;

        fn load_given_state(
            &self,
            _cmd: &PlaceOnlySpotOrderV2Cmd,
        ) -> Result<PlaceOnlySpotOrderV2State, Self::Error> {
            Ok(())
        }
    }

    impl StateSink<PlaceOnlySpotOrderV2UseCase> for FakePlaceOnlySpotOrderV2Outbound {
        type Error = FakePlaceOnlySpotOrderV2OutboundError;

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.persisted
                .lock()
                .expect("persist record mutex should not be poisoned")
                .push(events.to_vec());
            Ok(())
        }

        fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.replayed
                .lock()
                .expect("replay record mutex should not be poisoned")
                .push(events.to_vec());
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.published
                .lock()
                .expect("publish record mutex should not be poisoned")
                .push(events.to_vec());
            Ok(())
        }
    }

    fn limit_cmd(tif: &str) -> PlaceOnlySpotOrderV2OrderCmd {
        PlaceOnlySpotOrderV2OrderCmd {
            party_id: "trader-1".to_string(),
            asset: 10_001,
            order_id: "order-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            is_buy: true,
            price: "100".to_string(),
            size: "2".to_string(),
            order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: tif.to_string() },
            reduce_only: false,
            cloid: Some("cloid-1".to_string()),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 1,
            taker_fee_bps: 5,
        }
    }

    fn trigger_cmd(order_id: &str, is_market: bool) -> PlaceOnlySpotOrderV2OrderCmd {
        PlaceOnlySpotOrderV2OrderCmd {
            order_id: order_id.to_string(),
            is_buy: false,
            price: "95".to_string(),
            size: "1".to_string(),
            order_type: PlaceOnlySpotOrderV2OrderType::Trigger {
                is_market,
                trigger_price: "90".to_string(),
                trigger_role: "sl".to_string(),
            },
            reduce_only: true,
            cloid: None,
            ..limit_cmd("gtc")
        }
    }

    #[test]
    fn execute_place_only_single_limit_with_default_outbound_projects_events() {
        let result =
            execute_place_only_spot_order_v2(&PlaceOnlySpotOrderV2Cmd::Single(limit_cmd("gtc")))
                .expect("place-only single limit should execute with default outbound");

        let PlaceOnlySpotOrderV2Changes::Single { created_order } = &result.changes else {
            panic!("single limit command should produce single-order changes");
        };
        assert_eq!(created_order.status, SpotOrderStatus::Open);
        assert_eq!(result.events.len(), 1);
        assert!(result.events[0].is_created());
        assert_eq!(result.events[0].entity_type, SpotOrderV2::entity_type());
    }

    #[test]
    fn execute_place_only_with_custom_outbound_persists_replays_and_publishes_events() {
        let outbound = FakePlaceOnlySpotOrderV2Outbound::default();
        let result = execute_place_only_spot_order_v2_with_outbound(
            &PlaceOnlySpotOrderV2Cmd::Single(limit_cmd("gtc")),
            &outbound,
        )
        .expect("place-only single limit should execute with custom outbound");

        assert!(!result.events.is_empty());
        let persisted =
            outbound.persisted.lock().expect("persist record mutex should not be poisoned");
        let replayed =
            outbound.replayed.lock().expect("replay record mutex should not be poisoned");
        let published =
            outbound.published.lock().expect("publish record mutex should not be poisoned");

        assert_eq!(persisted.as_slice(), &[result.events.clone()]);
        assert_eq!(replayed.as_slice(), &[result.events.clone()]);
        assert_eq!(published.as_slice(), &[result.events]);
    }

    #[test]
    fn execute_place_only_normal_tpsl_creates_parent_and_children() {
        let result = execute_place_only_spot_order_v2(&PlaceOnlySpotOrderV2Cmd::NormalTpsl {
            parent: limit_cmd("gtc"),
            children: vec![trigger_cmd("child-tp", false), trigger_cmd("child-sl", true)],
        })
        .expect("place-only normal tpsl should execute with default outbound");

        let PlaceOnlySpotOrderV2Changes::NormalTpsl { created_parent_order, created_child_orders } =
            &result.changes
        else {
            panic!("normal tpsl command should produce parent and child changes");
        };

        assert_eq!(created_parent_order.group_relation, SpotOrderGroupRelation::NormalTpslParent);
        assert_eq!(created_child_orders.len(), 2);
        assert!(created_child_orders.iter().all(|child| child.reduce_only));
        assert!(created_child_orders.iter().all(|child| child.is_pending()));
        assert!(created_child_orders.iter().all(|child| {
            child.group_relation
                == SpotOrderGroupRelation::NormalTpslChild {
                    parent_order_id: created_parent_order.order_id.clone(),
                }
        }));
        assert_eq!(result.events.len(), 3);
        assert!(result.events.iter().all(EntityReplayableEvent::is_created));
        assert!(result.events.iter().all(|event| event.entity_type == SpotOrderV2::entity_type()));
        assert_eq!(
            result.events[0].entity_id,
            created_parent_order
                .track_create_event()
                .expect("parent order create event should project")
                .entity_id
        );
    }
}
