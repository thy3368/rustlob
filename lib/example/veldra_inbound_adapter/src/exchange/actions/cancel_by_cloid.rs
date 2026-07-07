use cmd_handler::command_use_case_def2::MiFamilyOutbound;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::common::parse::parse_json_request;
use crate::exchange::actions::cancel::{
    CancelSpotOrderV2Request, DEFAULT_EXCHANGE_PARTY_ID, DefaultSpotOrderV2CancelOutbound,
    SpotOrderV2CancelLoadedState, cancel_execution_error_message, execute_cancel_spot_order_v2,
};
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
    let outbound = DefaultSpotOrderV2CancelOutbound;
    let statuses = execute_with_outbound(request, &outbound);
    Ok(reply::CancelByCloidResponseWire {
        status: "ok",
        response: reply::CancelByCloidResponseEnvelopeWire {
            type_: "cancel",
            data: reply::CancelByCloidResponseDataWire { statuses },
        },
    })
}

fn execute_with_outbound<OB>(
    request: RequestWire,
    outbound: &OB,
) -> Vec<reply::CancelByCloidStatusWire>
where
    OB: MiFamilyOutbound<CancelSpotOrderV2Request, SpotOrderV2CancelLoadedState>,
    OB::Error: std::fmt::Display,
{
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    request
        .action
        .cancels
        .iter()
        .map(|cancel| {
            let cancel_request = CancelSpotOrderV2Request::from_cloid(
                party_id.clone(),
                cancel.asset,
                cancel.cloid.clone(),
            );
            match execute_cancel_spot_order_v2(&cancel_request, outbound) {
                Ok(_) => reply::CancelByCloidStatusWire::Success("success"),
                Err(error) => reply::CancelByCloidStatusWire::Error {
                    error: cancel_execution_error_message(error),
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use cmd_handler::command_use_case_def2::MiFamilyOutbound;

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
    async fn default_path_returns_item_error_when_state_is_unavailable() {
        let response = execute(
            parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
                .expect("request parses"),
        )
        .await
        .expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"cancel\",\n    \"data\": {\n      \"statuses\": [\n        {\n          \"error\": \"load_state failed: spot order v2 cancel state is not wired for default HTTP path\"\n        }\n      ]\n    }\n  }\n}"
        );
    }

    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    #[error("fake outbound error")]
    struct FakeOutboundError;

    #[derive(Debug, Default)]
    struct ObservingCancelOutbound {
        observed_lookup: Arc<Mutex<Option<CancelSpotOrderV2Lookup>>>,
    }

    impl MiFamilyOutbound<CancelSpotOrderV2Request, SpotOrderV2CancelLoadedState>
        for ObservingCancelOutbound
    {
        type Error = FakeOutboundError;

        fn load_state(
            &self,
            request: &CancelSpotOrderV2Request,
        ) -> Result<SpotOrderV2CancelLoadedState, Self::Error> {
            *self.observed_lookup.lock().expect("lookup observation lock should be available") =
                Some(request.lookup.clone());
            Err(FakeOutboundError)
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
    fn execute_uses_cloid_lookup_for_shared_cancel_executor() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request parses");
        let outbound = ObservingCancelOutbound::default();

        let statuses = execute_with_outbound(request, &outbound);

        assert_eq!(
            *outbound.observed_lookup.lock().expect("lookup observation lock should be available"),
            Some(CancelSpotOrderV2Lookup::Cloid("0x1234567890abcdef1234567890abcdef".to_string()))
        );
        assert_eq!(
            statuses,
            vec![reply::CancelByCloidStatusWire::Error {
                error: "load_state failed: fake outbound error".to_string()
            }]
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
