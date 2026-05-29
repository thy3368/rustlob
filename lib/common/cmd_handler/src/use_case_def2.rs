use crate::{HandlerLatencyMetrics, TraceableEventSet};

fn saturating_u64(value: u128) -> u64 {
    value.min(u64::MAX as u128) as u64
}

fn trace_field_or_placeholder(value: Option<&str>) -> &str {
    value.unwrap_or("-")
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandMeta {
    /// Tracing correlation id for spans/logs across retries and service hops.
    /// This is only for observability and troubleshooting, not for business idempotency.
    pub trace_id: Option<String>,
    /// Stable business command identity.
    /// Use this as the primary idempotency and deduplication key for the same business command.
    /// Retries of the same command should keep the same command_id.
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandEnvelope<C> {
    pub meta: CommandMeta,
    pub command: C,
}

/// Business actor instance carried by the command.
/// Semantically: `party_id` plays the `role()` of the use case and issues the command.
pub trait IssuedByParty {
    fn party_id(&self) -> Option<&str> {
        None
    }
}

fn trace_phase<T, E>(
    phase: &'static str,
    operation: &'static str,
    format_error: impl Fn(&E) -> Option<String>,
    f: impl FnOnce() -> Result<T, E>,
) -> Result<(T, u128), E> {
    use minstant::Instant;

    tracing::trace!(phase, operation, status = "start", "command use case phase started");
    let start = Instant::now();
    let result = f();
    let elapsed_ns = start.elapsed().as_nanos();

    match result {
        Ok(value) => {
            tracing::trace!(
                phase,
                operation,
                status = "ok",
                elapsed_ns = saturating_u64(elapsed_ns),
                "command use case phase completed"
            );
            Ok((value, elapsed_ns))
        }
        Err(error) => {
            if let Some(error_message) = format_error(&error) {
                tracing::trace!(
                    phase,
                    operation,
                    status = "err",
                    elapsed_ns = saturating_u64(elapsed_ns),
                    error_message = %error_message,
                    "command use case phase failed"
                );
            } else {
                tracing::trace!(
                    phase,
                    operation,
                    status = "err",
                    elapsed_ns = saturating_u64(elapsed_ns),
                    "command use case phase failed"
                );
            }
            Err(error)
        }
    }
}

/// 更贴近 Use Cases（用例）的命令型抽象：
/// 只定义业务输入、业务校验与可重放事件产出。
pub trait CommandUseCase2: Send + Sync {
    /// 对应cqrs的 command
    type Command: IssuedByParty;

    /// 对应clean 架构的 entity , 从数据库/内存/文件等
    type GivenState;

    /// 对应事件溯源的可重放事件
    type ThenTraceableEvents: TraceableEventSet;
    type Error;

    /// 对应四色建模的 role。
    /// 语义上：`party_id()` 标识哪个业务主体，以这个 role 下达当前 command。
    fn role(&self) -> &'static str {
        "UnknownActor用来做权限控制和追溯"
    }

    /// 提供错误的可读摘要，默认空实现。
    fn format_error(&self, _error: &Self::Error) -> Option<String> {
        None
    }

    /// 对command的检查
    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error>;

    /// 对状态较验
    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error>;

    /// 计算可重放事件
    fn gen_traceable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::ThenTraceableEvents, Self::Error>;
}

/// 对外回复映射移出核心 Use Case，交给 Interface Adapters（接口适配器）。
pub trait UseCaseReplyMapper<E>: Send + Sync {
    type Reply;

    fn map(&self, events: E) -> Self::Reply;
}

/// 事件执行管线也从核心 Use Case 中拆出。
pub trait TraceableEventPipeline<E, Err>: Send + Sync {
    fn persist(&self, events: &E) -> Result<(), Err>;

    fn replay(&self, events: &E) -> Result<(), Err>;

    fn publish(&self, events: &E) -> Result<(), Err>;
}

/// 状态加载端口 - 标准化从外部存储加载领域状态
///
/// 由执行编排侧注入，提供统一的状态加载接口。
pub trait LoadState<Cmd, State, Err>: Send + Sync {
    fn load_state(&self, cmd: &Cmd) -> Result<State, Err>;
}

/// latency 观察端口，由执行编排侧注入。
pub trait ObserveHandlerLatency: Send + Sync {
    fn observe_latency(&self, metrics: &HandlerLatencyMetrics);
}

impl ObserveHandlerLatency for () {
    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandUseCaseExecutor2;

impl CommandUseCaseExecutor2 {
    fn trace_span<U, P, L>(
        use_case: &U,
        meta: &CommandMeta,
        command: &U::Command,
    ) -> tracing::Span
    where
        U: CommandUseCase2,
        L: ?Sized + Send + Sync + LoadState<U::Command, U::GivenState, U::Error>,
        P: TraceableEventPipeline<U::ThenTraceableEvents, U::Error>,
    {
        tracing::span!(
            tracing::Level::TRACE,
            "command_use_case_execute",
            use_case = std::any::type_name::<U>(),
            role = use_case.role(),
            command_type = std::any::type_name::<U::Command>(),
            error_type = std::any::type_name::<U::Error>(),
            load_port = std::any::type_name::<L>(),
            pipeline = std::any::type_name::<P>(),
            trace_id = trace_field_or_placeholder(meta.trace_id.as_deref()),
            command_id = trace_field_or_placeholder(meta.command_id.as_deref()),
            party_id = trace_field_or_placeholder(command.party_id()),
        )
    }

    fn execute_inner<U, P, L, O>(
        &self,
        use_case: &U,
        command: U::Command,
        meta: &CommandMeta,
        load_port: &L,
        pipeline: &P,
        latency_observer: &O,
    ) -> Result<U::ThenTraceableEvents, U::Error>
    where
        U: CommandUseCase2,
        L: ?Sized + Send + Sync + LoadState<U::Command, U::GivenState, U::Error>,
        P: TraceableEventPipeline<U::ThenTraceableEvents, U::Error>,
        O: ?Sized + ObserveHandlerLatency,
    {
        use minstant::Instant;

        let total_start = Instant::now();
        let execution_span = Self::trace_span::<U, P, L>(use_case, meta, &command);
        let _execution_guard = execution_span.enter();

        tracing::trace!(
            phase = "total",
            operation = "executor.execute",
            status = "start",
            "command use case execution started"
        );

        let execution =
            (|| -> Result<(U::ThenTraceableEvents, HandlerLatencyMetrics), U::Error> {
                let ((), pre_check_ns) = trace_phase(
                    "pre_check",
                    "use_case.pre_check_command(&command)",
                    |error| use_case.format_error(error),
                    || use_case.pre_check_command(&command),
                )?;
                let (state, load_state_ns) = trace_phase(
                    "load_state",
                    "load_port.load_state(&command)",
                    |error| use_case.format_error(error),
                    || load_port.load_state(&command),
                )?;
                let ((), validate_in_lock_ns) = trace_phase(
                    "validate_against_state",
                    "use_case.validate_against_state(&command, &state)",
                    |error| use_case.format_error(error),
                    || use_case.validate_against_state(&command, &state),
                )?;
                let (events, apply_changes_ns) = trace_phase(
                    "gen_traceable_events",
                    "use_case.gen_traceable_events(&command, state)",
                    |error| use_case.format_error(error),
                    || use_case.gen_traceable_events(&command, state),
                )?;
                let domain_event_count = events.event_count();

                let ((), persist_domain_events_ns) = trace_phase(
                    "persist",
                    "pipeline.persist(&events)",
                    |error| use_case.format_error(error),
                    || pipeline.persist(&events),
                )?;
                let ((), replay_domain_events_ns) = trace_phase(
                    "replay",
                    "pipeline.replay(&events)",
                    |error| use_case.format_error(error),
                    || pipeline.replay(&events),
                )?;
                let ((), publish_domain_events_ns) = trace_phase(
                    "publish",
                    "pipeline.publish(&events)",
                    |error| use_case.format_error(error),
                    || pipeline.publish(&events),
                )?;

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
                    phase = "total",
                    operation = "executor.execute",
                    status = "ok",
                    total_ns = saturating_u64(metrics.total_ns),
                    domain_event_count = metrics.domain_event_count as u64,
                    "command use case execution completed"
                );
                latency_observer.observe_latency(&metrics);
                Ok(events)
            }
            Err(error) => {
                if let Some(error_message) = use_case.format_error(&error) {
                    tracing::trace!(
                        phase = "total",
                        operation = "executor.execute",
                        status = "err",
                        total_ns = saturating_u64(total_start.elapsed().as_nanos()),
                        error_message = %error_message,
                        "command use case execution failed"
                    );
                } else {
                    tracing::trace!(
                        phase = "total",
                        operation = "executor.execute",
                        status = "err",
                        total_ns = saturating_u64(total_start.elapsed().as_nanos()),
                        "command use case execution failed"
                    );
                }
                Err(error)
            }
        }
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
    pub fn execute<U, P, L, O>(
        &self,
        use_case: &U,
        command: U::Command,
        load_port: &L,
        pipeline: &P,
        latency_observer: &O,
    ) -> Result<U::ThenTraceableEvents, U::Error>
    where
        U: CommandUseCase2,
        L: ?Sized + Send + Sync + LoadState<U::Command, U::GivenState, U::Error>,
        P: TraceableEventPipeline<U::ThenTraceableEvents, U::Error>,
        O: ?Sized + ObserveHandlerLatency,
    {
        self.execute_inner(
            use_case,
            command,
            &CommandMeta::default(),
            load_port,
            pipeline,
            latency_observer,
        )
    }

    /// 用 envelope 携带技术追踪元数据，让业务 command 本身保持聚焦。
    pub fn execute_enveloped<U, P, L, O>(
        &self,
        use_case: &U,
        envelope: CommandEnvelope<U::Command>,
        load_port: &L,
        pipeline: &P,
        latency_observer: &O,
    ) -> Result<U::ThenTraceableEvents, U::Error>
    where
        U: CommandUseCase2,
        L: ?Sized + Send + Sync + LoadState<U::Command, U::GivenState, U::Error>,
        P: TraceableEventPipeline<U::ThenTraceableEvents, U::Error>,
        O: ?Sized + ObserveHandlerLatency,
    {
        self.execute_inner(
            use_case,
            envelope.command,
            &envelope.meta,
            load_port,
            pipeline,
            latency_observer,
        )
    }

    /// 在标准执行编排之后，把领域事件交给外部 reply mapper 转成对外响应。
    pub fn execute_and_map_reply<U, P, M, L, O>(
        &self,
        use_case: &U,
        command: U::Command,
        load_port: &L,
        pipeline: &P,
        latency_observer: &O,
        mapper: &M,
    ) -> Result<M::Reply, U::Error>
    where
        U: CommandUseCase2,
        L: ?Sized + Send + Sync + LoadState<U::Command, U::GivenState, U::Error>,
        P: TraceableEventPipeline<U::ThenTraceableEvents, U::Error>,
        O: ?Sized + ObserveHandlerLatency,
        M: UseCaseReplyMapper<U::ThenTraceableEvents>,
    {
        let events = self.execute(use_case, command, load_port, pipeline, latency_observer)?;
        Ok(mapper.map(events))
    }

    /// 用 envelope 执行并映射 reply。
    pub fn execute_enveloped_and_map_reply<U, P, M, L, O>(
        &self,
        use_case: &U,
        envelope: CommandEnvelope<U::Command>,
        load_port: &L,
        pipeline: &P,
        latency_observer: &O,
        mapper: &M,
    ) -> Result<M::Reply, U::Error>
    where
        U: CommandUseCase2,
        L: ?Sized + Send + Sync + LoadState<U::Command, U::GivenState, U::Error>,
        P: TraceableEventPipeline<U::ThenTraceableEvents, U::Error>,
        O: ?Sized + ObserveHandlerLatency,
        M: UseCaseReplyMapper<U::ThenTraceableEvents>,
    {
        let events =
            self.execute_enveloped(use_case, envelope, load_port, pipeline, latency_observer)?;
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

    use super::*;

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

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StubEvents {
        count: usize,
    }

    impl TraceableEventSet for StubEvents {
        fn event_count(&self) -> usize {
            self.count
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct StubUseCase;

    impl CommandUseCase2 for StubUseCase {
        type Command = StubCommand;
        type GivenState = u64;
        type ThenTraceableEvents = StubEvents;
        type Error = StubError;

        fn role(&self) -> &'static str {
            "StubRole"
        }

        fn format_error(&self, error: &Self::Error) -> Option<String> {
            match error {
                StubError::RiskRejected(reason) => Some(format!("risk rejected: {reason}")),
            }
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

        fn gen_traceable_events(
            &self,
            _cmd: &Self::Command,
            state: Self::GivenState,
        ) -> Result<Self::ThenTraceableEvents, Self::Error> {
            Ok(StubEvents { count: state as usize })
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct StubLoadPort;

    impl LoadState<StubCommand, u64, StubError> for StubLoadPort {
        fn load_state(&self, cmd: &StubCommand) -> Result<u64, StubError> {
            Ok(cmd.quantity)
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct StubPipeline;

    impl TraceableEventPipeline<StubEvents, StubError> for StubPipeline {
        fn persist(&self, _events: &StubEvents) -> Result<(), StubError> {
            Ok(())
        }

        fn replay(&self, _events: &StubEvents) -> Result<(), StubError> {
            Ok(())
        }

        fn publish(&self, _events: &StubEvents) -> Result<(), StubError> {
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
            let load_port = StubLoadPort;
            let pipeline = StubPipeline;
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

            let events =
                executor.execute_enveloped(&use_case, command, &load_port, &pipeline, &()).unwrap();
            assert_eq!(events.event_count(), 1);
        });

        let ok_operations = recording.operations_by_status("ok");
        let log = read_log(&log_path);

        assert!(ok_operations.iter().any(|op| op == "load_port.load_state(&command)"));
        assert!(ok_operations.iter().any(|op| op == "pipeline.persist(&events)"));
        assert!(ok_operations.iter().any(|op| op == "pipeline.replay(&events)"));
        assert!(ok_operations.iter().any(|op| op == "pipeline.publish(&events)"));
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
            let load_port = StubLoadPort;
            let pipeline = StubPipeline;
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

            let error = executor
                .execute_enveloped(&use_case, command, &load_port, &pipeline, &())
                .unwrap_err();
            assert_eq!(error, StubError::RiskRejected("symbol disabled"));
        });

        let validate_statuses =
            recording.statuses_for_operation("use_case.validate_against_state(&command, &state)");
        let total_statuses = recording.statuses_for_operation("executor.execute");
        let log = read_log(&log_path);

        assert!(validate_statuses.iter().any(|status| status == "err"));
        assert!(total_statuses.iter().any(|status| status == "err"));
        assert!(log.contains("error_message=risk rejected: symbol disabled"));
        assert!(log.contains("party_id=\"acct-999\""));
    }
}
