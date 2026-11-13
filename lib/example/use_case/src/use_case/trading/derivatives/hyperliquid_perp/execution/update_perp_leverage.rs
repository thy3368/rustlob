use common_entity::{
    Entity, EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges,
    UpdatedEntityPair, UseCase,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::account::symbol::Symbol;
use crate::entity::{
    MarginModeV3, PerpPositionV3, PerpPositionV3Error, PerpPositionV3Event, UserId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateHyperliquidPerpLeverageCmd {
    pub user_id: UserId,
    pub symbol: Symbol,
    pub margin_mode: MarginModeV3,
    pub leverage: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateHyperliquidPerpLeverageState {
    pub position: PerpPositionV3,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UpdateHyperliquidPerpLeverageError {
    #[error("leverage must be greater than zero")]
    InvalidLeverage,
    #[error("position does not match command user or symbol")]
    PositionMismatch,
    #[error("position margin mode does not match command")]
    MarginModeMismatch,
    #[error("isolated position has no valid isolated margin configuration")]
    InvalidIsolatedConfiguration,
    #[error("arithmetic overflow while updating leverage")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UpdateHyperliquidPerpLeverageUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateHyperliquidPerpLeverageChanges {
    pub changed_position: UpdatedEntityPair<PerpPositionV3>,
}

impl ReplayableChanges for UpdateHyperliquidPerpLeverageChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![
            self.changed_position.after.track_update_event_from(&self.changed_position.before)?,
        ])
    }
}

impl UseCase for UpdateHyperliquidPerpLeverageUseCase {
    type Command = LogEntry<UpdateHyperliquidPerpLeverageCmd>;
    type StateGiven = UpdateHyperliquidPerpLeverageState;
    type Error = UpdateHyperliquidPerpLeverageError;
    type StateChanged = UpdateHyperliquidPerpLeverageChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.leverage <= Decimal::ZERO {
            return Err(UpdateHyperliquidPerpLeverageError::InvalidLeverage);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.position.user_id() != cmd.user_id || state.position.coin() != cmd.symbol.base {
            return Err(UpdateHyperliquidPerpLeverageError::PositionMismatch);
        }
        if state.position.margin_mode() != cmd.margin_mode {
            return Err(UpdateHyperliquidPerpLeverageError::MarginModeMismatch);
        }
        if cmd.margin_mode == MarginModeV3::Isolated && state.position.isolated_margin().is_none() {
            return Err(UpdateHyperliquidPerpLeverageError::InvalidIsolatedConfiguration);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let before = state.position.clone();
        let mut after = before.clone();
        after
            .apply(PerpPositionV3Event::LeverageUpdated {
                leverage: cmd.leverage,
                margin_mode: cmd.margin_mode,
                logical_ts: cmd.logical_ts(),
            })
            .map_err(map_position_error)?;
        Ok(UpdateHyperliquidPerpLeverageChanges {
            changed_position: UpdatedEntityPair { before, after },
        })
    }
}

fn map_position_error(error: PerpPositionV3Error) -> UpdateHyperliquidPerpLeverageError {
    match error {
        PerpPositionV3Error::InvalidLeverage => UpdateHyperliquidPerpLeverageError::InvalidLeverage,
        PerpPositionV3Error::InvalidMarginConfiguration
        | PerpPositionV3Error::InvalidIsolatedMargin => {
            UpdateHyperliquidPerpLeverageError::InvalidIsolatedConfiguration
        }
        PerpPositionV3Error::ArithmeticOverflow => {
            UpdateHyperliquidPerpLeverageError::ArithmeticOverflow
        }
        PerpPositionV3Error::InvalidQuantity
        | PerpPositionV3Error::InvalidPrice
        | PerpPositionV3Error::InvalidState => UpdateHyperliquidPerpLeverageError::PositionMismatch,
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntrySequencer, ReplayableChanges};
    use example_core_entity::account::coin::Coin;

    use super::*;

    fn command(leverage: Decimal) -> LogEntry<UpdateHyperliquidPerpLeverageCmd> {
        LogEntrySequencer::from_metadata(
            1,
            42,
            1,
            1,
            common_entity::SYSTEM_USER_ID.to_owned(),
            UpdateHyperliquidPerpLeverageCmd {
                user_id: UserId(7),
                symbol: Symbol::new(3, Coin::BTC, Coin::USDC),
                margin_mode: MarginModeV3::Cross,
                leverage,
            },
        )
    }

    #[test]
    fn updates_position_and_projects_one_pair_event() {
        let use_case = UpdateHyperliquidPerpLeverageUseCase;
        let state = UpdateHyperliquidPerpLeverageState {
            position: PerpPositionV3::empty(9, UserId(7), Coin::BTC),
        };
        let changes = use_case.compute_state_changed(&command(Decimal::from(5)), &state).unwrap();

        assert_eq!(changes.changed_position.before.leverage(), Decimal::ONE);
        assert_eq!(changes.changed_position.after.leverage(), Decimal::from(5));
        assert_eq!(changes.changed_position.after.version(), 1);
        assert_eq!(changes.to_replayable_events().unwrap().len(), 1);
    }

    #[test]
    fn rejects_zero_and_negative_leverage() {
        let use_case = UpdateHyperliquidPerpLeverageUseCase;
        for leverage in [Decimal::ZERO, Decimal::NEGATIVE_ONE] {
            assert_eq!(
                use_case.check_command(&command(leverage)),
                Err(UpdateHyperliquidPerpLeverageError::InvalidLeverage)
            );
        }
    }
}
