use serde_json::json;

use crate::info::common::validate::{ensure_type, validate_hex_address_field, validate_oid_field};
use crate::info::common::wire::OidWire;
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = serde_json::Value;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    user: String,
    #[serde(rename = "type")]
    type_: String,
    oid: OidWire,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire =
        serde_json::from_slice(body).map_err(InfoHttpError::from_json_error)?;
    ensure_type(&request.type_, "orderStatus")?;
    validate_hex_address_field("user", &request.user)?;
    validate_oid_field("oid", &request.oid)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    json!({"status":"order","order":{"order":{"coin":"ETH","side":"A","limitPx":"2412.7","sz":"0.0","oid":1u64,"timestamp":1724361546645u64,"triggerCondition":"N/A","isTrigger":false,"triggerPx":"0.0","children":[],"isPositionTpsl":false,"reduceOnly":true,"orderType":"Market","origSz":"0.0076","tif":"FrontendMarket","cloid":null},"status":"filled","statusTimestamp":1724361546645u64}})
}
