use std::fmt::Debug;

use crate::{LogAttr, ReplayableDomainEvent};

/// V3 use case 的框架契约错误。
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
    DuplicateEventKey {
        lsn: u64,
        entity_type: u8,
        entity_id: u64,
    },
}

/// 区分业务拒绝与 V3 框架契约拒绝。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseCaseComputeError<E> {
    Business(E),
    Contract(UseCaseContractError),
}

/// 直接产出 after truth 与实体 FSM replay events 的 use case 契约。
///
/// V3 不依赖 `UseCase`、`StateDiff` 或 `ReplayableChanges`。事件顺序由实现返回的
/// `Vec` 决定，且一次成功计算必须至少产生一个事件。
pub trait UseCaseV3: Clone + Debug + Send + Sync {
    type Command: LogAttr;
    type StateGiven;
    type Error;
    type StateChanged;

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
    ) -> Result<(Self::StateChanged, Vec<ReplayableDomainEvent>), Self::Error>;

    /// 在 command 已检查后执行统一的 validate / compute 链路。
    fn validate_and_compute(
        &self,
        cmd: &Self::Command,
        state_given: &Self::StateGiven,
    ) -> Result<(Self::StateChanged, Vec<ReplayableDomainEvent>), UseCaseComputeError<Self::Error>>
    {
        self.validate_state_given(cmd, state_given)
            .map_err(UseCaseComputeError::Business)?;
        let (state_changed, events) = self
            .compute_state_changed_only(cmd, state_given)
            .map_err(UseCaseComputeError::Business)?;
        if events.is_empty() {
            return Err(UseCaseComputeError::Contract(
                UseCaseContractError::EmptyEvents,
            ));
        }
        Ok((state_changed, events))
    }

    fn compute_state_changed(
        &self,
        cmd: &Self::Command,
        state_given: &Self::StateGiven,
    ) -> Result<(Self::StateChanged, Vec<ReplayableDomainEvent>), UseCaseComputeError<Self::Error>>
    {
        self.check_command(cmd)
            .map_err(UseCaseComputeError::Business)?;
        self.validate_and_compute(cmd, state_given)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::Shard;

    #[derive(Debug, Clone, Copy)]
    struct Command;

    impl Shard for Command {
        fn partition(&self) -> u8 {
            0
        }
    }

    impl LogAttr for Command {
        fn lsn(&self) -> u64 {
            11
        }

        fn logical_ts(&self) -> u64 {
            22
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestError {
        Check,
        Validate,
        Compute,
    }

    #[derive(Debug, Clone)]
    struct TestUseCase {
        calls: Arc<Mutex<Vec<&'static str>>>,
        failure: Option<TestError>,
        empty_events: bool,
    }

    impl UseCaseV3 for TestUseCase {
        type Command = Command;
        type StateGiven = u64;
        type Error = TestError;
        type StateChanged = u64;

        fn check_command(&self, _cmd: &Command) -> Result<(), TestError> {
            self.calls.lock().unwrap().push("check");
            if self.failure == Some(TestError::Check) {
                return Err(TestError::Check);
            }
            Ok(())
        }

        fn validate_state_given(&self, _cmd: &Command, _state: &u64) -> Result<(), TestError> {
            self.calls.lock().unwrap().push("validate");
            if self.failure == Some(TestError::Validate) {
                return Err(TestError::Validate);
            }
            Ok(())
        }

        fn compute_state_changed_only(
            &self,
            _cmd: &Command,
            state: &u64,
        ) -> Result<(u64, Vec<ReplayableDomainEvent>), TestError> {
            self.calls.lock().unwrap().push("compute");
            if self.failure == Some(TestError::Compute) {
                return Err(TestError::Compute);
            }
            let events = if self.empty_events {
                Vec::new()
            } else {
                vec![ReplayableDomainEvent::new_created(22, 7, 3)]
            };
            Ok((state + 1, events))
        }
    }

    fn use_case(failure: Option<TestError>, empty_events: bool) -> TestUseCase {
        TestUseCase {
            calls: Arc::new(Mutex::new(Vec::new())),
            failure,
            empty_events,
        }
    }

    #[test]
    fn compute_runs_hooks_in_order_and_returns_tuple() {
        let use_case = use_case(None, false);

        let (state, events) = use_case.compute_state_changed(&Command, &4).unwrap();

        assert_eq!(state, 5);
        assert_eq!(events.len(), 1);
        assert_eq!(
            *use_case.calls.lock().unwrap(),
            ["check", "validate", "compute"]
        );
    }

    #[test]
    fn business_errors_are_wrapped_and_stop_the_chain() {
        for (failure, expected_calls) in [
            (TestError::Check, vec!["check"]),
            (TestError::Validate, vec!["check", "validate"]),
            (TestError::Compute, vec!["check", "validate", "compute"]),
        ] {
            let use_case = use_case(Some(failure), false);

            assert_eq!(
                use_case.compute_state_changed(&Command, &4),
                Err(UseCaseComputeError::Business(failure))
            );
            assert_eq!(*use_case.calls.lock().unwrap(), expected_calls);
        }
    }

    #[test]
    fn empty_events_are_rejected_as_contract_error() {
        let use_case = use_case(None, true);

        assert_eq!(
            use_case.compute_state_changed(&Command, &4),
            Err(UseCaseComputeError::Contract(
                UseCaseContractError::EmptyEvents
            ))
        );
    }
}
