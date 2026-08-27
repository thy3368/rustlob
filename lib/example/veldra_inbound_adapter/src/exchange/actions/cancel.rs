use cmd_handler::command_use_case_def2::{
    MiFamilyExecutionError, MiFamilyExecutionResult, MiFamilyExecutionSpec, MiFamilyOutbound,
    MiStateMachineFamilyExecutor,
};
pub use example_core_use_case::CancelSpotOrderV2LookupV3;
use example_core_use_case::{
    CancelSpotOrderV2CmdV3, SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3,
    SpotOrderV2UseCaseFamilyV3,
};
use example_outbound_adapter::DefaultSpotOrderV2CancelOutbound;
use serde::{Deserialize, Serialize};

use crate::exchange::common::runner::{
    ExchangeActionFuture, ExchangeActionHandler, run_exchange_action,
};
use crate::exchange::common::validate::validate_envelope_common;
use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, ok_statuses_response};
use crate::exchange::error::ExchangeHttpError;

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

    use crate::exchange::common::wire::{
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
    pub lookup: CancelSpotOrderV2LookupV3,
}

impl CancelSpotOrderV2Request {
    fn from_wire_cancel(party_id: String, cancel: &CancelItemWire) -> Self {
        Self { party_id, asset: cancel.a, lookup: CancelSpotOrderV2LookupV3::Oid(cancel.o) }
    }

    #[allow(dead_code)]
    pub fn from_cloid(party_id: String, asset: u32, cloid: String) -> Self {
        Self { party_id, asset, lookup: CancelSpotOrderV2LookupV3::Cloid(cloid) }
    }
}

pub struct SpotOrderV2CancelExecutionSpec;

impl MiFamilyExecutionSpec<SpotOrderV2UseCaseFamilyV3> for SpotOrderV2CancelExecutionSpec {
    type Request = CancelSpotOrderV2Request;

    fn command(request: &Self::Request) -> SpotOrderV2CommandV3 {
        SpotOrderV2CommandV3::Cancel(CancelSpotOrderV2CmdV3 {
            party_id: request.party_id.clone(),
            asset: request.asset,
            lookup: request.lookup.clone(),
        })
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
    let outbound = DefaultSpotOrderV2CancelOutbound;
    let party_id =
        request.common.vault_address.unwrap_or_else(|| DEFAULT_EXCHANGE_PARTY_ID.to_string());

    let statuses = request
        .action
        .cancels
        .iter()
        .map(|cancel| {
            let cancel_request =
                CancelSpotOrderV2Request::from_wire_cancel(party_id.clone(), cancel);
            match execute_cancel_spot_order_v2(&cancel_request, &outbound) {
                Ok(_) => reply::CancelStatusWire::Success("success"),
                Err(error) => {
                    reply::CancelStatusWire::Error { error: cancel_execution_error_message(error) }
                }
            }
        })
        .collect();

    Ok(ok_statuses_response("cancel", statuses))
}

pub fn execute_cancel_spot_order_v2<OB>(
    request: &CancelSpotOrderV2Request,
    outbound: &OB,
) -> Result<
    MiFamilyExecutionResult<SpotOrderV2CaseChangesV3>,
    MiFamilyExecutionError<example_core_use_case::SpotOrderV2UseCaseFamilyV3Error, OB::Error>,
>
where
    OB: MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3>,
{
    let command = SpotOrderV2CancelExecutionSpec::command(request);
    MiStateMachineFamilyExecutor.execute::<SpotOrderV2UseCaseFamilyV3, OB>(
        &SpotOrderV2UseCaseFamilyV3,
        &command,
        outbound,
    )
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
