use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::{ExchangeActionFuture, ExchangeActionHandler};
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_default_response};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum AuthorizeAqav2RoleContractError {
    #[error("Unexpected `action.type` for authorizeAqav2Role handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.role`. Expected `technical` or `treasury`.")]
    InvalidRole,
    #[error("`vaultAddress` is not supported for `authorizeAqav2Role`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `authorizeAqav2Role`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as AuthorizeAqav2RoleResponseWire;
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    token: u64,
    role: String,
}

pub(crate) struct AuthorizeAqav2RoleAction;

impl ExchangeActionHandler for AuthorizeAqav2RoleAction {
    type Request = RequestWire;
    type Reply = reply::AuthorizeAqav2RoleResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(_request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute())
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "authorizeAqav2Role" {
        return Err(ExchangeHttpError::contract(
            AuthorizeAqav2RoleContractError::UnexpectedActionType(request.action.type_.clone()),
        ));
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
            AuthorizeAqav2RoleContractError::VaultAddressNotSupported,
        ));
    }
    if request.common.expires_after.is_some() {
        return Err(ExchangeHttpError::contract(
            AuthorizeAqav2RoleContractError::ExpiresAfterNotSupported,
        ));
    }
    if !matches!(request.action.role.as_str(), "technical" | "treasury") {
        return Err(ExchangeHttpError::contract(AuthorizeAqav2RoleContractError::InvalidRole));
    }
    Ok(())
}

async fn execute() -> Result<reply::AuthorizeAqav2RoleResponseWire, ExchangeHttpError> {
    Ok(ok_default_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request =
            parse_json_request::<RequestWire>(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.role, "technical");
    }

    #[test]
    fn rejects_invalid_role() {
        let request = parse_json_request::<RequestWire>(
            br#"{
                "action": { "type": "authorizeAqav2Role", "token": 0, "role": "ops" },
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
        assert_eq!(error.to_string(), "Invalid `action.role`. Expected `technical` or `treasury`.");
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
            "action": { "type": "authorizeAqav2Role", "token": 0, "role": "technical" },
            "nonce": 1710000000000,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }"#
    }
}
