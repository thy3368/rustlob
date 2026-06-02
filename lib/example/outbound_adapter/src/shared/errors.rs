use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StoreError {
    #[error("store unavailable")]
    StoreUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceOrderOutboundError {
    #[error("account not found")]
    AccountNotFound,
    #[error("market rules not found")]
    MarketRulesNotFound,
    #[error("failed to decode replayable event")]
    EventDecodeFailed,
    #[error("outbound sequence overflow")]
    SequenceOverflow,
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DepositQuoteOutboundError {
    #[error("account not found")]
    AccountNotFound,
    #[error("failed to decode replayable event")]
    EventDecodeFailed,
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum WithdrawQuoteOutboundError {
    #[error("account not found")]
    AccountNotFound,
    #[error("failed to decode replayable event")]
    EventDecodeFailed,
    #[error(transparent)]
    Store(#[from] StoreError),
}
