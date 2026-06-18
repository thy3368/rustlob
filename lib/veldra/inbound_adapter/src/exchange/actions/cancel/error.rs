use thiserror::Error;

#[derive(Debug, Error)]
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
