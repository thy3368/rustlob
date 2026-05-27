use crate::{HandlerLatencyMetrics, TraceableEventSet};

fn saturating_u64(value: u128) -> u64 {
    value.min(u64::MAX as u128) as u64
}

fn trace_phase<T, E>(
    phase: &'static str,
    operation: &'static str,
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
            tracing::trace!(
                phase,
                operation,
                status = "err",
                elapsed_ns = saturating_u64(elapsed_ns),
                "command use case phase failed"
            );
            Err(error)
        }
    }
}

/// 更贴近 Use Cases（用例）的命令型抽象：
/// 只定义业务输入、业务校验与可重放事件产出。
pub trait CommandUseCase2: Send + Sync {
    /// 对应cqrs的 command
    type Command;

    /// 对应clean 架构的 entity , 从数据库/内存/文件等
    type GivenState;

    /// 对应事件溯源的可重放事件
    type ThenTraceableEvents: TraceableEventSet;
    type Error;

    /// 对应四色建模的role
    fn role(&self) -> &'static str {
        "UnknownActor用来做权限控制和追溯"
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
pub trait DomainEventPipeline<E, Err>: Send + Sync {
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
        P: DomainEventPipeline<U::ThenTraceableEvents, U::Error>,
        O: ?Sized + ObserveHandlerLatency,
    {
        use minstant::Instant;

        let total_start = Instant::now();
        let execution_span = tracing::span!(
            tracing::Level::TRACE,
            "command_use_case_execute",
            use_case = std::any::type_name::<U>(),
            role = use_case.role(),
            command_type = std::any::type_name::<U::Command>(),
            load_port = std::any::type_name::<L>(),
            pipeline = std::any::type_name::<P>(),
        );
        let _execution_guard = execution_span.enter();

        tracing::trace!(
            phase = "total",
            operation = "executor.execute",
            status = "start",
            "command use case execution started"
        );

        let execution = (|| -> Result<(U::ThenTraceableEvents, HandlerLatencyMetrics), U::Error> {
            let ((), pre_check_ns) = trace_phase(
                "pre_check",
                "use_case.pre_check_command(&command)",
                || use_case.pre_check_command(&command),
            )?;
            let (state, load_state_ns) =
                trace_phase("load_state", "load_port.load_state(&command)", || {
                    load_port.load_state(&command)
                })?;
            let ((), validate_in_lock_ns) = trace_phase(
                "validate_against_state",
                "use_case.validate_against_state(&command, &state)",
                || use_case.validate_against_state(&command, &state),
            )?;
            let (events, apply_changes_ns) = trace_phase(
                "gen_traceable_events",
                "use_case.gen_traceable_events(&command, state)",
                || use_case.gen_traceable_events(&command, state),
            )?;
            let domain_event_count = events.event_count();

            let ((), persist_domain_events_ns) =
                trace_phase("persist", "pipeline.persist(&events)", || pipeline.persist(&events))?;
            let ((), replay_domain_events_ns) =
                trace_phase("replay", "pipeline.replay(&events)", || pipeline.replay(&events))?;
            let ((), publish_domain_events_ns) =
                trace_phase("publish", "pipeline.publish(&events)", || pipeline.publish(&events))?;

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
                tracing::trace!(
                    phase = "total",
                    operation = "executor.execute",
                    status = "err",
                    total_ns = saturating_u64(total_start.elapsed().as_nanos()),
                    "command use case execution failed"
                );
                Err(error)
            }
        }
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
        P: DomainEventPipeline<U::ThenTraceableEvents, U::Error>,
        O: ?Sized + ObserveHandlerLatency,
        M: UseCaseReplyMapper<U::ThenTraceableEvents>,
    {
        let events = self.execute(use_case, command, load_port, pipeline, latency_observer)?;
        Ok(mapper.map(events))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::sync::{Arc, Mutex};

    use tracing::field::{Field, Visit};
    use tracing::span::{Attributes, Id, Record};
    use tracing::{Event, Metadata, Subscriber};

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct StubError;

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
        type Command = u64;
        type GivenState = u64;
        type ThenTraceableEvents = StubEvents;
        type Error = StubError;

        fn role(&self) -> &'static str {
            "StubRole"
        }

        fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
            Ok(())
        }

        fn validate_against_state(
            &self,
            _cmd: &Self::Command,
            _state: &Self::GivenState,
        ) -> Result<(), Self::Error> {
            Ok(())
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

    impl LoadState<u64, u64, StubError> for StubLoadPort {
        fn load_state(&self, cmd: &u64) -> Result<u64, StubError> {
            Ok(*cmd)
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct StubPipeline;

    impl DomainEventPipeline<StubEvents, StubError> for StubPipeline {
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
    struct RecordingSubscriber {
        events: Arc<Mutex<Vec<RecordedTraceEvent>>>,
    }

    impl RecordingSubscriber {
        fn operations_by_status(&self, status: &str) -> Vec<String> {
            self.events
                .lock()
                .unwrap()
                .iter()
                .filter(|event| event.status.as_deref() == Some(status))
                .filter_map(|event| event.operation.clone())
                .collect()
        }
    }

    impl Subscriber for RecordingSubscriber {
        fn enabled(&self, metadata: &Metadata<'_>) -> bool {
            *metadata.level() <= tracing::Level::TRACE
        }

        fn new_span(&self, _span: &Attributes<'_>) -> Id {
            Id::from_u64(1)
        }

        fn record(&self, _span: &Id, _values: &Record<'_>) {}

        fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

        fn event(&self, event: &Event<'_>) {
            let mut visitor = TraceFieldVisitor::default();
            event.record(&mut visitor);
            self.events.lock().unwrap().push(RecordedTraceEvent {
                phase: visitor.phase,
                operation: visitor.operation,
                status: visitor.status,
            });
        }

        fn enter(&self, _span: &Id) {}

        fn exit(&self, _span: &Id) {}
    }

    #[test]
    fn execute_traces_load_and_pipeline_phases() {
        let executor = CommandUseCaseExecutor2;
        let subscriber = RecordingSubscriber::default();

        tracing::subscriber::with_default(subscriber.clone(), || {
            let use_case = StubUseCase;
            let load_port = StubLoadPort;
            let pipeline = StubPipeline;

            let events = executor.execute(&use_case, 1, &load_port, &pipeline, &()).unwrap();
            assert_eq!(events.event_count(), 1);
        });

        let ok_operations = subscriber.operations_by_status("ok");

        assert!(ok_operations.iter().any(|op| op == "load_port.load_state(&command)"));
        assert!(ok_operations.iter().any(|op| op == "pipeline.persist(&events)"));
        assert!(ok_operations.iter().any(|op| op == "pipeline.replay(&events)"));
        assert!(ok_operations.iter().any(|op| op == "pipeline.publish(&events)"));
    }
}
