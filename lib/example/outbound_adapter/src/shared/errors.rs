use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StoreError {
    #[error("store unavailable")]
    StoreUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceOrderOutboundError {
    #[error("balance not found")]
    BalanceNotFound,
    #[error("market rules not found")]
    MarketRulesNotFound,
    #[error("failed to decode replayable event")]
    EventDecodeFailed,
    #[error("outbound sequence overflow")]
    SequenceOverflow,
    #[error("broker publish failed")]
    BrokerPublishFailed,
    #[error("unsupported spot order command branch")]
    UnsupportedCommandBranch,
    #[error("failed to build spot order v2 given state: {0}")]
    BuildSpotOrderV2State(String),
    #[error(transparent)]
    Store(#[from] StoreError),
}

impl From<example_core::SpotOrderV2UseCaseFamilyV3Error> for PlaceOrderOutboundError {
    fn from(error: example_core::SpotOrderV2UseCaseFamilyV3Error) -> Self {
        Self::BuildSpotOrderV2State(error.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DepositQuoteOutboundError {
    #[error("balance not found")]
    BalanceNotFound,
    #[error("failed to decode replayable event")]
    EventDecodeFailed,
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum WithdrawQuoteOutboundError {
    #[error("balance not found")]
    BalanceNotFound,
    #[error("failed to decode replayable event")]
    EventDecodeFailed,
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchSpotOrderOutboundError {
    #[error("order not found")]
    OrderNotFound,
    #[error("market rules not found")]
    MarketRulesNotFound,
    #[error("failed to decode replayable event")]
    EventDecodeFailed,
    #[error("broker publish failed")]
    BrokerPublishFailed,
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleSpotTradeOutboundError {
    #[error("trade not found")]
    TradeNotFound,
    #[error("balance not found")]
    BalanceNotFound,
    #[error("reservation not found")]
    ReservationNotFound,
    #[error("failed to decode replayable event")]
    EventDecodeFailed,
    #[error(transparent)]
    Store(#[from] StoreError),
}
