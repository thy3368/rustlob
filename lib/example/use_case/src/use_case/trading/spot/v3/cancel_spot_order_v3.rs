use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateMachineV2,
    StateMachineV2Diff, UpdatedEntityPair,
};
use example_core_entity::spot::spot_order_v3::SpotOrderV3Any;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CancelSpotOrderV3Cmd {
    pub cloid: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3State {
    pub order: SpotOrderV3Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3AfterChanges {
    pub canceled_order: SpotOrderV3Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV3Any>,
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
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let canceled_order = match &given_state.order {
            SpotOrderV3Any::Pending(order) => {
                SpotOrderV3Any::Canceled(order.cancel(cmd.logical_ts())?)
            }
            SpotOrderV3Any::Open(order) => {
                SpotOrderV3Any::Canceled(order.cancel(cmd.logical_ts())?)
            }
            SpotOrderV3Any::PartiallyFilled(order) => {
                SpotOrderV3Any::Canceled(order.cancel(cmd.logical_ts())?)
            }
            _ => return Err(CancelSpotOrderV3Error::OrderNotCancelable),
        };
        Ok(CancelSpotOrderV3AfterChanges { canceled_order })
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
        })
    }
}

impl ReplayableChanges for CancelSpotOrderV3Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![self.updated_order.to_replayable_event()?])
    }
}
