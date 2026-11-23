use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff, UpdatedEntityPair,
    UseCase,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{BalanceV2, BalanceV2Error, OrderSide, SpotOrderStatus, SpotOrderV7};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::use_case::trading::spot::v3::domain_service::atom_spot_order_domain_service::{
    CancelSpotOrderV7Error, SpotOrderDomainService,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderV3Cmd {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3State {
    pub order: SpotOrderV7,
    pub balance: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3AfterChanges {
    pub order_after: SpotOrderV7,
    pub balance_after: Option<BalanceV2>,
}

/// 订单 pair 是必有的取消事实；余额仅在实际释放冻结额时形成 update pair。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV7>,
    pub updated_balance: Option<UpdatedEntityPair<BalanceV2>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV3Error {
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
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderV3UseCase;

impl ReplayableChanges for CancelSpotOrderV3Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(1 + usize::from(self.updated_balance.is_some()));
        events.push(self.updated_order.to_replayable_event()?);
        if let Some(balance) = &self.updated_balance {
            events.push(balance.to_replayable_event()?);
        }
        Ok(events)
    }
}

impl UseCase for CancelSpotOrderV3UseCase {
    type Command = LogEntry<CancelSpotOrderV3Cmd>;
    type StateGiven = CancelSpotOrderV3State;
    type Error = CancelSpotOrderV3Error;
    type StateChanged = CancelSpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        check_cancel_command(cmd)
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        validate_cancel_state(cmd, state)
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        compute_cancel_after(cmd, state)
    }
}

pub(crate) fn check_cancel_command(
    cmd: &LogEntry<CancelSpotOrderV3Cmd>,
) -> Result<(), CancelSpotOrderV3Error> {
    match &cmd.payload().order_id {
        OrderId::Oid(0) => Err(CancelSpotOrderV3Error::InvalidOid),
        OrderId::Cloid(cloid) if cloid.trim().is_empty() => Err(CancelSpotOrderV3Error::EmptyCloid),
        OrderId::Oid(_) | OrderId::Cloid(_) => Ok(()),
    }
}

pub(crate) fn validate_cancel_state(
    cmd: &LogEntry<CancelSpotOrderV3Cmd>,
    state: &CancelSpotOrderV3State,
) -> Result<(), CancelSpotOrderV3Error> {
    let order = state.order.order_base();
    if order.owner.0 != common_entity::stable_string_entity_id(cmd.user_id()) {
        return Err(CancelSpotOrderV3Error::OwnerMismatch);
    }
    match &cmd.payload().order_id {
        OrderId::Oid(oid) if *oid != order.order_id => {
            return Err(CancelSpotOrderV3Error::OrderLookupMismatch);
        }
        OrderId::Cloid(cloid) => match order.cloid.as_deref() {
            None => return Err(CancelSpotOrderV3Error::OrderCloidMissing),
            Some(loaded) if loaded != cloid => {
                return Err(CancelSpotOrderV3Error::OrderLookupMismatch);
            }
            Some(_) => {}
        },
        OrderId::Oid(_) => {}
    }
    if !matches!(
        state.order.status(),
        SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
    ) {
        return Err(CancelSpotOrderV3Error::OrderNotCancelable);
    }
    if cmd.lsn() <= order.lsn {
        return Err(CancelSpotOrderV3Error::StaleOrder);
    }
    if state.balance.party_id() != order.owner.0 {
        return Err(CancelSpotOrderV3Error::PartyMismatch);
    }
    let held_coin = match order.side {
        OrderSide::Buy => order.symbol.quote,
        OrderSide::Sell => order.symbol.base,
    };
    if state.balance.coin_idx() != u64::from(held_coin.index()) {
        return Err(CancelSpotOrderV3Error::CoinMismatch);
    }
    Ok(())
}

pub(crate) fn compute_cancel_after(
    cmd: &LogEntry<CancelSpotOrderV3Cmd>,
    state: &CancelSpotOrderV3State,
) -> Result<CancelSpotOrderV3AfterChanges, CancelSpotOrderV3Error> {
    let (order_after, balance_after) = SpotOrderDomainService::cancel(
        state.order.clone(),
        state.balance.clone(),
        cmd.lsn(),
        cmd.logical_ts(),
    )
    .map_err(map_cancel_error)?;
    let balance_after = (balance_after != state.balance).then_some(balance_after);
    Ok(CancelSpotOrderV3AfterChanges {
        order_after,
        balance_after,
    })
}

impl StateDiff for CancelSpotOrderV3UseCase {
    type StateDiff = CancelSpotOrderV3Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(build_cancel_changes(state, changed))
    }
}

pub(crate) fn build_cancel_changes(
    state: CancelSpotOrderV3State,
    changed: CancelSpotOrderV3AfterChanges,
) -> CancelSpotOrderV3Changes {
    CancelSpotOrderV3Changes {
        updated_order: UpdatedEntityPair::new(state.order, changed.order_after),
        updated_balance: changed
            .balance_after
            .map(|after| UpdatedEntityPair::new(state.balance, after)),
    }
}

pub(crate) fn compute_cancel_changes(
    cmd: &LogEntry<CancelSpotOrderV3Cmd>,
    state: CancelSpotOrderV3State,
) -> Result<CancelSpotOrderV3Changes, CancelSpotOrderV3Error> {
    check_cancel_command(cmd)?;
    validate_cancel_state(cmd, &state)?;
    let changed = compute_cancel_after(cmd, &state)?;
    Ok(build_cancel_changes(state, changed))
}

fn map_cancel_error(error: CancelSpotOrderV7Error) -> CancelSpotOrderV3Error {
    match error {
        CancelSpotOrderV7Error::AssetMismatch => CancelSpotOrderV3Error::CoinMismatch,
        CancelSpotOrderV7Error::Balance(error) => match error {
            BalanceV2Error::InsufficientFrozenBalance => {
                CancelSpotOrderV3Error::InsufficientFrozenBalance
            }
            BalanceV2Error::BalanceClosed => CancelSpotOrderV3Error::BalanceClosed,
            BalanceV2Error::InvalidAmount | BalanceV2Error::InsufficientAvailableBalance => {
                CancelSpotOrderV3Error::InvalidBalanceAmount
            }
            BalanceV2Error::Replay(_)
            | BalanceV2Error::ArithmeticOverflow
            | BalanceV2Error::CannotCloseNonZeroBalance
            | BalanceV2Error::StaleLsn { .. } => CancelSpotOrderV3Error::ArithmeticOverflow,
        },
        CancelSpotOrderV7Error::Entity(error) => CancelSpotOrderV3Error::Entity(error),
    }
}
