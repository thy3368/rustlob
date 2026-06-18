use serde::{Deserialize, Serialize};

use crate::exchange::common::wire::CommonExchangeFields;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelRequestWire {
    pub action: CancelActionWire,
    #[serde(flatten)]
    pub common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelActionWire {
    #[serde(rename = "type")]
    pub type_: String,
    pub cancels: Vec<CancelItemWire>,
    pub f: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelItemWire {
    pub a: u32,
    pub o: u64,
}
