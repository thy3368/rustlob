pub mod context;
pub mod option;
pub mod perp;
pub mod spot;
pub mod treasury;

use std::collections::BTreeMap;

use crate::core::SpotSide;

pub(crate) type ExecuteTradingBatchError = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RestingSpotOrder {
    pub(crate) order_id: u64,
    pub(crate) trader_id: u64,
    pub(crate) market: String,
    pub(crate) side: SpotSide,
    pub(crate) price: u64,
    pub(crate) original_quantity: u64,
    pub(crate) remaining_quantity: u64,
}

pub(crate) type SpotOrderBook = BTreeMap<String, Vec<RestingSpotOrder>>;
