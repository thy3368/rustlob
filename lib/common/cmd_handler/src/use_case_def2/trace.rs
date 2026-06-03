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
                error_message = %error,
                "command use case phase failed"
            );
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
    ($command_summary:expr, $role:expr, $party_id:expr, $outbound_type:expr, $metrics:expr) => {
        tracing::trace!(
            call_stack = true,
            layer = "workflow",
            component = "command_use_case_execute",
            operation = "execute",
            phase = "total",
            request_command_summary = %$command_summary,
            request_role = %$role,
            request_party_id = $party_id.as_deref().unwrap_or("-"),
            request_outbound = %$outbound_type,
            response_result = "ok",
            response_domain_event_count = $metrics.domain_event_count as u64,
            status = "ok",
            latency_ns = $crate::use_case_def2::trace::saturating_u64($metrics.total_ns),
            total_ns = $crate::use_case_def2::trace::saturating_u64($metrics.total_ns),
            domain_event_count = $metrics.domain_event_count as u64,
            "command use case execution completed"
        );
    };
}

macro_rules! trace_command_use_case_failed {
    (
        $command_summary:expr,
        $role:expr,
        $party_id:expr,
        $outbound_type:expr,
        $total_elapsed_ns:expr,
        $error:expr
    ) => {
        tracing::trace!(
            call_stack = true,
            layer = "workflow",
            component = "command_use_case_execute",
            operation = "execute",
            phase = "total",
            request_command_summary = %$command_summary,
            request_role = %$role,
            request_party_id = $party_id.as_deref().unwrap_or("-"),
            request_outbound = %$outbound_type,
            response_result = "err",
            status = "err",
            latency_ns = $crate::use_case_def2::trace::saturating_u64($total_elapsed_ns),
            total_ns = $crate::use_case_def2::trace::saturating_u64($total_elapsed_ns),
            error_message = %$error,
            "command use case execution failed"
        );
    };
}

pub(super) use {
    trace_command_use_case_completed, trace_command_use_case_failed, trace_command_use_case_started,
};
