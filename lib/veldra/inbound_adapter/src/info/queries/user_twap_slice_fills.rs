use serde_json::json;

use crate::info::common::validate::{ensure_type, validate_hex_address_field};
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
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire =
        serde_json::from_slice(body).map_err(InfoHttpError::from_json_error)?;
    ensure_type(&request.type_, "userTwapSliceFills")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    json!([{"fill":{"closedPnl":"0.0","coin":"AVAX","crossed":true,"dir":"Open Long","hash":"0x0000000000000000000000000000000000000000000000000000000000000000","oid":90542681u64,"px":"18.435","side":"B","startPosition":"26.86","sz":"93.53","time":1681222254710u64,"fee":"0.01","feeToken":"USDC","tid":118906512037719u64},"twapId":3156u64}])
}
