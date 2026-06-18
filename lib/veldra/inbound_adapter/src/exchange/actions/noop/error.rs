use thiserror::Error;

#[derive(Debug, Error)]
pub enum NoopContractError {
    #[error("Unexpected `action.type` for noop handler: `{0}`.")]
    UnexpectedActionType(String),
}
