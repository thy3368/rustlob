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
) -> Result<reply::UserSetAbstractionResponseWire, ExchangeHttpError> {
    run_action(body, deps, parse, validate, |_, deps| Box::pin(execute(deps))).await
}

fn parse(body: &[u8]) -> Result<wire::UserSetAbstractionRequestWire, ExchangeHttpError> {
    serde_json::from_slice(body).map_err(ExchangeHttpError::from_json_error)
}

fn validate(request: &wire::UserSetAbstractionRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "userSetAbstraction" {
        return Err(error::UserSetAbstractionContractError::UnexpectedActionType(
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
        return Err(error::UserSetAbstractionContractError::VaultAddressNotSupported.into());
    }
    if request.common.expires_after.is_some() {
        return Err(error::UserSetAbstractionContractError::ExpiresAfterNotSupported.into());
    }
    validate_hyperliquid_chain(&request.action.hyperliquid_chain)
        .map_err(|_| error::UserSetAbstractionContractError::InvalidHyperliquidChain)?;
    validate_signature_chain_id(&request.action.signature_chain_id)
        .map_err(|_| error::UserSetAbstractionContractError::InvalidSignatureChainId)?;
    validate_hex_address(&request.action.user)
        .map_err(|_| error::UserSetAbstractionContractError::InvalidUserAddress)?;
    if !matches!(
        request.action.abstraction.as_str(),
        "disabled" | "unifiedAccount" | "portfolioMargin"
    ) {
        return Err(error::UserSetAbstractionContractError::InvalidAbstraction.into());
    }
    if request.action.nonce != request.common.nonce {
        return Err(error::UserSetAbstractionContractError::NonceMismatch.into());
    }
    Ok(())
}

async fn execute(
    deps: &ExchangeActionDeps,
) -> Result<reply::UserSetAbstractionResponseWire, ExchangeHttpError> {
    service::execute(deps)
}
