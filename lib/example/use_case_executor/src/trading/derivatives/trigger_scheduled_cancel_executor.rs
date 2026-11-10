use common_entity::{ExecutionError, ExecutionResult, StateSink, StateSource, UseCaseExecutor};
use example_core_use_case::{
    TriggerScheduledCancelChanges, TriggerScheduledCancelCmd, TriggerScheduledCancelError,
    TriggerScheduledCancelUseCase,
};

pub fn execute_trigger_scheduled_cancel<OB>(
    command: &common_entity::LogEntry<TriggerScheduledCancelCmd>,
    outbound: &OB,
) -> Result<
    ExecutionResult<TriggerScheduledCancelChanges>,
    ExecutionError<
        TriggerScheduledCancelError,
        <OB as StateSink<TriggerScheduledCancelUseCase>>::Error,
    >,
>
where
    OB: StateSource<
            TriggerScheduledCancelUseCase,
            Error = <OB as StateSink<TriggerScheduledCancelUseCase>>::Error,
        > + StateSink<TriggerScheduledCancelUseCase>,
{
    UseCaseExecutor.execute::<TriggerScheduledCancelUseCase, OB, OB>(
        &TriggerScheduledCancelUseCase,
        command,
        outbound,
        outbound,
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use common_entity::{EntityReplayableEvent, LogEntrySequencer};
    use example_core_use_case::{ScheduledCancel, TriggerScheduledCancelState, UserId};

    use super::*;

    const USER: UserId = UserId(7);
    const NOW_MS: u64 = 1_700_000_010_000;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
    #[error("fake trigger outbound error")]
    struct FakeError;

    #[derive(Debug)]
    struct FakeOutbound {
        state: TriggerScheduledCancelState,
        batches: Mutex<Vec<Vec<EntityReplayableEvent>>>,
    }

    impl StateSource<TriggerScheduledCancelUseCase> for FakeOutbound {
        type Error = FakeError;

        fn load_given_state(
            &self,
            _cmd: &common_entity::LogEntry<TriggerScheduledCancelCmd>,
        ) -> Result<TriggerScheduledCancelState, Self::Error> {
            Ok(self.state.clone())
        }
    }

    impl StateSink<TriggerScheduledCancelUseCase> for FakeOutbound {
        type Error = FakeError;

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record(events)
        }

        fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record(events)
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record(events)
        }
    }

    impl FakeOutbound {
        fn due() -> Self {
            let mut schedule = ScheduledCancel::new(USER.0);
            schedule.replace(Some(NOW_MS), NOW_MS - 5_000).expect("fixture should arm");
            Self {
                state: TriggerScheduledCancelState {
                    scheduled_cancel: schedule,
                    spot_orders: Vec::new(),
                    spot_balances: Vec::new(),
                    perp_orders: Vec::new(),
                },
                batches: Mutex::new(Vec::new()),
            }
        }

        fn record(&self, events: &[EntityReplayableEvent]) -> Result<(), FakeError> {
            self.batches.lock().map_err(|_| FakeError)?.push(events.to_vec());
            Ok(())
        }
    }

    #[test]
    fn due_schedule_runs_shared_executor_and_all_sink_phases() {
        let command = LogEntrySequencer::from_metadata(
            9,
            NOW_MS,
            1,
            2,
            TriggerScheduledCancelCmd { user_id: USER, now_ms: NOW_MS },
        );
        let outbound = FakeOutbound::due();

        let result = execute_trigger_scheduled_cancel(&command, &outbound)
            .expect("到期计划应通过共享执行器触发");

        assert_eq!(result.changes.changed_schedule.after.cancel_at_ms(), None);
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].lsn, 9);
        assert_eq!(result.events[0].logical_ts, NOW_MS);
        let batches = outbound.batches.lock().expect("test mutex should remain healthy");
        assert_eq!(batches.len(), 3);
        assert!(batches.windows(2).all(|pair| pair[0] == pair[1]));
    }
}
