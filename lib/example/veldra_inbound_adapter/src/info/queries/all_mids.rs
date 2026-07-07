use crate::hyperliquid_ws::AllMidsWire;
use crate::info::common::validate::{ensure_type, validate_non_empty_string_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = crate::hyperliquid_ws::AllMidsWire;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    dex: Option<String>,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "allMids")?;
    if let Some(dex) = request.dex.as_deref() {
        validate_non_empty_string_field("dex", dex)?;
    }
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    AllMidsWire::from([
        ("APE".to_string(), "4.33245".to_string()),
        ("ARB".to_string(), "1.21695".to_string()),
    ])
}
