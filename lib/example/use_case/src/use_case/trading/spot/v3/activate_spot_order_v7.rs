use common_entity::{
    EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::spot::spot_order_v7::{
    ActivateSpotOrderV7Output, SpotOrderV7, SpotOrderV7Event, SpotOrderV7FreezeRequirement,
    SpotOrderV7Output,
};
use example_core_entity::{BalanceV2, BalanceV2Error, BalanceV2Event, SpotOrderStatus};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivateSpotOrderV7Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV7State {
    pub order: SpotOrderV7,
    pub balance: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV7AfterChanges {
    pub order_after: SpotOrderV7,
    pub balance_after: BalanceV2,
    pub freeze_requirement: SpotOrderV7FreezeRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV7Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV7>,
    pub updated_balance: UpdatedEntityPair<BalanceV2>,
    pub freeze_requirement: SpotOrderV7FreezeRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ActivateSpotOrderV7Error {
    #[error("order id must be greater than zero")]
    InvalidOrderId,
    #[error("command order id does not match loaded order")]
    OrderMismatch,
    #[error("only pending orders can be activated")]
    OrderNotPending,
    #[error("balance owner does not match order owner")]
    OwnerMismatch,
    #[error("balance coin does not match freeze requirement")]
    CoinMismatch,
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActivateSpotOrderV7UseCase;

impl UseCase for ActivateSpotOrderV7UseCase {
    type Command = LogEntry<ActivateSpotOrderV7Cmd>;
    type StateGiven = ActivateSpotOrderV7State;
    type Error = ActivateSpotOrderV7Error;
    type StateChanged = ActivateSpotOrderV7AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.payload().order_id == 0 {
            return Err(ActivateSpotOrderV7Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.order.order_base().order_id != cmd.payload().order_id {
            return Err(ActivateSpotOrderV7Error::OrderMismatch);
        }
        if state.order.status() != SpotOrderStatus::Pending {
            return Err(ActivateSpotOrderV7Error::OrderNotPending);
        }
        if state.balance.party_id() != state.order.order_base().owner.0 {
            return Err(ActivateSpotOrderV7Error::OwnerMismatch);
        }
        let requirement = state.order.freeze_requirement().map_err(|error| {
            ActivateSpotOrderV7Error::Entity(EntityError::Custom(error.to_string()))
        })?;
        if state.balance.coin_idx() != u64::from(freeze_amount(requirement).0.index()) {
            return Err(ActivateSpotOrderV7Error::CoinMismatch);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut order_after = state.order.clone();
        let (_, output) =
            order_after.transition(SpotOrderV7Event::Activate { timestamp: cmd.logical_ts() })?;
        let SpotOrderV7Output::Activated(ActivateSpotOrderV7Output { freeze_requirement }) = output
        else {
            return Err(
                EntityError::Custom("activate event produced mismatched output".into()).into()
            );
        };
        let (coin, amount) = freeze_amount(freeze_requirement);
        if state.balance.coin_idx() != u64::from(coin.index()) {
            return Err(ActivateSpotOrderV7Error::CoinMismatch);
        }
        let mut balance_after = state.balance.clone();
        balance_after.apply(BalanceV2Event::Freeze { amount, timestamp: cmd.logical_ts() })?;
        Ok(ActivateSpotOrderV7AfterChanges { order_after, balance_after, freeze_requirement })
    }
}

fn freeze_amount(
    requirement: SpotOrderV7FreezeRequirement,
) -> (example_core_entity::account::coin::Coin, Decimal) {
    let amount = match requirement {
        SpotOrderV7FreezeRequirement::Buy { total_quote, .. } => total_quote,
        SpotOrderV7FreezeRequirement::Sell { base_quantity } => base_quantity,
    };
    (amount.currency(), amount.amount())
}

impl StateDiff for ActivateSpotOrderV7UseCase {
    type StateDiff = ActivateSpotOrderV7Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(ActivateSpotOrderV7Changes {
            updated_order: UpdatedEntityPair::new(state.order, after.order_after),
            updated_balance: UpdatedEntityPair::new(state.balance, after.balance_after),
            freeze_requirement: after.freeze_requirement,
        })
    }
}

impl ReplayableChanges for ActivateSpotOrderV7Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![
            self.updated_order.to_replayable_event()?,
            self.updated_balance.to_replayable_event()?,
        ])
    }
}
