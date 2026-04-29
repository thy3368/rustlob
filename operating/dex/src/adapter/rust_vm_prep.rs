use base_types::handler::handler_update::CmdHandlerForUpdate;
use l1_core::{
    BlockStateChanges, PendingRequest, ProductEvent, Receipt, VmExecutionInput, VmExecutionOutput,
    VmKind, VmRuntime, VmRuntimeError,
};

use crate::core::{
    ExchangeCommand, ExchangeCommandEnvelope, ExecuteTradingBatchHandler, PerpCommand,
    PerpPlaceOrderCmd, PerpSide, ProductType, TradingCommand,
};

pub struct RustVmPrepAdapter {
    handler: ExecuteTradingBatchHandler,
}

impl RustVmPrepAdapter {
    pub fn new() -> Self {
        Self { handler: ExecuteTradingBatchHandler::new() }
    }

    fn build_envelope(request: &PendingRequest) -> Result<ExchangeCommandEnvelope, VmRuntimeError> {
        match request.capability.0.as_str() {
            "dex.prep.place_order" => {}
            other => {
                return Err(VmRuntimeError::UnsupportedCapability {
                    vm_kind: request.vm_kind,
                    capability: other.to_string(),
                });
            }
        }

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

        Ok(ExchangeCommandEnvelope {
            command_id,
            trader_id,
            nonce: command_id,
            timestamp_ns: 1_000 + command_id,
            product_type: ProductType::Perp,
            command: ExchangeCommand::TradingCommand(TradingCommand::Perp(PerpCommand::PlaceOrder(
                PerpPlaceOrderCmd {
                    trader_id,
                    market: "BTC-PERP".into(),
                    side: PerpSide::Buy,
                    price: 100_000,
                    quantity: 2,
                    leverage: 2,
                    reduce_only: false,
                },
            ))),
        })
    }
}

impl Default for RustVmPrepAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl VmRuntime<PendingRequest> for RustVmPrepAdapter {
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
                product_type: "Perp".to_string(),
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
            capability: VmCapability::new("dex.prep.place_order"),
            action_type: "prep_order".to_string(),
            payload_hash: "payload-prep-1".to_string(),
        }
    }

    #[test]
    fn routes_prep_capability_to_dex_handler() {
        let adapter = RustVmPrepAdapter::new();
        let output = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.prep.place_order",
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
    fn rejects_unknown_capability() {
        let adapter = RustVmPrepAdapter::new();
        let error = adapter
            .execute(VmExecutionInput::from_pending_request(
                VmKind::RustVm,
                "dex.spot.place_order",
                PendingRequest {
                    capability: VmCapability::new("dex.spot.place_order"),
                    ..pending_request()
                },
            ))
            .unwrap_err();

        assert_eq!(
            error,
            VmRuntimeError::UnsupportedCapability {
                vm_kind: VmKind::RustVm,
                capability: "dex.spot.place_order".to_string(),
            }
        );
    }
}
