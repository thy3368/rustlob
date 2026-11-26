use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeResponseWire {
    pub status: &'static str,
    pub response: ExchangeResponsePayloadWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum ExchangeResponsePayloadWire {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "order")]
    Order { data: OrderResponseDataWire },
    #[serde(rename = "cancel")]
    Cancel { data: CancelResponseDataWire },
    #[serde(rename = "twapCancel")]
    TwapCancel { data: TwapCancelResponseDataWire },
    #[serde(rename = "twapOrder")]
    TwapOrder { data: TwapOrderResponseDataWire },
}

pub type OrderResponseDataWire = ExchangeStatusesDataWire<OrderStatusWire>;
pub type CancelResponseDataWire = ExchangeStatusesDataWire<CancelStatusWire>;
pub type TwapCancelResponseDataWire = ExchangeStatusDataWire<TwapCancelStatusWire>;
pub type TwapOrderResponseDataWire = ExchangeStatusDataWire<TwapOrderStatusWire>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeStatusesDataWire<TStatus> {
    pub statuses: Vec<TStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeStatusDataWire<TStatus> {
    pub status: TStatus,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum CancelStatusWire {
    Success(&'static str),
    Error { error: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TwapCancelStatusWire {
    Success(&'static str),
    Error { error: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TwapOrderStatusWire {
    Running { running: RunningTwapStatusWire },
    Error { error: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunningTwapStatusWire {
    #[serde(rename = "twapId")]
    pub twap_id: u64,
}

impl ExchangeResponseWire {
    pub fn default() -> Self {
        Self::new(ExchangeResponsePayloadWire::Default)
    }

    pub fn order(statuses: Vec<OrderStatusWire>) -> Self {
        Self::new(ExchangeResponsePayloadWire::Order {
            data: ExchangeStatusesDataWire { statuses },
        })
    }

    pub fn cancel(statuses: Vec<CancelStatusWire>) -> Self {
        Self::new(ExchangeResponsePayloadWire::Cancel {
            data: ExchangeStatusesDataWire { statuses },
        })
    }

    pub fn twap_cancel(status: TwapCancelStatusWire) -> Self {
        Self::new(ExchangeResponsePayloadWire::TwapCancel {
            data: ExchangeStatusDataWire { status },
        })
    }

    pub fn twap_order(status: TwapOrderStatusWire) -> Self {
        Self::new(ExchangeResponsePayloadWire::TwapOrder {
            data: ExchangeStatusDataWire { status },
        })
    }

    fn new(response: ExchangeResponsePayloadWire) -> Self {
        Self {
            status: "ok",
            response,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn serialized<T: Serialize>(value: T) -> serde_json::Value {
        serde_json::to_value(value).expect("wire response serializes")
    }

    #[test]
    fn serializes_default_response() {
        assert_eq!(
            serialized(ExchangeResponseWire::default()),
            json!({"status": "ok", "response": {"type": "default"}})
        );
    }

    #[test]
    fn serializes_all_order_statuses() {
        let response = ExchangeResponseWire::order(vec![
            OrderStatusWire::Resting {
                resting: RestingOrderStatusWire { oid: 77747313 },
            },
            OrderStatusWire::Filled {
                filled: FilledOrderStatusWire {
                    total_sz: "0.02".to_owned(),
                    avg_px: "1891.4".to_owned(),
                    oid: 77747314,
                },
            },
            OrderStatusWire::Error {
                error: "order failed".to_owned(),
            },
        ]);
        assert_eq!(
            serialized(response),
            json!({"status": "ok", "response": {"type": "order", "data": {
                "statuses": [
                    {"resting": {"oid": 77747313}},
                    {"filled": {"totalSz": "0.02", "avgPx": "1891.4", "oid": 77747314}},
                    {"error": "order failed"}
                ]
            }}})
        );
    }

    #[test]
    fn serializes_all_cancel_statuses() {
        let response = ExchangeResponseWire::cancel(vec![
            CancelStatusWire::Success("success"),
            CancelStatusWire::Error {
                error: "cancel failed".to_owned(),
            },
        ]);
        assert_eq!(
            serialized(response),
            json!({"status": "ok", "response": {"type": "cancel", "data": {
                "statuses": ["success", {"error": "cancel failed"}]
            }}})
        );
    }

    #[test]
    fn serializes_all_twap_cancel_statuses() {
        assert_eq!(
            serialized(ExchangeResponseWire::twap_cancel(
                TwapCancelStatusWire::Success("success"),
            )),
            json!({"status": "ok", "response": {"type": "twapCancel", "data": {
                "status": "success"
            }}})
        );

        assert_eq!(
            serialized(ExchangeResponseWire::twap_cancel(
                TwapCancelStatusWire::Error {
                    error: "twap cancel failed".to_owned(),
                },
            )),
            json!({"status": "ok", "response": {"type": "twapCancel", "data": {
                "status": {"error": "twap cancel failed"}
            }}})
        );
    }

    #[test]
    fn serializes_all_twap_order_statuses() {
        assert_eq!(
            serialized(ExchangeResponseWire::twap_order(
                TwapOrderStatusWire::Running {
                    running: RunningTwapStatusWire { twap_id: 77738308 },
                },
            )),
            json!({"status": "ok", "response": {"type": "twapOrder", "data": {
                "status": {"running": {"twapId": 77738308}}
            }}})
        );

        assert_eq!(
            serialized(ExchangeResponseWire::twap_order(
                TwapOrderStatusWire::Error {
                    error: "twap order failed".to_owned(),
                },
            )),
            json!({"status": "ok", "response": {"type": "twapOrder", "data": {
                "status": {"error": "twap order failed"}
            }}})
        );
    }
}
