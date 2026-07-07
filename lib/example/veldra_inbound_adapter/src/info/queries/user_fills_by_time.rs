use crate::info::common::validate::{
    ensure_type, validate_hex_address_field, validate_optional_positive_u64_field,
    validate_positive_u64_field,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::hyperliquid_ws::WsFillWire>;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    user: String,
    #[serde(rename = "startTime")]
    start_time: u64,
    #[serde(rename = "endTime")]
    end_time: Option<u64>,
    #[serde(rename = "aggregateByTime")]
    aggregate_by_time: Option<bool>,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "userFillsByTime")?;
    validate_hex_address_field("user", &request.user)?;
    validate_positive_u64_field("startTime", request.start_time)?;
    validate_optional_positive_u64_field("endTime", request.end_time)?;
    let _ = request.aggregate_by_time;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    super::user_fills::stub_response()
}
