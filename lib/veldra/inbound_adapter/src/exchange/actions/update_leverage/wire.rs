use serde::{Deserialize, Serialize};

use crate::exchange::common::wire::CommonExchangeFields;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateLeverageRequestWire {
    pub action: UpdateLeverageActionWire,
    #[serde(flatten)]
    pub common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateLeverageActionWire {
    #[serde(rename = "type")]
    pub type_: String,
    pub asset: u32,
    #[serde(rename = "isCross")]
    pub is_cross: bool,
    pub leverage: u64,
}
