use serde::{Deserialize, Serialize};

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::common::wire::{CommonExchangeFields, ExchangeEmptyResponseEnvelopeWire};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum UserOutcomeContractError {
    #[error("Unexpected `action.type` for userOutcome handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error(
        "Invalid `action`. Expected exactly one of `splitOutcome`, `mergeOutcome`, `mergeQuestion`, or `negateOutcome`."
    )]
    InvalidVariant,
    #[error("Invalid outcome amount. Expected a non-empty decimal string when provided.")]
    InvalidAmount,
    #[error("`vaultAddress` is not supported for `userOutcome`.")]
    VaultAddressNotSupported,
    #[error("`expiresAfter` is not supported for `userOutcome`.")]
    ExpiresAfterNotSupported,
}

pub mod reply {
    pub use crate::exchange::common::wire::ExchangeEmptyResponseWire as UserOutcomeResponseWire;
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
    #[serde(rename = "splitOutcome")]
    split_outcome: Option<AmountOutcomeWire>,
    #[serde(rename = "mergeOutcome")]
    merge_outcome: Option<OptionalAmountOutcomeWire>,
    #[serde(rename = "mergeQuestion")]
    merge_question: Option<OptionalAmountQuestionWire>,
    #[serde(rename = "negateOutcome")]
    negate_outcome: Option<AmountQuestionOutcomeWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AmountOutcomeWire {
    outcome: u64,
    amount: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionalAmountOutcomeWire {
    outcome: u64,
    amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionalAmountQuestionWire {
    question: u64,
    amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AmountQuestionOutcomeWire {
    question: u64,
    outcome: u64,
    amount: String,
}

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::UserOutcomeResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<RequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "userOutcome" {
        return Err(
            UserOutcomeContractError::UnexpectedActionType(request.action.type_.clone()).into()
        );
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
        return Err(UserOutcomeContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(UserOutcomeContractError::ExpiresAfterNotSupported.into());
    }
    let variant_count = [
        request.action.split_outcome.is_some(),
        request.action.merge_outcome.is_some(),
        request.action.merge_question.is_some(),
        request.action.negate_outcome.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    if variant_count != 1 {
        return Err(UserOutcomeContractError::InvalidVariant.into());
    }
    if let Some(split) = &request.action.split_outcome {
        if split.amount.trim().is_empty() {
            return Err(UserOutcomeContractError::InvalidAmount.into());
        }
    }
    if let Some(merge) = &request.action.merge_outcome {
        if matches!(merge.amount.as_deref(), Some(amount) if amount.trim().is_empty()) {
            return Err(UserOutcomeContractError::InvalidAmount.into());
        }
    }
    if let Some(merge) = &request.action.merge_question {
        if matches!(merge.amount.as_deref(), Some(amount) if amount.trim().is_empty()) {
            return Err(UserOutcomeContractError::InvalidAmount.into());
        }
    }
    if let Some(negate) = &request.action.negate_outcome {
        if negate.amount.trim().is_empty() {
            return Err(UserOutcomeContractError::InvalidAmount.into());
        }
    }
    Ok(())
}

async fn execute(
    _deps: &ExchangeActionDeps,
) -> Result<reply::UserOutcomeResponseWire, ExchangeHttpError> {
    Ok(reply::UserOutcomeResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_split_outcome_request() {
        let request = parse(valid_request_json()).expect("request should parse");
        assert!(request.action.split_outcome.is_some());
    }

    #[test]
    fn rejects_multiple_variants() {
        let request = parse(
            br#"{
                "action": {
                    "type": "userOutcome",
                    "splitOutcome": { "outcome": 1, "amount": "1.0" },
                    "mergeOutcome": { "outcome": 1, "amount": "1.0" }
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
            "Invalid `action`. Expected exactly one of `splitOutcome`, `mergeOutcome`, `mergeQuestion`, or `negateOutcome`."
        );
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
            "action": {
                "type": "userOutcome",
                "splitOutcome": { "outcome": 1, "amount": "1.0" }
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
