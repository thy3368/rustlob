use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler, run_exchange_action,
};
use crate::exchange::common::validate::{
    validate_cloid, validate_common_fields, validate_hex_address,
};
use crate::exchange::common::wire::ExchangeRequestEnvelopeWire;
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

type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
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

struct OrderAction;

impl ExchangeActionHandler for OrderAction {
    type Request = RequestWire;
    type Reply = reply::OrderResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute<'a>(
        request: Self::Request,
        deps: &'a ExchangeActionDeps,
    ) -> ExchangeActionFuture<'a, Self::Reply> {
        Box::pin(execute(request, deps))
    }
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::OrderResponseWire, ExchangeHttpError> {
    run_exchange_action::<OrderAction>(body, deps).await
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "order" {
        return Err(OrderContractError::UnexpectedActionType(request.action.type_.clone()).into());
    }
    // 通用字段校验统一复用 shared validator，避免各 action 分叉签名语义。
    validate_common_fields(
        request.common.nonce,
        request.common.expires_after,
        &request.common.signature.r,
        &request.common.signature.s,
        request.common.signature.v,
        request.common.vault_address.as_deref(),
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.action.orders.is_empty() {
        return Err(OrderContractError::EmptyOrders.into());
    }
    if !matches!(request.action.grouping.as_str(), "na" | "normalTpsl" | "positionTpsl") {
        return Err(OrderContractError::InvalidGrouping.into());
    }
    if let Some(builder) = &request.action.builder {
        validate_hex_address(&builder.b).map_err(|_| {
            ExchangeHttpError::OrderContract(OrderContractError::InvalidBuilderAddress)
        })?;
    }
    for order in &request.action.orders {
        // 这里仅检查 wire 最小合法性；价格/数量的业务精度约束留给更内层 use case。
        if order.p.trim().is_empty() {
            return Err(OrderContractError::InvalidPrice.into());
        }
        if order.s.trim().is_empty() {
            return Err(OrderContractError::InvalidSize.into());
        }
        if let Some(cloid) = &order.c {
            validate_cloid(cloid)
                .map_err(|_| ExchangeHttpError::OrderContract(OrderContractError::InvalidCloid))?;
        }
        match (&order.t.limit, &order.t.trigger) {
            (Some(limit), None) => {
                if !matches!(limit.tif.as_str(), "Alo" | "Ioc" | "Gtc") {
                    return Err(OrderContractError::InvalidTimeInForce.into());
                }
            }
            (None, Some(trigger)) => {
                if trigger.trigger_px.trim().is_empty() {
                    return Err(OrderContractError::InvalidTriggerPrice.into());
                }
                if !matches!(trigger.tpsl.as_str(), "tp" | "sl") {
                    return Err(OrderContractError::InvalidTriggerKind.into());
                }
            }
            _ => return Err(OrderContractError::InvalidOrderType.into()),
        }
    }
    Ok(())
}

const STUB_FILLED_PREFIX: &str = "stub-filled";
const STUB_ERROR_PREFIX: &str = "stub-error";
const STUB_ERROR_MESSAGE: &str = "Order must have minimum value of $10.";
const STUB_OID_BASE: u64 = 77738308;

async fn execute(
    request: RequestWire,
    _deps: &ExchangeActionDeps,
) -> Result<reply::OrderResponseWire, ExchangeHttpError> {
    // 当前 inbound adapter 先返回稳定的 stub 形状，便于前后端与协议快照测试对齐。
    // 后续接入真实下单 use case 时，应由 core/operating 层决定回执状态。
    let statuses = request
        .action
        .orders
        .iter()
        .enumerate()
        .map(|(index, order)| {
            let oid = STUB_OID_BASE + index as u64;
            match order.c.as_deref() {
                Some(cloid) if cloid.starts_with(STUB_FILLED_PREFIX) => {
                    reply::OrderStatusWire::Filled {
                        filled: reply::FilledOrderStatusWire {
                            total_sz: order.s.clone(),
                            avg_px: order.p.clone(),
                            oid,
                        },
                    }
                }
                Some(cloid) if cloid.starts_with(STUB_ERROR_PREFIX) => {
                    reply::OrderStatusWire::Error { error: STUB_ERROR_MESSAGE.to_string() }
                }
                _ => reply::OrderStatusWire::Resting {
                    resting: reply::RestingOrderStatusWire { oid },
                },
            }
        })
        .collect();

    Ok(reply::OrderResponseWire {
        status: "ok",
        response: reply::OrderResponseEnvelopeWire {
            type_: "order",
            data: reply::OrderResponseDataWire { statuses },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exchange::test_support::{
        valid_order_request_json, valid_request_with_grouping_json,
    };

    #[test]
    fn parses_minimal_order_request() {
        let request = parse_json_request::<RequestWire>(valid_order_request_json())
            .expect("request should parse");

        assert_eq!(request.action.type_, "order");
        assert_eq!(request.action.orders.len(), 1);
        assert_eq!(request.action.orders[0].p, "1891.4");
        assert_eq!(request.common.expires_after, None);
        assert_eq!(request.common.vault_address, None);
    }

    #[test]
    fn parses_order_request_with_optional_fields() {
        let request = parse_json_request::<RequestWire>(
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
        let request = parse_json_request::<RequestWire>(
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
        assert!(matches!(error, ExchangeHttpError::OrderContract(OrderContractError::EmptyOrders)));
    }

    #[test]
    fn rejects_invalid_grouping() {
        let request =
            parse_json_request::<RequestWire>(&valid_request_with_grouping_json("unknown"))
                .expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert!(matches!(
            error,
            ExchangeHttpError::OrderContract(OrderContractError::InvalidGrouping)
        ));
    }

    #[test]
    fn rejects_invalid_signature_shape() {
        let request = parse_json_request::<RequestWire>(
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
        let request = parse_json_request::<RequestWire>(
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
        assert!(matches!(
            error,
            ExchangeHttpError::OrderContract(OrderContractError::InvalidOrderType)
        ));
    }

    #[actix_web::test]
    async fn success_json_snapshot_is_stable() {
        let request = parse_json_request::<RequestWire>(valid_order_request_json())
            .expect("request parses");
        let response =
            execute(request, &ExchangeActionDeps::default()).await.expect("response builds");

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "order",
    "data": {
      "statuses": [
        {
          "resting": {
            "oid": 77738308
          }
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
                        reply::OrderStatusWire::Error { error: STUB_ERROR_MESSAGE.to_string() },
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
        let request = parse_json_request::<RequestWire>(valid_order_request_json())
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
}
