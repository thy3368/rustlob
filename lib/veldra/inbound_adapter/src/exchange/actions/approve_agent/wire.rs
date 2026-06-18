use serde::{Deserialize, Serialize};

use crate::exchange::common::wire::CommonExchangeFields;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApproveAgentRequestWire {
    pub action: ApproveAgentActionWire,
    #[serde(flatten)]
    pub common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApproveAgentActionWire {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "hyperliquidChain")]
    pub hyperliquid_chain: String,
    #[serde(rename = "signatureChainId")]
    pub signature_chain_id: String,
    #[serde(rename = "agentAddress")]
    pub agent_address: String,
    #[serde(rename = "agentName")]
    pub agent_name: Option<String>,
    pub nonce: u64,
}
