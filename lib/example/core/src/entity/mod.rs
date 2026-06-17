mod market_rules;

pub mod perp;

pub mod spot;

pub mod account;

pub use account::account::Account;
pub use account::balance::Balance;
pub use market_rules::MarketRules;
pub use perp::hyperliquid_perp_funding_rate::{
    HyperliquidPerpBookLevel, HyperliquidPerpFundingRateError, HyperliquidPerpFundingSample,
    compute_hourly_funding_rate_e8, compute_impact_ask_price, compute_impact_bid_price,
};
pub use perp::hyperliquid_perp_funding_settlement::HyperliquidPerpFundingSettlement;
pub use perp::hyperliquid_perp_leverage_setting::HyperliquidPerpLeverageSetting;
pub use perp::hyperliquid_perp_liquidation::{
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus,
    HyperliquidPerpLiquidationTriggerReason,
};
pub use perp::hyperliquid_perp_order::{
    HyperliquidPerpOrder, HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide,
    HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
};
pub use perp::hyperliquid_perp_position::{
    HyperliquidPerpFundingDirection, HyperliquidPerpMarginMode, HyperliquidPerpPosition,
    HyperliquidPerpPositionSide, required_position_margin,
};
pub use perp::hyperliquid_perp_settlement::HyperliquidPerpSettlement;
pub use perp::hyperliquid_perp_trade::HyperliquidPerpTrade;
#[cfg(test)]
pub(crate) use spot::spot_order::spot_order_scenarios::{
    ActiveSpotOrderScenario, active_spot_order_scenario_strategy,
};
pub use spot::spot_order::{
    SpotConditionalOrder, SpotConditionalOrderStatus, SpotOrder, SpotOrderExecution, SpotOrderSide,
    SpotOrderStatus, SpotOrderStatusReason, SpotOrderTimeInForce, SpotOrderTriggerRole,
};
pub(crate) use spot::spot_order::{SpotOrderFinalization, SpotOrderMatchError};
pub use spot::spot_settlement::SpotSettlement;
pub use spot::spot_trade::SpotTrade;
