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
    ensure_type(&request.type_, "portfolio")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    json!([["day",{"accountValueHistory":[[1741886630493u64,"0.0"],[1741895270493u64,"0.0"]],"pnlHistory":[[1741886630493u64,"0.0"],[1741895270493u64,"0.0"]],"vlm":"0.0"}],["week",{"accountValueHistory":[],"pnlHistory":[],"vlm":"0.0"}]])
}
