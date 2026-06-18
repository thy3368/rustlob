use thiserror::Error;

#[derive(Debug, Error)]
pub enum TwapOrderContractError {
    #[error("Unexpected `action.type` for twapOrder handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.twap.s`. Expected a non-empty decimal string.")]
    InvalidSize,
    #[error("Invalid `action.twap.m`. Expected a duration greater than zero minutes.")]
    InvalidMinutes,
}
