use cmd_handler::command_use_case_def2::{MiFamilyExecutionError, MiFamilyExecutionSpec};
pub use example_core_use_case::CancelSpotOrderV2Lookup;
use example_core_use_case::{CancelSpotOrderV2Cmd, CancelSpotOrderV2UseCase};
use serde::{Deserialize, Serialize};
pub use use_case_executor::trading::spot::cancel_spot_order_v2_executor::execute_cancel_spot_order_v2;

use crate::command::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler, run_exchange_action,
};
use crate::command::exchange::common::validate::validate_envelope_common;
use crate::command::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_statuses_response};
use crate::command::exchange::error::ExchangeHttpError;

#[derive(Debug, thiserror::Error)]
pub enum CancelContractError {
    #[error("Unexpected `action.type` for cancel handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("`action.cancels` must contain at least one cancel request.")]
    EmptyCancels,
    #[error("Invalid `action.cancels[].o`. Expected a positive order id.")]
    InvalidOid,
    #[error("Invalid `action.f`. Omit `f` unless fast cancel is enabled.")]
    InvalidFastFlag,
}

pub mod reply {
    use serde::Serialize;

    use crate::command::exchange::common::wire::{
        ExchangeResponseEnvelopeWire, ExchangeResponseWire, ExchangeStatusesDataWire,
    };

    pub type CancelResponseWire = ExchangeResponseWire<CancelResponseDataWire>;
    pub type CancelResponseEnvelopeWire = ExchangeResponseEnvelopeWire<CancelResponseDataWire>;
    pub type CancelResponseDataWire = ExchangeStatusesDataWire<CancelStatusWire>;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    #[serde(untagged)]
    pub enum CancelStatusWire {
        Success(&'static str),
        Error { error: String },
    }
}

pub(crate) type RequestWire = ExchangeRequestEnvelopeWire<ActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ActionWire {
    #[serde(rename = "type")]
    type_: String,
    cancels: Vec<CancelItemWire>,
    f: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CancelItemWire {
    a: u32,
    o: u64,
}

pub(crate) const DEFAULT_EXCHANGE_PARTY_ID: &str = "default-exchange-party";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2Request {
    pub party_id: String,
    pub asset: u32,
    pub lookup: CancelSpotOrderV2Lookup,
}

impl CancelSpotOrderV2Request {
    fn from_wire_cancel(party_id: String, cancel: &CancelItemWire) -> Self {
        Self { party_id, asset: cancel.a, lookup: CancelSpotOrderV2Lookup::Oid(cancel.o) }
    }

    #[allow(dead_code)]
    pub fn from_cloid(party_id: String, asset: u32, cloid: String) -> Self {
        Self { party_id, asset, lookup: CancelSpotOrderV2Lookup::Cloid(cloid) }
    }
}

pub struct SpotOrderV2CancelExecutionSpec;

impl MiFamilyExecutionSpec<CancelSpotOrderV2UseCase> for SpotOrderV2CancelExecutionSpec {
    type Request = CancelSpotOrderV2Request;

    fn command(request: &Self::Request) -> CancelSpotOrderV2Cmd {
        CancelSpotOrderV2Cmd {
            party_id: request.party_id.clone(),
            asset: request.asset,
            lookup: request.lookup.clone(),
        }
    }
}

pub struct CancelAction;

impl CancelAction {
    pub async fn run_json(body: &[u8]) -> Result<reply::CancelResponseWire, ExchangeHttpError> {
        run_exchange_action::<Self>(body).await
    }
}

impl ExchangeActionHandler for CancelAction {
    type Request = RequestWire;
    type Reply = reply::CancelResponseWire;

    fn validate(request: &Self::Request) -> Result<(), ExchangeHttpError> {
        validate(request)
    }

    fn execute(request: Self::Request) -> ExchangeActionFuture<'static, Self::Reply> {
        Box::pin(execute(request))
    }
}

fn validate(request: &RequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "cancel" {
        return Err(ExchangeHttpError::contract(CancelContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )));
    }
    validate_envelope_common(&request.common).map_err(ExchangeHttpError::SharedFields)?;
    if request.action.cancels.is_empty() {
        return Err(ExchangeHttpError::contract(CancelContractError::EmptyCancels));
    }
    if matches!(request.action.f, Some(false)) {
        return Err(ExchangeHttpError::contract(CancelContractError::InvalidFastFlag));
    }
    if request.action.cancels.iter().any(|cancel| cancel.o == 0) {
        return Err(ExchangeHttpError::contract(CancelContractError::InvalidOid));
    }
    Ok(())
}

async fn execute(request: RequestWire) -> Result<reply::CancelResponseWire, ExchangeHttpError> {
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    let statuses = request
        .action
        .cancels
        .iter()
        .map(|cancel| {
            let cancel_request =
                CancelSpotOrderV2Request::from_wire_cancel(party_id.clone(), cancel);
            let command = SpotOrderV2CancelExecutionSpec::command(&cancel_request);
            match execute_cancel_spot_order_v2(&command) {
                Ok(_) => reply::CancelStatusWire::Success("success"),
                Err(error) => {
                    reply::CancelStatusWire::Error { error: cancel_execution_error_message(error) }
                }
            }
        })
        .collect();

    Ok(ok_statuses_response("cancel", statuses))
}

pub(crate) fn cancel_execution_error_message<BE, OE>(
    error: MiFamilyExecutionError<BE, OE>,
) -> String
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
    use super::*;

    #[test]
    fn maps_cancel_by_oid_request_to_cancel_command() {
        let request = CancelSpotOrderV2Request {
            party_id: "buyer".to_string(),
            asset: 10_000,
            lookup: CancelSpotOrderV2Lookup::Oid(42),
        };

        let command = SpotOrderV2CancelExecutionSpec::command(&request);

        assert_eq!(
            command,
            CancelSpotOrderV2Cmd {
                party_id: "buyer".to_string(),
                asset: 10_000,
                lookup: CancelSpotOrderV2Lookup::Oid(42),
            }
        );
    }
}
