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
