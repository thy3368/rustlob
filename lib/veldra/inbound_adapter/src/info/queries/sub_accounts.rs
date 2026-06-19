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
    ensure_type(&request.type_, "subAccounts")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    json!([{"name":"Test","subAccountUser":"0x035605fc2f24d65300227189025e90a0d947f16c","master":"0x8c967e73e6b15087c42a10d344cff4c96d877f1d","clearinghouseState":{"marginSummary":{"accountValue":"29.78001","totalNtlPos":"0.0","totalRawUsd":"29.78001","totalMarginUsed":"0.0"},"crossMarginSummary":{"accountValue":"29.78001","totalNtlPos":"0.0","totalRawUsd":"29.78001","totalMarginUsed":"0.0"},"crossMaintenanceMarginUsed":"0.0","withdrawable":"29.78001","assetPositions":[],"time":1733968369395u64},"spotState":{"balances":[{"coin":"USDC","token":0,"total":"0.22","hold":"0.0","entryNtl":"0.0"}]}}])
}
