pub use domain_service::cancel_spot_order::cancel_spot_order;
pub use domain_service::freeze_spot_order::freeze_spot_order;
pub use domain_service::match_spot_order::{
    MatchSpotOrderChanges, MatchSpotOrderInput, match_spot_order,
};
pub use domain_service::place_spot_order::place_spot_order;
pub use domain_service::settle_matched_spot_trades::{
    SettleMatchedSpotTradesChanges, SettleMatchedSpotTradesError, SettleMatchedSpotTradesInput,
    SpotOrderSettlementBalances, settle_matched_spot_trades,
};
pub use domain_service::unfreeze_matched_spot_orders::{
    SpotOrderHeldBalance, UnfreezeMatchedSpotOrdersError, unfreeze_matched_spot_orders,
};
pub use pipeline::atom_spot_order_domain_service::{
    MatchSpotOrderV3Input, MatchSpotOrderV3Outcome,
};

pub mod pipeline;
pub use pipeline::*;

pub mod domain_service;

pub use pipeline::atom_spot_trade_domain_service::{
    SpotTradeDomainService, SpotTradeDomainServiceError, SpotTradeSettlementOutcome,
};
