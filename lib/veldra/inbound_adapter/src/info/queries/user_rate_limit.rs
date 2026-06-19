use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct ResponseWire {
        #[serde(rename = "cumVlm")]
        pub cum_vlm: String,
        #[serde(rename = "nRequestsUsed")]
        pub n_requests_used: u64,
        #[serde(rename = "nRequestsCap")]
        pub n_requests_cap: u64,
        #[serde(rename = "nRequestsSurplus")]
        pub n_requests_surplus: u64,
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    user: String,
    #[serde(rename = "type")]
    type_: String,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire =
        serde_json::from_slice(body).map_err(InfoHttpError::from_json_error)?;
    ensure_type(&request.type_, "userRateLimit")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    reply::ResponseWire {
        cum_vlm: "2854574.593578".to_string(),
        n_requests_used: 2890,
        n_requests_cap: 2864574,
        n_requests_surplus: 0,
    }
}
