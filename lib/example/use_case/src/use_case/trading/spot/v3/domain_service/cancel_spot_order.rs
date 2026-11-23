use common_entity::fsm_v2::{
    CommandMetadata as FsmCommandMetadata, Fsm as FsmV2, SequencedCommand,
};
use common_entity::{EntityError, EventPosition, PositionedEntity, Shard, UpdatedEntityEvents};
use example_core_entity::{
    BalanceV2, BalanceV2Command, BalanceV2Error, SpotOrderV7, SpotOrderV7Command,
};
use rust_decimal::Decimal;
/// 取消现货订单，并按同一日志序号生成订单与可选余额事件。
pub fn cancel_spot_order(
    order: &SpotOrderV7,
    balance: &BalanceV2,
    lsn: u64,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<
    (
        UpdatedEntityEvents<SpotOrderV7>,
        Option<UpdatedEntityEvents<BalanceV2>>,
    ),
    EntityError,
> {
    let order_loaded_position = order.event_position();
    let order_metadata = FsmCommandMetadata::new_at(
        EventPosition::new(lsn, 0),
        logical_ts,
        topic,
        partition,
        order.party_id(),
    );
    let (order_after, mut order_event) = FsmV2::transition(
        order,
        SequencedCommand::new(order_metadata, SpotOrderV7Command::Cancel),
    )?;
    let release_requirement = order_after
        .terminal_release_amount()
        .map_err(|error| EntityError::Custom(error.to_string()))?;
    order_event.t = topic;
    order_event.p = partition;
    let order = UpdatedEntityEvents::try_new(order_after, order_loaded_position, vec![order_event])
        .map_err(|error| EntityError::Custom(format!("invalid order event chain: {error:?}")))?;

    let release_amount = release_requirement.amount();
    let balance = if release_amount == Decimal::ZERO {
        None
    } else {
        if balance.coin_idx() != u64::from(release_requirement.currency().index()) {
            return Err(EntityError::Custom(
                "balance coin does not match the asset released by the order".to_owned(),
            ));
        }
        let loaded_position = balance.event_position();
        let balance_metadata = FsmCommandMetadata::new_at(
            EventPosition::new(lsn, 1),
            logical_ts,
            topic,
            partition,
            balance.party_id().to_string(),
        );
        let (after, mut event) = FsmV2::transition(
            balance,
            SequencedCommand::new(
                balance_metadata,
                BalanceV2Command::UnfreezeAndPay {
                    unfreeze_amount: release_amount,
                    payment_amount: Decimal::ZERO,
                },
            ),
        )
        .map_err(map_balance_error)?;
        event.t = topic;
        event.p = partition;
        Some(
            UpdatedEntityEvents::try_new(after, loaded_position, vec![event]).map_err(|error| {
                EntityError::Custom(format!("invalid balance event chain: {error:?}"))
            })?,
        )
    };

    Ok((order, balance))
}

fn map_balance_error(error: BalanceV2Error) -> EntityError {
    match error {
        BalanceV2Error::Replay(error) => error,
        error => EntityError::Custom(format!("failed to release frozen balance: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, EventPosition, PositionedEntity};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::PriceType;
    use example_core_entity::entity::common::UserId;
    use example_core_entity::entity::common::common_order::OrderBase;
    use example_core_entity::{OrderSide, OrderTif, SpotOrderStatus};

    use super::*;

    const PARTY: u64 = 11;
    const LSN: u64 = 20;
    const LOGICAL_TS: u64 = 500;
    const TOPIC: u8 = 7;
    const PARTITION: u8 = 9;

    fn cancel(
        order: &SpotOrderV7,
        balance: &BalanceV2,
    ) -> Result<
        (
            UpdatedEntityEvents<SpotOrderV7>,
            Option<UpdatedEntityEvents<BalanceV2>>,
        ),
        EntityError,
    > {
        cancel_spot_order(order, balance, LSN, LOGICAL_TS, TOPIC, PARTITION)
    }

    fn order(status: SpotOrderStatus, position: EventPosition) -> SpotOrderV7 {
        let result = SpotOrderV7::from_record_at(
            OrderBase {
                owner: UserId(PARTY),
                order_id: 7,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
                side: OrderSide::Buy,
                limit_px: Decimal::from(100),
                size: Decimal::from(2),
                orig_size: Decimal::from(2),
                tif: OrderTif::Gtc,
                status,
                execution: PriceType::Limit,
                buy_quote_fee_reserve_limit: Decimal::from(3),
                lsn: position.lsn,
                ..OrderBase::default()
            },
            position,
        );
        let Ok(order) = result else {
            panic!("测试订单夹具必须有效")
        };
        order
    }

    fn balance(coin: Coin, frozen: Decimal, position: EventPosition) -> BalanceV2 {
        BalanceV2::new(
            PARTY,
            2,
            u64::from(coin.index()),
            Decimal::from(10),
            frozen,
            position,
            1,
        )
    }

    #[test]
    fn open_and_partially_filled_cancel_order_then_balance_in_fixed_positions() {
        for status in [SpotOrderStatus::Open, SpotOrderStatus::PartiallyFilled] {
            let order_position = EventPosition::new(10, 3);
            let balance_position = EventPosition::new(11, 4);
            let subject = order(status, order_position);
            let funds = balance(Coin::USDT, Decimal::from(500), balance_position);

            let Ok((order_changes, balance_changes)) = cancel(&subject, &funds) else {
                panic!("可取消订单应成功")
            };
            let Some(balance_changes) = balance_changes.as_ref() else {
                panic!("非零释放金额应生成余额变化")
            };
            let [order_event] = order_changes.events() else {
                panic!("订单应恰好生成一个事件")
            };
            let [balance_event] = balance_changes.events() else {
                panic!("余额应恰好生成一个事件")
            };

            assert_eq!(order_changes.after().status(), SpotOrderStatus::Canceled);
            assert_eq!(
                PositionedEntity::event_position(order_changes.after()),
                EventPosition::new(LSN, 0)
            );
            assert_eq!(
                PositionedEntity::event_position(balance_changes.after()),
                EventPosition::new(LSN, 1)
            );
            assert_eq!(order_event.previous_position, order_position);
            assert_eq!(balance_event.previous_position, balance_position);
            assert_eq!(order_event.position, EventPosition::new(LSN, 0));
            assert_eq!(balance_event.position, EventPosition::new(LSN, 1));
            assert_eq!(order_event.entity_id, Entity::entity_id(&subject));
            assert_eq!(balance_event.entity_id, Entity::entity_id(&funds));
            assert_eq!(
                order_event.entity_type,
                <SpotOrderV7 as Entity>::entity_type()
            );
            assert_eq!(
                balance_event.entity_type,
                <BalanceV2 as Entity>::entity_type()
            );
            let events = [order_event, balance_event];
            for event in events {
                assert_eq!((event.t, event.p), (TOPIC, PARTITION));
                assert_eq!(event.logical_ts, LOGICAL_TS);
            }
            assert_eq!(
                events
                    .into_iter()
                    .map(|event| event.position)
                    .collect::<Vec<_>>(),
                vec![EventPosition::new(LSN, 0), EventPosition::new(LSN, 1)]
            );
        }
    }

    #[test]
    fn pending_cancel_only_changes_order() {
        let subject = order(SpotOrderStatus::Pending, EventPosition::new(10, 2));
        let funds = balance(Coin::USDT, Decimal::ZERO, EventPosition::new(11, 1));

        let Ok((order_changes, balance_changes)) = cancel(&subject, &funds) else {
            panic!("Pending 订单应成功取消")
        };

        assert!(balance_changes.is_none());
        let [order_event] = order_changes.events() else {
            panic!("订单应恰好生成一个事件")
        };
        assert_eq!(order_event.position, EventPosition::new(LSN, 0));
        assert_eq!(
            &funds,
            &balance(Coin::USDT, Decimal::ZERO, EventPosition::new(11, 1))
        );
    }

    #[test]
    fn failures_return_no_changes_and_leave_inputs_unchanged() {
        let cases = [
            (
                order(SpotOrderStatus::Open, EventPosition::new(10, 0)),
                balance(Coin::BTC, Decimal::from(500), EventPosition::new(11, 0)),
                EntityError::Custom(
                    "balance coin does not match the asset released by the order".to_owned(),
                ),
            ),
            (
                order(SpotOrderStatus::Open, EventPosition::new(10, 0)),
                balance(Coin::USDT, Decimal::ONE, EventPosition::new(11, 0)),
                EntityError::Custom(
                    "failed to release frozen balance: frozen balance is insufficient".to_owned(),
                ),
            ),
        ];
        for (subject, funds, expected) in cases {
            let order_before = subject.clone();
            let balance_before = funds.clone();
            assert_eq!(cancel(&subject, &funds), Err(expected));
            assert_eq!(subject, order_before);
            assert_eq!(funds, balance_before);
        }

        let subject = order(SpotOrderStatus::Canceled, EventPosition::new(10, 0));
        let funds = balance(Coin::USDT, Decimal::from(500), EventPosition::new(11, 0));
        let order_before = subject.clone();
        let balance_before = funds.clone();
        assert!(cancel(&subject, &funds).is_err());
        assert_eq!(subject, order_before);
        assert_eq!(funds, balance_before);
    }
}
