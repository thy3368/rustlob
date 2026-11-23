use common_entity::fsm_v2::{
    CommandMetadata as FsmCommandMetadata, Fsm as FsmV2, SequencedCommand,
};
use common_entity::{
    EntityError, EventPosition, PositionedEntity, ReplayableDomainEvent, ReplayableEntityChanges,
    Shard, UpdatedEntityEvents, UpdatedEntityEventsError,
};
use example_core_entity::{
    BalanceV2, BalanceV2Command, BalanceV2Error, SpotOrderV7, SpotOrderV7Command,
};
use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CancelSpotOrderDomainServiceV2Metadata {
    pub lsn: u64,
    pub logical_ts: u64,
    pub topic: u8,
    pub partition: u8,
}

/// 取消订单后的强类型实体状态与原生 replay 事件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderDomainServiceV2Changes {
    pub order: UpdatedEntityEvents<SpotOrderV7>,
    pub balance: Option<UpdatedEntityEvents<BalanceV2>>,
}

impl ReplayableEntityChanges for CancelSpotOrderDomainServiceV2Changes {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let mut events = Vec::with_capacity(if self.balance.is_some() { 2 } else { 1 });
        events.extend(self.order.events());
        if let Some(balance) = &self.balance {
            events.extend(balance.events());
        }
        events
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderDomainServiceV2Error {
    #[error("balance coin does not match the asset released by the order")]
    AssetMismatch,
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("balance is closed")]
    BalanceClosed,
    #[error("invalid balance amount")]
    InvalidBalanceAmount,
    #[error("arithmetic overflow while canceling the order")]
    ArithmeticOverflow,
    #[error("invalid entity event chain: {0:?}")]
    InvalidEntityEvents(UpdatedEntityEventsError),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SpotOrderDomainServiceV2;

impl SpotOrderDomainServiceV2 {
    pub fn cancel(
        &self,
        order: &SpotOrderV7,
        balance: &BalanceV2,
        metadata: CancelSpotOrderDomainServiceV2Metadata,
    ) -> Result<CancelSpotOrderDomainServiceV2Changes, CancelSpotOrderDomainServiceV2Error> {
        let order_loaded_position = order.event_position();
        let order_metadata = FsmCommandMetadata::new_at(
            EventPosition::new(metadata.lsn, 0),
            metadata.logical_ts,
            metadata.topic,
            metadata.partition,
            order.party_id(),
        );
        let (order_after, mut order_event) = FsmV2::transition(
            order,
            SequencedCommand::new(order_metadata, SpotOrderV7Command::Cancel),
        )?;
        let release_requirement = order_after
            .terminal_release_amount()
            .map_err(|error| EntityError::Custom(error.to_string()))?;
        order_event.t = metadata.topic;
        order_event.p = metadata.partition;
        let order =
            UpdatedEntityEvents::try_new(order_after, order_loaded_position, vec![order_event])
                .map_err(CancelSpotOrderDomainServiceV2Error::InvalidEntityEvents)?;

        let release_amount = release_requirement.amount();
        let balance = if release_amount == Decimal::ZERO {
            None
        } else {
            if balance.coin_idx() != u64::from(release_requirement.currency().index()) {
                return Err(CancelSpotOrderDomainServiceV2Error::AssetMismatch);
            }
            let loaded_position = balance.event_position();
            let balance_metadata = FsmCommandMetadata::new_at(
                EventPosition::new(metadata.lsn, 1),
                metadata.logical_ts,
                metadata.topic,
                metadata.partition,
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
            event.t = metadata.topic;
            event.p = metadata.partition;
            Some(
                UpdatedEntityEvents::try_new(after, loaded_position, vec![event])
                    .map_err(CancelSpotOrderDomainServiceV2Error::InvalidEntityEvents)?,
            )
        };

        Ok(CancelSpotOrderDomainServiceV2Changes { order, balance })
    }
}

fn map_balance_error(error: BalanceV2Error) -> CancelSpotOrderDomainServiceV2Error {
    match error {
        BalanceV2Error::InsufficientFrozenBalance => {
            CancelSpotOrderDomainServiceV2Error::InsufficientFrozenBalance
        }
        BalanceV2Error::BalanceClosed => CancelSpotOrderDomainServiceV2Error::BalanceClosed,
        BalanceV2Error::InvalidAmount | BalanceV2Error::InsufficientAvailableBalance => {
            CancelSpotOrderDomainServiceV2Error::InvalidBalanceAmount
        }
        BalanceV2Error::Replay(error) => CancelSpotOrderDomainServiceV2Error::Entity(error),
        BalanceV2Error::ArithmeticOverflow
        | BalanceV2Error::CannotCloseNonZeroBalance
        | BalanceV2Error::StaleLsn { .. } => {
            CancelSpotOrderDomainServiceV2Error::ArithmeticOverflow
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, EventPosition, PositionedEntity, ReplayableEntityChanges};
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

    fn metadata() -> CancelSpotOrderDomainServiceV2Metadata {
        CancelSpotOrderDomainServiceV2Metadata {
            lsn: LSN,
            logical_ts: LOGICAL_TS,
            topic: TOPIC,
            partition: PARTITION,
        }
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

            let Ok(changes) = SpotOrderDomainServiceV2.cancel(&subject, &funds, metadata()) else {
                panic!("可取消订单应成功")
            };
            let Some(balance_changes) = changes.balance.as_ref() else {
                panic!("非零释放金额应生成余额变化")
            };
            let [order_event] = changes.order.events() else {
                panic!("订单应恰好生成一个事件")
            };
            let [balance_event] = balance_changes.events() else {
                panic!("余额应恰好生成一个事件")
            };

            assert_eq!(changes.order.after().status(), SpotOrderStatus::Canceled);
            assert_eq!(
                PositionedEntity::event_position(changes.order.after()),
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
            for event in changes.replayable_events() {
                assert_eq!((event.t, event.p), (TOPIC, PARTITION));
                assert_eq!(event.logical_ts, LOGICAL_TS);
            }
            assert_eq!(
                changes
                    .replayable_events()
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

        let Ok(changes) = SpotOrderDomainServiceV2.cancel(&subject, &funds, metadata()) else {
            panic!("Pending 订单应成功取消")
        };

        assert!(changes.balance.is_none());
        assert_eq!(changes.replayable_events().len(), 1);
        let [order_event] = changes.order.events() else {
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
                CancelSpotOrderDomainServiceV2Error::AssetMismatch,
            ),
            (
                order(SpotOrderStatus::Open, EventPosition::new(10, 0)),
                balance(Coin::USDT, Decimal::ONE, EventPosition::new(11, 0)),
                CancelSpotOrderDomainServiceV2Error::InsufficientFrozenBalance,
            ),
        ];
        for (subject, funds, expected) in cases {
            let order_before = subject.clone();
            let balance_before = funds.clone();
            assert_eq!(
                SpotOrderDomainServiceV2.cancel(&subject, &funds, metadata()),
                Err(expected)
            );
            assert_eq!(subject, order_before);
            assert_eq!(funds, balance_before);
        }

        let subject = order(SpotOrderStatus::Canceled, EventPosition::new(10, 0));
        let funds = balance(Coin::USDT, Decimal::from(500), EventPosition::new(11, 0));
        let order_before = subject.clone();
        let balance_before = funds.clone();
        assert!(
            SpotOrderDomainServiceV2
                .cancel(&subject, &funds, metadata())
                .is_err()
        );
        assert_eq!(subject, order_before);
        assert_eq!(funds, balance_before);
    }
}
