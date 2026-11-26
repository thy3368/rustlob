use std::collections::HashSet;

use crate::{
    LogAttr, ReplayableDomainEvent, ReplayableEntityChanges, UseCaseComputeError,
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

/// Outbound 持久化和发布业务事件，并用权威 `StateChanged` 更新本地投影。
pub trait StateSinkV3<F: UseCaseV3>: Send + Sync {
    type Error: std::error::Error;
    fn persist(&mut self, events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error>;
    fn replay(&mut self, state_changed: &F::StateChanged) -> Result<(), Self::Error>;
    fn publish(&mut self, events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error>;
}

impl UseCaseExecutorV3 {
    pub fn execute<F, SS, OB>(
        &self,
        use_case: &F,
        command: &F::Command,
        state_source: &SS,
        state_sink: &mut OB,
    ) -> ExecutionOutcomeV3<F::StateChanged, F::Error, SS::Error, OB::Error>
    where
        F: UseCaseV3,
        SS: StateSourceV3<F>,
        OB: StateSinkV3<F>,
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
            .replay(&state_changed)
            .map_err(ExecutionErrorV3::Replay)?;
        state_sink
            .publish(&events)
            .map_err(ExecutionErrorV3::Publish)?;
        Ok(ExecutionResultV3 { state_changed })
    }

    /// 同一 outbound 同时承担 source/sink 时，先完成只读装载，再顺序执行可变副作用。
    pub fn execute_with_outbound<F, OB>(
        &self,
        use_case: &F,
        command: &F::Command,
        outbound: &mut OB,
    ) -> ExecutionOutcomeV3<
        F::StateChanged,
        F::Error,
        <OB as StateSourceV3<F>>::Error,
        <OB as StateSinkV3<F>>::Error,
    >
    where
        F: UseCaseV3,
        OB: StateSourceV3<F> + StateSinkV3<F>,
    {
        use_case
            .check_command(command)
            .map_err(ExecutionErrorV3::Business)?;
        let state_given = outbound
            .load_given_state(command)
            .map_err(ExecutionErrorV3::LoadState)?;
        let state_changed = use_case
            .validate_and_compute(command, &state_given)
            .map_err(map_compute_error)?;
        let events = state_changed.replayable_events();
        validate_events(command, &events).map_err(ExecutionErrorV3::Contract)?;
        outbound
            .persist(&events)
            .map_err(ExecutionErrorV3::Persist)?;
        outbound
            .replay(&state_changed)
            .map_err(ExecutionErrorV3::Replay)?;
        outbound
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
        if event.lsn != command.lsn() || event.logical_ts != command.logical_ts() {
            return Err(UseCaseContractError::EventMetadataMismatch {
                event_index,
                expected_lsn: command.lsn(),
                actual_lsn: event.lsn,
                expected_logical_ts: command.logical_ts(),
                actual_logical_ts: event.logical_ts,
            });
        }
        let position = (event.lsn, event.command_index);
        if previous.is_some_and(|previous| position <= previous) {
            return Err(UseCaseContractError::EventPositionOutOfOrder { event_index });
        }
        previous = Some(position);
        if !keys.insert((
            event.lsn,
            event.command_index,
            event.entity_type,
            event.entity_id,
        )) {
            return Err(UseCaseContractError::DuplicateEventKey {
                lsn: event.lsn,
                command_index: event.command_index,
                entity_type: event.entity_type,
                entity_id: event.entity_id,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::{LogEntry, LogEntrySequencer};

    #[derive(Debug, Clone)]
    struct CountingUseCase {
        computes: Arc<AtomicUsize>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Changes(ReplayableDomainEvent);

    impl ReplayableEntityChanges for Changes {
        fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
            vec![&self.0]
        }
    }

    impl UseCaseV3 for CountingUseCase {
        type Command = LogEntry<()>;
        type StateGiven = ();
        type Error = Infallible;
        type StateChanged = Changes;

        fn compute_state_changed_only(
            &self,
            command: &Self::Command,
            _state_given: &Self::StateGiven,
        ) -> Result<Self::StateChanged, Self::Error> {
            self.computes.fetch_add(1, Ordering::Relaxed);
            Ok(Changes(ReplayableDomainEvent::new(
                command.logical_ts(),
                0,
                command.lsn(),
                7,
                2,
                1,
            )))
        }
    }

    #[derive(Default)]
    struct FakeOutbound {
        calls: Mutex<Vec<&'static str>>,
    }

    impl StateSourceV3<CountingUseCase> for FakeOutbound {
        type Error = Infallible;

        fn load_given_state(&self, _command: &LogEntry<()>) -> Result<(), Self::Error> {
            self.calls.lock().unwrap().push("load");
            Ok(())
        }
    }

    impl StateSinkV3<CountingUseCase> for FakeOutbound {
        type Error = Infallible;

        fn persist(&mut self, _events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
            self.calls.lock().unwrap().push("persist");
            Ok(())
        }

        fn replay(&mut self, changes: &Changes) -> Result<(), Self::Error> {
            assert_eq!(changes.0.entity_id, 7);
            self.calls.lock().unwrap().push("replay");
            Ok(())
        }

        fn publish(&mut self, _events: &[&ReplayableDomainEvent]) -> Result<(), Self::Error> {
            self.calls.lock().unwrap().push("publish");
            Ok(())
        }
    }

    #[test]
    fn same_outbound_receives_state_changed_after_single_compute() {
        let computes = Arc::new(AtomicUsize::new(0));
        let use_case = CountingUseCase {
            computes: Arc::clone(&computes),
        };
        let command = LogEntrySequencer::from_metadata(9, 10, 0, 0, "user".into(), ());
        let mut outbound = FakeOutbound::default();

        UseCaseExecutorV3
            .execute_with_outbound(&use_case, &command, &mut outbound)
            .unwrap();

        assert_eq!(computes.load(Ordering::Relaxed), 1);
        assert_eq!(
            *outbound.calls.lock().unwrap(),
            ["load", "persist", "replay", "publish"]
        );
    }
}
