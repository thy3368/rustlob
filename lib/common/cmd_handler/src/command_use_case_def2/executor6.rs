use super::trace::{
    trace_command_use_case_completed, trace_command_use_case_failed,
    trace_command_use_case_started, trace_field_or_placeholder, trace_phase,
    use_case_command_summary,
};
use super::{
    CommandEnvelope, CommandMeta, CommandUseCase6, CommandUseCaseExecutionError,
    CommandUseCaseOutbound, CommandUseCaseOutboundPhase, EventProjectError, MainMiChanges,
    MainMiStatefulChanges, ObserveHandlerLatency, ReplayableChanges, UseCaseChanges,
    UseCaseReplyMapper,
};
use crate::HandlerLatencyMetrics;
use crate::command_use_case_def2::IssuedByParty;

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandUseCaseExecutor6;

impl CommandUseCaseExecutor6 {
    fn trace_span<U, O>(use_case: &U, meta: &CommandMeta, command: &U::Command) -> tracing::Span
    where
        U: CommandUseCase6,
        O: ?Sized
            + Send
            + Sync
            + CommandUseCaseOutbound<Command = U::Command, State = U::GivenState>,
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
            main_mi_name = use_case.main_mi_name(),
            main_mi_identity_field = use_case.main_mi_identity_field(),
            main_mi_state_field = use_case.main_mi_state_field(),
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
        use_case: &U,
        changes: &U::Changes,
    ) -> Result<(), EventProjectError>
    where
        U: CommandUseCase6,
    {
        if use_case.main_mi_name().is_empty() {
            return Err(EventProjectError::Custom(
                "CommandUseCase6 main_mi_name must not be empty".to_string(),
            ));
        }
        if use_case.main_mi_identity_field().is_empty() {
            return Err(EventProjectError::Custom(
                "CommandUseCase6 main_mi_identity_field must not be empty".to_string(),
            ));
        }

        changes.main_mi_truth().map(|_| ()).ok_or_else(|| {
            EventProjectError::Custom(format!(
                "missing authoritative main MI truth for {} ({})",
                use_case.main_mi_name(),
                use_case.main_mi_identity_field()
            ))
        })
    }

    fn validate_main_mi_state<U>(
        use_case: &U,
        changes: &U::Changes,
    ) -> Result<(), EventProjectError>
    where
        U: CommandUseCase6,
    {
        if use_case.main_mi_state_field().is_empty() {
            return Err(EventProjectError::Custom(
                "CommandUseCase6 main_mi_state_field must not be empty".to_string(),
            ));
        }

        let command_kind = changes.command_kind();
        if command_kind.is_empty() {
            return Err(EventProjectError::Custom(format!(
                "missing command_kind for {} ({})",
                use_case.main_mi_name(),
                use_case.main_mi_state_field()
            )));
        }

        changes.main_mi_current_state().map(|_| ()).ok_or_else(|| {
            EventProjectError::Custom(format!(
                "missing main MI current state for {}.{} on command {}",
                use_case.main_mi_name(),
                use_case.main_mi_state_field(),
                command_kind
            ))
        })
    }

    /// V6 标准编排：
    /// 1. pre_check_command
    /// 2. load_state
    /// 3. validate_against_state
    /// 4. compute_changes
    /// 5. validate_main_mi_truth
    /// 6. validate_main_mi_state
    /// 7. changes.to_replayable_events()
    /// 8. persist(events)
    /// 9. replay(events)
    /// 10. publish(events)
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
            + CommandUseCaseOutbound<Command = U::Command, State = U::GivenState>,
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
                        "-",
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
                        "-",
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
            + CommandUseCaseOutbound<Command = U::Command, State = U::GivenState>,
        O: ?Sized + ObserveHandlerLatency,
        M: UseCaseReplyMapper,
        OB::Error: 'static,
    {
        let result = self.execute(use_case, envelope, outbound, latency_observer)?;
        Ok(mapper.map(result.events))
    }
}
