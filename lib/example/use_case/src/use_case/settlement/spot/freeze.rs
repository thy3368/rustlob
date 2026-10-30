use common_entity::{LogEntry, StateMachineV2};
use example_core_entity::spot::spot_order_v3::SpotOrderV3Any;
use example_core_entity::{BalanceV2, BalanceV2Error, Open, SpotAmountConversionError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FreezeSpotOrderV2Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeSpotOrderV2State {
    pub order: SpotOrderV3Any,
    pub balance: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeSpotOrderV2AfterChanges {
    pub balance_after: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FreezeSpotOrderV2Error {
    #[error("order id must be greater than zero")]
    InvalidOrderId,
    #[error("command order id does not match the loaded order")]
    OrderIdMismatch,
    #[error("only pending orders can be frozen")]
    OrderNotPending,
    #[error("balance party does not match order owner")]
    PartyMismatch,
    #[error("balance asset does not match the order freeze asset")]
    AssetMismatch,
    #[error(transparent)]
    FreezeRequirement(#[from] SpotAmountConversionError),
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Freeze4SpotOrderV2UseCase;

impl StateMachineV2 for Freeze4SpotOrderV2UseCase {
    type Command = LogEntry<FreezeSpotOrderV2Cmd>;
    type StateGiven = FreezeSpotOrderV2State;
    type Error = FreezeSpotOrderV2Error;
    type StateChanged = FreezeSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.order_id == 0 {
            return Err(FreezeSpotOrderV2Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let order = given_state.order.order_base();
        if cmd.order_id != order.order_id {
            return Err(FreezeSpotOrderV2Error::OrderIdMismatch);
        }
        if !matches!(given_state.order, SpotOrderV3Any::Pending(_)) {
            return Err(FreezeSpotOrderV2Error::OrderNotPending);
        }
        if given_state.balance.party_id() != order.owner.0 {
            return Err(FreezeSpotOrderV2Error::PartyMismatch);
        }
        let SpotOrderV3Any::Pending(pending_order) = &given_state.order else {
            return Err(FreezeSpotOrderV2Error::OrderNotPending);
        };
        let requirement = pending_order.freeze_requirement()?;
        if given_state.balance.asset_id() != u64::from(requirement.currency().index()) {
            return Err(FreezeSpotOrderV2Error::AssetMismatch);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        _cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let SpotOrderV3Any::Pending(pending_order) = &given_state.order else {
            return Err(FreezeSpotOrderV2Error::OrderNotPending);
        };
        let requirement = pending_order.freeze_requirement()?;
        let mut balance_after = given_state.balance.clone();
        balance_after.freeze(requirement.amount())?;
        Ok(FreezeSpotOrderV2AfterChanges { balance_after })
    }
}
