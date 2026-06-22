use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct DelegatorRewardWire {
        pub time: u64,
        pub source: String,
        #[serde(rename = "totalAmount")]
        pub total_amount: String,
    }
    pub type ResponseWire = Vec<DelegatorRewardWire>;
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
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "delegatorRewards")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![
        reply::DelegatorRewardWire {
            time: 1736726400073,
            source: "delegation".to_string(),
            total_amount: "0.73117184".to_string(),
        },
        reply::DelegatorRewardWire {
            time: 1736726400073,
            source: "commission".to_string(),
            total_amount: "130.76445876".to_string(),
        },
    ]
}
