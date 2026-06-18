pub(crate) mod error;
pub(crate) mod reply;
mod service;
#[cfg(test)]
mod tests;
mod wire;

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::error::ExchangeHttpError;

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::TwapOrderResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<wire::TwapOrderRequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &wire::TwapOrderRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "twapOrder" {
        return Err(error::TwapOrderContractError::UnexpectedActionType(
            request.action.type_.clone(),
        )
        .into());
    }
    validate_common_fields(
        request.common.nonce,
        request.common.expires_after,
        &request.common.signature.r,
        &request.common.signature.s,
        request.common.signature.v,
        request.common.vault_address.as_deref(),
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.action.twap.s.trim().is_empty() {
        return Err(error::TwapOrderContractError::InvalidSize.into());
    }
    if request.action.twap.m == 0 {
        return Err(error::TwapOrderContractError::InvalidMinutes.into());
    }
    Ok(())
}

async fn execute(
    deps: &ExchangeActionDeps,
) -> Result<reply::TwapOrderResponseWire, ExchangeHttpError> {
    service::execute(deps)
}
