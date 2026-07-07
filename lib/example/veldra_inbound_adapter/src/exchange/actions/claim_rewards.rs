use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_default_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum ClaimRewardsContractError {
    #[error("Unexpected `action.type` for claimRewards handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("`vaultAddress` is not supported for `claimRewards`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `claimRewards`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as ClaimRewardsResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
}

pub(crate) struct ClaimRewardsAction;

impl ExchangeActionHandler for ClaimRewardsAction {
    type Request = RequestWire;
    type Reply = reply::ClaimRewardsResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "claimRewards" {
        return Err(ExchangeHttpError::contract(ClaimRewardsContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_common_fields(
        request.common.nonce,
        None,
        &request.common.signature.r,
        &request.common.signature.s,
        request.common.signature.v,
        None,
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.common.vault_address.is_some() {
        return Err(ExchangeHttpError::contract(
            ClaimRewardsContractError::VaultAddressNotSupported,
        ));
    }
    if request.common.expires_after.is_some() {
        return Err(ExchangeHttpError::contract(
            ClaimRewardsContractError::ExpiresAfterNotSupported,
        ));
    }
    Ok(())
}

async fn execute() -> Result<reply::ClaimRewardsResponseWire, ExchangeHttpError> {
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(valid_request_json())
            .expect("request should parse");
        assert_eq!(request.action.type_, "claimRewards");
    }

    #[test]
    fn rejects_expires_after() {
        let request = parse_json_request::<RequestWire, ExchangeHttpError>(
            br#"{
                "action": { "type": "claimRewards" },
                "nonce": 1710000000000,
                "expiresAfter": 1710000005000,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }"#,
        )
        .expect("request parses");
        let error = validate(&request).expect_err("validation should fail");
        assert_eq!(error.to_string(), "`expiresAfter` is not supported for `claimRewards`.");
    }

    #[actix_web::test]
    async fn reply_snapshot_is_stable() {
        let response = execute().await.expect("response should build");
        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        assert_eq!(
            actual,
            "{\n  \"status\": \"ok\",\n  \"response\": {\n    \"type\": \"default\"\n  }\n}"
        );
    }

    fn valid_request_json() -> &'static [u8] {
        br#"{
            "action": { "type": "claimRewards" },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
