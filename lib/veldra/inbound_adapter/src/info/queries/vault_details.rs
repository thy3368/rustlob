use serde_json::json;

use crate::info::common::validate::{
    ensure_type, validate_hex_address_field, validate_optional_hex_address_field,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = serde_json::Value;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "vaultAddress")]
    vault_address: String,
    user: Option<String>,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire =
        serde_json::from_slice(body).map_err(InfoHttpError::from_json_error)?;
    ensure_type(&request.type_, "vaultDetails")?;
    validate_hex_address_field("vaultAddress", &request.vault_address)?;
    validate_optional_hex_address_field("user", request.user.as_deref())?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    json!({"name":"Test","vaultAddress":"0xdfc24b077bc1425ad1dea75bcb6f8158e10df303","leader":"0x677d831aef5328190852e24f13c46cac05f984e7","description":"This community-owned vault provides liquidity to Hyperliquid through multiple market making strategies, performs liquidations, and accrues platform fees.","portfolio":[["day",{"accountValueHistory":[[1734397526634u64,"329265410.90790099"]],"pnlHistory":[[1734397526634u64,"0.0"]],"vlm":"0.0"}]],"apr":0.36387129259090006,"followerState":null,"leaderFraction":0.0007904828725729887,"leaderCommission":0,"followers":[{"user":"0x005844b2ffb2e122cf4244be7dbcb4f84924907c","vaultEquity":"714491.71026243","pnl":"3203.43026143","allTimePnl":"79843.74476743","daysFollowing":388,"vaultEntryTime":1700926145201u64,"lockupUntil":1734824439201u64}],"maxDistributable":94856870.164485,"maxWithdrawable":742557.680863,"isClosed":false,"relationship":{"type":"parent","data":{"childAddresses":["0x010461c14e146ac35fe42271bdc1134ee31c703a"]}},"allowDeposits":true,"alwaysCloseOnWithdraw":false})
}
