use common_entity::{
    Entity, EntityError, EntityReplayableEvent, ExecutionContext, ReplayableChanges,
    StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use thiserror::Error;

use crate::entity::{Balance, SpotOrderV2};
use crate::{
    ActivateSpotOrderV2AfterChanges, ActivateSpotOrderV2Changes, ActivateSpotOrderV2Cmd,
    ActivateSpotOrderV2Error, ActivateSpotOrderV2State, ActivateSpotOrderV2UseCase,
    MatchSpotOrderV2AfterChanges, MatchSpotOrderV2Changes, MatchSpotOrderV2Cmd,
    MatchSpotOrderV2Error, MatchSpotOrderV2State, MatchSpotOrderV2UseCase,
    PlaceOnlySpotOrderV2AfterChanges, PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2Error,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType, PlaceOnlySpotOrderV2UseCase,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceMatchSpotOrderV2State {
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<Balance>,
    pub fee_account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceMatchSpotOrderV2AfterChanges {
    SinglePlacedOnly {
        created_order: SpotOrderV2,
    },
    SinglePlacedAndMatched {
        created_taker_order: SpotOrderV2,
        activation_after: ActivateSpotOrderV2AfterChanges,
        match_after: MatchSpotOrderV2AfterChanges,
    },
    NormalTpslPlacedAndMatched {
        created_parent_order: SpotOrderV2,
        created_child_orders: Vec<SpotOrderV2>,
        activation_after: ActivateSpotOrderV2AfterChanges,
        match_after: MatchSpotOrderV2AfterChanges,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceMatchSpotOrderV2Changes {
    SinglePlacedOnly {
        created_order: SpotOrderV2,
    },
    SinglePlacedAndMatched {
        created_taker_order: SpotOrderV2,
        activation_changes: ActivateSpotOrderV2Changes,
        match_changes: MatchSpotOrderV2Changes,
    },
    NormalTpslPlacedAndMatched {
        created_parent_order: SpotOrderV2,
        created_child_orders: Vec<SpotOrderV2>,
        activation_changes: ActivateSpotOrderV2Changes,
        match_changes: MatchSpotOrderV2Changes,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceMatchSpotOrderV2Error {
    #[error(transparent)]
    PlaceOnly(#[from] PlaceOnlySpotOrderV2Error),
    #[error(transparent)]
    Activation(#[from] ActivateSpotOrderV2Error),
    #[error(transparent)]
    Match(#[from] MatchSpotOrderV2Error),
    #[error("place-match only supports a single active limit order")]
    InvalidPlaceBranch,
    #[error("fee account id must not be empty")]
    InvalidFeeAccountId,
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceMatchSpotOrderV2UseCase;

impl ReplayableChanges for PlaceMatchSpotOrderV2Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        match self {
            Self::SinglePlacedOnly { created_order } => {
                Ok(vec![created_order.track_create_event()?])
            }
            Self::SinglePlacedAndMatched {
                created_taker_order,
                activation_changes,
                match_changes,
            } => {
                let mut events = vec![created_taker_order.track_create_event()?];
                events.extend(activation_changes.to_replayable_events()?);
                events.extend(match_changes.to_replayable_events()?);
                Ok(events)
            }
            Self::NormalTpslPlacedAndMatched {
                created_parent_order,
                created_child_orders,
                activation_changes,
                match_changes,
            } => {
                let mut events = vec![created_parent_order.track_create_event()?];
                for child in created_child_orders {
                    events.push(child.track_create_event()?);
                }
                events.extend(activation_changes.to_replayable_events()?);
                events.extend(match_changes.to_replayable_events()?);
                Ok(events)
            }
        }
    }
}

impl StateMachineV2Unchecked for PlaceMatchSpotOrderV2UseCase {
    type Command = PlaceOnlySpotOrderV2Cmd;
    type StateGiven = PlaceMatchSpotOrderV2State;
    type Error = PlaceMatchSpotOrderV2Error;
    type StateChanged = PlaceMatchSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        PlaceOnlySpotOrderV2UseCase.check_command(cmd)?;
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        PlaceOnlySpotOrderV2UseCase.validate_state_given(cmd, &())?;
        if state.fee_account_id.is_empty() {
            return Err(PlaceMatchSpotOrderV2Error::InvalidFeeAccountId);
        }
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let place_after =
            PlaceOnlySpotOrderV2UseCase.compute_state_changed_with_context(cmd, &(), context)?;
        match (cmd, place_after) {
            (
                PlaceOnlySpotOrderV2Cmd::Single(order_cmd),
                PlaceOnlySpotOrderV2AfterChanges::Single { created_order },
            ) => {
                if !matches!(order_cmd.order_type, PlaceOnlySpotOrderV2OrderType::Limit { .. }) {
                    return Ok(PlaceMatchSpotOrderV2AfterChanges::SinglePlacedOnly {
                        created_order,
                    });
                }
                let (activation_after, match_after) =
                    compute_activate_and_match_after_for_created_taker(
                        order_cmd,
                        created_order.clone(),
                        state,
                        context,
                    )?;
                Ok(PlaceMatchSpotOrderV2AfterChanges::SinglePlacedAndMatched {
                    created_taker_order: created_order,
                    activation_after,
                    match_after,
                })
            }
            (
                PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, .. },
                PlaceOnlySpotOrderV2AfterChanges::NormalTpsl {
                    created_parent_order,
                    created_child_orders,
                },
            ) => {
                let (activation_after, match_after) =
                    compute_activate_and_match_after_for_created_taker(
                        parent,
                        created_parent_order.clone(),
                        state,
                        context,
                    )?;
                Ok(PlaceMatchSpotOrderV2AfterChanges::NormalTpslPlacedAndMatched {
                    created_parent_order,
                    created_child_orders,
                    activation_after,
                    match_after,
                })
            }
            _ => Err(PlaceMatchSpotOrderV2Error::InvalidPlaceBranch),
        }
    }
}

impl StateMachineOwnedV2Diff for PlaceMatchSpotOrderV2UseCase {
    type StateDiff = PlaceMatchSpotOrderV2Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        match after {
            PlaceMatchSpotOrderV2AfterChanges::SinglePlacedOnly { created_order } => {
                Ok(PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { created_order })
            }
            PlaceMatchSpotOrderV2AfterChanges::SinglePlacedAndMatched {
                created_taker_order,
                activation_after,
                match_after,
            } => {
                let activation_changes = compute_activation_changes(
                    &state,
                    &created_taker_order,
                    activation_after.clone(),
                )?;
                let match_changes = compute_match_changes(state, activation_after, match_after)?;
                Ok(PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
                    created_taker_order,
                    activation_changes,
                    match_changes,
                })
            }
            PlaceMatchSpotOrderV2AfterChanges::NormalTpslPlacedAndMatched {
                created_parent_order,
                created_child_orders,
                activation_after,
                match_after,
            } => {
                let activation_changes = compute_activation_changes(
                    &state,
                    &created_parent_order,
                    activation_after.clone(),
                )?;
                let match_changes = compute_match_changes(state, activation_after, match_after)?;
                Ok(PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
                    created_parent_order,
                    created_child_orders,
                    activation_changes,
                    match_changes,
                })
            }
        }
    }
}

fn compute_activate_and_match_after_for_created_taker(
    order_cmd: &PlaceOnlySpotOrderV2OrderCmd,
    created_taker_order: SpotOrderV2,
    state: &PlaceMatchSpotOrderV2State,
    context: &ExecutionContext,
) -> Result<
    (ActivateSpotOrderV2AfterChanges, MatchSpotOrderV2AfterChanges),
    PlaceMatchSpotOrderV2Error,
> {
    let activation_cmd = ActivateSpotOrderV2Cmd {
        party_id: order_cmd.party_id.clone(),
        asset: order_cmd.asset,
        order_id: order_cmd.order_id.clone(),
    };
    let activation_state = ActivateSpotOrderV2State {
        pending_order: created_taker_order,
        balances: state.settlement_balances.clone(),
        base_asset_id: order_cmd.base_asset_id.clone(),
        quote_asset_id: order_cmd.quote_asset_id.clone(),
        maker_fee_bps: order_cmd.maker_fee_bps,
        taker_fee_bps: order_cmd.taker_fee_bps,
    };
    let activation_after = ActivateSpotOrderV2UseCase.compute_state_changed_with_context(
        &activation_cmd,
        &activation_state,
        context,
    )?;
    let match_cmd = MatchSpotOrderV2Cmd {
        party_id: order_cmd.party_id.clone(),
        asset: order_cmd.asset,
        order_id: order_cmd.order_id.clone(),
    };
    let match_state = MatchSpotOrderV2State {
        taker_order: activation_after.activated_order_after.clone(),
        maker_orders: state.maker_orders.clone(),
        settlement_balances: activation_after.balances_after.clone(),
        base_asset_id: order_cmd.base_asset_id.clone(),
        quote_asset_id: order_cmd.quote_asset_id.clone(),
        fee_account_id: state.fee_account_id.clone(),
        maker_fee_bps: order_cmd.maker_fee_bps,
        taker_fee_bps: order_cmd.taker_fee_bps,
    };
    let match_after = MatchSpotOrderV2UseCase.compute_state_changed_with_context(
        &match_cmd,
        &match_state,
        context,
    )?;
    Ok((activation_after, match_after))
}

fn compute_activation_changes(
    state: &PlaceMatchSpotOrderV2State,
    created_taker_order: &SpotOrderV2,
    activation_after: ActivateSpotOrderV2AfterChanges,
) -> Result<ActivateSpotOrderV2Changes, PlaceMatchSpotOrderV2Error> {
    let activation_state = ActivateSpotOrderV2State {
        pending_order: created_taker_order.clone(),
        balances: state.settlement_balances.clone(),
        base_asset_id: String::new(),
        quote_asset_id: String::new(),
        maker_fee_bps: 0,
        taker_fee_bps: 0,
    };
    Ok(ActivateSpotOrderV2UseCase::do_compute_state_diff(activation_state, activation_after)?)
}

fn compute_match_changes(
    state: PlaceMatchSpotOrderV2State,
    activation_after: ActivateSpotOrderV2AfterChanges,
    match_after: MatchSpotOrderV2AfterChanges,
) -> Result<MatchSpotOrderV2Changes, PlaceMatchSpotOrderV2Error> {
    let activated_taker_order = activation_after.activated_order_after;
    let match_state = MatchSpotOrderV2State {
        taker_order: activated_taker_order.clone(),
        maker_orders: state.maker_orders,
        settlement_balances: activation_after.balances_after,
        base_asset_id: String::new(),
        quote_asset_id: String::new(),
        fee_account_id: state.fee_account_id,
        maker_fee_bps: 0,
        taker_fee_bps: 0,
    };
    Ok(MatchSpotOrderV2UseCase::do_compute_state_diff(match_state, match_after)?)
}
