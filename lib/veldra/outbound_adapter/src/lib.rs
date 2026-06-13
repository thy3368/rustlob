use std::collections::BTreeMap;
use std::sync::Arc;

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCase3;
use example_core::{
    Balance as ExampleBalance, MarketRules as ExampleMarketRules, PlaceImmediateOrderCmd,
    PlaceImmediateOrderExecution, PlaceImmediateOrderOutput, PlaceImmediateOrderState,
    PlaceImmediateOrderUseCase, PlaceOrderError, PlaceOrderTimeInForce,
    SpotOrderExecution as ExampleSpotOrderExecution, SpotOrderSide as ExampleSpotOrderSide,
};
use serde::{Deserialize, Serialize};
use veldra_core::entity::{
    PendingRequest, ProductContext, ProductPlugin, ProductPluginError, ProductPluginRegistry,
    RequestExecutionResult,
};

/// `spot.place_order` 的 Hyperliquid 风格 order type。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SpotPlaceOrderType {
    Limit { limit_px: u64, tif: SpotTimeInForce },
    Market { aggressive_price: u64 },
}

/// 现货订单方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpotOrderSide {
    Buy,
    Sell,
}

impl SpotOrderSide {
    const fn from_is_buy(is_buy: bool) -> Self {
        if is_buy { Self::Buy } else { Self::Sell }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// 现货订单有效方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpotTimeInForce {
    Gtc,
    Ioc,
    Alo,
}

impl SpotTimeInForce {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Gtc => "gtc",
            Self::Ioc => "ioc",
            Self::Alo => "alo",
        }
    }
}

/// 桥接层的现货订单执行意图快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SpotOrderExecution {
    Limit { price: u64 },
    Market { aggressive_price: u64 },
}

impl SpotOrderExecution {
    fn commitment(&self) -> String {
        match self {
            Self::Limit { price } => format!("limit:{price}"),
            Self::Market { aggressive_price } => format!("market:{aggressive_price}"),
        }
    }
}

/// 账户某个资产的余额快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBalanceSnapshot {
    pub asset: String,
    pub available: u64,
    pub reserved: u64,
    pub version: u64,
}

impl SpotBalanceSnapshot {
    fn commitment(&self) -> String {
        format!("{}:{}:{}:{}", self.asset, self.available, self.reserved, self.version)
    }
}

/// 现货市场规则快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotMarketRules {
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub min_qty: u64,
}

impl SpotMarketRules {
    fn reserve_asset(&self, is_buy: bool) -> &str {
        if is_buy { self.quote_asset.as_str() } else { self.base_asset.as_str() }
    }

    fn commitment(&self) -> String {
        format!("{}:{}:{}:{}", self.symbol, self.base_asset, self.quote_asset, self.min_qty)
    }
}

/// `spot.place_order` 请求载荷。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotPlaceOrderPayload {
    pub account_id: String,
    pub asset: u32,
    pub symbol: String,
    pub is_buy: bool,
    pub size: u64,
    pub reduce_only: bool,
    pub order_type: SpotPlaceOrderType,
    pub cloid: Option<String>,
}

/// `spot` 产品上下文。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotProductContext {
    pub account_id: String,
    pub balances: BTreeMap<String, SpotBalanceSnapshot>,
    pub market_rules: SpotMarketRules,
    pub next_order_sequence: u64,
    pub trading_enabled: bool,
}

impl SpotProductContext {
    fn balance(&self, asset: &str) -> Option<&SpotBalanceSnapshot> {
        self.balances.get(asset)
    }

    fn commitment(&self) -> String {
        let balances = self
            .balances
            .values()
            .map(SpotBalanceSnapshot::commitment)
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "{}|{}|{}|{}|{}",
            self.account_id,
            self.market_rules.commitment(),
            self.next_order_sequence,
            self.trading_enabled,
            balances
        )
    }

    fn into_core(self) -> ProductContext {
        ProductContext::new(
            "spot".to_string(),
            serde_json::to_string(&self).unwrap(),
            self.commitment(),
        )
    }
}

/// `spot.place_order` 结果中的订单快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotOrder {
    pub order_id: String,
    pub asset: u32,
    pub exchange_oid: Option<u64>,
    pub account_id: String,
    pub symbol: String,
    pub side: SpotOrderSide,
    pub execution: SpotOrderExecution,
    pub time_in_force: SpotTimeInForce,
    pub qty: u64,
    pub reserved_base: u64,
    pub reserved_quote: u64,
    pub client_order_id: Option<String>,
}

impl SpotOrder {
    fn commitment(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.order_id,
            self.asset,
            self.exchange_oid.map(|v| v.to_string()).unwrap_or_default(),
            self.account_id,
            self.symbol,
            self.side.as_str(),
            self.execution.commitment(),
            self.time_in_force.as_str(),
            self.qty,
            self.reserved_base,
            self.reserved_quote,
            self.client_order_id.clone().unwrap_or_default()
        )
    }
}

/// `spot.place_order` 的强类型结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotPlaceOrderResult {
    pub request_id: String,
    pub order: SpotOrder,
    pub affected_balance_before: SpotBalanceSnapshot,
    pub affected_balance_after: SpotBalanceSnapshot,
}

impl SpotPlaceOrderResult {
    fn commitment(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.request_id,
            self.order.commitment(),
            self.affected_balance_before.commitment(),
            self.affected_balance_after.commitment()
        )
    }
}

/// 基于 `example_core` 的 `spot.place_order` 产品插件桥接。
#[derive(Debug, Clone, Copy, Default)]
pub struct ExampleSpotProductPluginAdapter;

impl ProductPlugin for ExampleSpotProductPluginAdapter {
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
        if context.product_id != "spot" {
            return Err(ProductPluginError::ContextMismatch {
                expected: "spot".to_string(),
                actual: context.product_id.clone(),
            });
        }

        let spot_context: SpotProductContext =
            serde_json::from_str(&context.snapshot).map_err(|error| {
                ProductPluginError::PayloadDecodeFailed { reason: error.to_string() }
            })?;
        let payload: SpotPlaceOrderPayload =
            serde_json::from_str(&request.payload).map_err(|error| {
                ProductPluginError::PayloadDecodeFailed { reason: error.to_string() }
            })?;

        let example_cmd = build_example_cmd(payload.clone());
        let example_state = build_example_state(&payload, &spot_context)?;

        CommandUseCase3::pre_check_command(&PlaceImmediateOrderUseCase, &example_cmd)
            .map_err(|error| map_place_order_error(&payload, &spot_context, error))?;
        CommandUseCase3::validate_against_state(
            &PlaceImmediateOrderUseCase,
            &example_cmd,
            &example_state,
        )
        .map_err(|error| map_place_order_error(&payload, &spot_context, error))?;

        let use_case_result = CommandUseCase3::compute_output_and_events(
            &PlaceImmediateOrderUseCase,
            &example_cmd,
            example_state,
        )
        .map_err(|error| map_place_order_error(&payload, &spot_context, error))?;

        let reserve_asset = spot_context.market_rules.reserve_asset(payload.is_buy).to_string();
        let affected_balance_before =
            spot_context.balance(&reserve_asset).cloned().ok_or_else(|| {
                ProductPluginError::MissingRequiredState { key: reserve_asset.clone() }
            })?;
        let output = map_output(
            request.request_id.as_str(),
            use_case_result.output,
            affected_balance_before,
        );
        let next_context = apply_output_to_context(spot_context, &output)?;
        let events = canonicalize_local_events(use_case_result.events);

        Ok(RequestExecutionResult {
            request_id: request.request_id.clone(),
            product_id: request.product_id.clone(),
            action: request.action.clone(),
            result_kind: "spot.place_order".to_string(),
            result_payload: serde_json::to_string(&output).map_err(|error| {
                ProductPluginError::PayloadDecodeFailed { reason: error.to_string() }
            })?,
            result_commitment: output.commitment(),
            next_product_context: next_context.into_core(),
            events,
        })
    }
}

/// 返回注册了 example spot 插件的默认 registry。
pub fn build_default_product_registry() -> ProductPluginRegistry {
    ProductPluginRegistry::new(vec![
        Arc::new(ExampleSpotProductPluginAdapter) as Arc<dyn ProductPlugin>
    ])
}

fn build_example_cmd(payload: SpotPlaceOrderPayload) -> PlaceImmediateOrderCmd {
    PlaceImmediateOrderCmd {
        party_id: payload.account_id,
        asset: payload.asset,
        symbol: payload.symbol,
        is_buy: payload.is_buy,
        size: payload.size,
        reduce_only: payload.reduce_only,
        execution: match payload.order_type {
            SpotPlaceOrderType::Limit { limit_px, tif } => PlaceImmediateOrderExecution::Limit {
                price: limit_px,
                time_in_force: map_time_in_force(tif),
            },
            SpotPlaceOrderType::Market { aggressive_price } => {
                PlaceImmediateOrderExecution::Market { aggressive_price }
            }
        },
        cloid: payload.cloid,
    }
}

fn build_example_state(
    payload: &SpotPlaceOrderPayload,
    context: &SpotProductContext,
) -> Result<PlaceImmediateOrderState, ProductPluginError> {
    Ok(PlaceImmediateOrderState {
        trading_enabled: context.trading_enabled,
        next_order_sequence: context.next_order_sequence,
        account_id: context.account_id.clone(),
        base_balance: map_balance(
            context.balance(context.market_rules.base_asset.as_str()).ok_or_else(|| {
                ProductPluginError::MissingRequiredState {
                    key: context.market_rules.base_asset.clone(),
                }
            })?,
            context.account_id.as_str(),
        ),
        quote_balance: map_balance(
            context.balance(context.market_rules.quote_asset.as_str()).ok_or_else(|| {
                ProductPluginError::MissingRequiredState {
                    key: context.market_rules.quote_asset.clone(),
                }
            })?,
            context.account_id.as_str(),
        ),
        market_rules: ExampleMarketRules {
            symbol: payload.symbol.clone(),
            min_qty: context.market_rules.min_qty,
        },
    })
}

fn apply_output_to_context(
    mut context: SpotProductContext,
    output: &SpotPlaceOrderResult,
) -> Result<SpotProductContext, ProductPluginError> {
    context.next_order_sequence =
        context.next_order_sequence.checked_add(1).ok_or(ProductPluginError::ArithmeticOverflow)?;
    context
        .balances
        .insert(output.affected_balance_after.asset.clone(), output.affected_balance_after.clone());
    Ok(context)
}

fn map_balance(balance: &SpotBalanceSnapshot, account_id: &str) -> ExampleBalance {
    ExampleBalance::new(
        account_id.to_string(),
        balance.asset.clone(),
        balance.available,
        balance.reserved,
        balance.version,
    )
}

fn map_output(
    request_id: &str,
    output: PlaceImmediateOrderOutput,
    affected_balance_before: SpotBalanceSnapshot,
) -> SpotPlaceOrderResult {
    let order = SpotOrder {
        order_id: output.order.order_id.clone(),
        asset: output.order.asset,
        exchange_oid: output.order.exchange_oid,
        account_id: output.order.account_id.clone(),
        symbol: output.order.symbol.clone(),
        side: map_side_back(output.order.side),
        execution: map_execution_back(output.order.execution),
        time_in_force: map_time_in_force_back(output.order.time_in_force),
        qty: output.order.qty,
        reserved_base: output.order.reserved_base,
        reserved_quote: output.order.reserved_quote,
        client_order_id: output.order.client_order_id.clone(),
    };
    SpotPlaceOrderResult {
        request_id: request_id.to_string(),
        order,
        affected_balance_before,
        affected_balance_after: SpotBalanceSnapshot {
            asset: output.affected_balance_after.asset_id,
            available: output.affected_balance_after.available,
            reserved: output.affected_balance_after.frozen,
            version: output.affected_balance_after.version,
        },
    }
}

fn canonicalize_local_events(events: Vec<EntityReplayableEvent>) -> Vec<EntityReplayableEvent> {
    events
        .into_iter()
        .enumerate()
        .map(|(index, mut event)| {
            event.timestamp = 0;
            event.sequence = index as u64;
            event
        })
        .collect()
}

fn map_time_in_force(value: SpotTimeInForce) -> PlaceOrderTimeInForce {
    match value {
        SpotTimeInForce::Gtc => PlaceOrderTimeInForce::Gtc,
        SpotTimeInForce::Ioc => PlaceOrderTimeInForce::Ioc,
        SpotTimeInForce::Alo => PlaceOrderTimeInForce::Alo,
    }
}

fn map_time_in_force_back(value: PlaceOrderTimeInForce) -> SpotTimeInForce {
    match value {
        PlaceOrderTimeInForce::Gtc => SpotTimeInForce::Gtc,
        PlaceOrderTimeInForce::Ioc => SpotTimeInForce::Ioc,
        PlaceOrderTimeInForce::Alo => SpotTimeInForce::Alo,
    }
}

fn map_execution_back(value: ExampleSpotOrderExecution) -> SpotOrderExecution {
    match value {
        ExampleSpotOrderExecution::Limit { price } => SpotOrderExecution::Limit { price },
        ExampleSpotOrderExecution::Market { aggressive_price } => {
            SpotOrderExecution::Market { aggressive_price }
        }
    }
}

fn map_side_back(value: ExampleSpotOrderSide) -> SpotOrderSide {
    match value {
        ExampleSpotOrderSide::Buy => SpotOrderSide::Buy,
        ExampleSpotOrderSide::Sell => SpotOrderSide::Sell,
    }
}

fn map_place_order_error(
    payload: &SpotPlaceOrderPayload,
    context: &SpotProductContext,
    error: PlaceOrderError,
) -> ProductPluginError {
    match error {
        PlaceOrderError::UnsupportedReduceOnly => ProductPluginError::BusinessRuleRejected {
            reason: "reduce_only is not supported".to_string(),
        },
        PlaceOrderError::InvalidQty | PlaceOrderError::QtyBelowMin => {
            ProductPluginError::InvalidPayloadField {
                field: "size".to_string(),
                reason: payload.size.to_string(),
            }
        }
        PlaceOrderError::InvalidPrice => ProductPluginError::InvalidPayloadField {
            field: "order_type".to_string(),
            reason: match &payload.order_type {
                SpotPlaceOrderType::Limit { limit_px, .. } => format!("invalid price {limit_px}"),
                SpotPlaceOrderType::Market { aggressive_price } => {
                    format!("invalid price {aggressive_price}")
                }
            },
        },
        PlaceOrderError::TradingDisabled => {
            ProductPluginError::BusinessRuleRejected { reason: "trading is disabled".to_string() }
        }
        PlaceOrderError::SymbolNotTradable => ProductPluginError::ContextMismatch {
            expected: context.market_rules.symbol.clone(),
            actual: payload.symbol.clone(),
        },
        PlaceOrderError::AccountMismatch => ProductPluginError::ContextMismatch {
            expected: context.account_id.clone(),
            actual: payload.account_id.clone(),
        },
        PlaceOrderError::InsufficientQuoteBalance => {
            let key = context.market_rules.quote_asset.clone();
            let available =
                context.balance(key.as_str()).map(|balance| balance.available).unwrap_or(0);
            let required = match &payload.order_type {
                SpotPlaceOrderType::Limit { limit_px, .. } => {
                    payload.size.checked_mul(*limit_px).unwrap_or(u64::MAX)
                }
                SpotPlaceOrderType::Market { aggressive_price } => {
                    payload.size.checked_mul(*aggressive_price).unwrap_or(u64::MAX)
                }
            };
            ProductPluginError::InsufficientState { key, required, available }
        }
        PlaceOrderError::InsufficientBaseBalance => {
            let key = context.market_rules.base_asset.clone();
            let available =
                context.balance(key.as_str()).map(|balance| balance.available).unwrap_or(0);
            ProductPluginError::InsufficientState { key, required: payload.size, available }
        }
        PlaceOrderError::ArithmeticOverflow => ProductPluginError::ArithmeticOverflow,
        PlaceOrderError::UnsupportedSide => ProductPluginError::InvalidPayloadField {
            field: "is_buy".to_string(),
            reason: "unsupported side".to_string(),
        },
        PlaceOrderError::InvalidTriggerPrice => ProductPluginError::InvalidPayloadField {
            field: "order_type".to_string(),
            reason: "trigger price is not supported for immediate spot orders".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use veldra_core::use_case::{
        BuildBlockFromPendingRequestsCommand, BuildBlockFromPendingRequestsState,
        BuildBlockFromPendingRequestsUseCase,
    };

    use super::*;

    fn sample_context() -> SpotProductContext {
        let mut balances = BTreeMap::new();
        balances.insert(
            "USDT".to_string(),
            SpotBalanceSnapshot {
                asset: "USDT".to_string(),
                available: 10_000,
                reserved: 0,
                version: 3,
            },
        );
        balances.insert(
            "BTC".to_string(),
            SpotBalanceSnapshot { asset: "BTC".to_string(), available: 5, reserved: 0, version: 2 },
        );
        SpotProductContext {
            account_id: "trader-1".to_string(),
            balances,
            market_rules: SpotMarketRules {
                symbol: "BTCUSDT".to_string(),
                base_asset: "BTC".to_string(),
                quote_asset: "USDT".to_string(),
                min_qty: 1,
            },
            next_order_sequence: 7,
            trading_enabled: true,
        }
    }

    fn sample_payload() -> SpotPlaceOrderPayload {
        SpotPlaceOrderPayload {
            account_id: "trader-1".to_string(),
            asset: 10_001,
            symbol: "BTCUSDT".to_string(),
            is_buy: true,
            size: 2,
            reduce_only: false,
            order_type: SpotPlaceOrderType::Limit { limit_px: 100, tif: SpotTimeInForce::Gtc },
            cloid: Some("cl-1".to_string()),
        }
    }

    fn sample_request() -> PendingRequest {
        PendingRequest {
            request_id: "req-1".to_string(),
            product_id: "spot".to_string(),
            action: "place_order".to_string(),
            payload: serde_json::to_string(&sample_payload()).unwrap(),
        }
    }

    #[test]
    fn adapter_executes_place_order_and_normalizes_event_headers() {
        let adapter = ExampleSpotProductPluginAdapter;
        let result = adapter.execute(&sample_request(), &sample_context().into_core()).unwrap();

        assert_eq!(result.result_kind, "spot.place_order");
        assert!(result.result_payload.contains("trader-1-BTCUSDT-7"));
        assert_eq!(result.next_product_context.product_id, "spot");
        assert_eq!(result.events.len(), 2);
        assert_eq!(result.events[0].timestamp, 0);
        assert_eq!(result.events[0].sequence, 0);
        assert_eq!(result.events[1].sequence, 1);
    }

    #[test]
    fn adapter_rejects_reduce_only_using_example_business_rule() {
        let adapter = ExampleSpotProductPluginAdapter;
        let payload = SpotPlaceOrderPayload { reduce_only: true, ..sample_payload() };
        let request = PendingRequest {
            payload: serde_json::to_string(&payload).unwrap(),
            ..sample_request()
        };

        let result = adapter.execute(&request, &sample_context().into_core());

        assert_eq!(
            result,
            Err(ProductPluginError::BusinessRuleRejected {
                reason: "reduce_only is not supported".to_string()
            })
        );
    }

    #[test]
    fn adapter_maps_unsupported_action() {
        let adapter = ExampleSpotProductPluginAdapter;
        let request = PendingRequest { action: "cancel_order".to_string(), ..sample_request() };

        let result = adapter.execute(&request, &sample_context().into_core());

        assert!(matches!(
            result,
            Err(ProductPluginError::UnsupportedAction { product_id, action })
            if product_id == "spot" && action == "cancel_order"
        ));
    }

    #[test]
    fn build_block_with_adapter_produces_stable_commitments()
    -> Result<(), veldra_core::use_case::BuildBlockError> {
        let mut contexts = BTreeMap::new();
        contexts.insert("spot".to_string(), sample_context().into_core());
        let state = BuildBlockFromPendingRequestsState {
            parent_height: 1,
            parent_block_hash: "parent-1".to_string(),
            pending_requests: vec![sample_request()],
            product_plugins: build_default_product_registry(),
            product_contexts: contexts.clone(),
        };
        let left = CommandUseCase3::compute_output_and_events(
            &BuildBlockFromPendingRequestsUseCase,
            &BuildBlockFromPendingRequestsCommand { block_height: 2 },
            state,
        )?;
        let right = CommandUseCase3::compute_output_and_events(
            &BuildBlockFromPendingRequestsUseCase,
            &BuildBlockFromPendingRequestsCommand { block_height: 2 },
            BuildBlockFromPendingRequestsState {
                parent_height: 1,
                parent_block_hash: "parent-1".to_string(),
                pending_requests: vec![sample_request()],
                product_plugins: build_default_product_registry(),
                product_contexts: contexts,
            },
        )?;

        assert_eq!(left.output, right.output);
        assert_eq!(left.events, right.events);
        assert_eq!(left.output.new_block.events_root, right.output.new_block.events_root);
        assert_eq!(left.output.new_block.post_state_root, right.output.new_block.post_state_root);
        Ok(())
    }
}
