use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges,
};
use common_entity::MiStateMachineOwned;
use serde::{Deserialize, Serialize};

use crate::entity::spot::{
    CancelSpotOrderChanges as EntityCancelSpotOrderChanges,
    CancelSpotOrderCmd as EntityCancelSpotOrderCmd,
    MatchSpotOrderChanges as EntityMatchSpotOrderChanges,
    MatchSpotOrderCmd as EntityMatchSpotOrderCmd,
    PlaceSpotOrderChanges as EntityPlaceSpotOrderChanges,
    PlaceSpotOrderCmd as EntityPlaceSpotOrderCmd, SpotOrderMiChanges, SpotOrderMiCommand,
    SpotOrderMiStateMachineError,
};
use crate::entity::{Balance, SpotOrder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderState {
    pub order: SpotOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceSpotOrderCmd {
    pub party_id: String,
    pub taker_base_balance: Balance,
    pub taker_quote_balance: Balance,
}

impl IssuedByParty for PlaceSpotOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderChanges {
    pub inner: EntityPlaceSpotOrderChanges,
}

impl ReplayableChanges for PlaceSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        common_entity::ReplayableChanges::to_replayable_events(&self.inner)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceSpotOrderUseCase;

impl CommandUseCase4 for PlaceSpotOrderUseCase {
    type Command = PlaceSpotOrderCmd;
    type GivenState = PlaceSpotOrderState;
    type Error = SpotOrderMiStateMachineError;
    type Changes = PlaceSpotOrderChanges;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "party_id must not be empty",
            });
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.order.belongs_to_account(&cmd.party_id) {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "order does not belong to command party",
            });
        }
        MiStateMachineOwned::compute_after_changes(
            &state.order,
            &SpotOrderMiCommand::Place(EntityPlaceSpotOrderCmd {
                taker_base_balance: cmd.taker_base_balance.clone(),
                taker_quote_balance: cmd.taker_quote_balance.clone(),
            }),
            &(),
        )
        .map(|_| ())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let changes = MiStateMachineOwned::compute_after_changes(
            &state.order,
            &SpotOrderMiCommand::Place(EntityPlaceSpotOrderCmd {
                taker_base_balance: cmd.taker_base_balance.clone(),
                taker_quote_balance: cmd.taker_quote_balance.clone(),
            }),
            &(),
        )?;
        match changes {
            SpotOrderMiChanges::Place(inner) => Ok(PlaceSpotOrderChanges { inner }),
            SpotOrderMiChanges::Match(_) | SpotOrderMiChanges::Cancel(_) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "place use case received non-place changes",
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderMiState {
    pub order: SpotOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchSpotOrderMiCmd {
    pub party_id: String,
    pub match_id: String,
    pub makers: Vec<SpotOrder>,
}

impl IssuedByParty for MatchSpotOrderMiCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderMiChanges {
    pub inner: EntityMatchSpotOrderChanges,
}

impl ReplayableChanges for MatchSpotOrderMiChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        common_entity::ReplayableChanges::to_replayable_events(&self.inner)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderMiUseCase;

impl CommandUseCase4 for MatchSpotOrderMiUseCase {
    type Command = MatchSpotOrderMiCmd;
    type GivenState = MatchSpotOrderMiState;
    type Error = SpotOrderMiStateMachineError;
    type Changes = MatchSpotOrderMiChanges;

    fn role(&self) -> &'static str {
        "MatchingEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "party_id must not be empty",
            });
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.order.belongs_to_account(&cmd.party_id) {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "order does not belong to command party",
            });
        }
        MiStateMachineOwned::compute_after_changes(
            &state.order,
            &SpotOrderMiCommand::Match(EntityMatchSpotOrderCmd {
                match_id: cmd.match_id.clone(),
                makers: cmd.makers.clone(),
            }),
            &(),
        )
        .map(|_| ())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let changes = MiStateMachineOwned::compute_after_changes(
            &state.order,
            &SpotOrderMiCommand::Match(EntityMatchSpotOrderCmd {
                match_id: cmd.match_id.clone(),
                makers: cmd.makers.clone(),
            }),
            &(),
        )?;
        match changes {
            SpotOrderMiChanges::Match(inner) => Ok(MatchSpotOrderMiChanges { inner }),
            SpotOrderMiChanges::Place(_) | SpotOrderMiChanges::Cancel(_) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "match use case received non-match changes",
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderMiState {
    pub order: SpotOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderMiCmd {
    pub party_id: String,
}

impl IssuedByParty for CancelSpotOrderMiCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderMiChanges {
    pub inner: EntityCancelSpotOrderChanges,
}

impl ReplayableChanges for CancelSpotOrderMiChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        common_entity::ReplayableChanges::to_replayable_events(&self.inner)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderMiUseCase;

impl CommandUseCase4 for CancelSpotOrderMiUseCase {
    type Command = CancelSpotOrderMiCmd;
    type GivenState = CancelSpotOrderMiState;
    type Error = SpotOrderMiStateMachineError;
    type Changes = CancelSpotOrderMiChanges;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "party_id must not be empty",
            });
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.order.belongs_to_account(&cmd.party_id) {
            return Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "order does not belong to command party",
            });
        }
        MiStateMachineOwned::compute_after_changes(
            &state.order,
            &SpotOrderMiCommand::Cancel(EntityCancelSpotOrderCmd),
            &(),
        )
        .map(|_| ())
    }

    fn compute_changes(
        &self,
        _cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let changes = MiStateMachineOwned::compute_after_changes(
            &state.order,
            &SpotOrderMiCommand::Cancel(EntityCancelSpotOrderCmd),
            &(),
        )?;
        match changes {
            SpotOrderMiChanges::Cancel(inner) => Ok(CancelSpotOrderMiChanges { inner }),
            SpotOrderMiChanges::Place(_) | SpotOrderMiChanges::Match(_) => {
                Err(SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "cancel use case received non-cancel changes",
                })
            }
        }
    }
}
