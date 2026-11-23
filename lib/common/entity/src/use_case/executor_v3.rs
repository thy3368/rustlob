use std::collections::HashSet;

use crate::{
    EventPosition, LogAttr, ReplayableDomainEvent, ReplayableEntityChanges, UseCaseComputeError,
    UseCaseContractError, UseCaseV3,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct UseCaseExecutorV3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResultV3<C> {
    pub state_changed: C,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionErrorV3<BE, LE, OE> {
    Business(BE),
    Contract(UseCaseContractError),
    LoadState(LE),
    Persist(OE),
    Replay(OE),
    Publish(OE),
}

pub type ExecutionOutcomeV3<C, BE, LE, OE> =
    Result<ExecutionResultV3<C>, ExecutionErrorV3<BE, LE, OE>>;

pub trait StateSourceV3<F>: Send + Sync
where
    F: UseCaseV3,
{
    type Error: std::error::Error;
    fn load_given_state(&self, command: &F::Command) -> Result<F::StateGiven, Self::Error>;
}

/// Outbound 只消费由 `StateChanged` 借用的业务事件。
pub trait StateSinkV3: Send + Sync {
    type Error: std::error::Error;
    fn persist(&self, events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error>;
    fn replay(&self, events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error>;
    fn publish(&self, events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error>;
}

impl UseCaseExecutorV3 {
    pub fn execute<F, SS, OB>(
        &self,
        use_case: &F,
        command: &F::Command,
        state_source: &SS,
        state_sink: &OB,
    ) -> ExecutionOutcomeV3<F::StateChanged, F::Error, SS::Error, OB::Error>
    where
        F: UseCaseV3,
        SS: StateSourceV3<F>,
        OB: StateSinkV3,
    {
        use_case
            .check_command(command)
            .map_err(ExecutionErrorV3::Business)?;
        let state_given = state_source
            .load_given_state(command)
            .map_err(ExecutionErrorV3::LoadState)?;
        let state_changed = use_case
            .validate_and_compute(command, &state_given)
            .map_err(map_compute_error)?;
        let events = state_changed.replayable_events();
        validate_events(command, &events).map_err(ExecutionErrorV3::Contract)?;
        state_sink
            .persist(&events)
            .map_err(ExecutionErrorV3::Persist)?;
        state_sink
            .replay(&events)
            .map_err(ExecutionErrorV3::Replay)?;
        state_sink
            .publish(&events)
            .map_err(ExecutionErrorV3::Publish)?;
        Ok(ExecutionResultV3 { state_changed })
    }
}

fn map_compute_error<BE, LE, OE>(error: UseCaseComputeError<BE>) -> ExecutionErrorV3<BE, LE, OE> {
    match error {
        UseCaseComputeError::Business(error) => ExecutionErrorV3::Business(error),
        UseCaseComputeError::Contract(error) => ExecutionErrorV3::Contract(error),
    }
}

fn validate_events(
    command: &impl LogAttr,
    events: &[&ReplayableDomainEvent],
) -> Result<(), UseCaseContractError> {
    if events.is_empty() {
        return Err(UseCaseContractError::EmptyEvents);
    }
    let mut keys = HashSet::with_capacity(events.len());
    let mut previous = None;
    for (event_index, event) in events.iter().enumerate() {
        if event.position.lsn != command.lsn() || event.logical_ts != command.logical_ts() {
            return Err(UseCaseContractError::EventMetadataMismatch {
                event_index,
                expected_lsn: command.lsn(),
                actual_lsn: event.position.lsn,
                expected_logical_ts: command.logical_ts(),
                actual_logical_ts: event.logical_ts,
            });
        }
        if previous.is_some_and(|position: EventPosition| event.position <= position) {
            return Err(UseCaseContractError::EventPositionOutOfOrder { event_index });
        }
        previous = Some(event.position);
        if !keys.insert((event.position, event.entity_type, event.entity_id)) {
            return Err(UseCaseContractError::DuplicateEventKey {
                lsn: event.position.lsn,
                command_index: event.position.command_index,
                entity_type: event.entity_type,
                entity_id: event.entity_id,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Shard;

    struct Command;
    impl Shard for Command {
        fn partition(&self) -> u8 {
            0
        }
    }
    impl LogAttr for Command {
        fn lsn(&self) -> u64 {
            10
        }
        fn logical_ts(&self) -> u64 {
            20
        }
    }

    fn event(index: u32, id: u64, kind: u8) -> ReplayableDomainEvent {
        ReplayableDomainEvent::new_at(
            0,
            0,
            20,
            EventPosition::ZERO,
            EventPosition::new(10, index),
            id,
            kind,
            1,
        )
    }

    #[test]
    fn accepts_different_entity_types_in_global_command_order() {
        let first = event(0, 7, 2);
        let second = event(1, 8, 3);
        assert_eq!(validate_events(&Command, &[&first, &second]), Ok(()));
    }

    #[test]
    fn rejects_repeated_or_descending_command_indices() {
        let first = event(1, 7, 2);
        let repeated = event(1, 8, 3);
        let descending = event(0, 8, 3);
        assert_eq!(
            validate_events(&Command, &[&first, &repeated]),
            Err(UseCaseContractError::EventPositionOutOfOrder { event_index: 1 })
        );
        assert_eq!(
            validate_events(&Command, &[&first, &descending]),
            Err(UseCaseContractError::EventPositionOutOfOrder { event_index: 1 })
        );
    }

    #[test]
    fn rejects_metadata_mismatch() {
        let mut invalid = event(0, 7, 2);
        invalid.logical_ts = 21;
        assert!(matches!(
            validate_events(&Command, &[&invalid]),
            Err(UseCaseContractError::EventMetadataMismatch { event_index: 0, .. })
        ));
    }
}
