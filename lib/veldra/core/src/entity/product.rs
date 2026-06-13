use std::collections::BTreeMap;
use std::sync::Arc;

use cmd_handler::EntityReplayableEvent;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{NewBlock, stable_hash_hex};

/// 待执行产品请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingRequest {
    pub request_id: String,
    pub product_id: String,
    pub action: String,
    pub payload: String,
}

/// 产品上下文不透明快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductContext {
    pub product_id: String,
    pub snapshot: String,
    pub commitment: String,
}

impl ProductContext {
    pub fn new(product_id: String, snapshot: String, commitment: String) -> Self {
        Self { product_id, snapshot, commitment }
    }

    pub fn commitment(&self) -> &str {
        self.commitment.as_str()
    }

    pub fn apply_result(
        &mut self,
        result: &RequestExecutionResult,
    ) -> Result<(), ProductPluginError> {
        if self.product_id != result.product_id {
            return Err(ProductPluginError::ContextMismatch {
                expected: self.product_id.clone(),
                actual: result.product_id.clone(),
            });
        }
        self.snapshot = result.next_product_context.snapshot.clone();
        self.commitment = result.next_product_context.commitment.clone();
        Ok(())
    }
}

/// 产品插件业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ProductPluginError {
    #[error("product plugin '{product_id}' does not support action '{action}'")]
    UnsupportedAction { product_id: String, action: String },
    #[error("product plugin expected context '{expected}', got '{actual}'")]
    ContextMismatch { expected: String, actual: String },
    #[error("failed to decode payload: {reason}")]
    PayloadDecodeFailed { reason: String },
    #[error("invalid payload field '{field}': {reason}")]
    InvalidPayloadField { field: String, reason: String },
    #[error("missing required state '{key}'")]
    MissingRequiredState { key: String },
    #[error("insufficient state on '{key}': required {required}, available {available}")]
    InsufficientState { key: String, required: u64, available: u64 },
    #[error("business rule rejected request: {reason}")]
    BusinessRuleRejected { reason: String },
    #[error("arithmetic overflow")]
    ArithmeticOverflow,
}

/// 产品扩展点。
pub trait ProductPlugin: Send + Sync {
    fn product_id(&self) -> &'static str;

    fn supports_action(&self, action: &str) -> bool;

    fn execute(
        &self,
        request: &PendingRequest,
        context: &ProductContext,
    ) -> Result<RequestExecutionResult, ProductPluginError>;
}

/// 插件注册表。
#[derive(Clone, Default)]
pub struct ProductPluginRegistry {
    plugins: BTreeMap<String, Arc<dyn ProductPlugin>>,
}

impl std::fmt::Debug for ProductPluginRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys = self.plugins.keys().collect::<Vec<_>>();
        f.debug_struct("ProductPluginRegistry").field("plugins", &keys).finish()
    }
}

impl ProductPluginRegistry {
    pub fn new(plugins: impl IntoIterator<Item = Arc<dyn ProductPlugin>>) -> Self {
        let plugins =
            plugins.into_iter().map(|plugin| (plugin.product_id().to_string(), plugin)).collect();
        Self { plugins }
    }

    pub fn register(&mut self, plugin: Arc<dyn ProductPlugin>) {
        self.plugins.insert(plugin.product_id().to_string(), plugin);
    }

    pub fn plugin(&self, product_id: &str) -> Option<&Arc<dyn ProductPlugin>> {
        self.plugins.get(product_id)
    }
}

/// 单个请求的强类型业务结果壳。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestExecutionResult {
    pub request_id: String,
    pub product_id: String,
    pub action: String,
    pub result_kind: String,
    pub result_payload: String,
    pub result_commitment: String,
    pub next_product_context: ProductContext,
    pub events: Vec<EntityReplayableEvent>,
}

impl RequestExecutionResult {
    pub fn commitment(&self) -> String {
        let events_commitment =
            stable_hash_hex(&self.events.iter().map(event_commitment).collect::<Vec<_>>());
        stable_hash_hex(&[
            self.request_id.clone(),
            self.product_id.clone(),
            self.action.clone(),
            self.result_kind.clone(),
            self.result_commitment.clone(),
            self.next_product_context.commitment.clone(),
            events_commitment,
        ])
    }
}

/// 从请求结果与状态根生成新区块。
pub fn build_new_block(
    block_height: u64,
    parent_block_hash: String,
    pending_requests: &[PendingRequest],
    events: &[EntityReplayableEvent],
    product_contexts: &BTreeMap<String, ProductContext>,
) -> NewBlock {
    let request_ids_root = stable_hash_hex(
        &pending_requests.iter().map(|request| request.request_id.as_str()).collect::<Vec<_>>(),
    );
    let events_root = stable_hash_hex(&events.iter().map(event_commitment).collect::<Vec<_>>());
    let post_state_root = stable_hash_hex(
        &product_contexts
            .iter()
            .map(|(product_id, context)| format!("{product_id}:{}", context.commitment()))
            .collect::<Vec<_>>(),
    );
    NewBlock::new(block_height, parent_block_hash, request_ids_root, events_root, post_state_root)
}

pub(crate) fn event_commitment(event: &EntityReplayableEvent) -> String {
    let fields = event
        .field_changes
        .iter()
        .map(|change| {
            let name = change.field_name_as_str().unwrap_or_default();
            let old_value = std::str::from_utf8(change.old_value_bytes()).unwrap_or_default();
            let new_value = std::str::from_utf8(change.new_value_bytes()).unwrap_or_default();
            format!("{name}:{old_value}:{new_value}:{}", change.field_type)
        })
        .collect::<Vec<_>>()
        .join("|");
    stable_hash_hex(&[
        event.timestamp.to_string(),
        event.sequence.to_string(),
        event.old_version.to_string(),
        event.new_version.to_string(),
        event.entity_id.to_string(),
        event.entity_type.to_string(),
        event.change_type.to_string(),
        fields,
    ])
}
