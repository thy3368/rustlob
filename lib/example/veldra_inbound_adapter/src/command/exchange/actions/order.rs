use cmd_handler::command_use_case_def2::{MiFamilyExecutionError, MiFamilyExecutionSpec};
use example_core_use_case::{
    PlaceSpotOrderV2CmdV3, SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3,
    SpotOrderV2UseCaseFamilyV3,
};
use serde::{Deserialize, Serialize};
pub use use_case_executor::trading::spot::place_spot_order_v2::execute_place_spot_order_v2;

use crate::command::exchange::actions::cancel::DEFAULT_EXCHANGE_PARTY_ID;
use crate::command::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::command::exchange::common::validate::{
    validate_cloid, validate_envelope_common, validate_hex_address,
};
use crate::command::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_statuses_response};
use crate::command::exchange::error::ExchangeHttpError;

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

#[allow(dead_code)]
pub struct SpotOrderV2PlaceExecutionSpec;

impl MiFamilyExecutionSpec<SpotOrderV2UseCaseFamilyV3> for SpotOrderV2PlaceExecutionSpec {
    type Request = PlaceSpotOrderV2Request;

    fn command(request: &Self::Request) -> SpotOrderV2CommandV3 {
        SpotOrderV2CommandV3::Place(PlaceSpotOrderV2CmdV3 {
            party_id: request.party_id.clone(),
            asset: request.asset,
            is_buy: request.is_buy,
            price: decimal_wire_to_core_units(&request.price),
            size: decimal_wire_to_core_units(&request.size),
            tif: request.tif.clone(),
            cloid: request.cloid.clone(),
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

#[allow(dead_code)]
fn order_status_from_spot_order_v2_changes(
    request: &PlaceSpotOrderV2Request,
    changes: &SpotOrderV2CaseChangesV3,
) -> reply::OrderStatusWire {
    let SpotOrderV2CaseChangesV3::Place(place) = changes else {
        return reply::OrderStatusWire::Error { error: "unexpected spot order branch".to_string() };
    };

    let filled_qty: u64 = place
        .created_trades
        .iter()
        .filter(|trade| trade.taker_order_id == place.updated_taker_order.after.order_id())
        .map(|trade| trade.qty)
        .sum();

    if filled_qty > 0 {
        return reply::OrderStatusWire::Filled {
            filled: reply::FilledOrderStatusWire {
                total_sz: filled_qty.to_string(),
                avg_px: request.price.clone(),
                oid: place.updated_taker_order.after.exchange_oid().unwrap_or(0),
            },
        };
    }

    reply::OrderStatusWire::Resting {
        resting: reply::RestingOrderStatusWire {
            oid: place.updated_taker_order.after.exchange_oid().unwrap_or(0),
        },
    }
}

async fn execute(request: RequestWire) -> Result<reply::OrderResponseWire, ExchangeHttpError> {
    let statuses = execute_with_default_outbound(request);
    Ok(ok_statuses_response("order", statuses))
}

fn execute_with_default_outbound(request: RequestWire) -> Vec<reply::OrderStatusWire> {
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    request
        .action
        .orders
        .iter()
        .map(|order| match PlaceSpotOrderV2Request::from_wire_order(party_id.clone(), order) {
            Ok(place_request) => {
                let command = SpotOrderV2PlaceExecutionSpec::command(&place_request);
                match execute_place_spot_order_v2(&command) {
                    Ok(result) => {
                        order_status_from_spot_order_v2_changes(&place_request, &result.changes)
                    }
                    Err(error) => reply::OrderStatusWire::Error {
                        error: order_execution_error_message(error),
                    },
                }
            }
            Err(error) => reply::OrderStatusWire::Error { error: error.to_string() },
        })
        .collect()
}

fn order_execution_error_message<BE, OE>(error: MiFamilyExecutionError<BE, OE>) -> String
where
    BE: std::fmt::Display,
    OE: std::fmt::Display,
{
    match error {
        MiFamilyExecutionError::Business(error) => error.to_string(),
        MiFamilyExecutionError::ProjectEvents(error) => {
            format!("project replayable events failed: {error}")
        }
        MiFamilyExecutionError::LoadState(error) => format!("load_state failed: {error}"),
        MiFamilyExecutionError::Persist(error) => format!("persist failed: {error}"),
        MiFamilyExecutionError::Replay(error) => format!("replay failed: {error}"),
        MiFamilyExecutionError::Publish(error) => format!("publish failed: {error}"),
    }
}
