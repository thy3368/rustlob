mod spot;

pub use spot::{
    ExampleSpotProductPluginAdapter, SpotBalanceSnapshot, SpotMarketRules, SpotOrder,
    SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotPlaceOrderPayload,
    SpotPlaceOrderResult, SpotPlaceOrderType, SpotProductContext, SpotSettlementResult,
    SpotTimeInForce, SpotTradeResult, build_default_product_registry,
};
