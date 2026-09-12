use cmd_handler::command_use_case_def2::MiFamilyExecutionSpec;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use use_case_executor::trading::spot::cancel_spot_order_v2::execute_cancel_spot_order_v2_with_outbound;

use crate::command::exchange::actions::cancel::{
    CancelSpotOrderV2Request, DEFAULT_EXCHANGE_PARTY_ID, SpotOrderV2CancelExecutionSpec,
    cancel_execution_error_message, execute_cancel_spot_order_v2,
};
use crate::command::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::command::exchange::common::validate::{validate_cloid, validate_envelope_common};
use crate::command::exchange::common::wire::ExchangeRequestEnvelopeWire;
use crate::command::exchange::error::ExchangeHttpError;
#[cfg(test)]
use crate::common::parse::parse_json_request;

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
    pub use crate::command::exchange::actions::cancel::reply::{
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
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    let statuses = request
        .action
        .cancels
        .iter()
        .map(|cancel| {
            let cancel_request = CancelSpotOrderV2Request::from_cloid(
                party_id.clone(),
                cancel.asset,
                cancel.cloid.clone(),
            );
            let command = SpotOrderV2CancelExecutionSpec::command(&cancel_request);
            match execute_cancel_spot_order_v2(&command) {
                Ok(_) => reply::CancelByCloidStatusWire::Success("success"),
                Err(error) => reply::CancelByCloidStatusWire::Error {
                    error: cancel_execution_error_message(error),
                },
            }
        })
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
    use std::sync::{Arc, Mutex};

    use cmd_handler::command_use_case_def2::{StateSink, StateSource};
    use example_core_use_case::{
        SpotOrderV2CommandV3, SpotOrderV2GivenStateV3, SpotOrderV2UseCaseFamilyV3,
    };

    use super::*;
    use crate::command::exchange::actions::cancel::{
        CancelSpotOrderV2LookupV3, CancelSpotOrderV2Request,
    };

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
            CancelSpotOrderV2LookupV3::Cloid("0x1234567890abcdef1234567890abcdef".to_string())
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
        observed_lookup: Arc<Mutex<Option<CancelSpotOrderV2LookupV3>>>,
    }

    impl StateSource<SpotOrderV2UseCaseFamilyV3> for ObservingCancelOutbound {
        type Error = FakeOutboundError;

        fn load_given_state(
            &self,
            cmd: &SpotOrderV2CommandV3,
        ) -> Result<SpotOrderV2GivenStateV3, Self::Error> {
            let SpotOrderV2CommandV3::Cancel(request) = cmd else {
                panic!("expected cancel command");
            };
            *self.observed_lookup.lock().expect("lookup observation lock should be available") =
                Some(request.lookup.clone());
            Err(FakeOutboundError)
        }
    }

    impl StateSink<SpotOrderV2UseCaseFamilyV3> for ObservingCancelOutbound {
        type Error = FakeOutboundError;

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
    fn shared_cancel_executor_uses_cloid_lookup() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request parses");
        let cancel = &request.action.cancels[0];
        let cancel_request = CancelSpotOrderV2Request::from_cloid(
            DEFAULT_EXCHANGE_PARTY_ID.to_string(),
            cancel.asset,
            cancel.cloid.clone(),
        );
        let command = SpotOrderV2CancelExecutionSpec::command(&cancel_request);
        let outbound = ObservingCancelOutbound::default();

        let error = execute_cancel_spot_order_v2_with_outbound(&command, &outbound)
            .expect_err("fake outbound should fail while loading state");

        assert_eq!(
            *outbound.observed_lookup.lock().expect("lookup observation lock should be available"),
            Some(CancelSpotOrderV2LookupV3::Cloid(
                "0x1234567890abcdef1234567890abcdef".to_string()
            ))
        );
        assert_eq!(cancel_execution_error_message(error), "load_state failed: fake outbound error");
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
