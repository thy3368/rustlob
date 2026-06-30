use crate::{
    CommandUseCase6, CommandWithGivenState, EntityError, EntityReplayableEvent, IssuedByParty,
    MainMiStatefulChanges, MainMiTruth, ReplayableChanges,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandMeta {
    pub trace_id: Option<String>,
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandEnvelope<C> {
    pub meta: CommandMeta,
    pub command: C,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCaseChanges<C> {
    pub changes: C,
    pub events: Vec<EntityReplayableEvent>,
}

pub type EventProjectError = EntityError;

pub trait UseCaseReplyMapper: Send + Sync {
    type Reply;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply;
}

pub trait CommandUseCaseOutbound: Send + Sync {
    type Command;
    type State;
    type Error: std::error::Error;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error>;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
}

impl<T> CommandUseCaseOutbound for &T
where
    T: ?Sized + CommandUseCaseOutbound,
{
    type Command = T::Command;
    type State = T::State;
    type Error = T::Error;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        (*self).load_state(cmd)
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).persist(events)
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).replay(events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).publish(events)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandUseCaseOutboundPhase {
    LoadState,
    Persist,
    Replay,
    Publish,
}

impl std::fmt::Display for CommandUseCaseOutboundPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadState => f.write_str("load_state"),
            Self::Persist => f.write_str("persist"),
            Self::Replay => f.write_str("replay"),
            Self::Publish => f.write_str("publish"),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CommandUseCaseExecutionError<BusinessError, OutboundError>
where
    BusinessError: std::error::Error + 'static,
    OutboundError: std::error::Error + 'static,
{
    #[error(transparent)]
    Business(BusinessError),
    #[error("project replayable events failed: {0}")]
    EventProject(EventProjectError),
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
    pub fn event_project(
        source: EventProjectError,
    ) -> CommandUseCaseExecutionError<BusinessError, OutboundError> {
        Self::EventProject(source)
    }

    pub fn outbound(
        phase: CommandUseCaseOutboundPhase,
        source: OutboundError,
    ) -> CommandUseCaseExecutionError<BusinessError, OutboundError> {
        Self::Outbound { phase, source }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HandlerLatencyMetrics {
    pub total_ns: u128,
    pub pre_check_ns: u128,
    pub load_state_ns: u128,
    pub validate_in_lock_ns: u128,
    pub apply_changes_ns: u128,
    pub persist_domain_events_ns: u128,
    pub replay_domain_events_ns: u128,
    pub publish_domain_events_ns: u128,
    pub domain_event_count: usize,
}

pub trait ObserveHandlerLatency: Send + Sync {
    fn observe_latency(&self, metrics: &HandlerLatencyMetrics);
}

impl ObserveHandlerLatency for () {
    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}

pub(super) fn saturating_u64(value: u128) -> u64 {
    value.min(u64::MAX as u128) as u64
}

pub(super) fn trace_field_or_placeholder(value: Option<&str>) -> &str {
    value.unwrap_or("-")
}

pub(super) fn use_case_command_summary<U>() -> String {
    let type_name = std::any::type_name::<U>();
    let simple_name = type_name.rsplit("::").next().unwrap_or(type_name);
    let base_name = simple_name.strip_suffix("UseCase").unwrap_or(simple_name);
    let mut summary = String::with_capacity(base_name.len() + 8);

    for (index, ch) in base_name.chars().enumerate() {
        if ch.is_uppercase() {
            if index > 0 {
                summary.push('_');
            }
            for lower in ch.to_lowercase() {
                summary.push(lower);
            }
        } else {
            summary.push(ch);
        }
    }

    summary
}

pub(super) fn trace_phase<T, E>(
    phase: &'static str,
    operation: &'static str,
    f: impl FnOnce() -> Result<T, E>,
) -> Result<(T, u128), E>
where
    E: std::fmt::Display,
{
    use minstant::Instant;

    let trace_enabled = tracing::enabled!(tracing::Level::TRACE);
    if trace_enabled {
        tracing::trace!(phase, operation, status = "start", "command use case phase started");
    }
    let start = Instant::now();
    let result = f();
    let elapsed_ns = start.elapsed().as_nanos();

    match result {
        Ok(value) => {
            if trace_enabled {
                tracing::trace!(
                    phase,
                    operation,
                    status = "ok",
                    elapsed_ns = saturating_u64(elapsed_ns),
                    "command use case phase completed"
                );
            }
            Ok((value, elapsed_ns))
        }
        Err(error) => {
            if trace_enabled {
                tracing::trace!(
                    phase,
                    operation,
                    status = "err",
                    elapsed_ns = saturating_u64(elapsed_ns),
                    error_message = %error,
                    "command use case phase failed"
                );
            }
            Err(error)
        }
    }
}

macro_rules! trace_command_use_case_started {
    () => {
        tracing::trace!(
            phase = "total",
            operation = "executor.execute",
            status = "start",
            "command use case execution started"
        );
    };
}

macro_rules! trace_command_use_case_completed {
    ($command_summary:expr, $party_id:expr, $outbound_type:expr, $metrics:expr) => {
        tracing::trace!(
            call_stack = true,
            layer = "workflow",
            component = "command_use_case_execute",
            operation = "execute",
            phase = "total",
            request_command_summary = %$command_summary,
            request_role = "-",
            request_party_id = $party_id.as_deref().unwrap_or("-"),
            request_outbound = %$outbound_type,
            response_result = "ok",
            response_domain_event_count = $metrics.domain_event_count as u64,
            status = "ok",
            latency_ns = $crate::command_use_case_v6_runtime::saturating_u64($metrics.total_ns),
            total_ns = $crate::command_use_case_v6_runtime::saturating_u64($metrics.total_ns),
            domain_event_count = $metrics.domain_event_count as u64,
            "command use case execution completed"
        );
    };
}

macro_rules! trace_command_use_case_failed {
    ($command_summary:expr, $party_id:expr, $outbound_type:expr, $total_elapsed_ns:expr, $error:expr) => {
        tracing::trace!(
            call_stack = true,
            layer = "workflow",
            component = "command_use_case_execute",
            operation = "execute",
            phase = "total",
            request_command_summary = %$command_summary,
            request_role = "-",
            request_party_id = $party_id.as_deref().unwrap_or("-"),
            request_outbound = %$outbound_type,
            response_result = "err",
            status = "err",
            latency_ns = $crate::command_use_case_v6_runtime::saturating_u64($total_elapsed_ns),
            total_ns = $crate::command_use_case_v6_runtime::saturating_u64($total_elapsed_ns),
            error_message = %$error,
            "command use case execution failed"
        );
    };
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandUseCaseExecutor6;

impl CommandUseCaseExecutor6 {
    fn trace_span<U, O>(_use_case: &U, meta: &CommandMeta, command: &U::Command) -> tracing::Span
    where
        U: CommandUseCase6,
        O: ?Sized
            + Send
            + Sync
            + CommandUseCaseOutbound<
                Command = U::Command,
                State = <U::Command as CommandWithGivenState>::GivenState,
            >,
        O::Error: 'static,
    {
        if !tracing::enabled!(tracing::Level::TRACE) {
            return tracing::Span::none();
        }

        tracing::span!(
            tracing::Level::TRACE,
            "command_use_case_execute",
            use_case = std::any::type_name::<U>(),
            command_summary = ?use_case_command_summary::<U>(),
            command_type = std::any::type_name::<U::Command>(),
            business_error_type = std::any::type_name::<U::Error>(),
            changes_type = std::any::type_name::<U::Changes>(),
            outbound_error_type = std::any::type_name::<O::Error>(),
            outbound = std::any::type_name::<O>(),
            trace_id = trace_field_or_placeholder(meta.trace_id.as_deref()),
            command_id = trace_field_or_placeholder(meta.command_id.as_deref()),
            party_id = trace_field_or_placeholder(command.party_id()),
        )
    }

    fn validate_main_mi_truth<U>(
        _use_case: &U,
        changes: &U::Changes,
    ) -> Result<(), EventProjectError>
    where
        U: CommandUseCase6,
    {
        match changes.main_mi_truth() {
            Some(MainMiTruth::Created(_)) | Some(MainMiTruth::Updated(_)) => Ok(()),
            None => Err(EventProjectError::Custom(
                "missing authoritative main MI truth".to_string(),
            )),
        }
    }

    fn validate_main_mi_state<U>(
        _use_case: &U,
        changes: &U::Changes,
    ) -> Result<(), EventProjectError>
    where
        U: CommandUseCase6,
    {
        let command_kind = changes.command_kind();
        if command_kind.is_empty() {
            return Err(EventProjectError::Custom(
                "missing command_kind for main MI stateful changes".to_string(),
            ));
        }

        changes.main_mi_current_state().map(|_| ()).ok_or_else(|| {
            EventProjectError::Custom(format!(
                "missing main MI current state for command {command_kind}"
            ))
        })
    }

    pub fn execute<U, OB, O>(
        &self,
        use_case: &U,
        envelope: CommandEnvelope<U::Command>,
        outbound: &OB,
        latency_observer: &O,
    ) -> Result<UseCaseChanges<U::Changes>, CommandUseCaseExecutionError<U::Error, OB::Error>>
    where
        U: CommandUseCase6,
        OB: ?Sized
            + Send
            + Sync
            + CommandUseCaseOutbound<
                Command = U::Command,
                State = <U::Command as CommandWithGivenState>::GivenState,
            >,
        O: ?Sized + ObserveHandlerLatency,
        OB::Error: 'static,
    {
        use minstant::Instant;

        let CommandEnvelope { meta, command } = envelope;
        let trace_enabled = tracing::enabled!(tracing::Level::TRACE);
        let total_start = Instant::now();
        let execution_span = Self::trace_span::<U, OB>(use_case, &meta, &command);
        let _execution_guard = execution_span.enter();

        if trace_enabled {
            trace_command_use_case_started!();
        }

        let execution = (|| -> Result<
            (UseCaseChanges<U::Changes>, HandlerLatencyMetrics),
            CommandUseCaseExecutionError<U::Error, OB::Error>,
        > {
            let ((), pre_check_ns) = trace_phase(
                "pre_check",
                "workflow.pre_check_command(&command)",
                || use_case.pre_check_command(&command),
            )
            .map_err(CommandUseCaseExecutionError::Business)?;
            let (state, load_state_ns) =
                trace_phase("load_state", "outbound.load_state(&command)", || {
                    outbound.load_state(&command)
                })
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
            let (changes, apply_changes_ns) = trace_phase(
                "compute_changes",
                "workflow.compute_changes(&command, state)",
                || use_case.compute_changes(&command, state),
            )
            .map_err(CommandUseCaseExecutionError::Business)?;
            let ((), main_mi_truth_ns) = trace_phase(
                "validate_main_mi_truth",
                "changes.main_mi_truth()",
                || Self::validate_main_mi_truth(use_case, &changes),
            )
            .map_err(CommandUseCaseExecutionError::event_project)?;
            let ((), main_mi_state_ns) = trace_phase(
                "validate_main_mi_state",
                "changes.main_mi_current_state()",
                || Self::validate_main_mi_state(use_case, &changes),
            )
            .map_err(CommandUseCaseExecutionError::event_project)?;
            let (events, event_project_ns) = trace_phase(
                "project_replayable_events",
                "changes.to_replayable_events()",
                || changes.to_replayable_events(),
            )
            .map_err(CommandUseCaseExecutionError::event_project)?;
            let apply_changes_ns = apply_changes_ns
                .saturating_add(main_mi_truth_ns)
                .saturating_add(main_mi_state_ns)
                .saturating_add(event_project_ns);
            let domain_event_count = events.len();

            let ((), persist_domain_events_ns) =
                trace_phase("persist", "outbound.persist(&events)", || outbound.persist(&events))
                    .map_err(|error| {
                        CommandUseCaseExecutionError::outbound(
                            CommandUseCaseOutboundPhase::Persist,
                            error,
                        )
                    })?;
            let ((), replay_domain_events_ns) =
                trace_phase("replay", "outbound.replay(&events)", || outbound.replay(&events))
                    .map_err(|error| {
                        CommandUseCaseExecutionError::outbound(
                            CommandUseCaseOutboundPhase::Replay,
                            error,
                        )
                    })?;
            let ((), publish_domain_events_ns) =
                trace_phase("publish", "outbound.publish(&events)", || outbound.publish(&events))
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

            Ok((UseCaseChanges { changes, events }, metrics))
        })();

        match execution {
            Ok((result, metrics)) => {
                if trace_enabled {
                    trace_command_use_case_completed!(
                        use_case_command_summary::<U>(),
                        command.party_id(),
                        std::any::type_name::<OB>(),
                        metrics
                    );
                }
                latency_observer.observe_latency(&metrics);
                Ok(result)
            }
            Err(error) => {
                if trace_enabled {
                    trace_command_use_case_failed!(
                        use_case_command_summary::<U>(),
                        command.party_id(),
                        std::any::type_name::<OB>(),
                        total_start.elapsed().as_nanos(),
                        error
                    );
                }
                Err(error)
            }
        }
    }

    pub fn execute_and_map_reply<U, OB, M, O>(
        &self,
        use_case: &U,
        envelope: CommandEnvelope<U::Command>,
        outbound: &OB,
        latency_observer: &O,
        mapper: &M,
    ) -> Result<M::Reply, CommandUseCaseExecutionError<U::Error, OB::Error>>
    where
        U: CommandUseCase6,
        OB: ?Sized
            + Send
            + Sync
            + CommandUseCaseOutbound<
                Command = U::Command,
                State = <U::Command as CommandWithGivenState>::GivenState,
            >,
        O: ?Sized + ObserveHandlerLatency,
        M: UseCaseReplyMapper,
        OB::Error: 'static,
    {
        let result = self.execute(use_case, envelope, outbound, latency_observer)?;
        Ok(mapper.map(result.events))
    }
}
