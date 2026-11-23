use common_entity::fsm_v2::{CommandMetadata, SequencedCommand};
use common_entity::{CreatedEntityEvent, EntityError, EventPosition, TrackDiff};
use example_core_entity::{CreateSpotOrderV7Command, SpotOrderV7, SpotOrderV7Command};

/// 仅创建一个 `Pending` 现货订单及其 replay create event。
pub fn place_spot_order(
    command: CreateSpotOrderV7Command,
    lsn: u64,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<CreatedEntityEvent<SpotOrderV7>, EntityError> {
    let position = EventPosition::new(lsn, 0);
    let metadata = CommandMetadata::new_at(
        position,
        logical_ts,
        topic,
        partition,
        command.party_id.to_string(),
    );
    let order = SpotOrderV7::create(SequencedCommand::new(
        metadata,
        SpotOrderV7Command::Create(command),
    ))?;
    let mut event = order.track_create_event_at(position, logical_ts)?;
    event.t = topic;
    event.p = partition;
    event.domain_event_type = 1;

    CreatedEntityEvent::try_new(order, event)
        .map_err(|_| EntityError::Custom("invalid created spot order event chain".to_owned()))
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, PositionedEntity};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::PriceType;
    use example_core_entity::{
        OrderGroupRelation, OrderSide, OrderTif, OrderTriggerRole, OrderType, SpotOrderStatus,
    };
    use rust_decimal::Decimal;

    use super::*;

    const LSN: u64 = 20;
    const LOGICAL_TS: u64 = 500;
    const TOPIC: u8 = 7;
    const PARTITION: u8 = 9;

    fn command() -> CreateSpotOrderV7Command {
        CreateSpotOrderV7Command {
            party_id: 11,
            order_id: 7,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side: OrderSide::Buy,
            price: Decimal::from(100),
            size: Decimal::from(2),
            order_type: OrderType::Limit { tif: OrderTif::Alo },
            relation: OrderGroupRelation::Standalone,
            cloid: Some("client-1".to_owned()),
            fee_bps: 25,
        }
    }

    #[test]
    fn creates_pending_limit_buy_and_preserves_command_facts() {
        let Ok(created) = place_spot_order(command(), LSN, LOGICAL_TS, TOPIC, PARTITION) else {
            panic!("合法限价买单应创建成功")
        };
        let order = created.after().order_base();

        assert_eq!(order.order_id, 7);
        assert_eq!(order.symbol, Symbol::new(1, Coin::BTC, Coin::USDT));
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.limit_px, Decimal::from(100));
        assert_eq!(order.size, Decimal::from(2));
        assert_eq!(order.orig_size, Decimal::from(2));
        assert_eq!(order.tif, OrderTif::Alo);
        assert_eq!(order.group_relation, OrderGroupRelation::Standalone);
        assert_eq!(order.cloid.as_deref(), Some("client-1"));
        assert_eq!(order.status, SpotOrderStatus::Pending);
        assert_eq!(order.buy_quote_fee_reserve_limit, Decimal::new(5, 1));
    }

    #[test]
    fn creates_trigger_sell_with_non_standalone_relation() {
        let mut input = command();
        input.order_id = 8;
        input.side = OrderSide::Sell;
        input.relation = OrderGroupRelation::NormalTpslChild { parent_order_id: 6 };
        input.order_type = OrderType::Trigger {
            is_market: true,
            trigger_price: 90,
            tpsl: OrderTriggerRole::StopLoss,
        };

        let Ok(created) = place_spot_order(input, LSN, LOGICAL_TS, TOPIC, PARTITION) else {
            panic!("合法条件卖单应创建成功")
        };
        let order = created.after().order_base();

        assert_eq!(order.execution, PriceType::Market);
        assert_eq!(order.tif, OrderTif::Ioc);
        assert_eq!(order.trigger_px, Some(Decimal::from(90)));
        assert_eq!(
            order.group_relation,
            OrderGroupRelation::NormalTpslChild { parent_order_id: 6 }
        );
        assert_eq!(order.buy_quote_fee_reserve_limit, Decimal::ZERO);
    }

    #[test]
    fn emits_create_event_with_fixed_identity_and_metadata() {
        let Ok(created) = place_spot_order(command(), LSN, LOGICAL_TS, TOPIC, PARTITION) else {
            panic!("合法订单应创建成功")
        };
        let event = created.event();

        assert_eq!(
            PositionedEntity::event_position(created.after()),
            EventPosition::new(LSN, 0)
        );
        assert_eq!(event.entity_id, Entity::entity_id(created.after()));
        assert_eq!(event.entity_type, <SpotOrderV7 as Entity>::entity_type());
        assert_eq!(event.domain_event_type, 1);
        assert_eq!(event.previous_position, EventPosition::ZERO);
        assert_eq!(event.position, EventPosition::new(LSN, 0));
        assert_eq!(event.logical_ts, LOGICAL_TS);
        assert_eq!((event.t, event.p), (TOPIC, PARTITION));
    }

    #[test]
    fn rejects_invalid_entity_facts() {
        for mutate in [0_u8, 1, 2, 3] {
            let mut input = command();
            match mutate {
                0 => input.order_id = 0,
                1 => input.price = Decimal::ZERO,
                2 => input.size = Decimal::ZERO,
                _ => {
                    input.order_type = OrderType::Trigger {
                        is_market: false,
                        trigger_price: 0,
                        tpsl: OrderTriggerRole::TakeProfit,
                    }
                }
            }

            assert!(place_spot_order(input, LSN, LOGICAL_TS, TOPIC, PARTITION).is_err());
        }
    }
}
