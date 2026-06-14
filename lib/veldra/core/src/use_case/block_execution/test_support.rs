use std::collections::BTreeMap;
use std::sync::Arc;

use cmd_handler::command_use_case_def2::CommandUseCase3;
use cmd_handler::{EntityReplayableEvent, ReplayFieldChange};

use super::*;
use crate::entity::{
    PendingRequest, ProductContext, ProductPlugin, ProductPluginError, ProductPluginRegistry,
    RequestExecutionResult,
};

pub fn compute_result(
    cmd: &BuildBlockFromPendingRequestsCommand,
    state: BuildBlockFromPendingRequestsState,
) -> Result<
    cmd_handler::command_use_case_def2::UseCaseOutput<BuildBlockFromPendingRequestsOutput>,
    BuildBlockError,
> {
    CommandUseCase3::compute_output_and_events(&BuildBlockFromPendingRequestsUseCase, cmd, state)
}

pub fn string_field(name: &str, value: &str) -> ReplayFieldChange {
    ReplayFieldChange::new(ReplayFieldChange::field_name_from_str(name), &[], value.as_bytes(), 0)
}

pub fn int_field(name: &str, value: u64) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.to_string().as_bytes(),
        1,
    )
}

pub fn sample_command() -> BuildBlockFromPendingRequestsCommand {
    BuildBlockFromPendingRequestsCommand { block_height: 2 }
}

pub fn request(request_id: &str, payload: &str) -> PendingRequest {
    PendingRequest {
        request_id: request_id.to_string(),
        product_id: "spot".to_string(),
        action: "place_order".to_string(),
        payload: payload.to_string(),
    }
}

pub fn context(snapshot: &str, commitment: &str) -> ProductContext {
    ProductContext::new("spot".to_string(), snapshot.to_string(), commitment.to_string())
}

#[derive(Debug, Clone, Copy)]
pub struct DeterministicSpotPlugin;

impl ProductPlugin for DeterministicSpotPlugin {
    fn product_id(&self) -> &'static str {
        "spot"
    }

    fn supports_action(&self, action: &str) -> bool {
        action == "place_order"
    }

    fn execute(
        &self,
        request: &PendingRequest,
        _context: &ProductContext,
    ) -> Result<RequestExecutionResult, ProductPluginError> {
        let reserve_amount = if request.payload.contains("101") { 303 } else { 300 };
        let mut order_event = EntityReplayableEvent::new_created(9, 99, 101, 21);
        order_event.add_field_change(string_field("request_id", &request.request_id));
        order_event.add_field_change(int_field("reserve_amount", reserve_amount));

        let mut balance_event = EntityReplayableEvent::new_updated(9, 100, 1, 2, 201, 22);
        balance_event.add_field_change(string_field("asset", "USDT"));
        balance_event.add_field_change(int_field("available", 10_000 - reserve_amount));

        Ok(RequestExecutionResult {
            request_id: request.request_id.clone(),
            product_id: request.product_id.clone(),
            action: request.action.clone(),
            result_kind: "spot.place_order".to_string(),
            result_payload: format!(
                "{{\"order_id\":\"acct-1-BTCUSDT-9\",\"reserve_amount\":{reserve_amount}}}"
            ),
            result_commitment: format!("result-commitment-{reserve_amount}"),
            next_product_context: context(
                &format!("{{\"balance\":{}}}", 10_000 - reserve_amount),
                &format!("ctx-commitment-{reserve_amount}"),
            ),
            events: vec![order_event, balance_event],
        })
    }
}

pub fn sample_state() -> BuildBlockFromPendingRequestsState {
    let mut product_contexts = BTreeMap::new();
    product_contexts.insert("spot".to_string(), context("{\"balance\":10000}", "ctx-commitment-1"));
    BuildBlockFromPendingRequestsState {
        parent_height: 1,
        parent_block_hash: "parent-1".to_string(),
        pending_requests: vec![request("req-1", "{\"price\":100}")],
        product_plugins: ProductPluginRegistry::new(vec![
            Arc::new(DeterministicSpotPlugin) as Arc<dyn ProductPlugin>
        ]),
        product_contexts,
    }
}

#[derive(Debug)]
pub struct UnsupportedSpotActionPlugin;

impl ProductPlugin for UnsupportedSpotActionPlugin {
    fn product_id(&self) -> &'static str {
        "spot"
    }

    fn supports_action(&self, _action: &str) -> bool {
        false
    }

    fn execute(
        &self,
        _request: &PendingRequest,
        _context: &ProductContext,
    ) -> Result<RequestExecutionResult, ProductPluginError> {
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SequencedContextPlugin;

impl ProductPlugin for SequencedContextPlugin {
    fn product_id(&self) -> &'static str {
        "spot"
    }

    fn supports_action(&self, action: &str) -> bool {
        action == "place_order"
    }

    fn execute(
        &self,
        request: &PendingRequest,
        product_context: &ProductContext,
    ) -> Result<RequestExecutionResult, ProductPluginError> {
        match (request.request_id.as_str(), product_context.snapshot.as_str()) {
            ("req-1", "{\"step\":0}") => {
                let mut event = EntityReplayableEvent::new_updated(9, 100, 0, 1, 201, 22);
                event.add_field_change(int_field("step", 1));
                Ok(RequestExecutionResult {
                    request_id: request.request_id.clone(),
                    product_id: request.product_id.clone(),
                    action: request.action.clone(),
                    result_kind: "spot.step_1".to_string(),
                    result_payload: "{\"step\":1}".to_string(),
                    result_commitment: "result-step-1".to_string(),
                    next_product_context: context("{\"step\":1}", "ctx-commitment-step-1"),
                    events: vec![event],
                })
            }
            ("req-2", "{\"step\":1}") => {
                let mut event = EntityReplayableEvent::new_updated(9, 100, 1, 2, 201, 22);
                event.add_field_change(int_field("step", 2));
                Ok(RequestExecutionResult {
                    request_id: request.request_id.clone(),
                    product_id: request.product_id.clone(),
                    action: request.action.clone(),
                    result_kind: "spot.step_2".to_string(),
                    result_payload: "{\"step\":2}".to_string(),
                    result_commitment: "result-step-2".to_string(),
                    next_product_context: context("{\"step\":2}", "ctx-commitment-step-2"),
                    events: vec![event],
                })
            }
            ("req-2", actual_snapshot) => Err(ProductPluginError::BusinessRuleRejected {
                reason: format!("second request expected stepped context, got {actual_snapshot}"),
            }),
            ("req-1", actual_snapshot) => Err(ProductPluginError::BusinessRuleRejected {
                reason: format!("first request expected initial context, got {actual_snapshot}"),
            }),
            (request_id, _) => Err(ProductPluginError::BusinessRuleRejected {
                reason: format!("unexpected request id {request_id}"),
            }),
        }
    }
}
