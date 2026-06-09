use common_entity::EntityReplayableEvent;

use super::trace::{
    trace_command_use_case_completed, trace_command_use_case_failed,
    trace_command_use_case_started, trace_field_or_placeholder, trace_phase,
    use_case_command_summary,
};
use super::{
    CommandEnvelope, CommandMeta, CommandUseCase2, CommandUseCaseOutbound,
    CommandUseCaseOutboundPhase, IssuedByParty, ObserveHandlerLatency, UseCaseReplyMapper,
};
use crate::HandlerLatencyMetrics;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CommandUseCaseExecutionError<BusinessError, OutboundError>
where
    BusinessError: std::error::Error + 'static,
    OutboundError: std::error::Error + 'static,
{
    #[error(transparent)]
    Business(#[from] BusinessError),
    #[error("outbound {phase} failed: {source}")]
    Outbound {
        phase: CommandUseCaseOutboundPhase,
        #[source]
        source: OutboundError,
    },
}

impl<BusinessError, OutboundError> CommandUseCaseExecutionError<BusinessError, OutboundError>
where
    BusinessError: std::error::Error + 'static,
    OutboundError: std::error::Error + 'static,
{
    pub fn outbound(
        phase: CommandUseCaseOutboundPhase,
        source: OutboundError,
    ) -> CommandUseCaseExecutionError<BusinessError, OutboundError> {
        Self::Outbound { phase, source }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandUseCaseExecutor2;

impl CommandUseCaseExecutor2 {
    fn trace_span<U, O>(use_case: &U, meta: &CommandMeta, command: &U::Command) -> tracing::Span
    where
        U: CommandUseCase2,
        O: ?Sized
            + Send
            + Sync
            + CommandUseCaseOutbound<Command = U::Command, State = U::GivenState>,
        O::Error: 'static,
    {
        tracing::span!(
            tracing::Level::TRACE,
            "command_use_case_execute",
            use_case = std::any::type_name::<U>(),
            command_summary = ?use_case_command_summary::<U>(),
            role = use_case.role(),
            command_type = std::any::type_name::<U::Command>(),
            business_error_type = std::any::type_name::<U::Error>(),
            outbound_error_type = std::any::type_name::<O::Error>(),
            outbound = std::any::type_name::<O>(),
            trace_id = trace_field_or_placeholder(meta.trace_id.as_deref()),
            command_id = trace_field_or_placeholder(meta.command_id.as_deref()),
            party_id = trace_field_or_placeholder(command.party_id()),
        )
    }

    /// 执行命令型 use case 的标准编排：
    /// 1. 先做 command 级别的快速预检查
    /// 2. 通过外部 load port 加载当前 given state
    /// 3. 基于 state 做业务校验
    /// 4. 生成领域事件
    /// 5. 依次持久化、回放、发布领域事件
    /// 6. 最后把整条链路的 latency 交给外部 observer
    ///
    /// 这里故意不把加载和 metrics 观察放进 use case，
    /// 让核心 use case 只保留业务规则本身。
    pub fn execute<U, OB, O>(
        &self,
        use_case: &U,
        envelope: CommandEnvelope<U::Command>,
        outbound: &OB,
        latency_observer: &O,
    ) -> Result<Vec<EntityReplayableEvent>, CommandUseCaseExecutionError<U::Error, OB::Error>>
    where
        U: CommandUseCase2,
        OB: ?Sized
            + Send
            + Sync
            + CommandUseCaseOutbound<Command = U::Command, State = U::GivenState>,
        O: ?Sized + ObserveHandlerLatency,
        OB::Error: 'static,
    {
        use minstant::Instant;

        let CommandEnvelope { meta, command } = envelope;
        let command_summary = use_case_command_summary::<U>();
        let role = use_case.role().to_string();
        let party_id = command.party_id().map(str::to_string);
        let outbound_type = std::any::type_name::<OB>().to_string();
        let total_start = Instant::now();
        let execution_span = Self::trace_span::<U, OB>(use_case, &meta, &command);
        let _execution_guard = execution_span.enter();

        trace_command_use_case_started!();

        let execution = (|| -> Result<
            (Vec<EntityReplayableEvent>, HandlerLatencyMetrics),
            CommandUseCaseExecutionError<U::Error, OB::Error>,
        > {
                let ((), pre_check_ns) = trace_phase(
                    "pre_check",
                    "workflow.pre_check_command(&command)",
                    || use_case.pre_check_command(&command),
                )
                .map_err(CommandUseCaseExecutionError::Business)?;
                let (state, load_state_ns) = trace_phase(
                    "load_state",
                    "outbound.load_state(&command)",
                    || outbound.load_state(&command),
                )
                .map_err(|error| {
                    CommandUseCaseExecutionError::outbound(
                        CommandUseCaseOutboundPhase::LoadState,
                        error,
                    )
                })?;
                let ((), validate_in_lock_ns) = trace_phase(
                    "validate_against_state",
                    "workflow.validate_against_state(&command, &state)",
                    || use_case.validate_against_state(&command, &state),
                )
                .map_err(CommandUseCaseExecutionError::Business)?;
                let (events, apply_changes_ns) = trace_phase(
                    "compute_replayable_events",
                    "workflow.compute_replayable_events(&command, state)",
                    || use_case.compute_replayable_events(&command, state),
                )
                .map_err(CommandUseCaseExecutionError::Business)?;
                let domain_event_count = events.len();

                let ((), persist_domain_events_ns) = trace_phase(
                    "persist",
                    "outbound.persist(&events)",
                    || outbound.persist(&events),
                )
                .map_err(|error| {
                    CommandUseCaseExecutionError::outbound(
                        CommandUseCaseOutboundPhase::Persist,
                        error,
                    )
                })?;
                let ((), replay_domain_events_ns) = trace_phase(
                    "replay",
                    "outbound.replay(&events)",
                    || outbound.replay(&events),
                )
                .map_err(|error| {
                    CommandUseCaseExecutionError::outbound(
                        CommandUseCaseOutboundPhase::Replay,
                        error,
                    )
                })?;
                let ((), publish_domain_events_ns) = trace_phase(
                    "publish",
                    "outbound.publish(&events)",
                    || outbound.publish(&events),
                )
                .map_err(|error| {
                    CommandUseCaseExecutionError::outbound(
                        CommandUseCaseOutboundPhase::Publish,
                        error,
                    )
                })?;

                let metrics = HandlerLatencyMetrics {
                    total_ns: total_start.elapsed().as_nanos(),
                    pre_check_ns,
                    load_state_ns,
                    validate_in_lock_ns,
                    apply_changes_ns,
                    persist_domain_events_ns,
                    replay_domain_events_ns,
                    publish_domain_events_ns,
                    domain_event_count,
                };

                Ok((events, metrics))
            })();

        match execution {
            Ok((events, metrics)) => {
                trace_command_use_case_completed!(
                    command_summary,
                    role,
                    party_id,
                    outbound_type,
                    metrics
                );
                latency_observer.observe_latency(&metrics);
                Ok(events)
            }
            Err(error) => {
                trace_command_use_case_failed!(
                    command_summary,
                    role,
                    party_id,
                    outbound_type,
                    total_start.elapsed().as_nanos(),
                    error
                );
                Err(error)
            }
        }
    }

    /// 在标准执行编排之后，把领域事件交给外部 reply mapper 转成对外响应。
    pub fn execute_and_map_reply<U, OB, M, O>(
        &self,
        use_case: &U,
        envelope: CommandEnvelope<U::Command>,
        outbound: &OB,
        latency_observer: &O,
        mapper: &M,
    ) -> Result<M::Reply, CommandUseCaseExecutionError<U::Error, OB::Error>>
    where
        U: CommandUseCase2,
        OB: ?Sized
            + Send
            + Sync
            + CommandUseCaseOutbound<Command = U::Command, State = U::GivenState>,
        O: ?Sized + ObserveHandlerLatency,
        M: UseCaseReplyMapper,
        OB::Error: 'static,
    {
        // Executor keeps business/outbound failures typed and lets inbound adapters
        // translate them into transport-specific errors such as HTTP/CLI error models.
        let events = self.execute(use_case, envelope, outbound, latency_observer)?;
        Ok(mapper.map(events))
    }
}
