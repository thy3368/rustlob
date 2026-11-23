use common_entity::fsm_v2::{
    CommandMetadata as FsmCommandMetadata, Fsm as FsmV2, SequencedCommand,
};
use common_entity::{EntityError, EventPosition, PositionedEntity, Shard, UpdatedEntityEvents};
use example_core_entity::{
    BalanceV2, BalanceV2Command, BalanceV2Error, SpotOrderV7, SpotOrderV7Command,
};

/// 激活 Pending 现货订单，同时冻结其作为 taker 所需的余额。
pub fn freeze_spot_order(
    order: &SpotOrderV7,
    balance: &BalanceV2,
    lsn: u64,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<
    (
        UpdatedEntityEvents<SpotOrderV7>,
        UpdatedEntityEvents<BalanceV2>,
    ),
    EntityError,
> {
    if balance.party_id() != order.order_base().owner.0 {
        return Err(EntityError::Custom(
            "balance owner does not match the order owner".to_owned(),
        ));
    }
    let requirement = order
        .taker_freeze_amount()
        .map_err(|error| EntityError::Custom(format!("failed to derive freeze amount: {error}")))?;
    if balance.coin_idx() != u64::from(requirement.currency().index()) {
        return Err(EntityError::Custom(
            "balance coin does not match the asset frozen by the order".to_owned(),
        ));
    }

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
        SequencedCommand::new(order_metadata, SpotOrderV7Command::Freeze),
    )?;
    order_event.t = topic;
    order_event.p = partition;
    let order_changes =
        UpdatedEntityEvents::try_new(order_after, order_loaded_position, vec![order_event])
            .map_err(|error| {
                EntityError::Custom(format!("invalid order event chain: {error:?}"))
            })?;

    let balance_loaded_position = balance.event_position();
    let balance_metadata = FsmCommandMetadata::new_at(
        EventPosition::new(lsn, 1),
        logical_ts,
        topic,
        partition,
        balance.party_id().to_string(),
    );
    let (balance_after, mut balance_event) = FsmV2::transition(
        balance,
        SequencedCommand::new(
            balance_metadata,
            BalanceV2Command::Freeze {
                amount: requirement.amount(),
            },
        ),
    )
    .map_err(map_balance_error)?;
    balance_event.t = topic;
    balance_event.p = partition;
    let balance_changes = UpdatedEntityEvents::try_new(
        balance_after,
        balance_loaded_position,
        vec![balance_event],
    )
    .map_err(|error| EntityError::Custom(format!("invalid balance event chain: {error:?}")))?;

    Ok((order_changes, balance_changes))
}

fn map_balance_error(error: BalanceV2Error) -> EntityError {
    match error {
        BalanceV2Error::Replay(error) => error,
        error => EntityError::Custom(format!("failed to freeze balance: {error}")),
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
    use rust_decimal::Decimal;

    use super::*;

    const PARTY: u64 = 11;
    const LSN: u64 = 20;
    const LOGICAL_TS: u64 = 500;
    const TOPIC: u8 = 7;
    const PARTITION: u8 = 9;

    fn order(side: OrderSide, status: SpotOrderStatus, position: EventPosition) -> SpotOrderV7 {
        SpotOrderV7::from_record_at(
            OrderBase {
                owner: UserId(PARTY),
                order_id: 7,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
                side,
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
        )
        .unwrap_or_else(|error| panic!("测试订单夹具必须有效: {error}"))
    }

    fn balance(
        party_id: u64,
        coin: Coin,
        available: Decimal,
        position: EventPosition,
    ) -> BalanceV2 {
        BalanceV2::new(
            party_id,
            2,
            u64::from(coin.index()),
            available,
            Decimal::ZERO,
            position,
            1,
        )
    }

    fn freeze(
        order: &SpotOrderV7,
        balance: &BalanceV2,
    ) -> Result<
        (
            UpdatedEntityEvents<SpotOrderV7>,
            UpdatedEntityEvents<BalanceV2>,
        ),
        EntityError,
    > {
        freeze_spot_order(order, balance, LSN, LOGICAL_TS, TOPIC, PARTITION)
    }

    #[test]
    fn pending_limit_buy_opens_and_freezes_quote_principal_and_fee_reserve() {
        let subject = order(
            OrderSide::Buy,
            SpotOrderStatus::Pending,
            EventPosition::new(10, 3),
        );
        let funds = balance(
            PARTY,
            Coin::USDT,
            Decimal::from(500),
            EventPosition::new(11, 4),
        );

        let (order_changes, balance_changes) =
            freeze(&subject, &funds).unwrap_or_else(|error| panic!("买单冻结应成功: {error}"));

        assert_eq!(order_changes.after().status(), SpotOrderStatus::Open);
        assert_eq!(balance_changes.after().available(), Decimal::from(297));
        assert_eq!(balance_changes.after().frozen(), Decimal::from(203));
    }

    #[test]
    fn pending_sell_opens_and_freezes_original_base_size() {
        let subject = order(
            OrderSide::Sell,
            SpotOrderStatus::Pending,
            EventPosition::new(10, 0),
        );
        let funds = balance(
            PARTY,
            Coin::BTC,
            Decimal::from(5),
            EventPosition::new(11, 0),
        );

        let (order_changes, balance_changes) =
            freeze(&subject, &funds).unwrap_or_else(|error| panic!("卖单冻结应成功: {error}"));

        assert_eq!(order_changes.after().status(), SpotOrderStatus::Open);
        assert_eq!(balance_changes.after().available(), Decimal::from(3));
        assert_eq!(balance_changes.after().frozen(), Decimal::from(2));
    }

    #[test]
    fn emits_order_and_balance_events_with_fixed_identity_and_metadata() {
        let order_position = EventPosition::new(10, 3);
        let balance_position = EventPosition::new(11, 4);
        let subject = order(OrderSide::Buy, SpotOrderStatus::Pending, order_position);
        let funds = balance(PARTY, Coin::USDT, Decimal::from(500), balance_position);

        let (order_changes, balance_changes) =
            freeze(&subject, &funds).unwrap_or_else(|error| panic!("冻结应成功: {error}"));
        let [order_event] = order_changes.events() else {
            panic!("订单应恰好生成一个事件")
        };
        let [balance_event] = balance_changes.events() else {
            panic!("余额应恰好生成一个事件")
        };

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
        assert_eq!(
            (
                order_event.domain_event_type,
                balance_event.domain_event_type
            ),
            (2, 1)
        );
        assert_eq!(
            (
                order_event.previous_position,
                balance_event.previous_position
            ),
            (order_position, balance_position)
        );
        assert_eq!(
            (order_event.position, balance_event.position),
            (EventPosition::new(LSN, 0), EventPosition::new(LSN, 1))
        );
        assert_eq!(
            PositionedEntity::event_position(order_changes.after()),
            EventPosition::new(LSN, 0)
        );
        assert_eq!(
            PositionedEntity::event_position(balance_changes.after()),
            EventPosition::new(LSN, 1)
        );
        for event in [order_event, balance_event] {
            assert_eq!(event.logical_ts, LOGICAL_TS);
            assert_eq!((event.t, event.p), (TOPIC, PARTITION));
        }
    }

    #[test]
    fn failures_return_no_changes_and_leave_inputs_unchanged() {
        let cases = [
            (
                order(
                    OrderSide::Buy,
                    SpotOrderStatus::Open,
                    EventPosition::new(10, 0),
                ),
                balance(
                    PARTY,
                    Coin::USDT,
                    Decimal::from(500),
                    EventPosition::new(11, 0),
                ),
                EntityError::Custom("order is not pending".to_owned()),
            ),
            (
                order(
                    OrderSide::Buy,
                    SpotOrderStatus::Pending,
                    EventPosition::new(10, 0),
                ),
                balance(
                    PARTY + 1,
                    Coin::USDT,
                    Decimal::from(500),
                    EventPosition::new(11, 0),
                ),
                EntityError::Custom("balance owner does not match the order owner".to_owned()),
            ),
            (
                order(
                    OrderSide::Buy,
                    SpotOrderStatus::Pending,
                    EventPosition::new(10, 0),
                ),
                balance(
                    PARTY,
                    Coin::BTC,
                    Decimal::from(500),
                    EventPosition::new(11, 0),
                ),
                EntityError::Custom(
                    "balance coin does not match the asset frozen by the order".to_owned(),
                ),
            ),
            (
                order(
                    OrderSide::Buy,
                    SpotOrderStatus::Pending,
                    EventPosition::new(10, 0),
                ),
                balance(
                    PARTY,
                    Coin::USDT,
                    Decimal::from(202),
                    EventPosition::new(11, 0),
                ),
                EntityError::Custom(
                    "failed to freeze balance: available balance is insufficient".to_owned(),
                ),
            ),
            (
                order(
                    OrderSide::Buy,
                    SpotOrderStatus::Pending,
                    EventPosition::new(LSN + 1, 0),
                ),
                balance(
                    PARTY,
                    Coin::USDT,
                    Decimal::from(500),
                    EventPosition::new(11, 0),
                ),
                EntityError::Custom("order command LSN is older than the current LSN".to_owned()),
            ),
            (
                order(
                    OrderSide::Buy,
                    SpotOrderStatus::Pending,
                    EventPosition::new(10, 0),
                ),
                balance(
                    PARTY,
                    Coin::USDT,
                    Decimal::from(500),
                    EventPosition::new(LSN + 1, 0),
                ),
                EntityError::Custom(
                    "failed to freeze balance: incoming lsn 20 is older than current lsn 21"
                        .to_owned(),
                ),
            ),
        ];

        for (subject, funds, expected) in cases {
            let order_before = subject.clone();
            let balance_before = funds.clone();
            assert_eq!(freeze(&subject, &funds), Err(expected));
            assert_eq!(subject, order_before);
            assert_eq!(funds, balance_before);
        }
    }

    #[test]
    fn closed_balance_failure_has_freeze_context_and_leaves_inputs_unchanged() {
        let subject = order(
            OrderSide::Buy,
            SpotOrderStatus::Pending,
            EventPosition::new(10, 0),
        );
        let open = balance(PARTY, Coin::USDT, Decimal::ZERO, EventPosition::new(11, 0));
        let metadata = FsmCommandMetadata::new_at(
            EventPosition::new(12, 0),
            12,
            TOPIC,
            PARTITION,
            PARTY.to_string(),
        );
        let (closed, _) = FsmV2::transition(
            &open,
            SequencedCommand::new(metadata, BalanceV2Command::Close),
        )
        .unwrap_or_else(|error| panic!("零余额应可关闭: {error}"));
        let order_before = subject.clone();
        let balance_before = closed.clone();

        assert_eq!(
            freeze(&subject, &closed),
            Err(EntityError::Custom(
                "failed to freeze balance: balance is closed".to_owned()
            ))
        );
        assert_eq!(subject, order_before);
        assert_eq!(closed, balance_before);
    }
}
