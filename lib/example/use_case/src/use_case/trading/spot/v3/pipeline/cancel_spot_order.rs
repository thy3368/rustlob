use common_entity::{EntityError, LogEntry, UseCaseV3};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{BalanceV2, OrderSide, SpotOrderStatus, SpotOrderV7};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::use_case::trading::spot::v3::domain_service::cancel_spot_order::{
    CancelSpotOrderPipeChanges, cancel_spot_order,
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
pub type CancelSpotOrderFsmV3Changes = CancelSpotOrderPipeChanges;

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
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderFsmV3UseCase;

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
    cancel_spot_order(
        &state.order,
        &state.balance,
        cmd.lsn(),
        cmd.logical_ts(),
        cmd.topic(),
        cmd.partition(),
    )
    .map_err(CancelSpotOrderFsmV3Error::from)
}
