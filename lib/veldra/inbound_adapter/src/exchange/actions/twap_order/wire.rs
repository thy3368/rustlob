use serde::{Deserialize, Serialize};

use crate::exchange::common::wire::CommonExchangeFields;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TwapOrderRequestWire {
    pub action: TwapOrderActionWire,
    #[serde(flatten)]
    pub common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TwapOrderActionWire {
    #[serde(rename = "type")]
    pub type_: String,
    pub twap: TwapSpecWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TwapSpecWire {
    pub a: u32,
    pub b: bool,
    pub s: String,
    pub r: bool,
    pub m: u64,
    pub t: bool,
}
