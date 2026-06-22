use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::common::wire::{FrontendOrderWire, OrderStatusEnvelopeWire};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::info::common::wire::OrderStatusEnvelopeWire>;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    user: String,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "historicalOrders")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![OrderStatusEnvelopeWire {
        order: FrontendOrderWire {
            coin: "ETH".to_string(),
            side: "A".to_string(),
            sz: "0.0".to_string(),
            limit_px: "2412.7".to_string(),
            oid: 1,
            timestamp: 1724361546645,
            is_trigger: false,
            trigger_px: "0.0".to_string(),
            trigger_condition: "N/A".to_string(),
            is_position_tpsl: false,
            reduce_only: true,
            order_type: "Market".to_string(),
            orig_sz: "0.0076".to_string(),
            children: vec![],
            cloid: None,
            tif: Some("FrontendMarket".to_string()),
        },
        status: "filled".to_string(),
        status_timestamp: 1724361546645,
    }]
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn historical_orders_serializes_to_expected_wire_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value,
            json!([{
                "order": {
                    "coin": "ETH",
                    "side": "A",
                    "sz": "0.0",
                    "limitPx": "2412.7",
                    "oid": 1u64,
                    "timestamp": 1724361546645u64,
                    "isTrigger": false,
                    "triggerPx": "0.0",
                    "triggerCondition": "N/A",
                    "isPositionTpsl": false,
                    "reduceOnly": true,
                    "orderType": "Market",
                    "origSz": "0.0076",
                    "children": [],
                    "tif": "FrontendMarket"
                },
                "status": "filled",
                "statusTimestamp": 1724361546645u64
            }])
        );
    }
}
