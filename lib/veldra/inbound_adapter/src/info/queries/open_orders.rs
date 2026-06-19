use crate::hyperliquid_ws::WsBasicOrder;
use crate::info::common::validate::{
    ensure_type, validate_hex_address_field, validate_non_empty_string_field,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::hyperliquid_ws::WsBasicOrder>;
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
    let request: RequestWire =
        serde_json::from_slice(body).map_err(InfoHttpError::from_json_error)?;
    ensure_type(&request.type_, "openOrders")?;
    validate_hex_address_field("user", &request.user)?;
    if let Some(dex) = request.dex.as_deref() {
        validate_non_empty_string_field("dex", dex)?;
    }
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![WsBasicOrder {
        coin: "BTC".to_string(),
        side: "A".to_string(),
        sz: "0.0".to_string(),
        limit_px: Some("29792.0".to_string()),
        oid: 91490942,
        timestamp: 1681247412573,
        extra: Default::default(),
    }]
}
