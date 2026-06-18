use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateIsolatedMarginContractError {
    #[error("Unexpected `action.type` for updateIsolatedMargin handler: `{0}`.")]
    UnexpectedActionType(String),
}
