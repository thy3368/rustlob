use crate::{
    CommandWithGivenState, EntityError, EntityReplayableEvent, MiStateMachineOwnedV2BeforeAfter,
    ReplayableChanges,
};

/// 多聚合 MI state-machine family 的运行时编排器。
///
/// 该类型只固定 adapter / outbound 之间的执行顺序，不承载业务规则。
#[derive(Debug, Clone, Copy, Default)]
pub struct MiStateMachineFamilyExecutor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MiFamilyExecutionResult<C> {
    pub changes: C,
    pub events: Vec<EntityReplayableEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiFamilyExecutionError<BE, OE> {
    Business(BE),
    ProjectEvents(EntityError),
    LoadState(OE),
    Persist(OE),
    Replay(OE),
    Publish(OE),
}

/// 把 adapter-side request 与 family command / authoritative given state 绑定起来。
pub trait MiFamilyExecutionSpec<F>
where
    F: MiStateMachineOwnedV2BeforeAfter,
{
    type Request;
    type LoadedState;

    fn command<'a>(request: &'a Self::Request) -> F::Command<'a>;

    fn given_state<'a>(
        request: &'a Self::Request,
        loaded: &'a Self::LoadedState,
    ) -> <F::Command<'a> as CommandWithGivenState>::GivenState;
}

/// MI family runtime 所需的 outbound port。
pub trait MiFamilyOutbound<Request, LoadedState>: Send + Sync {
    type Error: std::error::Error;

    fn load_state(&self, request: &Request) -> Result<LoadedState, Self::Error>;
    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
}

impl MiStateMachineFamilyExecutor {
    pub fn execute<F, S, OB>(
        &self,
        family: &F,
        request: &S::Request,
        outbound: &OB,
    ) -> Result<
        MiFamilyExecutionResult<F::BeforeAfterChanges>,
        MiFamilyExecutionError<F::Error, OB::Error>,
    >
    where
        F: MiStateMachineOwnedV2BeforeAfter,
        S: MiFamilyExecutionSpec<F>,
        OB: MiFamilyOutbound<S::Request, S::LoadedState>,
    {
        {
            let cmd = S::command(request);
            family.pre_check_command(&cmd).map_err(MiFamilyExecutionError::Business)?;
        }

        let loaded = outbound.load_state(request).map_err(MiFamilyExecutionError::LoadState)?;

        let cmd = S::command(request);
        let given_state = S::given_state(request, &loaded);

        family
            .validate_against_given_state(&cmd, &given_state)
            .map_err(MiFamilyExecutionError::Business)?;

        let after = family
            .compute_after_changes_unchecked(&cmd, &given_state)
            .map_err(MiFamilyExecutionError::Business)?;

        let changes = F::merge_before_and_after(&given_state, after)
            .map_err(MiFamilyExecutionError::Business)?;

        let events =
            changes.to_replayable_events().map_err(MiFamilyExecutionError::ProjectEvents)?;

        outbound.persist(&events).map_err(MiFamilyExecutionError::Persist)?;
        outbound.replay(&events).map_err(MiFamilyExecutionError::Replay)?;
        outbound.publish(&events).map_err(MiFamilyExecutionError::Publish)?;

        Ok(MiFamilyExecutionResult { changes, events })
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::{CommandWithGivenState, EntityError, MiStateMachineV2Unchecked};

    #[derive(Debug, Clone)]
    struct StubRequest {
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    #[derive(Debug, Clone)]
    struct StubLoadedState {
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    #[derive(Debug, Clone)]
    struct StubCommand {
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    impl CommandWithGivenState for StubCommand {
        type GivenState = Arc<Mutex<Vec<&'static str>>>;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StubAfter;

    #[derive(Debug, Clone)]
    struct StubChanges {
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    impl ReplayableChanges for StubChanges {
        fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
            self.log.lock().unwrap().push("project");
            Ok(Vec::new())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum StubBusinessError {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StubOutboundError;

    impl fmt::Display for StubOutboundError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("stub outbound error")
        }
    }

    impl std::error::Error for StubOutboundError {}

    #[derive(Debug, Clone)]
    struct StubFamily;

    impl MiStateMachineV2Unchecked for StubFamily {
        type Command<'a>
            = StubCommand
        where
            Self: 'a;
        type Error = StubBusinessError;
        type AfterChanges = StubAfter;

        fn pre_check_command<'a>(&self, cmd: &Self::Command<'a>) -> Result<(), Self::Error> {
            cmd.log.lock().unwrap().push("pre_check");
            Ok(())
        }

        fn validate_against_given_state<'a>(
            &self,
            _cmd: &Self::Command<'a>,
            given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
        ) -> Result<(), Self::Error> {
            given_state.lock().unwrap().push("validate");
            Ok(())
        }

        fn compute_after_changes_unchecked<'a>(
            &self,
            _cmd: &Self::Command<'a>,
            given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
        ) -> Result<Self::AfterChanges, Self::Error> {
            given_state.lock().unwrap().push("compute");
            Ok(StubAfter)
        }
    }

    impl MiStateMachineOwnedV2BeforeAfter for StubFamily {
        type BeforeAfterChanges = StubChanges;

        fn merge_before_and_after(
            given_state: &<Self::Command<'_> as CommandWithGivenState>::GivenState,
            _after: Self::AfterChanges,
        ) -> Result<Self::BeforeAfterChanges, Self::Error> {
            given_state.lock().unwrap().push("merge");
            Ok(StubChanges { log: Arc::clone(given_state) })
        }
    }

    struct StubSpec;

    impl MiFamilyExecutionSpec<StubFamily> for StubSpec {
        type Request = StubRequest;
        type LoadedState = StubLoadedState;

        fn command<'a>(request: &'a Self::Request) -> StubCommand {
            StubCommand { log: Arc::clone(&request.log) }
        }

        fn given_state<'a>(
            _request: &'a Self::Request,
            loaded: &'a Self::LoadedState,
        ) -> <<StubFamily as MiStateMachineV2Unchecked>::Command<'a> as CommandWithGivenState>::GivenState
        {
            Arc::clone(&loaded.log)
        }
    }

    #[derive(Debug)]
    struct StubOutbound {
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    impl MiFamilyOutbound<StubRequest, StubLoadedState> for StubOutbound {
        type Error = StubOutboundError;

        fn load_state(&self, request: &StubRequest) -> Result<StubLoadedState, Self::Error> {
            request.log.lock().unwrap().push("load_state");
            Ok(StubLoadedState { log: Arc::clone(&request.log) })
        }

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            assert!(events.is_empty());
            self.log.lock().unwrap().push("persist");
            Ok(())
        }

        fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            assert!(events.is_empty());
            self.log.lock().unwrap().push("replay");
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            assert!(events.is_empty());
            self.log.lock().unwrap().push("publish");
            Ok(())
        }
    }

    #[test]
    fn mi_family_executor_runs_fixed_runtime_sequence() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let request = StubRequest { log: Arc::clone(&log) };
        let executor = MiStateMachineFamilyExecutor;
        let outbound = StubOutbound { log: Arc::clone(&log) };

        executor.execute::<StubFamily, StubSpec, _>(&StubFamily, &request, &outbound).unwrap();

        assert_eq!(
            *log.lock().unwrap(),
            vec![
                "pre_check",
                "load_state",
                "validate",
                "compute",
                "merge",
                "project",
                "persist",
                "replay",
                "publish"
            ]
        );
    }
}
