use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct DelegationWire {
        pub validator: String,
        pub amount: String,
        #[serde(rename = "lockedUntilTimestamp")]
        pub locked_until_timestamp: u64,
    }
    pub type ResponseWire = Vec<DelegationWire>;
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
    ensure_type(&request.type_, "delegations")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![reply::DelegationWire {
        validator: "0x5ac99df645f3414876c816caa18b2d234024b487".to_string(),
        amount: "12060.16529862".to_string(),
        locked_until_timestamp: 1735466781353,
    }]
}
