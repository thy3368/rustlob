use common_entity::{LogEntry, StateMachineV2};
use example_core_entity::spot::spot_order_v3::SpotOrderV3Any;
use example_core_entity::{BalanceV2, BalanceV2Error, Open, SpotAmountConversionError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UnFreezeSpotOrderV2Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnFreezeSpotOrderV2State {
    pub order: SpotOrderV3Any,
    pub balance: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnFreezeSpotOrderV2AfterChanges {
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
    type Command = LogEntry<UnFreezeSpotOrderV2Cmd>;
    type StateGiven = UnFreezeSpotOrderV2State;
    type Error = FreezeSpotOrderV2Error;
    type StateChanged = UnFreezeSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        _cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        ///todo 处理 cancel/reject等情况
        let SpotOrderV3Any::Pending(pending_order) = &given_state.order else {
            return Err(FreezeSpotOrderV2Error::OrderNotPending);
        };
        let requirement = pending_order.freeze_requirement()?;
        let mut balance_after = given_state.balance.clone();
        balance_after.freeze(requirement.amount())?;
        Ok(UnFreezeSpotOrderV2AfterChanges { balance_after })
    }
}
