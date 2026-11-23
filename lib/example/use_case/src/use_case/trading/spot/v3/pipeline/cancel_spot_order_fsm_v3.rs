use common_entity::fsm_v2::{
    CommandMetadata as FsmCommandMetadata, Fsm as FsmV2, SequencedCommand,
};
use common_entity::{
    EntityError, LogEntry, PositionedEntity, ReplayableDomainEvent, ReplayableEntityChanges, Shard,
    UpdatedEntityEvents, UpdatedEntityEventsError, UseCaseV3,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{
    BalanceV2, BalanceV2Command, BalanceV2Error, OrderSide, SpotOrderStatus, SpotOrderV7,
    SpotOrderV7Command, SpotOrderV7Output,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderFsmV3Cmd {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderFsmV3State {
    pub order: SpotOrderV7,
    pub balance: BalanceV2,
}

/// 取消后的聚合状态与 FSM 原生 replay 事件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderFsmV3Changes {
    pub order: UpdatedEntityEvents<SpotOrderV7>,
    pub balance: Option<UpdatedEntityEvents<BalanceV2>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderFsmV3Error {
    #[error("order oid must be greater than zero")]
    InvalidOid,
    #[error("order cloid must not be empty")]
    EmptyCloid,
    #[error("order lookup does not match the loaded order")]
    OrderLookupMismatch,
    #[error("loaded order owner does not match the command user")]
    OwnerMismatch,
    #[error("loaded order has no client order id")]
    OrderCloidMissing,
    #[error("order is not cancelable")]
    OrderNotCancelable,
    #[error("command LSN must be newer than the loaded order")]
    StaleOrder,
    #[error("balance party does not match the order owner")]
    PartyMismatch,
    #[error("balance coin does not match the asset held by the order")]
    CoinMismatch,
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("balance is closed")]
    BalanceClosed,
    #[error("invalid balance amount")]
    InvalidBalanceAmount,
    #[error("arithmetic overflow while canceling the order")]
    ArithmeticOverflow,
    #[error("order FSM returned an unexpected cancel output")]
    UnexpectedOrderOutput,
    #[error("invalid entity event chain: {0:?}")]
    InvalidEntityEvents(UpdatedEntityEventsError),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderFsmV3UseCase;

impl ReplayableEntityChanges for CancelSpotOrderFsmV3Changes {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let capacity = if self.balance.is_some() { 2 } else { 1 };
        let mut events = Vec::with_capacity(capacity);
        events.extend(self.order.events());
        if let Some(balance) = &self.balance {
            events.extend(balance.events());
        }
        events
    }
}

impl UseCaseV3 for CancelSpotOrderFsmV3UseCase {
    type Command = LogEntry<CancelSpotOrderFsmV3Cmd>;
    type StateGiven = CancelSpotOrderFsmV3State;
    type Error = CancelSpotOrderFsmV3Error;
    type StateChanged = CancelSpotOrderFsmV3Changes;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match &cmd.payload().order_id {
            OrderId::Oid(0) => Err(CancelSpotOrderFsmV3Error::InvalidOid),
            OrderId::Cloid(cloid) if cloid.trim().is_empty() => {
                Err(CancelSpotOrderFsmV3Error::EmptyCloid)
            }
            OrderId::Oid(_) | OrderId::Cloid(_) => Ok(()),
        }
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        validate_state(cmd, state)
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        compute_changes(cmd, state)
    }
}

fn validate_state(
    cmd: &LogEntry<CancelSpotOrderFsmV3Cmd>,
    state: &CancelSpotOrderFsmV3State,
) -> Result<(), CancelSpotOrderFsmV3Error> {
    let order = state.order.order_base();
    if order.owner.0 != common_entity::stable_string_entity_id(cmd.user_id()) {
        return Err(CancelSpotOrderFsmV3Error::OwnerMismatch);
    }
    match &cmd.payload().order_id {
        OrderId::Oid(oid) if *oid != order.order_id => {
            return Err(CancelSpotOrderFsmV3Error::OrderLookupMismatch);
        }
        OrderId::Cloid(cloid) => match order.cloid.as_deref() {
            None => return Err(CancelSpotOrderFsmV3Error::OrderCloidMissing),
            Some(loaded) if loaded != cloid => {
                return Err(CancelSpotOrderFsmV3Error::OrderLookupMismatch);
            }
            Some(_) => {}
        },
        OrderId::Oid(_) => {}
    }
    if !matches!(
        state.order.status(),
        SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
    ) {
        return Err(CancelSpotOrderFsmV3Error::OrderNotCancelable);
    }
    if cmd.lsn() <= order.lsn {
        return Err(CancelSpotOrderFsmV3Error::StaleOrder);
    }
    if state.balance.party_id() != order.owner.0 {
        return Err(CancelSpotOrderFsmV3Error::PartyMismatch);
    }
    let held_coin = match order.side {
        OrderSide::Buy => order.symbol.quote,
        OrderSide::Sell => order.symbol.base,
    };
    if state.balance.coin_idx() != u64::from(held_coin.index()) {
        return Err(CancelSpotOrderFsmV3Error::CoinMismatch);
    }
    Ok(())
}

fn compute_changes(
    cmd: &LogEntry<CancelSpotOrderFsmV3Cmd>,
    state: CancelSpotOrderFsmV3State,
) -> Result<CancelSpotOrderFsmV3Changes, CancelSpotOrderFsmV3Error> {
    let CancelSpotOrderFsmV3State {
        order: mut order_after,
        balance: mut balance_after,
    } = state;
    let order_loaded_position = order_after.event_position();
    let metadata = FsmCommandMetadata::new(
        cmd.lsn(),
        cmd.logical_ts(),
        order_after.topic(),
        order_after.partition(),
        order_after.party_id(),
    );
    let transition = FsmV2::transition(
        &mut order_after,
        SequencedCommand::new(metadata, SpotOrderV7Command::Cancel),
    )?;
    let SpotOrderV7Output::Canceled(output) = transition.output else {
        return Err(CancelSpotOrderFsmV3Error::UnexpectedOrderOutput);
    };

    let balance = if output.release_requirement.amount() == Decimal::ZERO {
        None
    } else {
        let balance_loaded_position = balance_after.event_position();
        let metadata = FsmCommandMetadata::new(
            cmd.lsn(),
            cmd.logical_ts(),
            balance_after.topic(),
            balance_after.partition(),
            balance_after.party_id().to_string(),
        );
        let balance_transition = FsmV2::transition(
            &mut balance_after,
            SequencedCommand::new(
                metadata,
                BalanceV2Command::UnfreezeAndPay {
                    unfreeze_amount: output.release_requirement.amount(),
                    payment_amount: Decimal::ZERO,
                },
            ),
        )
        .map_err(map_balance_error)?;
        Some(
            UpdatedEntityEvents::try_new(
                balance_after,
                balance_loaded_position,
                vec![balance_transition.event],
            )
            .map_err(CancelSpotOrderFsmV3Error::InvalidEntityEvents)?,
        )
    };

    Ok(CancelSpotOrderFsmV3Changes {
        order: UpdatedEntityEvents::try_new(
            order_after,
            order_loaded_position,
            vec![transition.event],
        )
        .map_err(CancelSpotOrderFsmV3Error::InvalidEntityEvents)?,
        balance,
    })
}

fn map_balance_error(error: BalanceV2Error) -> CancelSpotOrderFsmV3Error {
    match error {
        BalanceV2Error::InsufficientFrozenBalance => {
            CancelSpotOrderFsmV3Error::InsufficientFrozenBalance
        }
        BalanceV2Error::BalanceClosed => CancelSpotOrderFsmV3Error::BalanceClosed,
        BalanceV2Error::InvalidAmount | BalanceV2Error::InsufficientAvailableBalance => {
            CancelSpotOrderFsmV3Error::InvalidBalanceAmount
        }
        BalanceV2Error::Replay(error) => CancelSpotOrderFsmV3Error::Entity(error),
        BalanceV2Error::ArithmeticOverflow
        | BalanceV2Error::CannotCloseNonZeroBalance
        | BalanceV2Error::StaleLsn { .. } => CancelSpotOrderFsmV3Error::ArithmeticOverflow,
    }
}

#[cfg(test)]
mod tests {
    //! 业务矩阵：定位方式 OID/CLOID；状态 Pending/Open/PartiallyFilled；
    //! 非零释放产生 order+balance replay，零释放只产生 order replay；所有成功取消产生一个领域事件。

    use common_entity::{Entity, LogEntrySequencer, UseCaseComputeError, UseCaseV3};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::{OrderBase, OrderTrigger, PriceType, UserId};
    use example_core_entity::{OrderGroupRelation, OrderTif};

    use super::*;

    const OWNER: &str = "trader-1";
    const ORDER_ID: u64 = 11;
    const LSN: u64 = 7;
    const LOGICAL_TS: u64 = 100;

    fn order(status: SpotOrderStatus) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(common_entity::stable_string_entity_id(OWNER)),
            order_id: ORDER_ID,
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
            buy_quote_fee_reserve_limit: Decimal::ZERO,
            buy_quote_fee_reserve_consumed: Decimal::ZERO,
            cloid: Some("client-order-11".into()),
            created_at: 1,
            updated_at: 1,
            lsn: 0,
        })
        .expect("valid order fixture")
    }

    fn balance(frozen: Decimal) -> BalanceV2 {
        BalanceV2::new(
            common_entity::stable_string_entity_id(OWNER),
            7,
            u64::from(Coin::USDT.index()),
            Decimal::ZERO,
            frozen,
            0,
            0,
        )
    }

    fn state(status: SpotOrderStatus, frozen: Decimal) -> CancelSpotOrderFsmV3State {
        CancelSpotOrderFsmV3State {
            order: order(status),
            balance: balance(frozen),
        }
    }

    fn command(order_id: OrderId) -> LogEntry<CancelSpotOrderFsmV3Cmd> {
        LogEntrySequencer::from_metadata(
            LSN,
            LOGICAL_TS,
            1,
            2,
            OWNER.into(),
            CancelSpotOrderFsmV3Cmd { order_id },
        )
    }

    #[test]
    fn open_order_located_by_cloid_cancels_and_releases_its_hold() {
        // Rule: 活跃订单取消必须同时推进订单与冻结余额，并先投影订单 replay。
        // Given: 一个通过 CLOID 定位、冻结 100 USDT 的 Open 买单。
        let cmd = command(OrderId::Cloid("client-order-11".into()));
        let given = state(SpotOrderStatus::Open, Decimal::from(100));

        // When: 执行取消并形成可回放 changes。
        let changed = CancelSpotOrderFsmV3UseCase
            .compute_state_changed(&cmd, given)
            .expect("open order should cancel");

        // Then: after state 用于后续块内状态，FSM 原生事件用于 replay。
        assert_eq!(changed.order.after().status(), SpotOrderStatus::Canceled);
        let balance_after = changed
            .balance
            .as_ref()
            .expect("non-zero release must update balance")
            .after();
        assert_eq!(balance_after.frozen(), Decimal::ZERO);
        assert_eq!(balance_after.available(), Decimal::from(100));

        let events = changed.replayable_events();
        let [order_event, balance_event] = events.as_slice() else {
            panic!("non-zero release must emit order and balance events");
        };
        assert_eq!(*order_event, changed.order.events().first().unwrap());
        assert_eq!(
            *balance_event,
            changed.balance.as_ref().unwrap().events().first().unwrap()
        );
        assert_eq!(order_event.domain_event_type, 6);
        assert_eq!(balance_event.domain_event_type, 2);
        assert_eq!(
            (balance_event.lsn, balance_event.logical_ts),
            (LSN, LOGICAL_TS)
        );
        assert_eq!((order_event.lsn, order_event.logical_ts), (LSN, LOGICAL_TS));
        assert_eq!((order_event.previous_lsn, order_event.change_type), (0, 1));
        assert!(order_event.field_changes.iter().any(|change| {
            change.field_name_as_str() == Ok("status")
                && change.old_value_bytes() == b"Open"
                && change.new_value_bytes() == b"Canceled"
        }));
        assert_eq!(changed.order.events().first().unwrap().lsn, LSN);
        assert_eq!(
            changed.order.events().first().unwrap().logical_ts,
            LOGICAL_TS
        );
    }

    #[test]
    fn partially_filled_order_located_by_oid_releases_remaining_hold() {
        // Rule: OID 与 CLOID 只改变定位方式，不改变取消的状态与事件语义。
        // Given: 一个通过 OID 定位的 PartiallyFilled 买单。
        let cmd = command(OrderId::Oid(ORDER_ID));
        let given = state(SpotOrderStatus::PartiallyFilled, Decimal::from(100));

        // When: 执行取消。
        let changes = CancelSpotOrderFsmV3UseCase
            .compute_state_changed(&cmd, given)
            .expect("partially filled order should cancel");

        // Then: 返回两个 after state，事件顺序固定为 order、balance。
        assert_eq!(changes.order.after().status(), SpotOrderStatus::Canceled);
        assert!(changes.balance.is_some());
        let events = changes.replayable_events();
        let [order_event, balance_event] = events.as_slice() else {
            panic!("non-zero release must emit order and balance events");
        };
        assert_eq!(
            order_event.entity_id,
            changes.order.after().order_base().order_id
        );
        assert_eq!(
            balance_event.entity_id,
            Entity::entity_id(changes.balance.as_ref().expect("balance after").after())
        );
    }

    #[test]
    fn pending_order_cancels_without_touching_balance() {
        // Rule: Pending 订单未占用冻结额，取消不得制造余额变化。
        // Given: 一个 Pending 买单与不相关的零冻结余额。
        let cmd = command(OrderId::Oid(ORDER_ID));
        let given = state(SpotOrderStatus::Pending, Decimal::ZERO);

        // When: 执行取消。
        let changes = CancelSpotOrderFsmV3UseCase
            .compute_state_changed(&cmd, given)
            .expect("pending order should cancel");

        // Then: 仅订单 pair/replay 变化，但仍保留 canceled 事件码。
        assert_eq!(changes.order.after().status(), SpotOrderStatus::Canceled);
        assert_eq!(changes.balance, None);
        assert_eq!(changes.replayable_events().len(), 1);
        assert_eq!(changes.order.events().first().unwrap().domain_event_type, 6);
    }

    #[test]
    fn invalid_lookup_and_authoritative_state_are_rejected() {
        let valid_state = state(SpotOrderStatus::Open, Decimal::from(100));
        assert_eq!(
            CancelSpotOrderFsmV3UseCase.check_command(&command(OrderId::Oid(0))),
            Err(CancelSpotOrderFsmV3Error::InvalidOid)
        );
        assert_eq!(
            CancelSpotOrderFsmV3UseCase.check_command(&command(OrderId::Cloid(" ".into()))),
            Err(CancelSpotOrderFsmV3Error::EmptyCloid)
        );
        assert_eq!(
            CancelSpotOrderFsmV3UseCase
                .validate_state_given(&command(OrderId::Oid(ORDER_ID + 1)), &valid_state,),
            Err(CancelSpotOrderFsmV3Error::OrderLookupMismatch)
        );

        let mut wrong_owner = command(OrderId::Oid(ORDER_ID));
        wrong_owner = LogEntrySequencer::from_metadata(
            LSN,
            LOGICAL_TS,
            1,
            2,
            "other".into(),
            wrong_owner.payload().clone(),
        );
        assert_eq!(
            CancelSpotOrderFsmV3UseCase.validate_state_given(&wrong_owner, &valid_state),
            Err(CancelSpotOrderFsmV3Error::OwnerMismatch)
        );
    }

    #[test]
    fn failed_release_returns_no_changes() {
        // Rule: 余额解冻失败时取消必须原子失败，不得返回 changes 或泄漏领域事件。
        // Given: 活跃订单的冻结余额不足。
        let cmd = command(OrderId::Oid(ORDER_ID));
        let given = state(SpotOrderStatus::Open, Decimal::ONE);

        // When: 执行取消。
        let result = CancelSpotOrderFsmV3UseCase.compute_state_changed(&cmd, given);

        // Then: 不返回 changes/event。
        assert_eq!(
            result,
            Err(UseCaseComputeError::Business(
                CancelSpotOrderFsmV3Error::InsufficientFrozenBalance
            ))
        );
    }
}
