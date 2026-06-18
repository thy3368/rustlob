use serde::{Deserialize, Serialize};

use crate::exchange::common::wire::CommonExchangeFields;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TwapCancelRequestWire {
    pub action: TwapCancelActionWire,
    #[serde(flatten)]
    pub common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TwapCancelActionWire {
    #[serde(rename = "type")]
    pub type_: String,
    pub a: u32,
    pub t: u64,
}
