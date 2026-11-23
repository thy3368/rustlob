use std::fmt::Debug;

use crate::{LogAttr, ReplayableEntityChanges};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseCaseContractError {
    EmptyEvents,
    EventMetadataMismatch {
        event_index: usize,
        expected_lsn: u64,
        actual_lsn: u64,
        expected_logical_ts: u64,
        actual_logical_ts: u64,
    },
    EventPositionOutOfOrder {
        event_index: usize,
    },
    DuplicateEventKey {
        lsn: u64,
        command_index: u32,
        entity_type: u8,
        entity_id: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseCaseComputeError<E> {
    Business(E),
    Contract(UseCaseContractError),
}

pub type UseCaseComputeOutcome<C, E> = Result<C, UseCaseComputeError<E>>;

/// 直接产出包含 after truth 与逐实体 replay events 的 use case 契约。
pub trait UseCaseV3: Clone + Debug + Send + Sync {
    type Command: LogAttr;
    type StateGiven;
    type Error;
    type StateChanged: ReplayableEntityChanges;

    fn check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_state_given(
        &self,
        _cmd: &Self::Command,
        _state_given: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state_given: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error>;

    fn validate_and_compute(
        &self,
        cmd: &Self::Command,
        state_given: &Self::StateGiven,
    ) -> UseCaseComputeOutcome<Self::StateChanged, Self::Error> {
        self.validate_state_given(cmd, state_given)
            .map_err(UseCaseComputeError::Business)?;
        self.compute_state_changed_only(cmd, state_given)
            .map_err(UseCaseComputeError::Business)
    }

    fn compute_state_changed(
        &self,
        cmd: &Self::Command,
        state_given: &Self::StateGiven,
    ) -> UseCaseComputeOutcome<Self::StateChanged, Self::Error> {
        self.check_command(cmd)
            .map_err(UseCaseComputeError::Business)?;
        self.validate_and_compute(cmd, state_given)
    }
}
