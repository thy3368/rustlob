pub use domain_service::atom_spot_order_domain_service::{
    MatchSpotOrderV3Input, MatchSpotOrderV3Outcome,
};
pub use domain_service::atom_spot_order_domain_service_v2::{
    CancelSpotOrderDomainServiceV2Changes, CancelSpotOrderDomainServiceV2Error,
    CancelSpotOrderDomainServiceV2Metadata, SpotOrderDomainServiceV2,
};

pub mod pipeline;
pub use pipeline::*;

pub mod domain_service;

pub use domain_service::atom_spot_trade_domain_service::{
    SpotTradeDomainService, SpotTradeDomainServiceError, SpotTradeSettlementOutcome,
};
