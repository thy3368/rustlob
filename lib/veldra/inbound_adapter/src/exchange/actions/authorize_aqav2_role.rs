use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{CommonExchangeFields, DefaultExchangeResponseEnvelopeWire};
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
    pub use crate::exchange::common::wire::DefaultExchangeResponseWire as AuthorizeAqav2RoleResponseWire;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestWire {
    action: ActionWire,
    #[serde(flatten)]
    common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    token: u64,
    role: String,
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::AuthorizeAqav2RoleResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "authorizeAqav2Role" {
        return Err(AuthorizeAqav2RoleContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )
        .into());
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
        return Err(AuthorizeAqav2RoleContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(AuthorizeAqav2RoleContractError::ExpiresAfterNotSupported.into());
    }
    if !matches!(request.action.role.as_str(), "technical" | "treasury") {
        return Err(AuthorizeAqav2RoleContractError::InvalidRole.into());
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::AuthorizeAqav2RoleResponseWire, ExchangeHttpError> {
    Ok(reply::AuthorizeAqav2RoleResponseWire {
        status: "ok",
        response: DefaultExchangeResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request() {
        let request = parse(valid_request_json()).expect("request should parse");
        assert_eq!(request.action.role, "technical");
    }

    #[test]
    fn rejects_invalid_role() {
        let request = parse(
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
        let response =
            execute(&ExchangeActionDeps::default()).await.expect("response should build");
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
