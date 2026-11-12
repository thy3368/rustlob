pub use domain_service::spot_order_domain_service::{
    MatchSpotOrderV3Input, MatchSpotOrderV3Outcome,
};

pub mod pipeline;

pub mod domain_service;

pub use domain_service::spot_trade_domain_service::{
    SpotTradeDomainService, SpotTradeDomainServiceError, SpotTradeSettlementOutcome,
};
