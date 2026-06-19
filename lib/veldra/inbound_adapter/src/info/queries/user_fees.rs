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
    ensure_type(&request.type_, "userFees")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    json!({"dailyUserVlm":[{"date":"2025-05-23","userCross":"0.0","userAdd":"0.0","exchange":"2852367.0770729999"}],"feeSchedule":{"cross":"0.00045","add":"0.00015","spotCross":"0.0007","spotAdd":"0.0004","tiers":{"vip":[{"ntlCutoff":"5000000.0","cross":"0.0004","add":"0.00012","spotCross":"0.0006","spotAdd":"0.0003"}],"mm":[{"makerFractionCutoff":"0.005","add":"-0.00001"}]},"referralDiscount":"0.04","stakingDiscountTiers":[{"bpsOfMaxSupply":"0.0","discount":"0.0"}]},"userCrossRate":"0.000315","userAddRate":"0.000105","userSpotCrossRate":"0.00049","userSpotAddRate":"0.00028","activeReferralDiscount":"0.0","trial":null,"feeTrialReward":"0.0","nextTrialAvailableTimestamp":null,"stakingLink":{"type":"tradingUser","stakingUser":"0x54c049d9c7d3c92c2462bf3d28e083f3d6805061"},"activeStakingDiscount":{"bpsOfMaxSupply":"4.7577998927","discount":"0.3"}})
}
