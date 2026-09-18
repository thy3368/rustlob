mod funding;
mod shared;
mod trading;

pub mod state_source;

pub mod state_sink;

pub use funding::{
    InMemoryDepositQuoteOutbound, InMemoryWithdrawQuoteOutbound, MySqlDepositQuoteOutbound,
    MySqlWithdrawQuoteOutbound,
};
pub use shared::{
    DepositQuoteOutboundError, InMemorySpotPipelineBroker, InMemoryStore, MySqlStore,
    PlaceOrderOutboundError, SpotOrderPlacedMessage, SpotPipelineBroker, SpotPipelineBrokerError,
    SpotPipelineMessage, SpotTradeMatchedMessage, StoreError, StoreSnapshot,
    WithdrawQuoteOutboundError,
};
#[cfg(feature = "test-support")]
pub use trading::FakeSpotOrderV2CancelOutbound;
pub use trading::{
    DefaultSpotOrderV2CancelOutbound, DefaultSpotOrderV2CancelOutboundError,
    DefaultSpotOrderV2PlaceOnlyOutbound, DefaultSpotOrderV2PlaceOnlyOutboundError,
    DefaultSpotOrderV2PlaceOutbound, DefaultSpotOrderV2PlaceOutboundError,
    InMemoryPlaceOrderOutbound, MySqlPlaceOrderOutbound, base_asset_id_for, quote_asset_id_for,
    symbol_for_asset,
};
