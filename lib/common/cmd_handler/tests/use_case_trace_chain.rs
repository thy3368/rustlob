use std::collections::BTreeSet;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use cmd_handler::use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCase2, CommandUseCaseExecutor2, CommandUseCaseOutbound,
    IssuedByParty,
};
use cmd_handler::{EntityReplayableEvent, build_dual_trace_subscriber};
use serde_json::{Map, Value};
use tracing_subscriber::fmt::MakeWriter;

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
enum StubError {}

impl fmt::Display for StubError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
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
        _cmd: &Self::Command,
        _state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
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
struct TracingStubOutbound;

impl TracingStubOutbound {
    fn enter_operation_span(operation: &'static str) -> tracing::span::EnteredSpan {
        tracing::span!(
            tracing::Level::TRACE,
            "TracingStubOutbound",
            layer = "outbound",
            component = "TracingStubOutbound",
            operation = operation
        )
        .entered()
    }
}

impl CommandUseCaseOutbound for TracingStubOutbound {
    type Command = StubCommand;
    type State = u64;
    type Error = StubError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let _guard = Self::enter_operation_span("load_state");
        tracing::trace!(
            call_stack = true,
            layer = "outbound",
            component = "TracingStubOutbound",
            operation = "load_state",
            request_symbol = cmd.symbol.as_str(),
            request_quantity = cmd.quantity,
            response_state = cmd.quantity,
            status = "ok",
            "outbound load_state completed"
        );
        Ok(cmd.quantity)
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let _guard = Self::enter_operation_span("persist");
        tracing::trace!(
            call_stack = true,
            layer = "outbound",
            component = "TracingStubOutbound",
            operation = "persist",
            request_event_count = events.len() as u64,
            response_persisted = true,
            status = "ok",
            "outbound persist completed"
        );
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let _guard = Self::enter_operation_span("replay");
        tracing::trace!(
            call_stack = true,
            layer = "outbound",
            component = "TracingStubOutbound",
            operation = "replay",
            request_event_count = events.len() as u64,
            response_replayed = true,
            status = "ok",
            "outbound replay completed"
        );
        Ok(())
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let _guard = Self::enter_operation_span("publish");
        tracing::trace!(
            call_stack = true,
            layer = "outbound",
            component = "TracingStubOutbound",
            operation = "publish",
            request_event_count = events.len() as u64,
            response_published = true,
            status = "ok",
            "outbound publish completed"
        );
        Ok(())
    }
}

#[derive(Clone)]
struct SharedFileWriter {
    output: Arc<Mutex<File>>,
}

impl SharedFileWriter {
    fn new(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).truncate(true).write(true).open(path)?;
        Ok(Self { output: Arc::new(Mutex::new(file)) })
    }
}

struct SharedFileGuard<'a>(MutexGuard<'a, File>);

impl Write for SharedFileGuard<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<'a> MakeWriter<'a> for SharedFileWriter {
    type Writer = SharedFileGuard<'a>;

    fn make_writer(&'a self) -> Self::Writer {
        SharedFileGuard(self.output.lock().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MinimalTraceEvent {
    trace_id: Option<String>,
    span_id: String,
    parent_span_id: String,
    layer: Option<String>,
    component: Option<String>,
    operation: Option<String>,
    status: Option<String>,
    request: Map<String, Value>,
    response: Map<String, Value>,
}

fn workspace_target_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("cmd_handler should live under the workspace root")
        .join("target")
}

fn test_log_dir() -> PathBuf {
    let dir = workspace_target_dir().join("test-logs").join("cmd_handler");
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn minimal_log_path(test_name: &str) -> PathBuf {
    test_log_dir().join(format!("{test_name}.minimal.jsonl"))
}

fn full_log_path(test_name: &str) -> PathBuf {
    test_log_dir().join(format!("{test_name}.full.log"))
}

fn json_string(value: Option<&Value>) -> Option<String> {
    match value {
        Some(Value::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn json_object(value: Option<&Value>) -> Map<String, Value> {
    match value {
        Some(Value::Object(object)) => object.clone(),
        _ => Map::new(),
    }
}

fn parse_minimal_trace_event(line: &str) -> MinimalTraceEvent {
    let value: Value = serde_json::from_str(line).expect("minimal trace line should be valid json");
    let object = value.as_object().expect("minimal trace line should be a json object");

    MinimalTraceEvent {
        trace_id: json_string(object.get("trace_id")),
        span_id: json_string(object.get("span_id")).unwrap_or_else(|| "-".to_string()),
        parent_span_id: json_string(object.get("parent_span_id"))
            .unwrap_or_else(|| "-".to_string()),
        layer: json_string(object.get("layer")),
        component: json_string(object.get("component")),
        operation: json_string(object.get("operation")),
        status: json_string(object.get("status")),
        request: json_object(object.get("request")),
        response: json_object(object.get("response")),
    }
}

fn read_minimal_trace(path: &Path) -> Vec<MinimalTraceEvent> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_minimal_trace_event)
        .collect()
}

fn read_full_trace(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

fn string_value<'a>(object: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    object.get(key).and_then(Value::as_str)
}

fn u64_value(object: &Map<String, Value>, key: &str) -> Option<u64> {
    object.get(key).and_then(Value::as_u64)
}

#[test]
fn use_case_trace_chain_reaches_outbound_in_minimal_and_full_logs() {
    let test_name = "use_case_trace_chain_reaches_outbound_in_minimal_and_full_logs";
    let minimal_path = minimal_log_path(test_name);
    let full_path = full_log_path(test_name);
    let subscriber = build_dual_trace_subscriber(
        SharedFileWriter::new(&minimal_path).unwrap(),
        SharedFileWriter::new(&full_path).unwrap(),
    );

    tracing::subscriber::with_default(subscriber, || {
        let executor = CommandUseCaseExecutor2;
        let use_case = StubUseCase;
        let outbound = TracingStubOutbound;
        let command = CommandEnvelope {
            meta: CommandMeta {
                trace_id: Some("trace-chain-001".to_string()),
                command_id: Some("cmd-chain-001".to_string()),
            },
            command: StubCommand {
                account_id: "acct-007".to_string(),
                symbol: "BTCUSDT".to_string(),
                quantity: 2,
            },
        };

        let events = executor.execute(&use_case, command, &outbound, &()).unwrap();
        assert_eq!(events.len(), 2);
    });

    let minimal_events = read_minimal_trace(&minimal_path);
    let use_case_event = minimal_events
        .iter()
        .find(|event| {
            event.layer.as_deref() == Some("workflow")
                && event.component.as_deref() == Some("command_use_case_execute")
                && event.operation.as_deref() == Some("execute")
        })
        .expect("use case trace event should be present");

    assert_eq!(use_case_event.trace_id.as_deref(), Some("trace-chain-001"));
    assert_eq!(use_case_event.status.as_deref(), Some("ok"));
    assert_eq!(string_value(&use_case_event.request, "party_id"), Some("acct-007"));
    assert_eq!(string_value(&use_case_event.response, "result"), Some("ok"));
    assert_eq!(u64_value(&use_case_event.response, "domain_event_count"), Some(2));

    let outbound_events: Vec<&MinimalTraceEvent> = minimal_events
        .iter()
        .filter(|event| {
            event.layer.as_deref() == Some("outbound")
                && event.component.as_deref() == Some("TracingStubOutbound")
        })
        .collect();

    assert_eq!(outbound_events.len(), 4);

    let operations: BTreeSet<&str> =
        outbound_events.iter().filter_map(|event| event.operation.as_deref()).collect();
    assert_eq!(operations, BTreeSet::from(["load_state", "persist", "publish", "replay"]));

    for event in &outbound_events {
        assert_eq!(event.trace_id.as_deref(), Some("trace-chain-001"));
        assert_eq!(event.status.as_deref(), Some("ok"));
        assert_ne!(event.parent_span_id, "-");
        assert_eq!(event.parent_span_id, use_case_event.span_id);
    }

    let load_state_event = outbound_events
        .iter()
        .find(|event| event.operation.as_deref() == Some("load_state"))
        .unwrap();
    assert_eq!(string_value(&load_state_event.request, "symbol"), Some("BTCUSDT"));
    assert_eq!(u64_value(&load_state_event.request, "quantity"), Some(2));
    assert_eq!(u64_value(&load_state_event.response, "state"), Some(2));

    for operation in ["persist", "replay", "publish"] {
        let event = outbound_events
            .iter()
            .find(|event| event.operation.as_deref() == Some(operation))
            .unwrap();
        assert_eq!(u64_value(&event.request, "event_count"), Some(2));
    }

    let full_trace = read_full_trace(&full_path);
    for expected in [
        "command_use_case_execute",
        "TracingStubOutbound",
        "command use case phase started",
        "command use case phase completed",
        "span_id=",
        "parent_span_id=",
        "trace_id=\"trace-chain-001\"",
        "command_id=\"cmd-chain-001\"",
        "party_id=\"acct-007\"",
    ] {
        assert!(full_trace.contains(expected), "expected full trace to contain `{expected}`");
    }
}
