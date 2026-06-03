use diff::EntityReplayableEvent;

use super::trace::{
    saturating_u64, trace_field_or_placeholder, trace_phase, use_case_command_summary,
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

        tracing::trace!(
            phase = "total",
            operation = "executor.execute",
            status = "start",
            "command use case execution started"
        );

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
                tracing::trace!(
                    call_stack = true,
                    layer = "workflow",
                    component = "command_use_case_execute",
                    operation = "execute",
                    phase = "total",
                    request_command_summary = %command_summary,
                    request_role = %role,
                    request_party_id = party_id.as_deref().unwrap_or("-"),
                    request_outbound = %outbound_type,
                    response_result = "ok",
                    response_domain_event_count = metrics.domain_event_count as u64,
                    status = "ok",
                    latency_ns = saturating_u64(metrics.total_ns),
                    total_ns = saturating_u64(metrics.total_ns),
                    domain_event_count = metrics.domain_event_count as u64,
                    "command use case execution completed"
                );
                latency_observer.observe_latency(&metrics);
                Ok(events)
            }
            Err(error) => {
                tracing::trace!(
                    call_stack = true,
                    layer = "workflow",
                    component = "command_use_case_execute",
                    operation = "execute",
                    phase = "total",
                    request_command_summary = %command_summary,
                    request_role = %role,
                    request_party_id = party_id.as_deref().unwrap_or("-"),
                    request_outbound = %outbound_type,
                    response_result = "err",
                    status = "err",
                    latency_ns = saturating_u64(total_start.elapsed().as_nanos()),
                    total_ns = saturating_u64(total_start.elapsed().as_nanos()),
                    error_message = %error,
                    "command use case execution failed"
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

#[cfg(test)]
mod tests {
    use std::fs::{self, File, OpenOptions};
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::{fmt, str};

    use tracing::field::{Field, Visit};
    use tracing::{Event, Subscriber};
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::fmt::MakeWriter;
    use tracing_subscriber::layer::{Context, Layer};
    use tracing_subscriber::prelude::*;

    use super::CommandUseCaseExecutor2;
    use crate::EntityReplayableEvent;
    use crate::use_case_def2::{
        CommandEnvelope, CommandMeta, CommandUseCase2, CommandUseCaseExecutionError,
        CommandUseCaseOutbound, IssuedByParty,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StubCommand {
        account_id: String,
        symbol: String,
        quantity: u64,
    }

    impl IssuedByParty for StubCommand {
        fn party_id(&self) -> Option<&str> {
            Some(self.account_id.as_str())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum StubError {
        RiskRejected(&'static str),
    }

    impl fmt::Display for StubError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::RiskRejected(reason) => write!(f, "risk rejected: {reason}"),
            }
        }
    }

    impl std::error::Error for StubError {}

    fn stub_event(sequence: u64) -> EntityReplayableEvent {
        EntityReplayableEvent::new_created(0, sequence, sequence as i64 + 1, 1)
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct StubUseCase;

    impl CommandUseCase2 for StubUseCase {
        type Command = StubCommand;
        type GivenState = u64;
        type Error = StubError;

        fn role(&self) -> &'static str {
            "StubRole"
        }

        fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
            Ok(())
        }

        fn validate_against_state(
            &self,
            cmd: &Self::Command,
            _state: &Self::GivenState,
        ) -> Result<(), Self::Error> {
            if cmd.symbol == "REJECTED" {
                Err(StubError::RiskRejected("symbol disabled"))
            } else {
                Ok(())
            }
        }

        fn compute_replayable_events(
            &self,
            _cmd: &Self::Command,
            state: Self::GivenState,
        ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
            Ok((0..state).map(stub_event).collect())
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct StubOutbound;

    impl CommandUseCaseOutbound for StubOutbound {
        type Command = StubCommand;
        type State = u64;
        type Error = StubError;

        fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
            Ok(cmd.quantity)
        }

        fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RecordedTraceEvent {
        phase: Option<String>,
        operation: Option<String>,
        status: Option<String>,
    }

    #[derive(Default)]
    struct TraceFieldVisitor {
        phase: Option<String>,
        operation: Option<String>,
        status: Option<String>,
    }

    impl Visit for TraceFieldVisitor {
        fn record_str(&mut self, field: &Field, value: &str) {
            match field.name() {
                "phase" => self.phase = Some(value.to_string()),
                "operation" => self.operation = Some(value.to_string()),
                "status" => self.status = Some(value.to_string()),
                _ => {}
            }
        }

        fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
            let value = format!("{value:?}").trim_matches('"').to_string();
            match field.name() {
                "phase" if self.phase.is_none() => self.phase = Some(value),
                "operation" if self.operation.is_none() => self.operation = Some(value),
                "status" if self.status.is_none() => self.status = Some(value),
                _ => {}
            }
        }
    }

    #[derive(Clone, Default)]
    struct RecordingLayer {
        events: Arc<Mutex<Vec<RecordedTraceEvent>>>,
    }

    impl RecordingLayer {
        fn operations_by_status(&self, status: &str) -> Vec<String> {
            self.events
                .lock()
                .unwrap()
                .iter()
                .filter(|event| event.status.as_deref() == Some(status))
                .filter_map(|event| event.operation.clone())
                .collect()
        }

        fn statuses_for_operation(&self, operation: &str) -> Vec<String> {
            self.events
                .lock()
                .unwrap()
                .iter()
                .filter(|event| event.operation.as_deref() == Some(operation))
                .filter_map(|event| event.status.clone())
                .collect()
        }
    }

    impl<S> Layer<S> for RecordingLayer
    where
        S: Subscriber,
    {
        fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
            let mut visitor = TraceFieldVisitor::default();
            event.record(&mut visitor);
            self.events.lock().unwrap().push(RecordedTraceEvent {
                phase: visitor.phase,
                operation: visitor.operation,
                status: visitor.status,
            });
        }
    }

    #[derive(Clone)]
    struct SharedFileWriter {
        output: Arc<Mutex<SharedFileOutput>>,
    }

    struct SharedFileOutput {
        file: File,
        in_escape_sequence: bool,
    }

    impl SharedFileWriter {
        fn new(path: &Path) -> io::Result<Self> {
            let file = OpenOptions::new().create(true).truncate(true).write(true).open(path)?;
            Ok(Self {
                output: Arc::new(Mutex::new(SharedFileOutput { file, in_escape_sequence: false })),
            })
        }
    }

    struct SharedFileGuard<'a>(MutexGuard<'a, SharedFileOutput>);

    impl Write for SharedFileGuard<'_> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let mut filtered = Vec::with_capacity(buf.len());

            for byte in buf {
                if self.0.in_escape_sequence {
                    if (0x40..=0x7e).contains(byte) {
                        self.0.in_escape_sequence = false;
                    }
                    continue;
                }

                if *byte == 0x1b {
                    self.0.in_escape_sequence = true;
                    continue;
                }

                filtered.push(*byte);
            }

            self.0.file.write_all(&filtered)?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.0.file.flush()
        }
    }

    impl<'a> MakeWriter<'a> for SharedFileWriter {
        type Writer = SharedFileGuard<'a>;

        fn make_writer(&'a self) -> Self::Writer {
            SharedFileGuard(self.output.lock().unwrap())
        }
    }

    fn workspace_target_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(3)
            .expect("cmd_handler should live under the workspace root")
            .join("target")
    }

    fn test_log_path(test_name: &str) -> PathBuf {
        let dir = workspace_target_dir().join("test-logs").join("cmd_handler");
        fs::create_dir_all(&dir).unwrap();
        dir.join(format!("{test_name}.log"))
    }

    fn build_test_subscriber(
        test_name: &str,
        recording: RecordingLayer,
    ) -> (impl Subscriber + Send + Sync, PathBuf) {
        let log_path = test_log_path(test_name);
        let file_writer = SharedFileWriter::new(&log_path).unwrap();
        let subscriber = tracing_subscriber::registry()
            .with(LevelFilter::TRACE)
            .with(recording)
            .with(tracing_subscriber::fmt::layer().compact().with_writer(std::io::stderr))
            .with(
                tracing_subscriber::fmt::layer()
                    .compact()
                    .with_ansi(false)
                    .with_writer(file_writer),
            );
        (subscriber, log_path)
    }

    fn read_log(path: &Path) -> String {
        str::from_utf8(&fs::read(path).unwrap()).unwrap().to_string()
    }

    #[test]
    fn execute_traces_load_and_pipeline_phases() {
        let executor = CommandUseCaseExecutor2;
        let recording = RecordingLayer::default();
        let (subscriber, log_path) =
            build_test_subscriber("execute_traces_load_and_pipeline_phases", recording.clone());

        eprintln!("trace log file: {}", log_path.display());
        tracing::subscriber::with_default(subscriber, || {
            let use_case = StubUseCase;
            let outbound = StubOutbound;
            let command = CommandEnvelope {
                meta: CommandMeta {
                    trace_id: Some("trace-spot-001".into()),
                    command_id: Some("cmd-spot-001".into()),
                },
                command: StubCommand {
                    account_id: "acct-007".into(),
                    symbol: "BTCUSDT".into(),
                    quantity: 1,
                },
            };

            let events = executor.execute(&use_case, command, &outbound, &()).unwrap();
            assert_eq!(events.len(), 1);
        });

        let ok_operations = recording.operations_by_status("ok");
        let log = read_log(&log_path);

        assert!(ok_operations.iter().any(|op| op == "outbound.load_state(&command)"));
        assert!(ok_operations.iter().any(|op| op == "outbound.persist(&events)"));
        assert!(ok_operations.iter().any(|op| op == "outbound.replay(&events)"));
        assert!(ok_operations.iter().any(|op| op == "outbound.publish(&events)"));
        assert!(log.contains("trace_id=\"trace-spot-001\""));
        assert!(log.contains("command_id=\"cmd-spot-001\""));
        assert!(log.contains("party_id=\"acct-007\""));
    }

    #[test]
    fn execute_traces_error_details_for_failed_phase() {
        let executor = CommandUseCaseExecutor2;
        let recording = RecordingLayer::default();
        let (subscriber, log_path) = build_test_subscriber(
            "execute_traces_error_details_for_failed_phase",
            recording.clone(),
        );

        eprintln!("trace log file: {}", log_path.display());
        tracing::subscriber::with_default(subscriber, || {
            let use_case = StubUseCase;
            let outbound = StubOutbound;
            let command = CommandEnvelope {
                meta: CommandMeta {
                    trace_id: Some("trace-spot-err".into()),
                    command_id: Some("cmd-spot-err".into()),
                },
                command: StubCommand {
                    account_id: "acct-999".into(),
                    symbol: "REJECTED".into(),
                    quantity: 1,
                },
            };

            let error = executor.execute(&use_case, command, &outbound, &()).unwrap_err();
            assert!(matches!(
                error,
                CommandUseCaseExecutionError::Business(StubError::RiskRejected("symbol disabled"))
            ));
        });

        let validate_statuses =
            recording.statuses_for_operation("workflow.validate_against_state(&command, &state)");
        let total_statuses = recording.statuses_for_operation("execute");
        let log = read_log(&log_path);

        assert!(validate_statuses.iter().any(|status| status == "err"));
        assert!(total_statuses.iter().any(|status| status == "err"));
        assert!(log.contains("error_message=risk rejected: symbol disabled"));
        assert!(log.contains("party_id=\"acct-999\""));
    }
}
