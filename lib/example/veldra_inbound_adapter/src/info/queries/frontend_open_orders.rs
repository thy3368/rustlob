use crate::info::common::validate::{
    ensure_type, validate_hex_address_field, validate_non_empty_string_field,
};
use crate::info::common::wire::FrontendOrderWire;
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::info::common::wire::FrontendOrderWire>;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    user: String,
    dex: Option<String>,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "frontendOpenOrders")?;
    validate_hex_address_field("user", &request.user)?;
    if let Some(dex) = request.dex.as_deref() {
        validate_non_empty_string_field("dex", dex)?;
    }
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![FrontendOrderWire {
        coin: "BTC".to_string(),
        side: "A".to_string(),
        sz: "5.0".to_string(),
        limit_px: "29792.0".to_string(),
        oid: 91490942,
        timestamp: 1681247412573,
        is_trigger: false,
        trigger_px: "0.0".to_string(),
        trigger_condition: "N/A".to_string(),
        is_position_tpsl: false,
        reduce_only: false,
        order_type: "Limit".to_string(),
        orig_sz: "5.0".to_string(),
        children: vec![],
        cloid: None,
        tif: None,
    }]
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn frontend_open_orders_serializes_to_expected_wire_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value,
            json!([{
                "coin": "BTC",
                "side": "A",
                "sz": "5.0",
                "limitPx": "29792.0",
                "oid": 91490942u64,
                "timestamp": 1681247412573u64,
                "isTrigger": false,
                "triggerPx": "0.0",
                "triggerCondition": "N/A",
                "isPositionTpsl": false,
                "reduceOnly": false,
                "orderType": "Limit",
                "origSz": "5.0",
                "children": []
            }])
        );
    }
}
