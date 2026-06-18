use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateLeverageContractError {
    #[error("Unexpected `action.type` for updateLeverage handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("Invalid `action.leverage`. Expected an integer greater than or equal to 1.")]
    InvalidLeverage,
}
