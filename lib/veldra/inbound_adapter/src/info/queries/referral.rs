use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::common::wire::{
    ReferralReferredByWire, ReferralStateWire, ReferralWire, ReferrerRewardStateWire,
    ReferrerStateDataWire, ReferrerStateEnvelopeWire,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = crate::info::common::wire::ReferralWire;
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
    ReferralWire {
        referred_by: Some(ReferralReferredByWire {
            referrer: "0x5ac99df645f3414876c816caa18b2d234024b487".to_string(),
            code: "TESTNET".to_string(),
            extra: Default::default(),
        }),
        cum_vlm: "149428030.6628420055".to_string(),
        unclaimed_rewards: "11.047361".to_string(),
        claimed_rewards: "22.743781".to_string(),
        builder_rewards: "0.027802".to_string(),
        token_to_state: (
            0,
            ReferrerRewardStateWire {
                cum_vlm: "149428030.6628420055".to_string(),
                unclaimed_rewards: "11.047361".to_string(),
                claimed_rewards: "22.743781".to_string(),
                builder_rewards: "0.027802".to_string(),
                extra: Default::default(),
            },
        ),
        referrer_state: ReferrerStateEnvelopeWire {
            stage: "ready".to_string(),
            data: Some(ReferrerStateDataWire {
                code: "TEST".to_string(),
                referral_states: vec![ReferralStateWire {
                    cum_vlm: "960652.017122".to_string(),
                    cum_rewarded_fees_since_referred: "196.838825".to_string(),
                    cum_fees_rewarded_to_referrer: "19.683748".to_string(),
                    time_joined: 1679425029416,
                    user: "0x11af2b93dcb3568b7bf2b6bd6182d260a9495728".to_string(),
                    extra: Default::default(),
                }],
                extra: Default::default(),
            }),
            extra: Default::default(),
        },
        reward_history: vec![],
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn referral_serializes_to_expected_wire_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value["tokenToState"],
            json!([0, {
                "cumVlm":"149428030.6628420055",
                "unclaimedRewards":"11.047361",
                "claimedRewards":"22.743781",
                "builderRewards":"0.027802"
            }])
        );
    }
}
