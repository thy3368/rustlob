use std::collections::HashSet;

use crate::use_case::log_entry::LogEntryMetadata;
use crate::{EntityError, EntityReplayableEvent, ReplayableChanges, StateMachineOwnedV2Diff};

/// 多聚合 MI state-machine family 的运行时编排器。
///
/// 该类型只固定 adapter / outbound 之间的执行顺序，不承载业务规则。
#[derive(Debug, Clone, Copy, Default)]
pub struct StateMachineExecutor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult<C> {
    pub changes: C,
    pub events: Vec<EntityReplayableEvent>,
}

pub type ExecutionOutcome<C, BE, OE> = Result<ExecutionResult<C>, ExecutionError<BE, OE>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionError<BE, OE> {
    Business(BE),
    ProjectEvents(EntityError),
    LoadState(OE),
    Persist(OE),
    Replay(OE),
    Publish(OE),
    DuplicateEventKey { lsn: u64, entity_type: u8, entity_id: i64 },
}

/// adapter-side request 到 family on_command 的映射约定。
///
/// executor 不直接依赖该 trait；adapter 可用它把 request 转成 on_command 后再执行。
pub trait MiFamilyExecutionSpec<F>
where
    F: StateMachineOwnedV2Diff,
{
    type Request;

    fn command(
        request: &Self::Request,
        lsn: u64,
        logical_ts: u64,
        client_id: u64,
        request_id: u64,
    ) -> F::Command;
}

/// MI family runtime 所需的 authoritative given state source port。
pub trait StateSource<F>: Send + Sync
where
    F: StateMachineOwnedV2Diff,
{
    type Error: std::error::Error;

    fn load_given_state(&self, cmd: &F::Command) -> Result<F::StateGiven, Self::Error>;
}

/// MI family runtime 所需的事件副作用 outbound port。
pub trait StateSink<F>: Send + Sync
where
    F: StateMachineOwnedV2Diff,
{
    type Error: std::error::Error;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
}

impl StateMachineExecutor {
    /// 执行一个 MI family use case 的运行时编排。
    ///
    /// adapter 应在调用前完成 request 到 on_command 的转换。state source 基于 on_command 加载
    /// authoritative given state，后续业务校验与计算都只读取 on_command 和 owned given state。
    ///
    /// 固定执行顺序为：pre-check -> load state -> validate -> compute ->
    /// merge -> project events -> persist -> replay -> publish。该函数只负
    /// 责顺序编排和错误映射，不承载业务规则。
    ///
    /// family 返回的业务错误映射为 [`ExecutionError::Business`]，
    /// 事件投影错误映射为 [`ExecutionError::ProjectEvents`]，
    /// outbound 端错误按发生阶段分别映射为 load / persist / replay /
    /// publish 对应的执行错误。
    pub fn execute<F, SS, OB>(
        &self,
        family: &F,
        command: &F::Command,
        state_source: &SS,
        state_sink: &OB,
    ) -> ExecutionOutcome<F::StateDiff, F::Error, OB::Error>
    where
        F: StateMachineOwnedV2Diff,
        SS: StateSource<F, Error = OB::Error>,
        OB: StateSink<F>,
    {
        family.check_command(command).map_err(ExecutionError::Business)?;
        // 加载 authoritative given state，后续业务校验与计算都以该状态为准。
        let given_state =
            state_source.load_given_state(command).map_err(ExecutionError::LoadState)?;

        let changes =
            family.compute_state_diff(command, given_state).map_err(ExecutionError::Business)?;

        // 将 changes 投影为事件后，按固定顺序执行 outbound 副作用。
        let mut events = changes.to_replayable_events().map_err(ExecutionError::ProjectEvents)?;
        for event in &mut events {
            event.lsn = command.lsn();
            event.new_version = event.lsn;
            event.logical_ts = command.logical_ts();
        }
        let mut keys = HashSet::with_capacity(events.len());
        for event in &events {
            if !keys.insert((event.lsn, event.entity_type, event.entity_id)) {
                return Err(ExecutionError::ProjectEvents(EntityError::Custom(format!(
                    "duplicate event key ({}, {}, {})",
                    event.lsn, event.entity_type, event.entity_id
                ))));
            }
        }

        state_sink.persist(&events).map_err(ExecutionError::Persist)?;
        state_sink.replay(&events).map_err(ExecutionError::Replay)?;
        state_sink.publish(&events).map_err(ExecutionError::Publish)?;

        Ok(ExecutionResult { changes, events })
    }
}
