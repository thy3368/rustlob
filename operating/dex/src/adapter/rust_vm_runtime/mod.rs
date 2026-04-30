mod option;
mod perp;
mod shared;
mod spot;
mod treasury;

use base_types::handler::handler_update::CmdHandlerForUpdate;
use l1_core::{
    BlockStateChanges, PendingRequest, ProductEvent, Receipt, VmExecutionInput, VmExecutionOutput,
    VmKind, VmRuntime, VmRuntimeError,
};

use crate::core::{ExchangeCommandEnvelope, ExecuteTradingBatchHandler, ProductType};

use self::option::build_option_envelope;
use self::perp::build_perp_envelope;
use self::shared::unsupported_capability;
use self::spot::build_spot_envelope;
use self::treasury::build_treasury_envelope;

pub struct RustVmRuntimeAdapter {
    handler: ExecuteTradingBatchHandler,
}

impl RustVmRuntimeAdapter {
    pub fn new() -> Self {
        Self { handler: ExecuteTradingBatchHandler::new() }
    }

    fn build_envelope(request: &PendingRequest) -> Result<ExchangeCommandEnvelope, VmRuntimeError> {
        match request.capability.0.as_str() {
            // Keep the legacy prep alias until upstream callers are migrated to dex.perp.place_order.
            "dex.prep.place_order" | "dex.perp.place_order" => build_perp_envelope(request),
            "dex.spot.place_order" => build_spot_envelope(request),
            "dex.option.place_order" => build_option_envelope(request),
            "dex.treasury.deposit" => build_treasury_envelope(request),
            other => Err(unsupported_capability(request, other)),
        }
    }
}

impl Default for RustVmRuntimeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl VmRuntime<PendingRequest> for RustVmRuntimeAdapter {
    fn execute(
        &self,
        input: VmExecutionInput<PendingRequest>,
    ) -> Result<VmExecutionOutput, VmRuntimeError> {
        if input.vm_kind != VmKind::RustVm {
            return Err(VmRuntimeError::UnsupportedCapability {
                vm_kind: input.vm_kind,
                capability: input.capability.0,
            });
        }

        let envelope = Self::build_envelope(&input.transaction)?;
        let product_type = match envelope.product_type {
            ProductType::Spot => "Spot",
            ProductType::Perp => "Perp",
            ProductType::Option => "Option",
            ProductType::Treasury => "Treasury",
        };
        let writes = self
            .handler
            .cmd_handle(vec![envelope], |writes, _| writes.clone())
            .map_err(VmRuntimeError::ExecutionFailed)?;

        Ok(VmExecutionOutput {
            vm_kind: input.vm_kind,
            capability: input.capability,
            state_changes: BlockStateChanges::default(),
            receipts: vec![Receipt {
                success: true,
                cumulative_gas_used: writes.summary.accepted_commands as u64,
                logs: vec![],
                bloom: alloy_primitives::Bloom::ZERO,
            }],
            gas_used: writes.summary.accepted_commands as u64,
            product_events: vec![ProductEvent {
                product_type: product_type.to_string(),
                event_type: format!(
                    "accepted:{}:{}:{}",
                    writes.summary.accepted_commands,
                    writes.summary.orders_created,
                    writes.summary.trades_executed
                ),
                payload: input.transaction.payload_hash.into_bytes(),
            }],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use l1_core::VmCapability;

    fn pending_request() -> PendingRequest {
        PendingRequest {
            request_id: "req-7".to_string(),
            performer: "acct-9".to_string(),
            vm_kind: VmKind::RustVm,
            capability: VmCapability::new("dex.perp.place_order"),
            action_type: "perp_order".to_string(),
            payload_hash: "payload-perp-1".to_string(),
        }
    }

    #[test]
    fn routes_legacy_prep_capability_to_dex_handler() {
        let adapter = RustVmRuntimeAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.prep.place_order",
                PendingRequest {
                    capability: VmCapability::new("dex.prep.place_order"),
                    action_type: "perp_order_legacy_alias".to_string(),
                    payload_hash: "payload-perp-legacy-alias-1".to_string(),
                    ..pending_request()
                },
            ))
            .unwrap();

        assert_eq!(output.vm_kind, VmKind::RustVm);
        assert_eq!(output.gas_used, 1);
        assert_eq!(output.receipts.len(), 1);
        assert_eq!(output.product_events.len(), 1);
        assert_eq!(output.product_events[0].product_type, "Perp");
        assert!(output.product_events[0].event_type.starts_with("accepted:1"));
    }

    #[test]
    fn routes_perp_capability_with_default_pending_request() {
        let adapter = RustVmRuntimeAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.perp.place_order",
                pending_request(),
            ))
            .unwrap();

        assert_eq!(output.vm_kind, VmKind::RustVm);
        assert_eq!(output.gas_used, 1);
        assert_eq!(output.receipts.len(), 1);
        assert_eq!(output.product_events.len(), 1);
        assert_eq!(output.product_events[0].product_type, "Perp");
        assert!(output.product_events[0].event_type.starts_with("accepted:1"));
    }

    #[test]
    fn routes_perp_capability_to_dex_handler() {
        let adapter = RustVmRuntimeAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.perp.place_order",
                PendingRequest {
                    capability: VmCapability::new("dex.perp.place_order"),
                    action_type: "perp_order".to_string(),
                    payload_hash: "payload-perp-1".to_string(),
                    ..pending_request()
                },
            ))
            .unwrap();

        assert_eq!(output.vm_kind, VmKind::RustVm);
        assert_eq!(output.gas_used, 1);
        assert_eq!(output.receipts.len(), 1);
        assert_eq!(output.product_events.len(), 1);
        assert_eq!(output.product_events[0].product_type, "Perp");
        assert!(output.product_events[0].event_type.starts_with("accepted:1"));
    }

    #[test]
    fn routes_spot_capability_to_dex_handler() {
        let adapter = RustVmRuntimeAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.spot.place_order",
                PendingRequest {
                    capability: VmCapability::new("dex.spot.place_order"),
                    action_type: "spot_order".to_string(),
                    payload_hash: "payload-spot-1".to_string(),
                    ..pending_request()
                },
            ))
            .unwrap();

        assert_eq!(output.vm_kind, VmKind::RustVm);
        assert_eq!(output.gas_used, 1);
        assert_eq!(output.receipts.len(), 1);
        assert_eq!(output.product_events.len(), 1);
        assert_eq!(output.product_events[0].product_type, "Spot");
        assert_eq!(output.product_events[0].event_type, "accepted:1:1:0");
    }

    #[test]
    fn routes_option_capability_to_dex_handler() {
        let adapter = RustVmRuntimeAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.option.place_order",
                PendingRequest {
                    capability: VmCapability::new("dex.option.place_order"),
                    action_type: "option_order".to_string(),
                    payload_hash: "payload-option-1".to_string(),
                    ..pending_request()
                },
            ))
            .unwrap();

        assert_eq!(output.vm_kind, VmKind::RustVm);
        assert_eq!(output.gas_used, 1);
        assert_eq!(output.receipts.len(), 1);
        assert_eq!(output.product_events.len(), 1);
        assert_eq!(output.product_events[0].product_type, "Option");
        assert!(output.product_events[0].event_type.starts_with("accepted:1"));
    }

    #[test]
    fn routes_treasury_capability_to_dex_handler() {
        let adapter = RustVmRuntimeAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.treasury.deposit",
                PendingRequest {
                    capability: VmCapability::new("dex.treasury.deposit"),
                    action_type: "deposit".to_string(),
                    payload_hash: "payload-treasury-1".to_string(),
                    ..pending_request()
                },
            ))
            .unwrap();

        assert_eq!(output.vm_kind, VmKind::RustVm);
        assert_eq!(output.gas_used, 1);
        assert_eq!(output.receipts.len(), 1);
        assert_eq!(output.product_events.len(), 1);
        assert_eq!(output.product_events[0].product_type, "Treasury");
        assert!(output.product_events[0].event_type.starts_with("accepted:1"));
    }

    #[test]
    fn rejects_unknown_capability() {
        let adapter = RustVmRuntimeAdapter::new();
        let error = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.unknown.place_order",
                PendingRequest {
                    capability: VmCapability::new("dex.unknown.place_order"),
                    ..pending_request()
                },
            ))
            .unwrap_err();

        assert_eq!(
            error,
            VmRuntimeError::UnsupportedCapability {
                vm_kind: VmKind::RustVm,
                capability: "dex.unknown.place_order".to_string(),
            }
        );
    }
}
