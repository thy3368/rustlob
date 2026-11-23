pub use domain_service::atom_spot_order_domain_service::{
    MatchSpotOrderV3Input, MatchSpotOrderV3Outcome,
};
pub use domain_service::cancel_spot_order::cancel_spot_order;
pub use domain_service::place_spot_order::place_spot_order;

pub mod pipeline;
pub use pipeline::*;

pub mod domain_service;

pub use domain_service::atom_spot_trade_domain_service::{
    SpotTradeDomainService, SpotTradeDomainServiceError, SpotTradeSettlementOutcome,
};
