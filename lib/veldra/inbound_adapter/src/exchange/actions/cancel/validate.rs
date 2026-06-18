use crate::exchange::actions::cancel::error::CancelContractError;
use crate::exchange::actions::cancel::wire::CancelRequestWire;
use crate::exchange::common::validate::validate_common_fields;
use crate::exchange::error::ExchangeHttpError;

pub fn validate(request: &CancelRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "cancel" {
        return Err(CancelContractError::UnexpectedActionType(request.action.type_.clone()).into());
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
    if request.action.cancels.is_empty() {
        return Err(CancelContractError::EmptyCancels.into());
    }
    if matches!(request.action.f, Some(false)) {
        return Err(CancelContractError::InvalidFastFlag.into());
    }
    if request.action.cancels.iter().any(|cancel| cancel.o == 0) {
        return Err(CancelContractError::InvalidOid.into());
    }
    Ok(())
}
