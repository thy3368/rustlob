use std::collections::BTreeMap;
use std::sync::Arc;

use cmd_handler::EntityReplayableEvent;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    FIELD_TYPE_BOOL, NewBlock, SpotPlaceOrderPayload, SpotPlaceOrderResult, SpotProductContext,
    SpotProductPlugin, int_field, stable_hash_hex, stable_positive_i64, string_field,
    updated_int_field,
};

const ORDER_ENTITY_TYPE: u8 = 21;
const BALANCE_ENTITY_TYPE: u8 = 22;

/// 待执行产品请求。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingRequest {
    /// 请求唯一标识。
    pub request_id: String,
    /// 产品标识，例如 `spot`。
    pub product_id: String,
    /// 产品动作，例如 `place_order`。
    pub action: String,
    /// 动作载荷，当前以 JSON 纯数据承载。
    pub payload: String,
}

/// 产品上下文快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductContext {
    Spot(SpotProductContext),
}

impl ProductContext {
    /// 返回上下文所属产品 ID。
    pub fn product_id(&self) -> &'static str {
        match self {
            Self::Spot(_) => "spot",
        }
    }

    /// 返回当前上下文的稳定业务承诺。
    pub fn commitment(&self) -> String {
        match self {
            Self::Spot(context) => context.commitment(),
        }
    }

    /// 应用单个请求结果后的新状态。
    pub fn apply_result(
        &mut self,
        result: &RequestExecutionResult,
    ) -> Result<(), ProductPluginError> {
        match (self, &result.effect) {
            (Self::Spot(context), ProductExecutionEffect::SpotPlaceOrder(result)) => {
                context.apply_place_order(result)
            }
        }
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
    #[error("market is closed")]
    MarketClosed,
    #[error("invalid price {price}")]
    InvalidPrice { price: u64 },
    #[error("invalid qty {qty}")]
    InvalidQty { qty: u64 },
    #[error("missing balance for asset '{asset}'")]
    MissingBalance { asset: String },
    #[error("insufficient balance on '{asset}': required {required}, available {available}")]
    InsufficientBalance { asset: String, required: u64, available: u64 },
    #[error("account mismatch: expected '{expected}', got '{actual}'")]
    AccountMismatch { expected: String, actual: String },
    #[error("symbol mismatch: expected '{expected}', got '{actual}'")]
    SymbolMismatch { expected: String, actual: String },
    #[error("arithmetic overflow")]
    ArithmeticOverflow,
}

/// 产品扩展点。
pub trait ProductPlugin: Send + Sync {
    /// 插件对应的产品 ID。
    fn product_id(&self) -> &'static str;

    /// 判断是否支持指定动作。
    fn supports_action(&self, action: &str) -> bool;

    /// 执行单个产品请求。
    fn execute(
        &self,
        request: &PendingRequest,
        context: &ProductContext,
    ) -> Result<RequestExecutionResult, ProductPluginError>;
}

impl ProductPlugin for SpotProductPlugin {
    fn product_id(&self) -> &'static str {
        "spot"
    }

    fn supports_action(&self, action: &str) -> bool {
        action == "place_order"
    }

    fn execute(
        &self,
        request: &PendingRequest,
        context: &ProductContext,
    ) -> Result<RequestExecutionResult, ProductPluginError> {
        if !self.supports_action(&request.action) {
            return Err(ProductPluginError::UnsupportedAction {
                product_id: self.product_id().to_string(),
                action: request.action.clone(),
            });
        }

        let spot_context = match context {
            ProductContext::Spot(spot_context) => spot_context,
        };

        let payload: SpotPlaceOrderPayload =
            serde_json::from_str(&request.payload).map_err(|error| {
                ProductPluginError::PayloadDecodeFailed { reason: error.to_string() }
            })?;
        let result = self.place_order(&request.request_id, payload, spot_context)?;
        Ok(RequestExecutionResult {
            request_id: request.request_id.clone(),
            product_id: request.product_id.clone(),
            action: request.action.clone(),
            effect: ProductExecutionEffect::SpotPlaceOrder(result),
        })
    }
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
    /// 从插件列表构建注册表。
    pub fn new(plugins: impl IntoIterator<Item = Arc<dyn ProductPlugin>>) -> Self {
        let plugins =
            plugins.into_iter().map(|plugin| (plugin.product_id().to_string(), plugin)).collect();
        Self { plugins }
    }

    /// 注册单个插件。
    pub fn register(&mut self, plugin: Arc<dyn ProductPlugin>) {
        self.plugins.insert(plugin.product_id().to_string(), plugin);
    }

    /// 查找产品插件。
    pub fn plugin(&self, product_id: &str) -> Option<&Arc<dyn ProductPlugin>> {
        self.plugins.get(product_id)
    }
}

/// 单个产品请求的强类型业务结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestExecutionResult {
    /// 请求 ID。
    pub request_id: String,
    /// 产品 ID。
    pub product_id: String,
    /// 动作名。
    pub action: String,
    /// 强类型产品执行结果。
    pub effect: ProductExecutionEffect,
}

impl RequestExecutionResult {
    /// 派生单个请求的 replayable events。
    pub fn to_events(&self, sequence_start: u64) -> Vec<EntityReplayableEvent> {
        match &self.effect {
            ProductExecutionEffect::SpotPlaceOrder(result) => {
                vec![
                    encode_spot_order_created(result, sequence_start),
                    encode_balance_updated(result, sequence_start + 1),
                ]
            }
        }
    }

    /// 返回执行结果的稳定业务承诺。
    pub fn commitment(&self) -> String {
        match &self.effect {
            ProductExecutionEffect::SpotPlaceOrder(result) => {
                let parts = vec![
                    self.request_id.clone(),
                    self.product_id.clone(),
                    self.action.clone(),
                    result.commitment(),
                ];
                stable_hash_hex(&parts)
            }
        }
    }
}

/// 当前已支持的产品执行结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductExecutionEffect {
    SpotPlaceOrder(SpotPlaceOrderResult),
}

fn encode_spot_order_created(
    result: &SpotPlaceOrderResult,
    sequence: u64,
) -> EntityReplayableEvent {
    let mut event = EntityReplayableEvent::new_created(
        0,
        sequence,
        stable_positive_i64(&result.order.order_id),
        ORDER_ENTITY_TYPE,
    );
    event.add_field_change(string_field("request_id", &result.request_id));
    event.add_field_change(string_field("product_id", "spot"));
    event.add_field_change(string_field("action", "place_order"));
    event.add_field_change(string_field("order_id", &result.order.order_id));
    event.add_field_change(string_field("account_id", &result.order.account_id));
    event.add_field_change(string_field("symbol", &result.order.symbol));
    event.add_field_change(string_field("side", result.order.side.as_str()));
    event.add_field_change(int_field("price", result.order.price));
    event.add_field_change(int_field("qty", result.order.qty));
    if let Some(client_order_id) = result.order.client_order_id.as_deref() {
        event.add_field_change(string_field("client_order_id", client_order_id));
    }
    event
}

fn encode_balance_updated(result: &SpotPlaceOrderResult, sequence: u64) -> EntityReplayableEvent {
    let balance = &result.affected_balance_after;
    let before = &result.affected_balance_before;
    let mut event = EntityReplayableEvent::new_updated(
        0,
        sequence,
        before.version,
        balance.version,
        stable_positive_i64(&format!("{}:{}", result.order.account_id, balance.asset)),
        BALANCE_ENTITY_TYPE,
    );
    event.add_field_change(string_field("request_id", &result.request_id));
    event.add_field_change(string_field("account_id", &result.order.account_id));
    event.add_field_change(string_field("asset", &balance.asset));
    event.add_field_change(updated_int_field("available", before.available, balance.available));
    event.add_field_change(updated_int_field("reserved", before.reserved, balance.reserved));
    event.add_field_change(cmd_handler::ReplayFieldChange::new(
        cmd_handler::ReplayFieldChange::field_name_from_str("trading_enabled"),
        &[],
        true.to_string().as_bytes(),
        FIELD_TYPE_BOOL,
    ));
    event
}

/// 从请求结果与状态根生成新区块。
pub fn build_new_block(
    block_height: u64,
    parent_block_hash: String,
    pending_requests: &[PendingRequest],
    request_results: &[RequestExecutionResult],
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
    let _result_root = stable_hash_hex(
        &request_results.iter().map(RequestExecutionResult::commitment).collect::<Vec<_>>(),
    );
    NewBlock::new(block_height, parent_block_hash, request_ids_root, events_root, post_state_root)
}

fn event_commitment(event: &EntityReplayableEvent) -> String {
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
