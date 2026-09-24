use common_entity::{
    EntityError, EntityReplayableEvent, ExecutionContext, ReplayableChanges,
    StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::{Balance, SpotOrderV2};
use crate::{
    ActivateSpotOrderV2AfterChanges, ActivateSpotOrderV2Changes, ActivateSpotOrderV2Cmd,
    ActivateSpotOrderV2Error, ActivateSpotOrderV2State, ActivateSpotOrderV2UseCase,
    MatchSpotOrderV3AfterChanges, MatchSpotOrderV3Changes, MatchSpotOrderV3Cmd,
    MatchSpotOrderV3Error, MatchSpotOrderV3State, MatchSpotOrderV3UseCase,
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ActivateMatchSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub order_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateMatchSpotOrderV2State {
    pub pending_order: SpotOrderV2,
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub fee_account_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateMatchSpotOrderV2AfterChanges {
    pub activation_after: ActivateSpotOrderV2AfterChanges,
    pub match_after: MatchSpotOrderV3AfterChanges,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateMatchSpotOrderV2Changes {
    pub activation_changes: ActivateSpotOrderV2Changes,
    pub match_changes: MatchSpotOrderV3Changes,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ActivateMatchSpotOrderV2Error {
    #[error(transparent)]
    Activation(#[from] ActivateSpotOrderV2Error),
    #[error(transparent)]
    Match(#[from] MatchSpotOrderV3Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActivateMatchSpotOrderV2UseCase;

impl ReplayableChanges for ActivateMatchSpotOrderV2Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = self.activation_changes.to_replayable_events()?;
        events.extend(self.match_changes.to_replayable_events()?);
        Ok(events)
    }
}

impl StateMachineV2Unchecked for ActivateMatchSpotOrderV2UseCase {
    type Command = ActivateMatchSpotOrderV2Cmd;
    type StateGiven = ActivateMatchSpotOrderV2State;
    type Error = ActivateMatchSpotOrderV2Error;
    type StateChanged = ActivateMatchSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        ActivateSpotOrderV2UseCase.check_command(&activation_cmd(cmd))?;
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let activation_state = activation_state_from_given(state);
        ActivateSpotOrderV2UseCase.validate_state_given(&activation_cmd(cmd), &activation_state)?;
        if state.fee_account_id.is_empty() {
            return Err(MatchSpotOrderV3Error::InvalidFeeAccountId.into());
        }
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let activation_cmd = activation_cmd(cmd);
        let activation_state = activation_state_from_given(state);
        let activation_after = ActivateSpotOrderV2UseCase.compute_state_changed_with_context(
            &activation_cmd,
            &activation_state,
            context,
        )?;

        let match_cmd = match_cmd(cmd);
        let match_state = match_state_from_activation(state, &activation_after);
        let match_after = MatchSpotOrderV3UseCase.compute_state_changed_with_context(
            &match_cmd,
            &match_state,
            context,
        )?;

        Ok(ActivateMatchSpotOrderV2AfterChanges { activation_after, match_after })
    }
}

impl StateMachineOwnedV2Diff for ActivateMatchSpotOrderV2UseCase {
    type StateDiff = ActivateMatchSpotOrderV2Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        let activation_state = ActivateSpotOrderV2State {
            pending_order: state.pending_order,
            balances: state.settlement_balances,
            base_asset_id: state.base_asset_id.clone(),
            quote_asset_id: state.quote_asset_id.clone(),
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
        };
        let match_state = MatchSpotOrderV3State {
            taker_order: after.activation_after.activated_order_after.clone(),
            maker_orders: state.maker_orders,
            settlement_balances: after.activation_after.balances_after.clone(),
            base_asset_id: state.base_asset_id,
            quote_asset_id: state.quote_asset_id,
            fee_account_id: state.fee_account_id,
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
        };

        let activation_changes = ActivateSpotOrderV2UseCase::do_compute_state_diff(
            activation_state,
            after.activation_after,
        )?;
        let match_changes =
            MatchSpotOrderV3UseCase::do_compute_state_diff(match_state, after.match_after)?;

        Ok(ActivateMatchSpotOrderV2Changes { activation_changes, match_changes })
    }
}

fn activation_cmd(cmd: &ActivateMatchSpotOrderV2Cmd) -> ActivateSpotOrderV2Cmd {
    ActivateSpotOrderV2Cmd {
        party_id: cmd.party_id.clone(),
        asset: cmd.asset,
        order_id: cmd.order_id.clone(),
    }
}

fn match_cmd(cmd: &ActivateMatchSpotOrderV2Cmd) -> MatchSpotOrderV3Cmd {
    MatchSpotOrderV3Cmd {
        party_id: cmd.party_id.clone(),
        asset: cmd.asset,
        order_id: cmd.order_id.clone(),
    }
}

fn activation_state_from_given(state: &ActivateMatchSpotOrderV2State) -> ActivateSpotOrderV2State {
    ActivateSpotOrderV2State {
        pending_order: state.pending_order.clone(),
        balances: state.settlement_balances.clone(),
        base_asset_id: state.base_asset_id.clone(),
        quote_asset_id: state.quote_asset_id.clone(),
        maker_fee_bps: state.maker_fee_bps,
        taker_fee_bps: state.taker_fee_bps,
    }
}

fn match_state_from_activation(
    state: &ActivateMatchSpotOrderV2State,
    activation_after: &ActivateSpotOrderV2AfterChanges,
) -> MatchSpotOrderV3State {
    MatchSpotOrderV3State {
        taker_order: activation_after.activated_order_after.clone(),
        maker_orders: state.maker_orders.clone(),
        settlement_balances: activation_after.balances_after.clone(),
        base_asset_id: state.base_asset_id.clone(),
        quote_asset_id: state.quote_asset_id.clone(),
        fee_account_id: state.fee_account_id.clone(),
        maker_fee_bps: state.maker_fee_bps,
        taker_fee_bps: state.taker_fee_bps,
    }
}
