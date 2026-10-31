use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateMachineV2,
    StateMachineV2Diff, UpdatedEntityPair,
};
use example_core_entity::spot::spot_order_v3::SpotOrderV3Any;
use example_core_entity::{BalanceV2, BalanceV2Error, Open, SpotAmountConversionError};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CancelSpotOrderV3Cmd {
    pub cloid: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3State {
    pub order: SpotOrderV3Any,
    pub balance: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3AfterChanges {
    pub canceled_order: SpotOrderV3Any,
    pub balance: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV3Any>,
    pub updated_balance: UpdatedEntityPair<example_core_entity::BalanceV2Record>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV3Error {
    #[error("client order id must not be empty")]
    EmptyCloid,
    #[error("command client order id does not match loaded order")]
    CloidMismatch,
    #[error("loaded order has no client order id")]
    MissingCloid,
    #[error("order is not cancelable")]
    OrderNotCancelable,
    #[error("balance party does not match order owner")]
    PartyMismatch,
    #[error("balance asset does not match the order release asset")]
    AssetMismatch,
    #[error(transparent)]
    ReleaseRequirement(#[from] SpotAmountConversionError),
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderV3UseCase;

impl StateMachineV2 for CancelSpotOrderV3UseCase {
    type Command = LogEntry<CancelSpotOrderV3Cmd>;
    type StateGiven = CancelSpotOrderV3State;
    type Error = CancelSpotOrderV3Error;
    type StateChanged = CancelSpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.payload().cloid.trim().is_empty() {
            return Err(CancelSpotOrderV3Error::EmptyCloid);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let Some(cloid) = given_state.order.order_base().cloid.as_deref() else {
            return Err(CancelSpotOrderV3Error::MissingCloid);
        };
        if cloid != cmd.payload().cloid {
            return Err(CancelSpotOrderV3Error::CloidMismatch);
        }
        if !matches!(
            given_state.order,
            SpotOrderV3Any::Pending(_)
                | SpotOrderV3Any::Open(_)
                | SpotOrderV3Any::PartiallyFilled(_)
        ) {
            return Err(CancelSpotOrderV3Error::OrderNotCancelable);
        }
        if given_state.balance.party_id() != given_state.order.order_base().owner.0 {
            return Err(CancelSpotOrderV3Error::PartyMismatch);
        }
        let requirement = match &given_state.order {
            SpotOrderV3Any::Pending(order) => {
                order.cancel(cmd.logical_ts())?.release_requirement()?
            }
            SpotOrderV3Any::Open(order) => order.cancel(cmd.logical_ts())?.release_requirement()?,
            SpotOrderV3Any::PartiallyFilled(order) => {
                order.cancel(cmd.logical_ts())?.release_requirement()?
            }
            _ => return Err(CancelSpotOrderV3Error::OrderNotCancelable),
        };
        if given_state.balance.coin_idx() != u64::from(requirement.currency().index()) {
            return Err(CancelSpotOrderV3Error::AssetMismatch);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let canceled_order = match &given_state.order {
            SpotOrderV3Any::Pending(order) => order.cancel(cmd.logical_ts())?,
            SpotOrderV3Any::Open(order) => order.cancel(cmd.logical_ts())?,
            SpotOrderV3Any::PartiallyFilled(order) => order.cancel(cmd.logical_ts())?,
            _ => return Err(CancelSpotOrderV3Error::OrderNotCancelable),
        };
        let requirement = canceled_order.release_requirement()?;
        if given_state.balance.coin_idx() != u64::from(requirement.currency().index()) {
            return Err(CancelSpotOrderV3Error::AssetMismatch);
        }
        let mut balance = given_state.balance.clone();
        balance.unfreeze_and_pay(requirement.amount(), Decimal::ZERO)?;
        Ok(CancelSpotOrderV3AfterChanges {
            canceled_order: SpotOrderV3Any::Canceled(canceled_order),
            balance,
        })
    }
}

impl StateMachineV2Diff for CancelSpotOrderV3UseCase {
    type StateDiff = CancelSpotOrderV3Changes;

    fn do_compute_state_diff(
        given_state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(CancelSpotOrderV3Changes {
            updated_order: UpdatedEntityPair::new(given_state.order, after.canceled_order),
            updated_balance: UpdatedEntityPair::new(
                given_state.balance.into_record(),
                after.balance.into_record(),
            ),
        })
    }
}

impl ReplayableChanges for CancelSpotOrderV3Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![
            self.updated_order.to_replayable_event()?,
            self.updated_balance.to_replayable_event()?,
        ])
    }
}
