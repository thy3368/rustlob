use l1_core::{PendingRequest, VmRuntimeError};

pub(super) fn parse_request_ids(request: &PendingRequest) -> (u64, u64) {
    let command_id = request
        .request_id
        .trim_start_matches("req-")
        .parse::<u64>()
        .unwrap_or(1);
    let trader_id = request
        .performer
        .trim_start_matches("acct-")
        .parse::<u64>()
        .unwrap_or(1);

    (command_id, trader_id)
}

pub(super) fn unsupported_capability(
    request: &PendingRequest,
    capability: &str,
) -> VmRuntimeError {
    VmRuntimeError::UnsupportedCapability {
        vm_kind: request.vm_kind,
        capability: capability.to_string(),
    }
}
