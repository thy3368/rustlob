use crate::{
    EntityError, EntityReplayableEvent, MiStateMachineOwnedV2BeforeAfter, ReplayableChanges,
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

pub type MiFamilyExecutionOutcome<C, BE, OE> =
    Result<MiFamilyExecutionResult<C>, MiFamilyExecutionError<BE, OE>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiFamilyExecutionError<BE, OE> {
    Business(BE),
    ProjectEvents(EntityError),
    LoadState(OE),
    Persist(OE),
    Replay(OE),
    Publish(OE),
}

/// adapter-side request 到 family command 的映射约定。
///
/// executor 不直接依赖该 trait；adapter 可用它把 request 转成 command 后再执行。
pub trait MiFamilyExecutionSpec<F>
where
    F: MiStateMachineOwnedV2BeforeAfter,
{
    type Request;

    fn command(request: &Self::Request) -> F::Command;
}

/// MI family runtime 所需的 authoritative given state source port。
pub trait MiFamilyStateSource<F>: Send + Sync
where
    F: MiStateMachineOwnedV2BeforeAfter,
{
    type Error: std::error::Error;

    fn load_given_state(&self, cmd: &F::Command) -> Result<F::GivenState, Self::Error>;
}

/// MI family runtime 所需的事件副作用 outbound port。
pub trait MiFamilyStateSink<F>: Send + Sync
where
    F: MiStateMachineOwnedV2BeforeAfter,
{
    type Error: std::error::Error;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
}

impl MiStateMachineFamilyExecutor {
    /// 执行一个 MI family use case 的运行时编排。
    ///
    /// adapter 应在调用前完成 request 到 command 的转换。state source 基于 command 加载
    /// authoritative given state，后续业务校验与计算都只读取 command 和 owned given state。
    ///
    /// 固定执行顺序为：pre-check -> load state -> validate -> compute ->
    /// merge -> project events -> persist -> replay -> publish。该函数只负
    /// 责顺序编排和错误映射，不承载业务规则。
    ///
    /// family 返回的业务错误映射为 [`MiFamilyExecutionError::Business`]，
    /// 事件投影错误映射为 [`MiFamilyExecutionError::ProjectEvents`]，
    /// outbound 端错误按发生阶段分别映射为 load / persist / replay /
    /// publish 对应的执行错误。
    pub fn execute<F, SS, OB>(
        &self,
        family: &F,
        command: &F::Command,
        state_source: &SS,
        outbound: &OB,
    ) -> MiFamilyExecutionOutcome<F::BeforeAfterChanges, F::Error, OB::Error>
    where
        F: MiStateMachineOwnedV2BeforeAfter,
        SS: MiFamilyStateSource<F, Error = OB::Error>,
        OB: MiFamilyStateSink<F>,
    {
        family.pre_check_command(command).map_err(MiFamilyExecutionError::Business)?;

        // 加载 authoritative given state，后续业务校验与计算都以该状态为准。
        let given_state =
            state_source.load_given_state(command).map_err(MiFamilyExecutionError::LoadState)?;

        // 在已加载状态上校验 command，并计算 / 合并 before-after changes。
        family
            .validate_against_given_state(command, &given_state)
            .map_err(MiFamilyExecutionError::Business)?;

        let after = family
            .compute_after_changes_unchecked(command, &given_state)
            .map_err(MiFamilyExecutionError::Business)?;

        let changes = F::merge_before_and_after(given_state, after)
            .map_err(MiFamilyExecutionError::Business)?;

        // 将 changes 投影为事件后，按固定顺序执行 outbound 副作用。
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
    use crate::{EntityError, MiStateMachineV2Unchecked};

    #[derive(Debug, Clone)]
    struct StubCommand {
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StubAfter;

    #[derive(Debug, Clone)]
    struct StubChanges {
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    impl ReplayableChanges for StubChanges {
        fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
            if let Ok(mut log) = self.log.lock() {
                log.push("project");
            }
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
        type Command = StubCommand;
        type GivenState = Arc<Mutex<Vec<&'static str>>>;
        type Error = StubBusinessError;
        type AfterChanges = StubAfter;

        fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
            if let Ok(mut log) = cmd.log.lock() {
                log.push("pre_check");
            }
            Ok(())
        }

        fn validate_against_given_state(
            &self,
            _cmd: &Self::Command,
            given_state: &Self::GivenState,
        ) -> Result<(), Self::Error> {
            if let Ok(mut log) = given_state.lock() {
                log.push("validate");
            }
            Ok(())
        }

        fn compute_after_changes_unchecked(
            &self,
            _cmd: &Self::Command,
            given_state: &Self::GivenState,
        ) -> Result<Self::AfterChanges, Self::Error> {
            if let Ok(mut log) = given_state.lock() {
                log.push("compute");
            }
            Ok(StubAfter)
        }
    }

    impl MiStateMachineOwnedV2BeforeAfter for StubFamily {
        type BeforeAfterChanges = StubChanges;

        fn merge_before_and_after(
            given_state: Self::GivenState,
            _after: Self::AfterChanges,
        ) -> Result<Self::BeforeAfterChanges, Self::Error> {
            if let Ok(mut log) = given_state.lock() {
                log.push("merge");
            }
            Ok(StubChanges { log: Arc::clone(&given_state) })
        }
    }

    #[derive(Debug)]
    struct StubOutbound {
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    impl MiFamilyStateSource<StubFamily> for StubOutbound {
        type Error = StubOutboundError;

        fn load_given_state(
            &self,
            cmd: &<StubFamily as MiStateMachineV2Unchecked>::Command,
        ) -> Result<<StubFamily as MiStateMachineV2Unchecked>::GivenState, Self::Error> {
            if let Ok(mut log) = cmd.log.lock() {
                log.push("load_state");
            }
            Ok(Arc::clone(&cmd.log))
        }
    }

    impl MiFamilyStateSink<StubFamily> for StubOutbound {
        type Error = StubOutboundError;

        fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            assert!(events.is_empty());
            if let Ok(mut log) = self.log.lock() {
                log.push("persist");
            }
            Ok(())
        }

        fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            assert!(events.is_empty());
            if let Ok(mut log) = self.log.lock() {
                log.push("replay");
            }
            Ok(())
        }

        fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            assert!(events.is_empty());
            if let Ok(mut log) = self.log.lock() {
                log.push("publish");
            }
            Ok(())
        }
    }

    #[test]
    fn mi_family_executor_runs_fixed_runtime_sequence() -> Result<(), String> {
        let log = Arc::new(Mutex::new(Vec::new()));
        let command = StubCommand { log: Arc::clone(&log) };
        let executor = MiStateMachineFamilyExecutor;
        let outbound = StubOutbound { log: Arc::clone(&log) };

        executor
            .execute::<StubFamily, _, _>(&StubFamily, &command, &outbound, &outbound)
            .map_err(|err| format!("executor failed: {err:?}"))?;
        let actual = log.lock().map_err(|err| format!("log mutex poisoned: {err}"))?;

        assert_eq!(
            *actual,
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
        Ok(())
    }
}
