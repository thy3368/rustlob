use std::collections::HashSet;

use crate::entity_v2::shard::LogAttr;
use crate::{
    EntityError, ReplayableChanges, ReplayableDomainEvent, SequencedDomainEvent, StateDiff,
};

/// 已在进入 executor 前完成定序的批命令。
pub trait PresequencedCommand: LogAttr {
    fn contains_event_metadata(&self, event: &ReplayableDomainEvent) -> bool;
}

/// 多聚合 MI state-machine family 的运行时编排器。
///
/// 该类型只固定 adapter / outbound 之间的执行顺序，不承载业务规则。
#[derive(Debug, Clone, Copy, Default)]
pub struct UseCaseExecutor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult<C> {
    pub changes: C,
    pub events: Vec<ReplayableDomainEvent>,
}

/// 同时返回 replay 事实与业务订阅事实的执行结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainEventExecutionResult<C, E> {
    pub changes: C,
    pub replay_events: Vec<ReplayableDomainEvent>,
    pub domain_events: Vec<SequencedDomainEvent<E>>,
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
    PublishDomainEvent(OE),
    DuplicateEventKey {
        lsn: u64,
        entity_type: u8,
        entity_id: u64,
    },
}

/// 从 family changes 读取已按 transition 顺序编号的领域事件。
pub trait DomainEventChanges {
    type DomainEvent;

    fn domain_events(&self) -> &[SequencedDomainEvent<Self::DomainEvent>];
}

/// 领域事件独立 outbound port；wire format 由 adapter 决定。
pub trait DomainEventSink<E>: Send + Sync {
    type Error: std::error::Error;

    fn publish_domain_events(&self, events: &[SequencedDomainEvent<E>]) -> Result<(), Self::Error>;
}

/// MI family runtime 所需的 authoritative given state source port。
pub trait StateSource<F>: Send + Sync
where
    F: StateDiff,
{
    type Error: std::error::Error;

    fn load_given_state(&self, cmd: &F::Command) -> Result<F::StateGiven, Self::Error>;
}

/// MI family runtime 所需的事件副作用 outbound port。
pub trait StateSink<F>: Send + Sync
where
    F: StateDiff,
{
    type Error: std::error::Error;

    fn persist(&self, events: &[ReplayableDomainEvent]) -> Result<(), Self::Error>;
    fn replay(&self, events: &[ReplayableDomainEvent]) -> Result<(), Self::Error>;
    fn publish(&self, events: &[ReplayableDomainEvent]) -> Result<(), Self::Error>;
}

impl UseCaseExecutor {
    /// 执行带领域事件的 use case，并保持 replay 与领域发布链相互独立。
    ///
    /// 固定副作用顺序：persist replay -> replay -> publish replay -> publish domain。
    pub fn execute_with_domain_events<F, SS, OB, DS>(
        &self,
        family: &F,
        command: &F::Command,
        state_source: &SS,
        state_sink: &OB,
        domain_sink: &DS,
    ) -> Result<
        DomainEventExecutionResult<F::StateDiff, <F::StateDiff as DomainEventChanges>::DomainEvent>,
        ExecutionError<F::Error, OB::Error>,
    >
    where
        F: StateDiff,
        F::StateDiff: DomainEventChanges,
        <F::StateDiff as DomainEventChanges>::DomainEvent: Clone,
        SS: StateSource<F, Error = OB::Error>,
        OB: StateSink<F>,
        DS: DomainEventSink<<F::StateDiff as DomainEventChanges>::DomainEvent, Error = OB::Error>,
    {
        family
            .check_command(command)
            .map_err(ExecutionError::Business)?;
        let given_state = state_source
            .load_given_state(command)
            .map_err(ExecutionError::LoadState)?;
        let changes = family
            .compute_state_diff(command, given_state)
            .map_err(ExecutionError::Business)?;
        let mut replay_events = changes
            .to_replayable_events()
            .map_err(ExecutionError::ProjectEvents)?;
        for event in &mut replay_events {
            event.lsn = command.lsn();
            event.logical_ts = command.logical_ts();
        }
        reject_duplicate_replay_keys(&replay_events)?;
        let domain_events = changes.domain_events().to_vec();

        state_sink
            .persist(&replay_events)
            .map_err(ExecutionError::Persist)?;
        state_sink
            .replay(&replay_events)
            .map_err(ExecutionError::Replay)?;
        state_sink
            .publish(&replay_events)
            .map_err(ExecutionError::Publish)?;
        domain_sink
            .publish_domain_events(&domain_events)
            .map_err(ExecutionError::PublishDomainEvent)?;

        Ok(DomainEventExecutionResult {
            changes,
            replay_events,
            domain_events,
        })
    }

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
        F: StateDiff,
        SS: StateSource<F, Error = OB::Error>,
        OB: StateSink<F>,
    {
        family
            .check_command(command)
            .map_err(ExecutionError::Business)?;
        // 加载 authoritative given state，后续业务校验与计算都以该状态为准。
        let given_state = state_source
            .load_given_state(command)
            .map_err(ExecutionError::LoadState)?;

        let changes = family
            .compute_state_diff(command, given_state)
            .map_err(ExecutionError::Business)?;

        // 将 changes 投影为事件后，按固定顺序执行 outbound 副作用。
        let mut events = changes
            .to_replayable_events()
            .map_err(ExecutionError::ProjectEvents)?;
        for event in &mut events {
            event.lsn = command.lsn();
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

        state_sink
            .persist(&events)
            .map_err(ExecutionError::Persist)?;
        state_sink.replay(&events).map_err(ExecutionError::Replay)?;
        state_sink
            .publish(&events)
            .map_err(ExecutionError::Publish)?;

        Ok(ExecutionResult { changes, events })
    }

    /// 执行已定序批命令，保留 changes 投影出的逐项事件元数据。
    pub fn execute_presequenced<F, SS, OB>(
        &self,
        family: &F,
        command: &F::Command,
        state_source: &SS,
        state_sink: &OB,
    ) -> ExecutionOutcome<F::StateDiff, F::Error, OB::Error>
    where
        F: StateDiff,
        F::Command: PresequencedCommand,
        SS: StateSource<F, Error = OB::Error>,
        OB: StateSink<F>,
    {
        family
            .check_command(command)
            .map_err(ExecutionError::Business)?;
        let given_state = state_source
            .load_given_state(command)
            .map_err(ExecutionError::LoadState)?;
        let changes = family
            .compute_state_diff(command, given_state)
            .map_err(ExecutionError::Business)?;
        let events = changes
            .to_replayable_events()
            .map_err(ExecutionError::ProjectEvents)?;
        let mut keys = HashSet::with_capacity(events.len());
        for event in &events {
            if !command.contains_event_metadata(event) {
                return Err(ExecutionError::ProjectEvents(EntityError::Custom(
                    "event metadata is not present in the presequenced command".to_owned(),
                )));
            }
            if !keys.insert((event.lsn, event.entity_type, event.entity_id)) {
                return Err(ExecutionError::ProjectEvents(EntityError::Custom(
                    "duplicate event key in presequenced command".to_owned(),
                )));
            }
        }
        state_sink
            .persist(&events)
            .map_err(ExecutionError::Persist)?;
        state_sink.replay(&events).map_err(ExecutionError::Replay)?;
        state_sink
            .publish(&events)
            .map_err(ExecutionError::Publish)?;
        Ok(ExecutionResult { changes, events })
    }
}

fn reject_duplicate_replay_keys<BE, OE>(
    events: &[ReplayableDomainEvent],
) -> Result<(), ExecutionError<BE, OE>> {
    let mut keys = HashSet::with_capacity(events.len());
    for event in events {
        if !keys.insert((event.lsn, event.entity_type, event.entity_id)) {
            return Err(ExecutionError::ProjectEvents(EntityError::Custom(format!(
                "duplicate event key ({}, {}, {})",
                event.lsn, event.entity_type, event.entity_id
            ))));
        }
    }
    Ok(())
}
