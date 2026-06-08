mod market_rules;
mod trading_account;
mod usds_m_futures_order;

pub mod perp;

pub mod spot;

pub mod account;

pub use account::account::Account;
pub use account::balance::Balance;
pub use perp::hyperliquid_perp_order::{
    HyperliquidPerpOrder, HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide,
    HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
};
pub use perp::hyperliquid_perp_position::{
    required_position_margin, HyperliquidPerpPosition, HyperliquidPerpPositionSide,
};
pub use perp::hyperliquid_perp_settlement::HyperliquidPerpSettlement;
pub use perp::hyperliquid_perp_trade::HyperliquidPerpTrade;
pub use market_rules::MarketRules;
#[cfg(test)]
pub(crate) use spot::spot_order::spot_order_scenarios::{
    active_spot_order_scenario_strategy, ActiveSpotOrderScenario,
};
pub use spot::spot_order::{
    SpotConditionalOrder, SpotConditionalOrderStatus, SpotOrder, SpotOrderExecution, SpotOrderSide,
    SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce, SpotOrderTriggerRole,
};
pub use spot::spot_settlement::SpotSettlement;
pub use spot::spot_trade::SpotTrade;
pub use trading_account::TradingAccount;
pub use usds_m_futures_order::{
    required_margin, UsdsMFuturesOrder, UsdsMFuturesOrderExecution, UsdsMFuturesOrderSide,
    UsdsMFuturesOrderStatus, UsdsMFuturesOrderTimeInForce, UsdsMFuturesPositionSide,
};
