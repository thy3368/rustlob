use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct ResponseWire {
        pub delegated: String,
        pub undelegated: String,
        #[serde(rename = "totalPendingWithdrawal")]
        pub total_pending_withdrawal: String,
        #[serde(rename = "nPendingWithdrawals")]
        pub n_pending_withdrawals: u64,
    }
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
    ensure_type(&request.type_, "delegatorSummary")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    reply::ResponseWire {
        delegated: "12060.16529862".to_string(),
        undelegated: "0.0".to_string(),
        total_pending_withdrawal: "0.0".to_string(),
        n_pending_withdrawals: 0,
    }
}
