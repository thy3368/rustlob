use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommonExchangeFields {
    pub nonce: u64,
    pub signature: SignatureWire,
    #[serde(rename = "vaultAddress", skip_serializing_if = "Option::is_none")]
    pub vault_address: Option<String>,
    #[serde(rename = "expiresAfter", skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureWire {
    pub r: String,
    pub s: String,
    pub v: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonObjectWire(pub BTreeMap<String, JsonValue>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExchangeRequestEnvelopeWire<TAction> {
    pub action: TAction,
    #[serde(flatten)]
    pub common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ExchangeActionTypeProbe {
    pub action: ExchangeActionTypeField,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ExchangeActionTypeField {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeErrorResponseWire {
    pub status: &'static str,
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DefaultExchangeResponseWire {
    pub status: &'static str,
    pub response: DefaultExchangeResponseEnvelopeWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DefaultExchangeResponseEnvelopeWire {
    #[serde(rename = "type")]
    pub type_: &'static str,
}
