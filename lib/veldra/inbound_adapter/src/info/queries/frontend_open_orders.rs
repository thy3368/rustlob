use serde_json::json;

use crate::info::common::validate::{
    ensure_type, validate_hex_address_field, validate_non_empty_string_field,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = serde_json::Value;
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
    ensure_type(&request.type_, "frontendOpenOrders")?;
    validate_hex_address_field("user", &request.user)?;
    if let Some(dex) = request.dex.as_deref() {
        validate_non_empty_string_field("dex", dex)?;
    }
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    json!([{"coin":"BTC","isPositionTpsl":false,"isTrigger":false,"limitPx":"29792.0","oid":91490942u64,"orderType":"Limit","origSz":"5.0","reduceOnly":false,"side":"A","sz":"5.0","timestamp":1681247412573u64,"triggerCondition":"N/A","triggerPx":"0.0"}])
}
