use std::collections::BTreeMap;
use std::fmt;
use std::time::{SystemTime as StdSystemTime, UNIX_EPOCH};

use serde_json::{Map, Number, Value};
use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::{FormatFields, Writer};
use tracing_subscriber::fmt::time::{FormatTime, SystemTime};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormattedFields};
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;

#[derive(Debug, Clone)]
pub struct FullTraceLogFormatter {
    timer: SystemTime,
}

impl Default for FullTraceLogFormatter {
    fn default() -> Self {
        Self { timer: SystemTime }
    }
}

#[derive(Debug, Clone)]
pub struct MinimalTraceLogFormatter;

impl Default for MinimalTraceLogFormatter {
    fn default() -> Self {
        Self
    }
}

pub type TraceLogFormatter = FullTraceLogFormatter;

pub fn build_dual_trace_subscriber<MW, FW>(
    minimal_writer: MW,
    full_writer: FW,
) -> impl Subscriber + Send + Sync
where
    MW: for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + Send + Sync + 'static,
    FW: for<'writer> tracing_subscriber::fmt::MakeWriter<'writer> + Send + Sync + 'static,
{
    tracing_subscriber::registry()
        .with(LevelFilter::TRACE)
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(FullTraceLogFormatter::default())
                .with_ansi(false)
                .with_writer(full_writer),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(MinimalTraceLogFormatter)
                .with_ansi(false)
                .with_writer(minimal_writer),
        )
}

fn format_span_id(raw: &str) -> &str {
    raw.strip_prefix("Id(").and_then(|value| value.strip_suffix(')')).unwrap_or(raw)
}

fn strip_ansi_sequences(input: &str) -> String {
    let mut cleaned = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for next in chars.by_ref() {
                if ('@'..='~').contains(&next) {
                    break;
                }
            }
            continue;
        }

        cleaned.push(ch);
    }

    cleaned
}

fn should_keep_field(token: &str) -> bool {
    match token.split_once('=') {
        Some((_, value)) => value != "\"-\"" && value != "-",
        None => !token.is_empty(),
    }
}

fn split_field_tokens(fields: &str) -> Vec<String> {
    let sanitized_fields = strip_ansi_sequences(fields);
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escape_next = false;

    for ch in sanitized_fields.chars() {
        if escape_next {
            current.push(ch);
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if in_quotes => {
                current.push(ch);
                escape_next = true;
            }
            '"' => {
                current.push(ch);
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if should_keep_field(&current) {
                    tokens.push(current.clone());
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if should_keep_field(&current) {
        tokens.push(current);
    }

    tokens
}

fn filter_empty_placeholder_fields(fields: &str) -> String {
    split_field_tokens(fields).join(" ")
}

fn parse_scalar_token(raw: &str) -> Value {
    let value = raw.trim();

    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        return Value::String(value[1..value.len() - 1].replace("\\\"", "\""));
    }

    if value == "true" {
        return Value::Bool(true);
    }

    if value == "false" {
        return Value::Bool(false);
    }

    if let Ok(parsed) = value.parse::<u64>() {
        return Value::Number(Number::from(parsed));
    }

    if let Ok(parsed) = value.parse::<i64>() {
        return Value::Number(Number::from(parsed));
    }

    Value::String(value.to_string())
}

fn parse_formatted_fields(fields: &str) -> BTreeMap<String, Value> {
    split_field_tokens(fields)
        .into_iter()
        .filter_map(|token| {
            let (key, value) = token.split_once('=')?;
            Some((key.to_string(), parse_scalar_token(value)))
        })
        .collect()
}

fn merge_span_scope_fields<S, N>(ctx: &FmtContext<'_, S, N>) -> BTreeMap<String, Value>
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    let mut merged = BTreeMap::new();

    if let Some(current_span) = ctx.lookup_current() {
        for span in current_span.scope().from_root() {
            let ext = span.extensions();
            if let Some(fields) = ext.get::<FormattedFields<N>>() {
                for (key, value) in parse_formatted_fields(fields).into_iter() {
                    merged.insert(key, value);
                }
            }
        }
    }

    merged
}

fn normalized_message(fields: &BTreeMap<String, Value>) -> Option<&str> {
    match fields.get("message") {
        Some(Value::String(message)) => Some(message.as_str()),
        _ => None,
    }
}

fn json_string(value: Option<&Value>) -> Option<String> {
    match value {
        Some(Value::String(value)) => Some(value.clone()),
        Some(Value::Number(value)) => Some(value.to_string()),
        Some(Value::Bool(value)) => Some(value.to_string()),
        _ => None,
    }
}

fn json_u64(value: Option<&Value>) -> Option<u64> {
    match value {
        Some(Value::Number(value)) => value.as_u64(),
        Some(Value::String(value)) => value.parse().ok(),
        _ => None,
    }
}

fn json_bool(value: Option<&Value>) -> Option<bool> {
    match value {
        Some(Value::Bool(value)) => Some(*value),
        Some(Value::String(value)) => value.parse().ok(),
        _ => None,
    }
}

fn value_or_null(value: Option<Value>) -> Value {
    value.unwrap_or(Value::Null)
}

fn extract_prefixed_object(
    event_fields: &BTreeMap<String, Value>,
    prefix: &str,
) -> Map<String, Value> {
    let mut object = Map::new();

    for (key, value) in event_fields.iter() {
        if let Some(stripped) = key.strip_prefix(prefix) {
            object.insert(stripped.to_string(), value.clone());
        }
    }

    object
}

fn current_timestamp_millis() -> String {
    StdSystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

#[derive(Default)]
struct EventFieldVisitor {
    fields: BTreeMap<String, Value>,
}

impl Visit for EventFieldVisitor {
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields.insert(field.name().to_string(), Value::Bool(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields.insert(field.name().to_string(), Value::Number(Number::from(value)));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields.insert(field.name().to_string(), Value::Number(Number::from(value)));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields.insert(field.name().to_string(), Value::String(value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        let debug = format!("{value:?}");
        self.fields.insert(field.name().to_string(), parse_scalar_token(debug.as_str()));
    }
}

impl<S, N> FormatEvent<S, N> for FullTraceLogFormatter
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let meta = event.metadata();
        self.timer.format_time(&mut writer)?;
        write!(writer, " {:<5} ", meta.level())?;

        if let Some(current_span) = ctx.lookup_current() {
            let current_span_id = format!("{:?}", current_span.id());
            let parent_span_id = current_span
                .parent()
                .map(|span| format!("{:?}", span.id()))
                .unwrap_or_else(|| "-".to_string());
            write!(
                writer,
                "span_id={} parent_span_id={} ",
                format_span_id(&current_span_id),
                format_span_id(&parent_span_id)
            )?;

            for span in current_span.scope().from_root() {
                write!(writer, "{}{{", span.name())?;
                let ext = span.extensions();
                if let Some(fields) = ext.get::<FormattedFields<N>>() {
                    if !fields.is_empty() {
                        let filtered_fields = filter_empty_placeholder_fields(fields);
                        if !filtered_fields.is_empty() {
                            write!(writer, "{filtered_fields}")?;
                        }
                    }
                }
                writer.write_char('}')?;
                writer.write_char(':')?;
            }
            writer.write_char(' ')?;
        } else {
            write!(writer, "span_id=- parent_span_id=- ")?;
        }

        write!(writer, "{}: ", meta.target())?;
        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

impl<S, N> FormatEvent<S, N> for MinimalTraceLogFormatter
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut visitor = EventFieldVisitor::default();
        event.record(&mut visitor);

        if json_bool(visitor.fields.get("call_stack")) != Some(true) {
            return Ok(());
        }

        let span_fields = merge_span_scope_fields(ctx);
        let current_span = ctx.lookup_current();
        let span_id = current_span
            .as_ref()
            .map(|span| format_span_id(&format!("{:?}", span.id())).to_string())
            .unwrap_or_else(|| "-".to_string());
        let parent_span_id = current_span
            .as_ref()
            .and_then(|span| span.parent())
            .map(|span| format_span_id(&format!("{:?}", span.id())).to_string())
            .unwrap_or_else(|| "-".to_string());

        let mut object = Map::new();
        object.insert("ts".to_string(), Value::String(current_timestamp_millis()));
        object.insert(
            "trace_id".to_string(),
            value_or_null(
                json_string(visitor.fields.get("trace_id").or(span_fields.get("trace_id")))
                    .map(Value::String),
            ),
        );
        object.insert("span_id".to_string(), Value::String(span_id));
        object.insert("parent_span_id".to_string(), Value::String(parent_span_id));
        object.insert(
            "layer".to_string(),
            value_or_null(json_string(visitor.fields.get("layer")).map(Value::String)),
        );
        object.insert(
            "component".to_string(),
            value_or_null(json_string(visitor.fields.get("component")).map(Value::String)),
        );
        object.insert(
            "operation".to_string(),
            value_or_null(json_string(visitor.fields.get("operation")).map(Value::String)),
        );
        object.insert(
            "status".to_string(),
            value_or_null(json_string(visitor.fields.get("status")).map(Value::String)),
        );
        object.insert(
            "latency_ns".to_string(),
            value_or_null(
                json_u64(visitor.fields.get("latency_ns")).map(Number::from).map(Value::Number),
            ),
        );
        object.insert(
            "command_id".to_string(),
            value_or_null(
                json_string(visitor.fields.get("command_id").or(span_fields.get("command_id")))
                    .map(Value::String),
            ),
        );
        object.insert(
            "request".to_string(),
            Value::Object(extract_prefixed_object(&visitor.fields, "request_")),
        );
        object.insert(
            "response".to_string(),
            Value::Object(extract_prefixed_object(&visitor.fields, "response_")),
        );
        if let Some(error_message) = json_string(visitor.fields.get("error_message")) {
            object.insert("error_message".to_string(), Value::String(error_message));
        }
        if let Some(message) = normalized_message(&visitor.fields) {
            object.insert("message".to_string(), Value::String(message.to_string()));
        }

        let json_line = serde_json::to_string(&object).map_err(|_| fmt::Error)?;
        writer.write_str(json_line.as_str())?;
        writer.write_char('\n')
    }
}
