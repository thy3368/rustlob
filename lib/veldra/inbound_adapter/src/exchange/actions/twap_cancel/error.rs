use thiserror::Error;

#[derive(Debug, Error)]
pub enum TwapCancelContractError {
    #[error("Unexpected `action.type` for twapCancel handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.t`. Expected a positive twap id.")]
    InvalidTwapId,
}
