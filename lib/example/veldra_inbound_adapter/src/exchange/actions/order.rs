use cmd_handler::command_use_case_def2::{
    MiFamilyExecutionError, MiFamilyExecutionResult, MiFamilyExecutionSpec, MiFamilyOutbound,
    MiStateMachineFamilyExecutor,
};
use example_core::{
    PlaceSpotOrderV2CmdV3, SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3, SpotOrderV2GivenStateV3,
    SpotOrderV2UseCaseFamilyV3,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::common::parse::parse_json_request;
use crate::exchange::actions::cancel::DEFAULT_EXCHANGE_PARTY_ID;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::{
    validate_cloid, validate_envelope_common, validate_hex_address,
};
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_statuses_response};
use crate::exchange::error::ExchangeHttpError;

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

    use crate::exchange::common::wire::{
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

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultSpotOrderV2PlaceOutboundError {
    #[error("spot order v2 place state is not wired for default HTTP path")]
    StateUnavailable,
}

#[derive(Debug, Default)]
pub struct DefaultSpotOrderV2PlaceOutbound;

impl MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3> for DefaultSpotOrderV2PlaceOutbound {
    type Error = DefaultSpotOrderV2PlaceOutboundError;

    fn load_given_state(
        &self,
        _cmd: &SpotOrderV2CommandV3,
    ) -> Result<SpotOrderV2GivenStateV3, Self::Error> {
        Err(DefaultSpotOrderV2PlaceOutboundError::StateUnavailable)
    }

    fn persist(&self, _events: &[cmd_handler::EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay(&self, _events: &[cmd_handler::EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn publish(&self, _events: &[cmd_handler::EntityReplayableEvent]) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[allow(dead_code)]
pub fn execute_place_spot_order_v2<OB>(
    request: &PlaceSpotOrderV2Request,
    outbound: &OB,
) -> Result<
    MiFamilyExecutionResult<SpotOrderV2CaseChangesV3>,
    MiFamilyExecutionError<example_core::SpotOrderV2UseCaseFamilyV3Error, OB::Error>,
>
where
    OB: MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3>,
{
    MiStateMachineFamilyExecutor
        .execute::<SpotOrderV2UseCaseFamilyV3, SpotOrderV2PlaceExecutionSpec, OB>(
            &SpotOrderV2UseCaseFamilyV3,
            request,
            outbound,
        )
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
    let outbound = DefaultSpotOrderV2PlaceOutbound;
    let statuses = execute_with_outbound(request, &outbound);
    Ok(ok_statuses_response("order", statuses))
}

fn execute_with_outbound<OB>(request: RequestWire, outbound: &OB) -> Vec<reply::OrderStatusWire>
where
    OB: MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3>,
    OB::Error: std::fmt::Display,
{
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    request
        .action
        .orders
        .iter()
        .map(|order| match PlaceSpotOrderV2Request::from_wire_order(party_id.clone(), order) {
            Ok(place_request) => match execute_place_spot_order_v2(&place_request, outbound) {
                Ok(result) => {
                    order_status_from_spot_order_v2_changes(&place_request, &result.changes)
                }
                Err(error) => {
                    reply::OrderStatusWire::Error { error: order_execution_error_message(error) }
                }
            },
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

#[cfg(test)]
mod tests {
    use std::fmt;

    use cmd_handler::command_use_case_def2::MiFamilyOutbound;
    use example_core::{
        Balance, SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderTimeInForce,
        SpotOrderV2,
    };

    use super::*;
    use crate::exchange::test_support::{
        valid_order_request_json, valid_request_with_grouping_json,
    };

    #[test]
    fn parses_minimal_order_request() {
        let request =
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_order_request_json())
                .expect("request should parse");

        assert_eq!(request.action.type_, "order");
        assert_eq!(request.action.orders.len(), 1);
        assert_eq!(request.action.orders[0].p, "1891.4");
        assert_eq!(request.common.expires_after, None);
        assert_eq!(request.common.vault_address, None);
    }

    #[test]
    fn parses_order_request_with_optional_fields() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000,
                        "b": false,
                        "p": "1800",
                        "s": "0.5",
                        "r": true,
                        "t": {
                            "trigger": {
                                "isMarket": true,
                                "triggerPx": "1790",
                                "tpsl": "sl"
                            }
                        },
                        "c": "0x1234567890abcdef1234567890abcdef"
                    }],
                    "grouping": "positionTpsl",
                    "builder": {
                        "b": "0x1111111111111111111111111111111111111111",
                        "f": 10
                    }
                },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 28
                },
                "vaultAddress": "0x2222222222222222222222222222222222222222",
                "expiresAfter": 1710000001000
            }"#,
        )
        .expect("request should parse");

        assert_eq!(request.action.grouping, "positionTpsl");
        assert!(request.action.builder.is_some());
        assert_eq!(
            request.action.orders[0].c.as_deref(),
            Some("0x1234567890abcdef1234567890abcdef")
        );
        assert_eq!(
            request.common.vault_address.as_deref(),
            Some("0x2222222222222222222222222222222222222222")
        );
        assert_eq!(request.common.expires_after, Some(1710000001000));
    }

    #[test]
    fn rejects_empty_orders() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "order",
                    "orders": [],
                    "grouping": "na"
                },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(error.to_string(), "`action.orders` must contain at least one order.");
    }

    #[test]
    fn rejects_invalid_grouping() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            &valid_request_with_grouping_json("unknown"),
        )
        .expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(
            error.to_string(),
            "Invalid `action.grouping`. Expected one of `na`, `normalTpsl`, `positionTpsl`."
        );
    }

    #[test]
    fn rejects_invalid_signature_shape() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000,
                        "b": true,
                        "p": "1891.4",
                        "s": "0.02",
                        "r": false,
                        "t": { "limit": { "tif": "Gtc" } }
                    }],
                    "grouping": "na"
                },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1234",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(
            error.to_string(),
            "Invalid `signature` shape. Expected hex `r`/`s` and numeric `v`."
        );
    }

    #[test]
    fn rejects_invalid_order_type_shape() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000,
                        "b": true,
                        "p": "1891.4",
                        "s": "0.02",
                        "r": false,
                        "t": {
                            "limit": { "tif": "Gtc" },
                            "trigger": {
                                "isMarket": false,
                                "triggerPx": "1900",
                                "tpsl": "tp"
                            }
                        }
                    }],
                    "grouping": "na"
                },
                "nonce": 1710000000000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(
            error.to_string(),
            "Invalid `action.orders[].t`. Expected exactly one of `limit` or `trigger`."
        );
    }

    #[actix_web::test]
    async fn default_path_returns_item_error_when_state_is_unavailable() {
        let request =
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_order_request_json())
                .expect("request parses");
        let response = execute(request).await.expect("response builds");

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "order",
    "data": {
      "statuses": [
        {
          "error": "load_state failed: spot order v2 place state is not wired for default HTTP path"
        }
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn resting_filled_and_error_shapes_are_stable() {
        let response = reply::OrderResponseWire {
            status: "ok",
            response: reply::OrderResponseEnvelopeWire {
                type_: "order",
                data: reply::OrderResponseDataWire {
                    statuses: vec![
                        reply::OrderStatusWire::Resting {
                            resting: reply::RestingOrderStatusWire { oid: 1 },
                        },
                        reply::OrderStatusWire::Filled {
                            filled: reply::FilledOrderStatusWire {
                                total_sz: "0.02".to_string(),
                                avg_px: "1891.4".to_string(),
                                oid: 2,
                            },
                        },
                        reply::OrderStatusWire::Error {
                            error: "Order must have minimum value of $10.".to_string(),
                        },
                    ],
                },
            },
        };

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "order",
    "data": {
      "statuses": [
        {
          "resting": {
            "oid": 1
          }
        },
        {
          "filled": {
            "totalSz": "0.02",
            "avgPx": "1891.4",
            "oid": 2
          }
        },
        {
          "error": "Order must have minimum value of $10."
        }
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn request_parse_snapshot_is_stable() {
        let request =
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_order_request_json())
                .expect("request parses");
        let actual = serde_json::to_string_pretty(&request).expect("request serializes");
        let expected = r#"{
  "action": {
    "type": "order",
    "orders": [
      {
        "a": 10000,
        "b": true,
        "p": "1891.4",
        "s": "0.02",
        "r": false,
        "t": {
          "limit": {
            "tif": "Gtc"
          },
          "trigger": null
        },
        "c": null
      }
    ],
    "grouping": "na",
    "builder": null
  },
  "nonce": 1710000000000,
  "signature": {
    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
    "v": 27
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FakeOutboundError;

    impl fmt::Display for FakeOutboundError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("fake outbound error")
        }
    }

    impl std::error::Error for FakeOutboundError {}

    #[derive(Debug, Default)]
    struct FakeSpotOrderV2Outbound;

    impl MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3> for FakeSpotOrderV2Outbound {
        type Error = FakeOutboundError;

        fn load_given_state(
            &self,
            cmd: &SpotOrderV2CommandV3,
        ) -> Result<SpotOrderV2GivenStateV3, Self::Error> {
            let SpotOrderV2CommandV3::Place(request) = cmd else {
                panic!("expected place command");
            };
            let price = request.price.parse::<u64>().map_err(|_| FakeOutboundError)?;
            let qty = request.size.parse::<u64>().map_err(|_| FakeOutboundError)?;
            let taker_notional = qty.checked_mul(price).ok_or(FakeOutboundError)?;
            let taker_reservation = SpotOrderV2::principal_reservation(
                "taker-buy",
                request.party_id.as_str(),
                SpotOrderSide::Buy,
                qty,
                price,
                "BTC",
                "USDT",
            )
            .map_err(|_| FakeOutboundError)?;
            let taker_fee_reservation = SpotOrderV2::fee_reservation(
                "taker-buy",
                request.party_id.as_str(),
                SpotOrderSide::Buy,
                qty,
                price,
                "USDT",
                10,
                20,
            )
            .map_err(|_| FakeOutboundError)?;
            let taker_fee_hold = taker_fee_reservation.remaining_amount;
            let taker_hold = taker_notional.checked_add(taker_fee_hold).ok_or(FakeOutboundError)?;
            let taker_order = SpotOrderV2::new_with_fee_reservation(
                "taker-buy".to_string(),
                request.asset,
                None,
                request.party_id.clone(),
                "BTCUSDT".to_string(),
                SpotOrderSide::Buy,
                SpotOrderExecution::Limit { price },
                SpotOrderTimeInForce::Gtc,
                qty,
                0,
                SpotOrderStatus::Open,
                None,
                0,
                taker_notional,
                taker_reservation,
                taker_fee_reservation,
                request.cloid.clone(),
                1,
            );
            let maker_reservation = SpotOrderV2::principal_reservation(
                "maker-1",
                "seller",
                SpotOrderSide::Sell,
                1,
                price,
                "BTC",
                "USDT",
            )
            .map_err(|_| FakeOutboundError)?;
            let maker_fee_reservation = SpotOrderV2::fee_reservation(
                "maker-1",
                "seller",
                SpotOrderSide::Sell,
                1,
                price,
                "USDT",
                10,
                20,
            )
            .map_err(|_| FakeOutboundError)?;
            let maker_fee_hold = maker_fee_reservation.remaining_amount;
            let maker_orders = vec![SpotOrderV2::new_with_fee_reservation(
                "maker-1".to_string(),
                request.asset,
                Some(100),
                "seller".to_string(),
                "BTCUSDT".to_string(),
                SpotOrderSide::Sell,
                SpotOrderExecution::Limit { price },
                SpotOrderTimeInForce::Gtc,
                1,
                0,
                SpotOrderStatus::Open,
                None,
                1,
                0,
                maker_reservation,
                maker_fee_reservation,
                None,
                1,
            )];

            Ok(SpotOrderV2GivenStateV3::Place {
                settlement_balances: vec![
                    Balance::new(
                        request.party_id.clone(),
                        "USDT".to_string(),
                        taker_hold,
                        taker_hold,
                        1,
                    ),
                    Balance::new(request.party_id.clone(), "BTC".to_string(), 0, 0, 1),
                    Balance::new("seller".to_string(), "BTC".to_string(), 0, 1, 1),
                    Balance::new("seller".to_string(), "USDT".to_string(), 0, maker_fee_hold, 1),
                    Balance::new("fee".to_string(), "USDT".to_string(), 0, 0, 1),
                ],
                taker_order,
                maker_orders,
                base_asset_id: "BTC".to_string(),
                quote_asset_id: "USDT".to_string(),
                fee_account_id: "fee".to_string(),
                maker_fee_bps: 10,
                taker_fee_bps: 20,
            })
        }

        fn persist(
            &self,
            _events: &[cmd_handler::EntityReplayableEvent],
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn replay(
            &self,
            _events: &[cmd_handler::EntityReplayableEvent],
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn publish(
            &self,
            _events: &[cmd_handler::EntityReplayableEvent],
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn spot_order_v2_place_request_maps_wire_and_executes_with_fake_outbound() {
        let wire = parse_json_request::<RequestWire, ExchangeHttpError>(valid_order_request_json())
            .expect("request parses");
        let request =
            PlaceSpotOrderV2Request::from_wire_order("buyer".to_string(), &wire.action.orders[0])
                .expect("wire order maps");

        assert_eq!(request.asset, 10000);
        assert!(request.is_buy);
        assert_eq!(request.price, "1891.4");
        assert_eq!(request.size, "0.02");
        assert_eq!(request.tif, "Gtc");

        let result = execute_place_spot_order_v2(&request, &FakeSpotOrderV2Outbound)
            .expect("spot order v2 family should execute");
        let status = order_status_from_spot_order_v2_changes(&request, &result.changes);

        assert_eq!(
            status,
            reply::OrderStatusWire::Filled {
                filled: reply::FilledOrderStatusWire {
                    total_sz: "1".to_string(),
                    avg_px: "1891.4".to_string(),
                    oid: 0
                }
            }
        );
    }
}
