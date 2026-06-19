use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<String>;
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
    ensure_type(&request.type_, "approvedBuilders")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    vec!["0x476fa87b4d3818f437f38f1263bee508d7672d82".to_string()]
}
