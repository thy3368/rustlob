use serde::{Deserialize, Serialize};

use crate::exchange::common::wire::CommonExchangeFields;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateIsolatedMarginRequestWire {
    pub action: UpdateIsolatedMarginActionWire,
    #[serde(flatten)]
    pub common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateIsolatedMarginActionWire {
    #[serde(rename = "type")]
    pub type_: String,
    pub asset: u32,
    #[serde(rename = "isBuy")]
    pub is_buy: bool,
    pub ntli: i64,
}
