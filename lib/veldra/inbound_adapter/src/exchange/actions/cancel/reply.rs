use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CancelResponseWire {
    pub status: &'static str,
    pub response: CancelResponseEnvelopeWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CancelResponseEnvelopeWire {
    #[serde(rename = "type")]
    pub type_: &'static str,
    pub data: CancelResponseDataWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CancelResponseDataWire {
    pub statuses: Vec<CancelStatusWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum CancelStatusWire {
    Success(&'static str),
    Error { error: String },
}
