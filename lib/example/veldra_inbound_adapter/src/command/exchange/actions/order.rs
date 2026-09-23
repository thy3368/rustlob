use cmd_handler::command_use_case_def2::ExecutionError;
use example_core_use_case::{
    MatchSpotOrderV2Changes, PlaceMatchSpotOrderV2Changes, PlaceOnlySpotOrderV2Cmd,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType,
};
use example_outbound_adapter::{base_asset_id_for, quote_asset_id_for, symbol_for_asset};
use serde::{Deserialize, Serialize};
pub use use_case_executor::trading::spot::place_match_spot_order_v2_executor::execute_place_match_spot_order_v2;

use crate::command::exchange::actions::cancel::DEFAULT_EXCHANGE_PARTY_ID;
use crate::command::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::command::exchange::common::validate::{
    validate_cloid, validate_envelope_common, validate_hex_address,
};
use crate::command::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_statuses_response};
use crate::command::exchange::error::ExchangeHttpError;

const DEFAULT_MAKER_FEE_BPS: u64 = 5;
const DEFAULT_TAKER_FEE_BPS: u64 = 10;
const UNSUPPORTED_POSITION_TPSL_ERROR: &str =
    "positionTpsl order grouping is not supported by place-match spot order v2";

/// `order` 动作的入站 contract 错误。
///
/// 这一层只负责 HTTP/wire 形状校验，不承载撮合或风控业务规则。
#[derive(Debug, thiserror::Error)]
pub enum OrderContractError {
    #[error("Unexpected `action.type` for order handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("`action.orders` must contain at least one order.")]
    EmptyOrders,
    #[error("Invalid `action.grouping`. Expected one of `na`, `normalTpsl`, `positionTpsl`.")]
    InvalidGrouping,
    #[error("Invalid `action.builder.b`. Expected a 42-character hexadecimal address.")]
    InvalidBuilderAddress,
    #[error("Invalid `action.orders[].c`. Expected a 128-bit hexadecimal cloid.")]
    InvalidCloid,
    #[error("Invalid `action.orders[].t`. Expected exactly one of `limit` or `trigger`.")]
    InvalidOrderType,
    #[error("Invalid `action.orders[].t.limit.tif`. Expected one of `Alo`, `Ioc`, `Gtc`.")]
    InvalidTimeInForce,
    #[error("Invalid `action.orders[].t.trigger.tpsl`. Expected `tp` or `sl`.")]
    InvalidTriggerKind,
    #[error("Invalid `action.orders[].p`. Expected a non-empty decimal string.")]
    InvalidPrice,
    #[error("Invalid `action.orders[].s`. Expected a non-empty decimal string.")]
    InvalidSize,
    #[error("Invalid `action.orders[].t.trigger.triggerPx`. Expected a non-empty decimal string.")]
    InvalidTriggerPrice,
}

pub mod reply {
    use serde::Serialize;

    use crate::command::exchange::common::wire::{
        ExchangeResponseEnvelopeWire, ExchangeResponseWire, ExchangeStatusesDataWire,
    };

    /// `/exchange` 下单动作的顶层成功响应。
    pub type OrderResponseWire = ExchangeResponseWire<OrderResponseDataWire>;

    /// `order` 响应体，按请求中的订单顺序返回逐笔状态。
    pub type OrderResponseEnvelopeWire = ExchangeResponseEnvelopeWire<OrderResponseDataWire>;
    pub type OrderResponseDataWire = ExchangeStatusesDataWire<OrderStatusWire>;

    /// 单笔订单回执。
    ///
    /// - `Resting`: 订单成功挂入订单簿
    /// - `Filled`: 订单立即成交
    /// - `Error`: 该笔订单被拒绝
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(untagged)]
    pub enum OrderStatusWire {
        Resting { resting: RestingOrderStatusWire },
        Filled { filled: FilledOrderStatusWire },
        Error { error: String },
    }

    /// 挂单成功时仅返回订单 id。
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct RestingOrderStatusWire {
        pub oid: u64,
    }

    /// 成交回执，字段命名保持外部 API 兼容。
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct FilledOrderStatusWire {
        #[serde(rename = "totalSz")]
        pub total_sz: String,
        #[serde(rename = "avgPx")]
        pub avg_px: String,
        pub oid: u64,
    }
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    /// 动作类型，当前 handler 固定接收 `order`。
    type_: String,
    /// 本次批量提交的订单列表；即使只有一笔，也走数组协议。
    orders: Vec<OrderWire>,
    /// 订单分组策略，沿用上游 `na` / `normalTpsl` / `positionTpsl`。
    grouping: String,
    /// 可选 builder 费用信息。
    builder: Option<BuilderWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderWire {
    /// `a`: 资产或市场标识。
    a: u32,
    /// `b`: `true` 表示买，`false` 表示卖。
    b: bool,
    /// `p`: 价格，按外部接口约定保留为十进制字符串。
    p: String,
    /// `s`: 数量，按外部接口约定保留为十进制字符串。
    s: String,
    /// `r`: 是否 reduce-only。
    r: bool,
    /// `t`: 订单类型，limit / trigger 二选一。
    t: OrderTypeWire,
    /// `c`: client order id，可选。
    c: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderTypeWire {
    /// 限价单配置。
    limit: Option<LimitOrderTypeWire>,
    /// 条件单配置。
    trigger: Option<TriggerOrderTypeWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LimitOrderTypeWire {
    /// `tif`: Time In Force。
    tif: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TriggerOrderTypeWire {
    #[serde(rename = "isMarket")]
    /// 触发后是否以市价执行。
    is_market: bool,
    #[serde(rename = "triggerPx")]
    /// 触发价格，保持字符串形态，避免在 adapter 层提前引入精度语义。
    trigger_px: String,
    /// `tp` 或 `sl`。
    tpsl: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BuilderWire {
    /// builder 地址。
    b: String,
    /// builder fee。
    f: u64,
}

/// adapter-side 下单请求，只保留 wire 已校验字段，不装配业务 state。
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2Request {
    pub party_id: String,
    pub asset: u32,
    pub is_buy: bool,
    pub price: String,
    pub size: String,
    pub tif: String,
    pub cloid: Option<String>,
}

impl PlaceSpotOrderV2Request {
    #[allow(dead_code)]
    fn from_wire_order(party_id: String, order: &OrderWire) -> Result<Self, OrderContractError> {
        let Some(limit) = &order.t.limit else {
            return Err(OrderContractError::InvalidOrderType);
        };

        Ok(Self {
            party_id,
            asset: order.a,
            is_buy: order.b,
            price: order.p.clone(),
            size: order.s.clone(),
            tif: limit.tif.clone(),
            cloid: order.c.clone(),
        })
    }
}

fn decimal_wire_to_core_units(raw: &str) -> String {
    let trimmed = raw.trim();
    let mut seen_dot = false;
    let mut normalized = String::with_capacity(trimmed.len());

    for ch in trimmed.chars() {
        if ch == '.' {
            if seen_dot {
                return raw.to_string();
            }
            seen_dot = true;
            continue;
        }
        if !ch.is_ascii_digit() {
            return raw.to_string();
        }
        normalized.push(ch);
    }

    if normalized.is_empty() { raw.to_string() } else { normalized }
}

pub(crate) struct OrderAction;

impl ExchangeActionHandler for OrderAction {
    type Request = RequestWire;
    type Reply = reply::OrderResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "order" {
        return Err(ExchangeHttpError::contract(OrderContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    // 通用字段校验统一复用 shared validator，避免各 action 分叉签名语义。
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.orders.is_empty() {
        return Err(ExchangeHttpError::contract(OrderContractError::EmptyOrders));
    }
    if !matches!(request.action.grouping.as_str(), "na" | "normalTpsl" | "positionTpsl") {
        return Err(ExchangeHttpError::contract(OrderContractError::InvalidGrouping));
    }
    if let Some(builder) = &request.action.builder {
        validate_hex_address(&builder.b)
            .map_err(|_| ExchangeHttpError::contract(OrderContractError::InvalidBuilderAddress))?;
    }
    for order in &request.action.orders {
        // 这里仅检查 wire 最小合法性；价格/数量的业务精度约束留给更内层 use case。
        if order.p.trim().is_empty() {
            return Err(ExchangeHttpError::contract(OrderContractError::InvalidPrice));
        }
        if order.s.trim().is_empty() {
            return Err(ExchangeHttpError::contract(OrderContractError::InvalidSize));
        }
        if let Some(cloid) = &order.c {
            validate_cloid(cloid)
                .map_err(|_| ExchangeHttpError::contract(OrderContractError::InvalidCloid))?;
        }
        match (&order.t.limit, &order.t.trigger) {
            (Some(limit), None) => {
                if !matches!(limit.tif.as_str(), "Alo" | "Ioc" | "Gtc") {
                    return Err(ExchangeHttpError::contract(
                        OrderContractError::InvalidTimeInForce,
                    ));
                }
            }
            (None, Some(trigger)) => {
                if trigger.trigger_px.trim().is_empty() {
                    return Err(ExchangeHttpError::contract(
                        OrderContractError::InvalidTriggerPrice,
                    ));
                }
                if !matches!(trigger.tpsl.as_str(), "tp" | "sl") {
                    return Err(ExchangeHttpError::contract(
                        OrderContractError::InvalidTriggerKind,
                    ));
                }
            }
            _ => return Err(ExchangeHttpError::contract(OrderContractError::InvalidOrderType)),
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlaceMatchExecutionUnit {
    command: PlaceOnlySpotOrderV2Cmd,
    input_order_count: usize,
}

fn build_place_match_execution_units(
    request: &RequestWire,
) -> Result<Vec<PlaceMatchExecutionUnit>, OrderContractError> {
    let party_id = request
        .common
        .vault_address
        .clone()
        .unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    match request.action.grouping.as_str() {
        "na" => request
            .action
            .orders
            .iter()
            .enumerate()
            .map(|(index, order)| {
                Ok(PlaceMatchExecutionUnit {
                    command: PlaceOnlySpotOrderV2Cmd::Single(order_cmd_from_wire(
                        party_id.clone(),
                        index,
                        order,
                    )?),
                    input_order_count: 1,
                })
            })
            .collect(),
        "normalTpsl" => {
            let mut orders = request.action.orders.iter().enumerate();
            let Some((parent_index, parent_order)) = orders.next() else {
                return Err(OrderContractError::EmptyOrders);
            };
            let parent = order_cmd_from_wire(party_id.clone(), parent_index, parent_order)?;
            let children = orders
                .map(|(index, order)| order_cmd_from_wire(party_id.clone(), index, order))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(vec![PlaceMatchExecutionUnit {
                command: PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, children },
                input_order_count: request.action.orders.len(),
            }])
        }
        "positionTpsl" => Ok(Vec::new()),
        _ => Err(OrderContractError::InvalidGrouping),
    }
}

fn order_cmd_from_wire(
    party_id: String,
    index: usize,
    order: &OrderWire,
) -> Result<PlaceOnlySpotOrderV2OrderCmd, OrderContractError> {
    let symbol = symbol_for_asset(order.a).to_string();
    Ok(PlaceOnlySpotOrderV2OrderCmd {
        party_id,
        asset: order.a,
        order_id: order_id_for_wire_order(index, order.c.as_deref()),
        symbol: symbol.clone(),
        is_buy: order.b,
        price: decimal_wire_to_core_units(&order.p),
        size: decimal_wire_to_core_units(&order.s),
        order_type: order_type_from_wire(&order.t)?,
        reduce_only: order.r,
        cloid: order.c.clone(),
        base_asset_id: base_asset_id_for(symbol.as_str()).to_string(),
        quote_asset_id: quote_asset_id_for(symbol.as_str()).to_string(),
        maker_fee_bps: DEFAULT_MAKER_FEE_BPS,
        taker_fee_bps: DEFAULT_TAKER_FEE_BPS,
    })
}

fn order_type_from_wire(
    order_type: &OrderTypeWire,
) -> Result<PlaceOnlySpotOrderV2OrderType, OrderContractError> {
    match (&order_type.limit, &order_type.trigger) {
        (Some(limit), None) => Ok(PlaceOnlySpotOrderV2OrderType::Limit { tif: limit.tif.clone() }),
        (None, Some(trigger)) => Ok(PlaceOnlySpotOrderV2OrderType::Trigger {
            is_market: trigger.is_market,
            trigger_price: decimal_wire_to_core_units(&trigger.trigger_px),
            trigger_role: trigger.tpsl.clone(),
        }),
        _ => Err(OrderContractError::InvalidOrderType),
    }
}

fn order_id_for_wire_order(index: usize, cloid: Option<&str>) -> String {
    match cloid {
        Some(cloid) => format!("wire-order-{index}-{cloid}"),
        None => format!("wire-order-{index}"),
    }
}

async fn execute(request: RequestWire) -> Result<reply::OrderResponseWire, ExchangeHttpError> {
    let statuses = execute_with_default_outbound(request);
    Ok(ok_statuses_response("order", statuses))
}

fn execute_with_default_outbound(request: RequestWire) -> Vec<reply::OrderStatusWire> {
    if request.action.grouping == "positionTpsl" {
        return request
            .action
            .orders
            .iter()
            .map(|_| reply::OrderStatusWire::Error {
                error: UNSUPPORTED_POSITION_TPSL_ERROR.to_string(),
            })
            .collect();
    }

    let units = match build_place_match_execution_units(&request) {
        Ok(units) => units,
        Err(error) => {
            return request
                .action
                .orders
                .iter()
                .map(|_| reply::OrderStatusWire::Error { error: error.to_string() })
                .collect();
        }
    };

    units
        .into_iter()
        .flat_map(|unit| match execute_place_match_spot_order_v2(&unit.command) {
            Ok(result) => order_statuses_from_place_match_changes(&result.changes),
            Err(error) => repeated_error_statuses(unit.input_order_count, error),
        })
        .collect()
}

fn repeated_error_statuses<BE, OE>(
    count: usize,
    error: ExecutionError<BE, OE>,
) -> Vec<reply::OrderStatusWire>
where
    BE: std::fmt::Display,
    OE: std::fmt::Display,
{
    let message = order_execution_error_message(error);
    (0..count).map(|_| reply::OrderStatusWire::Error { error: message.clone() }).collect()
}

fn order_statuses_from_place_match_changes(
    changes: &PlaceMatchSpotOrderV2Changes,
) -> Vec<reply::OrderStatusWire> {
    match changes {
        PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { created_order } => {
            vec![resting_status(created_order.exchange_oid().unwrap_or(0))]
        }
        PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
            created_taker_order,
            match_changes,
        } => vec![order_status_from_match_changes(created_taker_order, match_changes)],
        PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
            created_parent_order,
            created_child_orders,
            match_changes,
        } => {
            let mut statuses = Vec::with_capacity(1 + created_child_orders.len());
            statuses.push(order_status_from_match_changes(created_parent_order, match_changes));
            statuses.extend(
                created_child_orders
                    .iter()
                    .map(|order| resting_status(order.exchange_oid().unwrap_or(0))),
            );
            statuses
        }
    }
}

fn order_status_from_match_changes(
    order: &example_core_use_case::SpotOrderV2,
    match_changes: &MatchSpotOrderV2Changes,
) -> reply::OrderStatusWire {
    let taker_trades = match_changes
        .created_trades
        .iter()
        .filter(|trade| trade.taker_order_id == order.order_id())
        .collect::<Vec<_>>();
    let filled_qty = taker_trades.iter().map(|trade| trade.qty).sum::<u64>();

    if filled_qty == 0 {
        return resting_status(order.exchange_oid().unwrap_or(0));
    }

    let notional =
        taker_trades.iter().filter_map(|trade| trade.price.checked_mul(trade.qty)).sum::<u64>();
    let avg_px = if filled_qty == 0 { 0 } else { notional / filled_qty };
    reply::OrderStatusWire::Filled {
        filled: reply::FilledOrderStatusWire {
            total_sz: filled_qty.to_string(),
            avg_px: avg_px.to_string(),
            oid: order.exchange_oid().unwrap_or(0),
        },
    }
}

fn resting_status(oid: u64) -> reply::OrderStatusWire {
    reply::OrderStatusWire::Resting { resting: reply::RestingOrderStatusWire { oid } }
}

fn order_execution_error_message<BE, OE>(error: ExecutionError<BE, OE>) -> String
where
    BE: std::fmt::Display,
    OE: std::fmt::Display,
{
    match error {
        ExecutionError::Business(error) => error.to_string(),
        ExecutionError::ProjectEvents(error) => {
            format!("project replayable events failed: {error}")
        }
        ExecutionError::LoadState(error) => format!("load_state failed: {error}"),
        ExecutionError::Persist(error) => format!("persist failed: {error}"),
        ExecutionError::Replay(error) => format!("replay failed: {error}"),
        ExecutionError::Publish(error) => format!("publish failed: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use example_core_use_case::{
        SpotOrderSide, SpotOrderTif, SpotOrderType, SpotOrderV2, SpotTrade,
    };

    use super::*;

    fn request_with(grouping: &str, orders: Vec<OrderWire>) -> RequestWire {
        RequestWire {
            action: ActionWire {
                type_: "order".to_string(),
                orders,
                grouping: grouping.to_string(),
                builder: None,
            },
            common: crate::command::exchange::common::wire::CommonExchangeFields {
                nonce: 1,
                signature: crate::command::exchange::common::wire::SignatureWire {
                    r: "0x0000000000000000000000000000000000000000000000000000000000000000"
                        .to_string(),
                    s: "0x0000000000000000000000000000000000000000000000000000000000000000"
                        .to_string(),
                    v: 27,
                },
                vault_address: None,
                expires_after: None,
            },
        }
    }

    fn limit_order(cloid: Option<&str>) -> OrderWire {
        OrderWire {
            a: 10_001,
            b: true,
            p: "100.5".to_string(),
            s: "2".to_string(),
            r: false,
            t: OrderTypeWire {
                limit: Some(LimitOrderTypeWire { tif: "Gtc".to_string() }),
                trigger: None,
            },
            c: cloid.map(str::to_string),
        }
    }

    fn trigger_order() -> OrderWire {
        OrderWire {
            a: 10_001,
            b: false,
            p: "90".to_string(),
            s: "1".to_string(),
            r: true,
            t: OrderTypeWire {
                limit: None,
                trigger: Some(TriggerOrderTypeWire {
                    is_market: true,
                    trigger_px: "91.5".to_string(),
                    tpsl: "sl".to_string(),
                }),
            },
            c: None,
        }
    }

    fn spot_order(order_id: &str, exchange_oid: u64) -> SpotOrderV2 {
        match SpotOrderV2::new_active(
            order_id.to_string(),
            10_001,
            Some(exchange_oid),
            "buyer".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            100,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            2,
            "BTC",
            "USDT",
            DEFAULT_MAKER_FEE_BPS,
            DEFAULT_TAKER_FEE_BPS,
            None,
        ) {
            Ok(order) => order,
            Err(error) => panic!("test order should be valid: {error}"),
        }
    }

    fn build_units(request: &RequestWire) -> Vec<PlaceMatchExecutionUnit> {
        match build_place_match_execution_units(request) {
            Ok(units) => units,
            Err(error) => panic!("command should build: {error}"),
        }
    }

    fn empty_match_changes() -> MatchSpotOrderV2Changes {
        MatchSpotOrderV2Changes {
            updated_taker_order: None,
            updated_maker_orders: vec![],
            updated_balances: vec![],
            created_trades: vec![],
            created_vouchers: vec![],
            created_balance_ledger_entries: vec![],
        }
    }

    #[test]
    fn na_limit_order_builds_single_place_match_command() {
        let request = request_with("na", vec![limit_order(Some("0xabc"))]);

        let units = build_units(&request);

        assert_eq!(units.len(), 1);
        let PlaceOnlySpotOrderV2Cmd::Single(command) = &units[0].command else {
            panic!("na grouping should build single command");
        };
        assert_eq!(command.party_id, DEFAULT_EXCHANGE_PARTY_ID);
        assert_eq!(command.asset, 10_001);
        assert!(command.is_buy);
        assert_eq!(command.price, "1005");
        assert_eq!(command.size, "2");
        assert_eq!(command.cloid.as_deref(), Some("0xabc"));
        assert!(!command.reduce_only);
        assert_eq!(command.symbol, "BTCUSDT");
        assert_eq!(command.base_asset_id, "BTC");
        assert_eq!(command.quote_asset_id, "USDT");
        assert_eq!(command.maker_fee_bps, DEFAULT_MAKER_FEE_BPS);
        assert_eq!(command.taker_fee_bps, DEFAULT_TAKER_FEE_BPS);
        assert_eq!(command.order_id, "wire-order-0-0xabc");
        assert_eq!(
            command.order_type,
            PlaceOnlySpotOrderV2OrderType::Limit { tif: "Gtc".to_string() }
        );
    }

    #[test]
    fn trigger_order_builds_single_trigger_command() {
        let request = request_with("na", vec![trigger_order()]);

        let units = build_units(&request);

        let PlaceOnlySpotOrderV2Cmd::Single(command) = &units[0].command else {
            panic!("na grouping should build single command");
        };
        assert_eq!(
            command.order_type,
            PlaceOnlySpotOrderV2OrderType::Trigger {
                is_market: true,
                trigger_price: "915".to_string(),
                trigger_role: "sl".to_string(),
            }
        );
    }

    #[test]
    fn normal_tpsl_builds_one_parent_children_command() {
        let request = request_with("normalTpsl", vec![limit_order(None), trigger_order()]);

        let units = build_units(&request);

        assert_eq!(units.len(), 1);
        assert_eq!(units[0].input_order_count, 2);
        let PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, children } = &units[0].command else {
            panic!("normalTpsl should build grouped command");
        };
        assert_eq!(parent.order_id, "wire-order-0");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].order_id, "wire-order-1");
    }

    #[test]
    fn position_tpsl_execution_returns_unsupported_error_per_order() {
        let request = request_with("positionTpsl", vec![trigger_order(), trigger_order()]);

        let statuses = execute_with_default_outbound(request);

        assert_eq!(
            statuses,
            vec![
                reply::OrderStatusWire::Error {
                    error: UNSUPPORTED_POSITION_TPSL_ERROR.to_string(),
                },
                reply::OrderStatusWire::Error {
                    error: UNSUPPORTED_POSITION_TPSL_ERROR.to_string(),
                },
            ]
        );
    }

    #[test]
    fn placed_only_maps_to_resting() {
        let order = spot_order("taker-1", 42);
        let changes = PlaceMatchSpotOrderV2Changes::SinglePlacedOnly { created_order: order };

        let statuses = order_statuses_from_place_match_changes(&changes);

        assert_eq!(
            statuses,
            vec![reply::OrderStatusWire::Resting {
                resting: reply::RestingOrderStatusWire { oid: 42 },
            }]
        );
    }

    #[test]
    fn placed_and_matched_with_taker_trade_maps_to_filled() {
        let order = spot_order("taker-1", 43);
        let mut match_changes = empty_match_changes();
        match_changes.created_trades.push(SpotTrade::new(
            "trade-1".to_string(),
            "match-1".to_string(),
            10_001,
            "BTCUSDT".to_string(),
            "taker-1".to_string(),
            "maker-1".to_string(),
            "buyer".to_string(),
            "seller".to_string(),
            SpotOrderSide::Buy,
            100,
            2,
            1,
            1,
            2,
        ));
        let changes = PlaceMatchSpotOrderV2Changes::SinglePlacedAndMatched {
            created_taker_order: order,
            match_changes,
        };

        let statuses = order_statuses_from_place_match_changes(&changes);

        assert_eq!(
            statuses,
            vec![reply::OrderStatusWire::Filled {
                filled: reply::FilledOrderStatusWire {
                    total_sz: "2".to_string(),
                    avg_px: "100".to_string(),
                    oid: 43,
                },
            }]
        );
    }

    #[test]
    fn normal_tpsl_maps_parent_then_children_statuses() {
        let parent = spot_order("parent", 44);
        let child = spot_order("child", 45);
        let changes = PlaceMatchSpotOrderV2Changes::NormalTpslPlacedAndMatched {
            created_parent_order: parent,
            created_child_orders: vec![child],
            match_changes: empty_match_changes(),
        };

        let statuses = order_statuses_from_place_match_changes(&changes);

        assert_eq!(
            statuses,
            vec![
                reply::OrderStatusWire::Resting {
                    resting: reply::RestingOrderStatusWire { oid: 44 },
                },
                reply::OrderStatusWire::Resting {
                    resting: reply::RestingOrderStatusWire { oid: 45 },
                },
            ]
        );
    }
}
