use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::common::wire::{
    ActiveStakingDiscountWire, DailyUserVlmWire, FeeScheduleTiersWire, FeeScheduleWire, MmTierWire,
    StakingDiscountTierWire, StakingLinkWire, UserFeesWire, VipTierWire,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = crate::info::common::wire::UserFeesWire;
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
    ensure_type(&request.type_, "userFees")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    UserFeesWire {
        daily_user_vlm: vec![DailyUserVlmWire {
            date: "2025-05-23".to_string(),
            user_cross: "0.0".to_string(),
            user_add: "0.0".to_string(),
            exchange: "2852367.0770729999".to_string(),
            extra: Default::default(),
        }],
        fee_schedule: FeeScheduleWire {
            cross: "0.00045".to_string(),
            add: "0.00015".to_string(),
            spot_cross: "0.0007".to_string(),
            spot_add: "0.0004".to_string(),
            tiers: FeeScheduleTiersWire {
                vip: vec![VipTierWire {
                    ntl_cutoff: "5000000.0".to_string(),
                    cross: "0.0004".to_string(),
                    add: "0.00012".to_string(),
                    spot_cross: "0.0006".to_string(),
                    spot_add: "0.0003".to_string(),
                    extra: Default::default(),
                }],
                mm: vec![MmTierWire {
                    maker_fraction_cutoff: "0.005".to_string(),
                    add: "-0.00001".to_string(),
                    extra: Default::default(),
                }],
                extra: Default::default(),
            },
            referral_discount: "0.04".to_string(),
            staking_discount_tiers: vec![StakingDiscountTierWire {
                bps_of_max_supply: "0.0".to_string(),
                discount: "0.0".to_string(),
                extra: Default::default(),
            }],
            extra: Default::default(),
        },
        user_cross_rate: "0.000315".to_string(),
        user_add_rate: "0.000105".to_string(),
        user_spot_cross_rate: "0.00049".to_string(),
        user_spot_add_rate: "0.00028".to_string(),
        active_referral_discount: "0.0".to_string(),
        trial: None,
        fee_trial_reward: "0.0".to_string(),
        next_trial_available_timestamp: None,
        staking_link: StakingLinkWire {
            type_: "tradingUser".to_string(),
            staking_user: "0x54c049d9c7d3c92c2462bf3d28e083f3d6805061".to_string(),
            extra: Default::default(),
        },
        active_staking_discount: ActiveStakingDiscountWire {
            bps_of_max_supply: "4.7577998927".to_string(),
            discount: "0.3".to_string(),
            extra: Default::default(),
        },
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn user_fees_serializes_to_expected_wire_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value["feeSchedule"]["tiers"],
            json!({
                "vip": [{
                    "ntlCutoff": "5000000.0",
                    "cross": "0.0004",
                    "add": "0.00012",
                    "spotCross": "0.0006",
                    "spotAdd": "0.0003"
                }],
                "mm": [{
                    "makerFractionCutoff": "0.005",
                    "add": "-0.00001"
                }]
            })
        );
    }
}
