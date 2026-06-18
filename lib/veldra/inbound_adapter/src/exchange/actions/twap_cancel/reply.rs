use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TwapCancelResponseWire {
    pub status: &'static str,
    pub response: TwapCancelResponseEnvelopeWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TwapCancelResponseEnvelopeWire {
    #[serde(rename = "type")]
    pub type_: &'static str,
    pub data: TwapCancelResponseDataWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TwapCancelResponseDataWire {
    pub status: TwapCancelStatusWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TwapCancelStatusWire {
    Success(&'static str),
    Error { error: String },
}
