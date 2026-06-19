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
    ensure_type(&request.type_, "referral")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    json!({"referredBy":{"referrer":"0x5ac99df645f3414876c816caa18b2d234024b487","code":"TESTNET"},"cumVlm":"149428030.6628420055","unclaimedRewards":"11.047361","claimedRewards":"22.743781","builderRewards":"0.027802","tokenToState":[0,{"cumVlm":"149428030.6628420055","unclaimedRewards":"11.047361","claimedRewards":"22.743781","builderRewards":"0.027802"}],"referrerState":{"stage":"ready","data":{"code":"TEST","referralStates":[{"cumVlm":"960652.017122","cumRewardedFeesSinceReferred":"196.838825","cumFeesRewardedToReferrer":"19.683748","timeJoined":1679425029416u64,"user":"0x11af2b93dcb3568b7bf2b6bd6182d260a9495728"}]}},"rewardHistory":[]})
}
