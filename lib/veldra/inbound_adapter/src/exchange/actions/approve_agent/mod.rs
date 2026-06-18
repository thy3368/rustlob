pub(crate) mod error;
pub(crate) mod reply;
mod service;
#[cfg(test)]
mod tests;
mod wire;

use crate::exchange::actions::ExchangeActionDeps;
use crate::exchange::common::runner::run_action;
use crate::exchange::common::validate::{
    validate_common_fields, validate_hex_address, validate_hyperliquid_chain,
    validate_signature_chain_id,
};
use crate::exchange::error::ExchangeHttpError;

pub async fn handle(
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<reply::ApproveAgentResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<wire::ApproveAgentRequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &wire::ApproveAgentRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "approveAgent" {
        return Err(error::ApproveAgentContractError::UnexpectedActionType(
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
        return Err(error::ApproveAgentContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(error::ApproveAgentContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| error::ApproveAgentContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| error::ApproveAgentContractError::InvalidSignatureChainId)?;
    validate_hex_address(&request.action.agent_address)
        .map_err(|_| error::ApproveAgentContractError::InvalidAgentAddress)?;
    if request.action.nonce != request.common.nonce {
        return Err(error::ApproveAgentContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute(
    deps: &ExchangeActionDeps,
) -> Result<reply::ApproveAgentResponseWire, ExchangeHttpError> {
    service::execute(deps)
}
