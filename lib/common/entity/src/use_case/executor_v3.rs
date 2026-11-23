use std::collections::HashSet;

use crate::{LogAttr, ReplayableDomainEvent, UseCaseComputeError, UseCaseContractError, UseCaseV3};

#[derive(Debug, Clone, Copy, Default)]
pub struct UseCaseExecutorV3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResultV3<C> {
    pub state_changed: C,
    pub events: Vec<ReplayableDomainEvent>,
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

pub trait StateSourceV3<F>: Send + Sync
where
    F: UseCaseV3,
{
    type Error: std::error::Error;

    fn load_given_state(&self, command: &F::Command) -> Result<F::StateGiven, Self::Error>;
}

pub trait StateSinkV3: Send + Sync {
    type Error: std::error::Error;

    fn persist(&self, events: &[ReplayableDomainEvent]) -> Result<(), Self::Error>;
    fn replay(&self, events: &[ReplayableDomainEvent]) -> Result<(), Self::Error>;
    fn publish(&self, events: &[ReplayableDomainEvent]) -> Result<(), Self::Error>;
}

impl UseCaseExecutorV3 {
    pub fn execute<F, SS, OB>(
        &self,
        use_case: &F,
        command: &F::Command,
        state_source: &SS,
        state_sink: &OB,
    ) -> Result<ExecutionResultV3<F::StateChanged>, ExecutionErrorV3<F::Error, SS::Error, OB::Error>>
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
        let (state_changed, events) = use_case
            .validate_and_compute(command, &state_given)
            .map_err(map_compute_error)?;

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

        Ok(ExecutionResultV3 {
            state_changed,
            events,
        })
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
    events: &[ReplayableDomainEvent],
) -> Result<(), UseCaseContractError> {
    if events.is_empty() {
        return Err(UseCaseContractError::EmptyEvents);
    }

    let mut keys = HashSet::with_capacity(events.len());
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
        if !keys.insert((event.lsn, event.entity_type, event.entity_id)) {
            return Err(UseCaseContractError::DuplicateEventKey {
                lsn: event.lsn,
                entity_type: event.entity_type,
                entity_id: event.entity_id,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::Shard;

    #[derive(Debug, Clone, Copy)]
    struct Command;

    impl Shard for Command {
        fn partition(&self) -> u8 {
            1
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestError {
        Business,
        Load,
        Persist,
        Replay,
        Publish,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "{self:?}")
        }
    }

    impl std::error::Error for TestError {}

    #[derive(Debug, Clone)]
    struct TestUseCase {
        events: Vec<ReplayableDomainEvent>,
        business_error: bool,
    }

    impl UseCaseV3 for TestUseCase {
        type Command = Command;
        type StateGiven = u64;
        type Error = TestError;
        type StateChanged = u64;

        fn validate_state_given(&self, _cmd: &Command, _state: &u64) -> Result<(), TestError> {
            if self.business_error {
                return Err(TestError::Business);
            }
            Ok(())
        }

        fn compute_state_changed_only(
            &self,
            _cmd: &Command,
            state: &u64,
        ) -> Result<(u64, Vec<ReplayableDomainEvent>), TestError> {
            Ok((*state, self.events.clone()))
        }
    }

    struct Source {
        calls: Arc<Mutex<Vec<&'static str>>>,
        failure: bool,
    }

    impl StateSourceV3<TestUseCase> for Source {
        type Error = TestError;

        fn load_given_state(&self, _command: &Command) -> Result<u64, TestError> {
            self.calls.lock().unwrap().push("load");
            if self.failure {
                return Err(TestError::Load);
            }
            Ok(9)
        }
    }

    struct Sink {
        calls: Arc<Mutex<Vec<&'static str>>>,
        failure: Option<TestError>,
    }

    impl StateSinkV3 for Sink {
        type Error = TestError;

        fn persist(&self, _events: &[ReplayableDomainEvent]) -> Result<(), TestError> {
            self.calls.lock().unwrap().push("persist");
            match self.failure {
                Some(TestError::Persist) => Err(TestError::Persist),
                _ => Ok(()),
            }
        }

        fn replay(&self, _events: &[ReplayableDomainEvent]) -> Result<(), TestError> {
            self.calls.lock().unwrap().push("replay");
            match self.failure {
                Some(TestError::Replay) => Err(TestError::Replay),
                _ => Ok(()),
            }
        }

        fn publish(&self, _events: &[ReplayableDomainEvent]) -> Result<(), TestError> {
            self.calls.lock().unwrap().push("publish");
            match self.failure {
                Some(TestError::Publish) => Err(TestError::Publish),
                _ => Ok(()),
            }
        }
    }

    fn event(entity_id: u64, entity_type: u8) -> ReplayableDomainEvent {
        ReplayableDomainEvent::new(20, 0, 10, entity_id, entity_type, 0)
    }

    fn execute(
        events: Vec<ReplayableDomainEvent>,
        load_failure: bool,
        sink_failure: Option<TestError>,
    ) -> (
        Result<ExecutionResultV3<u64>, ExecutionErrorV3<TestError, TestError, TestError>>,
        Vec<&'static str>,
    ) {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let result = UseCaseExecutorV3.execute(
            &TestUseCase {
                events,
                business_error: false,
            },
            &Command,
            &Source {
                calls: Arc::clone(&calls),
                failure: load_failure,
            },
            &Sink {
                calls: Arc::clone(&calls),
                failure: sink_failure,
            },
        );
        let observed = calls.lock().unwrap().clone();
        (result, observed)
    }

    #[test]
    fn executes_outbound_in_order_and_preserves_event_order() {
        let events = vec![event(2, 7), event(1, 8)];

        let (result, calls) = execute(events.clone(), false, None);

        assert_eq!(
            result,
            Ok(ExecutionResultV3 {
                state_changed: 9,
                events,
            })
        );
        assert_eq!(calls, ["load", "persist", "replay", "publish"]);
    }

    #[test]
    fn maps_load_error_before_outbound_calls() {
        let (result, calls) = execute(vec![event(1, 7)], true, None);

        assert_eq!(result, Err(ExecutionErrorV3::LoadState(TestError::Load)));
        assert_eq!(calls, ["load"]);
    }

    #[test]
    fn rejects_empty_events_before_outbound_calls() {
        let (result, calls) = execute(Vec::new(), false, None);

        assert_eq!(
            result,
            Err(ExecutionErrorV3::Contract(
                UseCaseContractError::EmptyEvents
            ))
        );
        assert_eq!(calls, ["load"]);
    }

    #[test]
    fn rejects_metadata_mismatch_without_rewriting_event() {
        let mut invalid = event(1, 7);
        invalid.logical_ts = 21;

        let (result, calls) = execute(vec![invalid], false, None);

        assert_eq!(
            result,
            Err(ExecutionErrorV3::Contract(
                UseCaseContractError::EventMetadataMismatch {
                    event_index: 0,
                    expected_lsn: 10,
                    actual_lsn: 10,
                    expected_logical_ts: 20,
                    actual_logical_ts: 21,
                }
            ))
        );
        assert_eq!(calls, ["load"]);
    }

    #[test]
    fn rejects_duplicate_event_key_before_outbound_calls() {
        let duplicate = event(1, 7);

        let (result, calls) = execute(vec![duplicate.clone(), duplicate], false, None);

        assert_eq!(
            result,
            Err(ExecutionErrorV3::Contract(
                UseCaseContractError::DuplicateEventKey {
                    lsn: 10,
                    entity_type: 7,
                    entity_id: 1,
                }
            ))
        );
        assert_eq!(calls, ["load"]);
    }

    #[test]
    fn maps_each_outbound_error_and_stops_later_stages() {
        for (failure, expected, calls) in [
            (
                TestError::Persist,
                ExecutionErrorV3::Persist(TestError::Persist),
                vec!["load", "persist"],
            ),
            (
                TestError::Replay,
                ExecutionErrorV3::Replay(TestError::Replay),
                vec!["load", "persist", "replay"],
            ),
            (
                TestError::Publish,
                ExecutionErrorV3::Publish(TestError::Publish),
                vec!["load", "persist", "replay", "publish"],
            ),
        ] {
            let (result, observed_calls) = execute(vec![event(1, 7)], false, Some(failure));

            assert_eq!(result, Err(expected));
            assert_eq!(observed_calls, calls);
        }
    }
}
