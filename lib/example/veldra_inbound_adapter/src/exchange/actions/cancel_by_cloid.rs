use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::{validate_cloid, validate_envelope_common};
use crate::exchange::common::wire::ExchangeRequestEnvelopeWire;
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum CancelByCloidContractError {
    #[error("Unexpected `action.type` for cancelByCloid handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.cancels`. Expected at least one cancel entry.")]
    EmptyCancels,
    #[error("Invalid `action.cancels[].cloid`. Expected a 128-bit hex client order id.")]
    InvalidCloid,
    #[error("Invalid `action.f`. `f` must be omitted when false.")]
    InvalidFastFlag,
}

pub mod reply {
    pub use crate::exchange::actions::cancel::reply::{
        CancelResponseDataWire as CancelByCloidResponseDataWire,
        CancelResponseEnvelopeWire as CancelByCloidResponseEnvelopeWire,
        CancelResponseWire as CancelByCloidResponseWire,
        CancelStatusWire as CancelByCloidStatusWire,
    };
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    cancels: Vec<CancelWire>,
    f: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CancelWire {
    asset: u32,
    cloid: String,
}

pub(crate) struct CancelByCloidAction;

impl ExchangeActionHandler for CancelByCloidAction {
    type Request = RequestWire;
    type Reply = reply::CancelByCloidResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "cancelByCloid" {
        return Err(ExchangeHttpError::contract(CancelByCloidContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.cancels.is_empty() {
        return Err(ExchangeHttpError::contract(CancelByCloidContractError::EmptyCancels));
    }
    if matches!(request.action.f, Some(false)) {
        return Err(ExchangeHttpError::contract(CancelByCloidContractError::InvalidFastFlag));
    }
    for cancel in &request.action.cancels {
        validate_cloid(&cancel.cloid)
            .map_err(|_| ExchangeHttpError::contract(CancelByCloidContractError::InvalidCloid))?;
    }
    Ok(())
}

async fn execute(
    request: RequestWire,
) -> Result<reply::CancelByCloidResponseWire, ExchangeHttpError> {
    // 官方 exchange 文档没有给出 cancelByCloid 的成功响应示例。
    // 当前采用与 cancel 一致的最小成功形状，后续若拿到官方响应样例再收敛。
    let statuses = request
        .action
        .cancels
        .iter()
        .map(|_| reply::CancelByCloidStatusWire::Success("success"))
        .collect();
    Ok(reply::CancelByCloidResponseWire {
        status: "ok",
        response: reply::CancelByCloidResponseEnvelopeWire {
            type_: "cancel",
            data: reply::CancelByCloidResponseDataWire { statuses },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exchange::actions::cancel::{CancelSpotOrderV2Lookup, CancelSpotOrderV2Request};

    #[test]
    fn parses_request() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.cancels.len(), 1);
    }

    #[test]
    fn rejects_false_fast_flag() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": {
                    "type": "cancelByCloid",
                    "cancels": [{ "asset": 10000, "cloid": "0x1234567890abcdef1234567890abcdef" }],
                    "f": false
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
        assert_eq!(error.to_string(), "Invalid `action.f`. `f` must be omitted when false.");
    }

    #[test]
    fn maps_cancel_by_cloid_wire_to_cancel_lookup() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request parses");
        let cancel = &request.action.cancels[0];

        let mapped = CancelSpotOrderV2Request::from_cloid(
            "buyer".to_string(),
            cancel.asset,
            cancel.cloid.clone(),
        );

        assert_eq!(mapped.asset, 10000);
        assert_eq!(
            mapped.lookup,
            CancelSpotOrderV2Lookup::Cloid("0x1234567890abcdef1234567890abcdef".to_string())
        );
    }

    #[actix_web::test]
    async fn reply_snapshot_is_stable() {
        let response = execute(
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
                .expect("request parses"),
        )
        .await
        .expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"cancel\",\n    \"data\": {\n      \"statuses\": [\n        \"success\"\n      ]\n    }\n  }\n}"
        );
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": {
                "type": "cancelByCloid",
                "cancels": [{ "asset": 10000, "cloid": "0x1234567890abcdef1234567890abcdef" }]
            },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
