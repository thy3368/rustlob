use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TwapOrderResponseWire {
    pub status: &'static str,
    pub response: TwapOrderResponseEnvelopeWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TwapOrderResponseEnvelopeWire {
    #[serde(rename = "type")]
    pub type_: &'static str,
    pub data: TwapOrderResponseDataWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TwapOrderResponseDataWire {
    pub status: TwapOrderStatusWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TwapOrderStatusWire {
    Running { running: TwapRunningStatusWire },
    Error { error: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TwapRunningStatusWire {
    #[serde(rename = "twapId")]
    pub twap_id: u64,
}
