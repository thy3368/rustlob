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
        state: &Self::StateGiven,
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
    state: &CancelSpotOrderFsmV3State,
) -> Result<CancelSpotOrderFsmV3Changes, CancelSpotOrderFsmV3Error> {
    let order_before = &state.order;
    let balance_before = &state.balance;
    let order_loaded_position = order_before.event_position();
    let metadata = FsmCommandMetadata::new(
        cmd.lsn(),
        cmd.logical_ts(),
        order_before.topic(),
        order_before.partition(),
        order_before.party_id(),
    );
    let (order_after, order_event, order_output) = FsmV2::transition(
        order_before,
        SequencedCommand::new(metadata, SpotOrderV7Command::Cancel),
    )?;
    let SpotOrderV7Output::Canceled(output) = order_output else {
        return Err(CancelSpotOrderFsmV3Error::UnexpectedOrderOutput);
    };

    let balance = if output.release_requirement.amount() == Decimal::ZERO {
        None
    } else {
        let balance_loaded_position = balance_before.event_position();
        let metadata = FsmCommandMetadata::new(
            cmd.lsn(),
            cmd.logical_ts(),
            balance_before.topic(),
            balance_before.partition(),
            balance_before.party_id().to_string(),
        );
        let (balance_after, balance_event, ()) = FsmV2::transition(
            balance_before,
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
                vec![balance_event],
            )
            .map_err(CancelSpotOrderFsmV3Error::InvalidEntityEvents)?,
        )
    };

    Ok(CancelSpotOrderFsmV3Changes {
        order: UpdatedEntityEvents::try_new(order_after, order_loaded_position, vec![order_event])
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

