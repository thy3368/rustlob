use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderResponseWire {
    pub status: &'static str,
    pub response: OrderResponseEnvelopeWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderResponseEnvelopeWire {
    #[serde(rename = "type")]
    pub type_: &'static str,
    pub data: OrderResponseDataWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderResponseDataWire {
    pub statuses: Vec<OrderStatusWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum OrderStatusWire {
    Resting { resting: RestingOrderStatusWire },
    Filled { filled: FilledOrderStatusWire },
    Error { error: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestingOrderStatusWire {
    pub oid: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FilledOrderStatusWire {
    #[serde(rename = "totalSz")]
    pub total_sz: String,
    #[serde(rename = "avgPx")]
    pub avg_px: String,
    pub oid: u64,
}
