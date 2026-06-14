use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCase3;
use example_core::{
    Balance as ExampleBalance, MarketRules as ExampleMarketRules, MatchSpotOrderCmd,
    MatchSpotOrderError, MatchSpotOrderOutput, MatchSpotOrderState, MatchSpotOrderUseCase,
    PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceImmediateOrderOutput,
    PlaceImmediateOrderState, PlaceImmediateOrderUseCase, PlaceOrderError, PlaceOrderTimeInForce,
    SettleSpotTradeCmd, SettleSpotTradeError, SettleSpotTradeOutput, SettleSpotTradeState,
    SettleSpotTradeUseCase, SpotOrder as ExampleSpotOrder,
    SpotOrderExecution as ExampleSpotOrderExecution, SpotOrderSide as ExampleSpotOrderSide,
    SpotOrderStatus as ExampleSpotOrderStatus,
    SpotOrderStatusReason as ExampleSpotOrderStatusReason,
    SpotOrderTimeInForce as ExampleSpotOrderTimeInForce, SpotSettlement as ExampleSpotSettlement,
    SpotTrade as ExampleSpotTrade,
};
use serde::{Deserialize, Serialize};
use veldra_core::entity::{
    PendingRequest, ProductContext, ProductPlugin, ProductPluginError, ProductPluginRegistry,
    RequestExecutionResult,
};

const SPOT_PRODUCT_ID: &str = "spot";

/// `spot.place_order` 的 Hyperliquid 风格 order type。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// 现货订单生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpotOrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

impl SpotOrderStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::PartiallyFilled => "partially_filled",
            Self::Filled => "filled",
            Self::Canceled => "canceled",
            Self::Rejected => "rejected",
        }
    }
}

/// 账户某个资产上的余额快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBalanceSnapshot {
    pub account_id: String,
    pub asset: String,
    pub available: u64,
    pub reserved: u64,
    pub version: u64,
}

impl SpotBalanceSnapshot {
    fn key(&self) -> String {
        balance_key(self.account_id.as_str(), self.asset.as_str())
    }

    fn commitment(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.account_id, self.asset, self.available, self.reserved, self.version
        )
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
    pub maker_orders: Vec<SpotOrder>,
    pub settled_trade_ids: Vec<String>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
}

impl SpotProductContext {
    fn balance(&self, account_id: &str, asset: &str) -> Option<&SpotBalanceSnapshot> {
        self.balances.get(balance_key(account_id, asset).as_str())
    }

    fn commitment(&self) -> String {
        let balances = self
            .balances
            .iter()
            .map(|(key, balance)| format!("{key}:{}", balance.commitment()))
            .collect::<Vec<_>>()
            .join("|");
        let maker_orders =
            self.maker_orders.iter().map(SpotOrder::commitment).collect::<Vec<_>>().join("|");
        let settled_trade_ids = self.settled_trade_ids.join("|");
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.account_id,
            self.market_rules.commitment(),
            self.next_order_sequence,
            self.trading_enabled,
            balances,
            maker_orders,
            settled_trade_ids,
            self.base_asset_id,
            self.quote_asset_id
        )
    }

    fn into_core(self) -> Result<ProductContext, ProductPluginError> {
        Ok(ProductContext::new(
            SPOT_PRODUCT_ID.to_string(),
            serde_json::to_string(&self).map_err(serde_error)?,
            self.commitment(),
        ))
    }
}

/// 订单快照。
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
    pub filled_qty: u64,
    pub status: SpotOrderStatus,
    pub status_reason: Option<String>,
    pub reserved_base: u64,
    pub reserved_quote: u64,
    pub client_order_id: Option<String>,
    pub version: u64,
}

impl SpotOrder {
    fn commitment(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.order_id,
            self.asset,
            self.exchange_oid.map(|v| v.to_string()).unwrap_or_default(),
            self.account_id,
            self.symbol,
            self.side.as_str(),
            self.execution.commitment(),
            self.time_in_force.as_str(),
            self.qty,
            self.filled_qty,
            self.status.as_str(),
            self.status_reason.clone().unwrap_or_default(),
            self.reserved_base,
            self.reserved_quote,
            self.client_order_id.clone().unwrap_or_default(),
            self.version
        )
    }
}

/// 成交结果快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotTradeResult {
    pub trade_id: String,
    pub match_id: String,
    pub asset: u32,
    pub symbol: String,
    pub taker_order_id: String,
    pub maker_order_id: String,
    pub taker_account_id: String,
    pub maker_account_id: String,
    pub taker_side: SpotOrderSide,
    pub price: u64,
    pub qty: u64,
}

impl SpotTradeResult {
    fn commitment(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.trade_id,
            self.match_id,
            self.asset,
            self.symbol,
            self.taker_order_id,
            self.maker_order_id,
            self.taker_account_id,
            self.maker_account_id,
            self.taker_side.as_str(),
            self.price,
            self.qty
        )
    }
}

/// 清结算结果快照。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotSettlementResult {
    pub settlement_id: String,
    pub trade_id: String,
    pub match_id: String,
    pub buyer_account_id: String,
    pub seller_account_id: String,
    pub base_qty: u64,
    pub quote_qty: u64,
    pub price: u64,
}

impl SpotSettlementResult {
    fn commitment(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            self.settlement_id,
            self.trade_id,
            self.match_id,
            self.buyer_account_id,
            self.seller_account_id,
            self.base_qty,
            self.quote_qty,
            self.price
        )
    }
}

/// `spot.place_order` 的 pipeline 结果摘要。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotPlaceOrderResult {
    pub request_id: String,
    pub placed_order: SpotOrder,
    pub matched_trades: Vec<SpotTradeResult>,
    pub settlements: Vec<SpotSettlementResult>,
    pub affected_balances_after: Vec<SpotBalanceSnapshot>,
}

impl SpotPlaceOrderResult {
    fn commitment(&self) -> String {
        let matched_trades = self
            .matched_trades
            .iter()
            .map(SpotTradeResult::commitment)
            .collect::<Vec<_>>()
            .join("|");
        let settlements = self
            .settlements
            .iter()
            .map(SpotSettlementResult::commitment)
            .collect::<Vec<_>>()
            .join("|");
        let balances = self
            .affected_balances_after
            .iter()
            .map(SpotBalanceSnapshot::commitment)
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "{}|{}|{}|{}|{}",
            self.request_id,
            self.placed_order.commitment(),
            matched_trades,
            settlements,
            balances
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SpotExecutionOutcome {
    result: SpotPlaceOrderResult,
    next_context: SpotProductContext,
}

/// 基于 `example_core` 的 `spot.place_order` 产品插件桥接。
#[derive(Debug, Clone, Copy, Default)]
pub struct ExampleSpotProductPluginAdapter;

impl ProductPlugin for ExampleSpotProductPluginAdapter {
    fn product_id(&self) -> &'static str {
        SPOT_PRODUCT_ID
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
        if context.product_id != SPOT_PRODUCT_ID {
            return Err(ProductPluginError::ContextMismatch {
                expected: SPOT_PRODUCT_ID.to_string(),
                actual: context.product_id.clone(),
            });
        }

        let spot_context: SpotProductContext =
            serde_json::from_str(&context.snapshot).map_err(serde_error)?;
        let payload: SpotPlaceOrderPayload =
            serde_json::from_str(&request.payload).map_err(serde_error)?;

        let (outcome, raw_events) =
            execute_spot_pipeline(request.request_id.as_str(), &payload, spot_context)
                .map_err(|error| map_pipeline_error(&payload, error))?;
        let events = canonicalize_local_events(raw_events);

        Ok(RequestExecutionResult {
            request_id: request.request_id.clone(),
            product_id: request.product_id.clone(),
            action: request.action.clone(),
            result_kind: "spot.place_order".to_string(),
            result_payload: serde_json::to_string(&outcome.result).map_err(serde_error)?,
            result_commitment: outcome.result.commitment(),
            next_product_context: outcome.next_context.into_core()?,
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

fn build_place_cmd(payload: SpotPlaceOrderPayload) -> PlaceImmediateOrderCmd {
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

fn execute_spot_pipeline(
    request_id: &str,
    payload: &SpotPlaceOrderPayload,
    context: SpotProductContext,
) -> Result<(SpotExecutionOutcome, Vec<EntityReplayableEvent>), SpotPipelineError> {
    let place_cmd = build_place_cmd(payload.clone());
    let place_state = build_place_state(payload, &context).map_err(SpotPipelineError::Plugin)?;
    CommandUseCase3::pre_check_command(&PlaceImmediateOrderUseCase, &place_cmd)
        .map_err(SpotPipelineError::Place)?;
    CommandUseCase3::validate_against_state(&PlaceImmediateOrderUseCase, &place_cmd, &place_state)
        .map_err(SpotPipelineError::Place)?;
    let place_result = CommandUseCase3::compute_output_and_events(
        &PlaceImmediateOrderUseCase,
        &place_cmd,
        place_state,
    )
    .map_err(SpotPipelineError::Place)?;

    let mut all_events = place_result.events;
    let PlaceImmediateOrderOutput { order: taker_order, affected_balance_after } =
        place_result.output;
    let mut next_context = context;
    next_context.next_order_sequence = next_context
        .next_order_sequence
        .checked_add(1)
        .ok_or(ProductPluginError::ArithmeticOverflow)
        .map_err(SpotPipelineError::Plugin)?;

    if !should_enter_matching(&taker_order, &next_context.maker_orders)? {
        let affected_balances_after = vec![map_balance_back(&affected_balance_after)];
        for balance in &affected_balances_after {
            next_context.balances.insert(balance.key(), balance.clone());
        }
        let result = SpotPlaceOrderResult {
            request_id: request_id.to_string(),
            placed_order: map_order_back(&taker_order),
            matched_trades: Vec::new(),
            settlements: Vec::new(),
            affected_balances_after,
        };
        return Ok((SpotExecutionOutcome { result, next_context }, all_events));
    }

    let match_cmd = MatchSpotOrderCmd {
        party_id: payload.account_id.clone(),
        taker_order_id: taker_order.order_id.clone(),
        match_id: format!("{request_id}:match"),
    };
    let match_state = MatchSpotOrderState {
        taker_order,
        maker_orders: next_context
            .maker_orders
            .iter()
            .map(map_order_to_example)
            .collect::<Result<_, _>>()
            .map_err(SpotPipelineError::Plugin)?,
    };
    CommandUseCase3::pre_check_command(&MatchSpotOrderUseCase, &match_cmd)
        .map_err(SpotPipelineError::Match)?;
    CommandUseCase3::validate_against_state(&MatchSpotOrderUseCase, &match_cmd, &match_state)
        .map_err(SpotPipelineError::Match)?;
    let match_result =
        CommandUseCase3::compute_output_and_events(&MatchSpotOrderUseCase, &match_cmd, match_state)
            .map_err(SpotPipelineError::Match)?;
    all_events.extend(match_result.events);

    let MatchSpotOrderOutput { trades, taker_order_after, maker_orders_after } =
        match_result.output;
    for maker_order in &maker_orders_after {
        let updated = map_order_back(maker_order);
        if let Some(existing) =
            next_context.maker_orders.iter_mut().find(|order| order.order_id == updated.order_id)
        {
            *existing = updated;
        }
    }

    if trades.is_empty() {
        let affected_balances_after = vec![map_balance_back(&affected_balance_after)];
        for balance in &affected_balances_after {
            next_context.balances.insert(balance.key(), balance.clone());
        }
        let result = SpotPlaceOrderResult {
            request_id: request_id.to_string(),
            placed_order: map_order_back(&taker_order_after),
            matched_trades: Vec::new(),
            settlements: Vec::new(),
            affected_balances_after,
        };
        return Ok((SpotExecutionOutcome { result, next_context }, all_events));
    }

    let trade_ids = trades.iter().map(|trade| trade.trade_id.clone()).collect::<Vec<_>>();
    let settle_cmd = SettleSpotTradeCmd {
        party_id: payload.account_id.clone(),
        settlement_batch_id: format!("{request_id}:settle"),
        trade_ids,
    };
    let settlement_balances = settlement_balances_after_place(
        next_context.balances.values().map(map_balance).collect(),
        affected_balance_after,
    );
    let settle_state = SettleSpotTradeState {
        trades: trades.clone(),
        base_asset_id: next_context.base_asset_id.clone(),
        quote_asset_id: next_context.quote_asset_id.clone(),
        balances: settlement_balances,
        settled_trade_ids: next_context.settled_trade_ids.clone(),
    };
    CommandUseCase3::pre_check_command(&SettleSpotTradeUseCase, &settle_cmd)
        .map_err(SpotPipelineError::Settle)?;
    CommandUseCase3::validate_against_state(&SettleSpotTradeUseCase, &settle_cmd, &settle_state)
        .map_err(SpotPipelineError::Settle)?;
    let settle_result = CommandUseCase3::compute_output_and_events(
        &SettleSpotTradeUseCase,
        &settle_cmd,
        settle_state,
    )
    .map_err(SpotPipelineError::Settle)?;
    all_events.extend(settle_result.events);

    let SettleSpotTradeOutput { settlements, balances_after } = settle_result.output;
    let mut affected_balances_after =
        balances_after.iter().map(map_balance_back).collect::<Vec<_>>();
    affected_balances_after.sort_by(|left, right| left.key().cmp(&right.key()));
    for balance in &affected_balances_after {
        next_context.balances.insert(balance.key(), balance.clone());
    }
    next_context.settled_trade_ids = append_unique_trade_ids(
        next_context.settled_trade_ids,
        settlements.iter().map(|settlement| settlement.trade_id.clone()),
    );

    let result = SpotPlaceOrderResult {
        request_id: request_id.to_string(),
        placed_order: map_order_back(&taker_order_after),
        matched_trades: trades.iter().map(map_trade_back).collect(),
        settlements: settlements.iter().map(map_settlement_back).collect(),
        affected_balances_after,
    };
    Ok((SpotExecutionOutcome { result, next_context }, all_events))
}

fn build_place_state(
    payload: &SpotPlaceOrderPayload,
    context: &SpotProductContext,
) -> Result<PlaceImmediateOrderState, ProductPluginError> {
    Ok(PlaceImmediateOrderState {
        trading_enabled: context.trading_enabled,
        next_order_sequence: context.next_order_sequence,
        account_id: context.account_id.clone(),
        base_balance: map_balance(
            context
                .balance(context.account_id.as_str(), context.market_rules.base_asset.as_str())
                .ok_or_else(|| ProductPluginError::MissingRequiredState {
                    key: balance_key(
                        context.account_id.as_str(),
                        context.market_rules.base_asset.as_str(),
                    ),
                })?,
        ),
        quote_balance: map_balance(
            context
                .balance(context.account_id.as_str(), context.market_rules.quote_asset.as_str())
                .ok_or_else(|| ProductPluginError::MissingRequiredState {
                    key: balance_key(
                        context.account_id.as_str(),
                        context.market_rules.quote_asset.as_str(),
                    ),
                })?,
        ),
        market_rules: ExampleMarketRules {
            symbol: payload.symbol.clone(),
            min_qty: context.market_rules.min_qty,
        },
    })
}

fn append_unique_trade_ids(
    existing: Vec<String>,
    appended: impl IntoIterator<Item = String>,
) -> Vec<String> {
    let mut seen = existing.iter().cloned().collect::<BTreeSet<_>>();
    let mut merged = existing;
    for trade_id in appended {
        if seen.insert(trade_id.clone()) {
            merged.push(trade_id);
        }
    }
    merged
}

fn map_balance(balance: &SpotBalanceSnapshot) -> ExampleBalance {
    ExampleBalance::new(
        balance.account_id.clone(),
        balance.asset.clone(),
        balance.available,
        balance.reserved,
        balance.version,
    )
}

fn map_order_to_example(order: &SpotOrder) -> Result<ExampleSpotOrder, ProductPluginError> {
    let mut mapped = ExampleSpotOrder::new(
        order.order_id.clone(),
        order.asset,
        order.exchange_oid,
        order.account_id.clone(),
        order.symbol.clone(),
        map_side_to_example(order.side),
        map_execution_to_example(order.execution),
        map_time_in_force_to_example(order.time_in_force),
        order.qty,
        order.reserved_base,
        order.reserved_quote,
        order.client_order_id.clone(),
    )
    .with_version(order.version);
    mapped = mapped.with_execution_state(map_status_to_example(order.status), order.filled_qty);
    if let Some(status_reason) = order.status_reason.as_deref() {
        mapped = mapped.with_status_reason(parse_status_reason_to_example(status_reason)?);
    }
    Ok(mapped)
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

fn map_time_in_force_to_example(value: SpotTimeInForce) -> ExampleSpotOrderTimeInForce {
    match value {
        SpotTimeInForce::Gtc => ExampleSpotOrderTimeInForce::Gtc,
        SpotTimeInForce::Ioc => ExampleSpotOrderTimeInForce::Ioc,
        SpotTimeInForce::Alo => ExampleSpotOrderTimeInForce::Alo,
    }
}

fn map_execution_to_example(value: SpotOrderExecution) -> ExampleSpotOrderExecution {
    match value {
        SpotOrderExecution::Limit { price } => ExampleSpotOrderExecution::Limit { price },
        SpotOrderExecution::Market { aggressive_price } => {
            ExampleSpotOrderExecution::Market { aggressive_price }
        }
    }
}

fn map_side_to_example(value: SpotOrderSide) -> ExampleSpotOrderSide {
    match value {
        SpotOrderSide::Buy => ExampleSpotOrderSide::Buy,
        SpotOrderSide::Sell => ExampleSpotOrderSide::Sell,
    }
}

fn map_status_to_example(value: SpotOrderStatus) -> ExampleSpotOrderStatus {
    match value {
        SpotOrderStatus::Open => ExampleSpotOrderStatus::Open,
        SpotOrderStatus::PartiallyFilled => ExampleSpotOrderStatus::PartiallyFilled,
        SpotOrderStatus::Filled => ExampleSpotOrderStatus::Filled,
        SpotOrderStatus::Canceled => ExampleSpotOrderStatus::Canceled,
        SpotOrderStatus::Rejected => ExampleSpotOrderStatus::Rejected,
    }
}

fn parse_status_back(value: &str) -> Result<SpotOrderStatus, ProductPluginError> {
    match value {
        "open" => Ok(SpotOrderStatus::Open),
        "partially_filled" => Ok(SpotOrderStatus::PartiallyFilled),
        "filled" => Ok(SpotOrderStatus::Filled),
        "canceled" => Ok(SpotOrderStatus::Canceled),
        "rejected" => Ok(SpotOrderStatus::Rejected),
        _ => Err(ProductPluginError::PayloadDecodeFailed {
            reason: format!("unknown spot order status '{value}'"),
        }),
    }
}

fn parse_status_reason_to_example(
    value: &str,
) -> Result<ExampleSpotOrderStatusReason, ProductPluginError> {
    match value {
        "filled" => Ok(ExampleSpotOrderStatusReason::Filled),
        "canceled" => Ok(ExampleSpotOrderStatusReason::CanceledByUser),
        "triggered" => Ok(ExampleSpotOrderStatusReason::Triggered),
        "rejected" => Ok(ExampleSpotOrderStatusReason::RejectedAtPlacement),
        "marginCanceled" => Ok(ExampleSpotOrderStatusReason::MarginCanceled),
        "vaultWithdrawalCanceled" => Ok(ExampleSpotOrderStatusReason::VaultWithdrawalCanceled),
        "openInterestCapCanceled" => Ok(ExampleSpotOrderStatusReason::OpenInterestCapCanceled),
        "selfTradeCanceled" => Ok(ExampleSpotOrderStatusReason::SelfTradeCanceled),
        "reduceOnlyCanceled" => Ok(ExampleSpotOrderStatusReason::ReduceOnlyCanceled),
        "siblingFilledCanceled" => Ok(ExampleSpotOrderStatusReason::SiblingFilledCanceled),
        "delistedCanceled" => Ok(ExampleSpotOrderStatusReason::DelistedCanceled),
        "liquidatedCanceled" => Ok(ExampleSpotOrderStatusReason::LiquidatedCanceled),
        "scheduledCancel" => Ok(ExampleSpotOrderStatusReason::ScheduledCancel),
        "tickRejected" => Ok(ExampleSpotOrderStatusReason::TickRejected),
        "minTradeNtlRejected" => Ok(ExampleSpotOrderStatusReason::MinTradeNtlRejected),
        "perpMarginRejected" => Ok(ExampleSpotOrderStatusReason::PerpMarginRejected),
        "reduceOnlyRejected" => Ok(ExampleSpotOrderStatusReason::ReduceOnlyRejected),
        "badAloPxRejected" => Ok(ExampleSpotOrderStatusReason::BadAloPxRejected),
        "iocCancelRejected" => Ok(ExampleSpotOrderStatusReason::IocCancelRejected),
        "badTriggerPxRejected" => Ok(ExampleSpotOrderStatusReason::BadTriggerPxRejected),
        "marketOrderNoLiquidityRejected" => {
            Ok(ExampleSpotOrderStatusReason::MarketOrderNoLiquidityRejected)
        }
        "positionIncreaseAtOpenInterestCapRejected" => {
            Ok(ExampleSpotOrderStatusReason::PositionIncreaseAtOpenInterestCapRejected)
        }
        "positionFlipAtOpenInterestCapRejected" => {
            Ok(ExampleSpotOrderStatusReason::PositionFlipAtOpenInterestCapRejected)
        }
        "tooAggressiveAtOpenInterestCapRejected" => {
            Ok(ExampleSpotOrderStatusReason::TooAggressiveAtOpenInterestCapRejected)
        }
        "openInterestIncreaseRejected" => {
            Ok(ExampleSpotOrderStatusReason::OpenInterestIncreaseRejected)
        }
        "insufficientSpotBalanceRejected" => {
            Ok(ExampleSpotOrderStatusReason::InsufficientSpotBalanceRejected)
        }
        "oracleRejected" => Ok(ExampleSpotOrderStatusReason::OracleRejected),
        "perpMaxPositionRejected" => Ok(ExampleSpotOrderStatusReason::PerpMaxPositionRejected),
        _ => Err(ProductPluginError::PayloadDecodeFailed {
            reason: format!("unknown spot order status reason '{value}'"),
        }),
    }
}

#[derive(Debug)]
enum SpotPipelineError {
    Place(PlaceOrderError),
    Match(MatchSpotOrderError),
    Settle(SettleSpotTradeError),
    Plugin(ProductPluginError),
}

fn map_pipeline_error(
    payload: &SpotPlaceOrderPayload,
    error: SpotPipelineError,
) -> ProductPluginError {
    match error {
        SpotPipelineError::Place(error) => map_place_order_error(payload, error),
        SpotPipelineError::Match(error) => map_match_error(error),
        SpotPipelineError::Settle(error) => map_settle_error(error),
        SpotPipelineError::Plugin(error) => error,
    }
}

fn map_place_order_error(
    payload: &SpotPlaceOrderPayload,
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
            expected: payload.symbol.clone(),
            actual: payload.symbol.clone(),
        },
        PlaceOrderError::AccountMismatch => ProductPluginError::ContextMismatch {
            expected: payload.account_id.clone(),
            actual: payload.account_id.clone(),
        },
        PlaceOrderError::InsufficientQuoteBalance => {
            let key = "spot.quote_balance".to_string();
            let required = match &payload.order_type {
                SpotPlaceOrderType::Limit { limit_px, .. } => {
                    payload.size.checked_mul(*limit_px).unwrap_or(u64::MAX)
                }
                SpotPlaceOrderType::Market { aggressive_price } => {
                    payload.size.checked_mul(*aggressive_price).unwrap_or(u64::MAX)
                }
            };
            ProductPluginError::InsufficientState { key, required, available: 0 }
        }
        PlaceOrderError::InsufficientBaseBalance => ProductPluginError::InsufficientState {
            key: "spot.base_balance".to_string(),
            required: payload.size,
            available: 0,
        },
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

fn map_match_error(error: MatchSpotOrderError) -> ProductPluginError {
    match error {
        MatchSpotOrderError::ArithmeticOverflow => ProductPluginError::ArithmeticOverflow,
        _ => ProductPluginError::BusinessRuleRejected { reason: error.to_string() },
    }
}

fn map_settle_error(error: SettleSpotTradeError) -> ProductPluginError {
    match error {
        SettleSpotTradeError::AccountNotFound => {
            ProductPluginError::MissingRequiredState { key: "settlement_balances".to_string() }
        }
        SettleSpotTradeError::ArithmeticOverflow => ProductPluginError::ArithmeticOverflow,
        SettleSpotTradeError::InsufficientBuyerFrozenQuote => {
            ProductPluginError::BusinessRuleRejected {
                reason: "insufficient buyer frozen quote".to_string(),
            }
        }
        SettleSpotTradeError::InsufficientSellerFrozenBase => {
            ProductPluginError::BusinessRuleRejected {
                reason: "insufficient seller frozen base".to_string(),
            }
        }
        _ => ProductPluginError::BusinessRuleRejected { reason: error.to_string() },
    }
}

fn serde_error(error: serde_json::Error) -> ProductPluginError {
    ProductPluginError::PayloadDecodeFailed { reason: error.to_string() }
}

fn balance_key(account_id: &str, asset: &str) -> String {
    format!("{account_id}:{asset}")
}

fn should_enter_matching(
    taker_order: &ExampleSpotOrder,
    maker_orders: &[SpotOrder],
) -> Result<bool, SpotPipelineError> {
    if matches!(taker_order.time_in_force, ExampleSpotOrderTimeInForce::Ioc) {
        return Ok(true);
    }
    let Some(best_maker) = maker_orders.first() else {
        return Ok(false);
    };
    let maker = map_order_to_example(best_maker).map_err(SpotPipelineError::Plugin)?;
    if maker.side == taker_order.side {
        return Ok(true);
    }
    let Some(maker_price) = maker.limit_price() else {
        return Ok(true);
    };
    Ok(match taker_order.side {
        ExampleSpotOrderSide::Buy => taker_order.order_price() >= maker_price,
        ExampleSpotOrderSide::Sell => taker_order.order_price() <= maker_price,
    })
}

fn settlement_balances_after_place(
    mut balances: Vec<ExampleBalance>,
    affected_balance_after: ExampleBalance,
) -> Vec<ExampleBalance> {
    if let Some(balance) = balances.iter_mut().find(|balance| {
        balance.account_id == affected_balance_after.account_id
            && balance.asset_id == affected_balance_after.asset_id
    }) {
        *balance = affected_balance_after;
        return balances;
    }
    balances.push(affected_balance_after);
    balances
}

fn map_order_back(order: &ExampleSpotOrder) -> SpotOrder {
    SpotOrder {
        order_id: order.order_id.clone(),
        asset: order.asset,
        exchange_oid: order.exchange_oid,
        account_id: order.account_id.clone(),
        symbol: order.symbol.clone(),
        side: match order.side {
            ExampleSpotOrderSide::Buy => SpotOrderSide::Buy,
            ExampleSpotOrderSide::Sell => SpotOrderSide::Sell,
        },
        execution: match order.execution {
            ExampleSpotOrderExecution::Limit { price } => SpotOrderExecution::Limit { price },
            ExampleSpotOrderExecution::Market { aggressive_price } => {
                SpotOrderExecution::Market { aggressive_price }
            }
        },
        time_in_force: match order.time_in_force {
            ExampleSpotOrderTimeInForce::Gtc => SpotTimeInForce::Gtc,
            ExampleSpotOrderTimeInForce::Ioc => SpotTimeInForce::Ioc,
            ExampleSpotOrderTimeInForce::Alo => SpotTimeInForce::Alo,
        },
        qty: order.qty,
        filled_qty: order.filled_qty,
        status: parse_status_back(order.status.as_str()).unwrap_or(SpotOrderStatus::Open),
        status_reason: order.status_reason.map(|reason| reason.as_str().to_string()),
        reserved_base: order.reserved_base,
        reserved_quote: order.reserved_quote,
        client_order_id: order.client_order_id.clone(),
        version: order.version,
    }
}

fn map_trade_back(trade: &ExampleSpotTrade) -> SpotTradeResult {
    SpotTradeResult {
        trade_id: trade.trade_id.clone(),
        match_id: trade.match_id.clone(),
        asset: trade.asset,
        symbol: trade.symbol.clone(),
        taker_order_id: trade.taker_order_id.clone(),
        maker_order_id: trade.maker_order_id.clone(),
        taker_account_id: trade.taker_account_id.clone(),
        maker_account_id: trade.maker_account_id.clone(),
        taker_side: match trade.taker_side {
            ExampleSpotOrderSide::Buy => SpotOrderSide::Buy,
            ExampleSpotOrderSide::Sell => SpotOrderSide::Sell,
        },
        price: trade.price,
        qty: trade.qty,
    }
}

fn map_settlement_back(settlement: &ExampleSpotSettlement) -> SpotSettlementResult {
    SpotSettlementResult {
        settlement_id: settlement.settlement_id.clone(),
        trade_id: settlement.trade_id.clone(),
        match_id: settlement.match_id.clone(),
        buyer_account_id: settlement.buyer_account_id.clone(),
        seller_account_id: settlement.seller_account_id.clone(),
        base_qty: settlement.base_qty,
        quote_qty: settlement.quote_qty,
        price: settlement.price,
    }
}

fn map_balance_back(balance: &ExampleBalance) -> SpotBalanceSnapshot {
    SpotBalanceSnapshot {
        account_id: balance.account_id.clone(),
        asset: balance.asset_id.clone(),
        available: balance.available,
        reserved: balance.frozen,
        version: balance.version,
    }
}

#[cfg(test)]
mod tests;
