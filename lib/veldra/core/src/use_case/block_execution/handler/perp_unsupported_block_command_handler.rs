use crate::entity::{CommandEnvelope, ExchangeState, ProductCommand};
use crate::use_case::BuildBlockError;

pub(in crate::use_case::block_execution) static PERP_UNSUPPORTED_BLOCK_COMMAND_HANDLER:
    PerpUnsupportedBlockCommandHandler = PerpUnsupportedBlockCommandHandler;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::use_case::block_execution) struct PerpUnsupportedBlockCommandHandler;

pub(in crate::use_case::block_execution) fn validate_unsupported_perp()
-> Result<(), BuildBlockError> {
    Err(BuildBlockError::UnsupportedPerpCommand)
}

pub(in crate::use_case::block_execution) fn execute_unsupported_perp(
    _handler: &PerpUnsupportedBlockCommandHandler,
    _envelope: &CommandEnvelope<ProductCommand>,
    _exchange_state: &ExchangeState,
) -> Result<(), BuildBlockError> {
    Err(BuildBlockError::UnsupportedPerpCommand)
}
