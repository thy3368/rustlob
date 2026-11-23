use common_entity::fsm_v2::{
    CommandMetadata as FsmCommandMetadata, Fsm as FsmV2, SequencedCommand,
};
use common_entity::{
    EntityError, Fsm, LogEntry, ReplayableChanges, ReplayableDomainEvent, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, OrderSide, SpotOrderStatus, SpotOrderV7,
    SpotOrderV7Command, SpotOrderV7Output,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderV2Cmd {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2State {
    pub order: SpotOrderV7,
    pub balance: BalanceV2,
}

/// FSM 产生的订单事件与两个聚合的 pair 共同构成取消用例的唯一事实源。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV7>,
    pub updated_balance: Option<UpdatedEntityPair<BalanceV2>>,
    pub order_event: ReplayableDomainEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV2Error {
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
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderV2UseCase;

impl ReplayableChanges for CancelSpotOrderV2Changes {
    fn to_replayable_events(&self) -> Result<Vec<ReplayableDomainEvent>, EntityError> {
        let mut events = Vec::with_capacity(1 + usize::from(self.updated_balance.is_some()));
        events.push(self.order_event.clone());
        if let Some(balance) = &self.updated_balance {
            events.push(balance.to_replayable_event()?);
        }
        Ok(events)
    }
}

impl UseCase for CancelSpotOrderV2UseCase {
    type Command = LogEntry<CancelSpotOrderV2Cmd>;
    type StateGiven = CancelSpotOrderV2State;
    type Error = CancelSpotOrderV2Error;
    type StateChanged = CancelSpotOrderV2Changes;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match &cmd.payload().order_id {
            OrderId::Oid(0) => Err(CancelSpotOrderV2Error::InvalidOid),
            OrderId::Cloid(cloid) if cloid.trim().is_empty() => {
                Err(CancelSpotOrderV2Error::EmptyCloid)
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
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        compute_changes(cmd, state)
    }
}

impl StateDiff for CancelSpotOrderV2UseCase {
    type StateDiff = CancelSpotOrderV2Changes;

    fn do_compute_state_diff(
        _state: Self::StateGiven,
        changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(changed)
    }
}

fn validate_state(
    cmd: &LogEntry<CancelSpotOrderV2Cmd>,
    state: &CancelSpotOrderV2State,
) -> Result<(), CancelSpotOrderV2Error> {
    let order = state.order.order_base();
    if order.owner.0 != common_entity::stable_string_entity_id(cmd.user_id()) {
        return Err(CancelSpotOrderV2Error::OwnerMismatch);
    }
    match &cmd.payload().order_id {
        OrderId::Oid(oid) if *oid != order.order_id => {
            return Err(CancelSpotOrderV2Error::OrderLookupMismatch);
        }
        OrderId::Cloid(cloid) => match order.cloid.as_deref() {
            None => return Err(CancelSpotOrderV2Error::OrderCloidMissing),
            Some(loaded) if loaded != cloid => {
                return Err(CancelSpotOrderV2Error::OrderLookupMismatch);
            }
            Some(_) => {}
        },
        OrderId::Oid(_) => {}
    }
    if !matches!(
        state.order.status(),
        SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
    ) {
        return Err(CancelSpotOrderV2Error::OrderNotCancelable);
    }
    if cmd.lsn() <= order.lsn {
        return Err(CancelSpotOrderV2Error::StaleOrder);
    }
    if state.balance.party_id() != order.owner.0 {
        return Err(CancelSpotOrderV2Error::PartyMismatch);
    }
    let held_coin = match order.side {
        OrderSide::Buy => order.symbol.quote,
        OrderSide::Sell => order.symbol.base,
    };
    if state.balance.coin_idx() != u64::from(held_coin.index()) {
        return Err(CancelSpotOrderV2Error::CoinMismatch);
    }
    Ok(())
}

fn compute_changes(
    cmd: &LogEntry<CancelSpotOrderV2Cmd>,
    state: &CancelSpotOrderV2State,
) -> Result<CancelSpotOrderV2Changes, CancelSpotOrderV2Error> {
    let order_before = state.order.clone();
    let mut order_after = order_before.clone();
    let transition = FsmV2::transition(
        &mut order_after,
        SequencedCommand::new(
            FsmCommandMetadata::new(cmd.lsn(), cmd.logical_ts()),
            SpotOrderV7Command::Cancel,
        ),
    )?;
    let SpotOrderV7Output::Canceled(output) = transition.output else {
        return Err(CancelSpotOrderV2Error::UnexpectedOrderOutput);
    };

    let updated_balance = if output.release_requirement.amount() == Decimal::ZERO {
        None
    } else {
        let balance_before = state.balance.clone();
        let mut balance_after = balance_before.clone();
        balance_after
            .apply(BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: output.release_requirement.amount(),
                payment_amount: Decimal::ZERO,
                timestamp: cmd.logical_ts(),
                lsn: cmd.lsn(),
            })
            .map_err(map_balance_error)?;
        Some(UpdatedEntityPair::new(balance_before, balance_after))
    };

    Ok(CancelSpotOrderV2Changes {
        updated_order: UpdatedEntityPair::new(order_before, order_after),
        updated_balance,
        order_event: transition.event,
    })
}

fn map_balance_error(error: BalanceV2Error) -> CancelSpotOrderV2Error {
    match error {
        BalanceV2Error::InsufficientFrozenBalance => {
            CancelSpotOrderV2Error::InsufficientFrozenBalance
        }
        BalanceV2Error::BalanceClosed => CancelSpotOrderV2Error::BalanceClosed,
        BalanceV2Error::InvalidAmount | BalanceV2Error::InsufficientAvailableBalance => {
            CancelSpotOrderV2Error::InvalidBalanceAmount
        }
        BalanceV2Error::Replay(error) => CancelSpotOrderV2Error::Entity(error),
        BalanceV2Error::ArithmeticOverflow
        | BalanceV2Error::CannotCloseNonZeroBalance
        | BalanceV2Error::StaleLsn { .. } => CancelSpotOrderV2Error::ArithmeticOverflow,
    }
}

#[cfg(test)]
mod tests {
    //! 业务矩阵：定位方式 OID/CLOID；状态 Pending/Open/PartiallyFilled；
    //! 非零释放产生 order+balance replay，零释放只产生 order replay；所有成功取消产生一个领域事件。

    use common_entity::{Entity, LogEntrySequencer, ReplayableChanges, StateDiff, UseCase};
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

    fn state(status: SpotOrderStatus, frozen: Decimal) -> CancelSpotOrderV2State {
        CancelSpotOrderV2State {
            order: order(status),
            balance: balance(frozen),
        }
    }

    fn command(order_id: OrderId) -> LogEntry<CancelSpotOrderV2Cmd> {
        LogEntrySequencer::from_metadata(
            LSN,
            LOGICAL_TS,
            1,
            2,
            OWNER.into(),
            CancelSpotOrderV2Cmd { order_id },
        )
    }

    #[test]
    fn open_order_located_by_cloid_cancels_and_releases_its_hold() {
        // Rule: 活跃订单取消必须同时推进订单与冻结余额，并先投影订单 replay。
        // Given: 一个通过 CLOID 定位、冻结 100 USDT 的 Open 买单。
        let cmd = command(OrderId::Cloid("client-order-11".into()));
        let given = state(SpotOrderStatus::Open, Decimal::from(100));

        // When: 执行取消并形成可回放 changes。
        let changed = CancelSpotOrderV2UseCase
            .compute_state_changed(&cmd, &given)
            .expect("open order should cancel");

        // Then: pair 是唯一状态事实，领域事件内嵌 replay 是订单事件权威来源。
        assert_eq!(changed.updated_order.before, given.order);
        assert_eq!(
            changed.updated_order.after.status(),
            SpotOrderStatus::Canceled
        );
        let balance_pair = changed
            .updated_balance
            .as_ref()
            .expect("non-zero release must update balance");
        assert_eq!(balance_pair.before.frozen(), Decimal::from(100));
        assert_eq!(balance_pair.after.frozen(), Decimal::ZERO);
        assert_eq!(balance_pair.after.available(), Decimal::from(100));

        let events = changed.to_replayable_events().expect("events project");
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], changed.order_event);
        assert_eq!(events[0].domain_event_type, 6);
        assert_eq!((events[0].lsn, events[0].logical_ts), (LSN, LOGICAL_TS));
        assert_eq!((events[0].previous_lsn, events[0].change_type), (0, 1));
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str() == Ok("status")
                && change.old_value_bytes() == b"Open"
                && change.new_value_bytes() == b"Canceled"
        }));
        assert_eq!(changed.order_event.lsn, LSN);
        assert_eq!(changed.order_event.logical_ts, LOGICAL_TS);
    }

    #[test]
    fn partially_filled_order_located_by_oid_releases_remaining_hold() {
        // Rule: OID 与 CLOID 只改变定位方式，不改变取消的状态与事件语义。
        // Given: 一个通过 OID 定位的 PartiallyFilled 买单。
        let cmd = command(OrderId::Oid(ORDER_ID));
        let given = state(SpotOrderStatus::PartiallyFilled, Decimal::from(100));

        // When: 执行取消。
        let changes = CancelSpotOrderV2UseCase
            .compute_state_diff(&cmd, given)
            .expect("partially filled order should cancel");

        // Then: 订单和余额均形成 pair，事件顺序固定为 order、balance。
        assert_eq!(
            changes.updated_order.after.status(),
            SpotOrderStatus::Canceled
        );
        assert!(changes.updated_balance.is_some());
        let events = changes.to_replayable_events().expect("events project");
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[0].entity_id,
            changes.updated_order.after.order_base().order_id
        );
        assert_eq!(
            events[1].entity_id,
            changes
                .updated_balance
                .as_ref()
                .expect("balance pair")
                .after
                .entity_id()
        );
    }

    #[test]
    fn pending_order_cancels_without_touching_balance() {
        // Rule: Pending 订单未占用冻结额，取消不得制造余额变化。
        // Given: 一个 Pending 买单与不相关的零冻结余额。
        let cmd = command(OrderId::Oid(ORDER_ID));
        let given = state(SpotOrderStatus::Pending, Decimal::ZERO);

        // When: 执行取消。
        let changes = CancelSpotOrderV2UseCase
            .compute_state_changed(&cmd, &given)
            .expect("pending order should cancel");

        // Then: 仅订单 pair/replay 变化，但仍保留 canceled 事件码。
        assert_eq!(
            changes.updated_order.after.status(),
            SpotOrderStatus::Canceled
        );
        assert_eq!(changes.updated_balance, None);
        assert_eq!(
            changes
                .to_replayable_events()
                .expect("events project")
                .len(),
            1
        );
        assert_eq!(changes.order_event.domain_event_type, 6);
    }

    #[test]
    fn invalid_lookup_and_authoritative_state_are_rejected() {
        let valid_state = state(SpotOrderStatus::Open, Decimal::from(100));
        assert_eq!(
            CancelSpotOrderV2UseCase.check_command(&command(OrderId::Oid(0))),
            Err(CancelSpotOrderV2Error::InvalidOid)
        );
        assert_eq!(
            CancelSpotOrderV2UseCase.check_command(&command(OrderId::Cloid(" ".into()))),
            Err(CancelSpotOrderV2Error::EmptyCloid)
        );
        assert_eq!(
            CancelSpotOrderV2UseCase
                .validate_state_given(&command(OrderId::Oid(ORDER_ID + 1)), &valid_state,),
            Err(CancelSpotOrderV2Error::OrderLookupMismatch)
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
            CancelSpotOrderV2UseCase.validate_state_given(&wrong_owner, &valid_state),
            Err(CancelSpotOrderV2Error::OwnerMismatch)
        );
    }

    #[test]
    fn failed_release_returns_no_changes_and_preserves_given_state() {
        // Rule: 余额解冻失败时取消必须原子失败，不得修改输入 state 或泄漏领域事件。
        // Given: 活跃订单的冻结余额不足。
        let cmd = command(OrderId::Oid(ORDER_ID));
        let given = state(SpotOrderStatus::Open, Decimal::ONE);
        let before = given.clone();

        // When: 执行取消。
        let result = CancelSpotOrderV2UseCase.compute_state_changed(&cmd, &given);

        // Then: 不返回 changes/event，且 authoritative input 保持不变。
        assert_eq!(
            result,
            Err(CancelSpotOrderV2Error::InsufficientFrozenBalance)
        );
        assert_eq!(given, before);
    }
}
