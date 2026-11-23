use common_entity::{
    EntityError, LogEntry, ReplayableDomainEvent, ReplayableEntityChanges, UpdatedEntityEvents,
    UpdatedEntityEventsError, UseCaseV3,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{BalanceV2, OrderSide, SpotOrderStatus, SpotOrderV7};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::use_case::trading::spot::v3::domain_service::atom_spot_order_domain_service_v2::{
    CancelSpotOrderDomainServiceV2Error, CancelSpotOrderDomainServiceV2Metadata,
    SpotOrderDomainServiceV2,
};

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
    let changes = SpotOrderDomainServiceV2
        .cancel(
            &state.order,
            &state.balance,
            CancelSpotOrderDomainServiceV2Metadata {
                lsn: cmd.lsn(),
                logical_ts: cmd.logical_ts(),
                topic: cmd.topic(),
                partition: cmd.partition(),
            },
        )
        .map_err(map_domain_service_error)?;
    Ok(CancelSpotOrderFsmV3Changes {
        order: changes.order,
        balance: changes.balance,
    })
}

fn map_domain_service_error(
    error: CancelSpotOrderDomainServiceV2Error,
) -> CancelSpotOrderFsmV3Error {
    match error {
        CancelSpotOrderDomainServiceV2Error::AssetMismatch => {
            CancelSpotOrderFsmV3Error::CoinMismatch
        }
        CancelSpotOrderDomainServiceV2Error::InsufficientFrozenBalance => {
            CancelSpotOrderFsmV3Error::InsufficientFrozenBalance
        }
        CancelSpotOrderDomainServiceV2Error::BalanceClosed => {
            CancelSpotOrderFsmV3Error::BalanceClosed
        }
        CancelSpotOrderDomainServiceV2Error::InvalidBalanceAmount => {
            CancelSpotOrderFsmV3Error::InvalidBalanceAmount
        }
        CancelSpotOrderDomainServiceV2Error::ArithmeticOverflow => {
            CancelSpotOrderFsmV3Error::ArithmeticOverflow
        }
        CancelSpotOrderDomainServiceV2Error::UnexpectedOrderOutput => {
            CancelSpotOrderFsmV3Error::UnexpectedOrderOutput
        }
        CancelSpotOrderDomainServiceV2Error::InvalidEntityEvents(error) => {
            CancelSpotOrderFsmV3Error::InvalidEntityEvents(error)
        }
        CancelSpotOrderDomainServiceV2Error::Entity(error) => {
            CancelSpotOrderFsmV3Error::Entity(error)
        }
    }
}
