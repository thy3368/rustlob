use crate::info::common::validate::{
    ensure_type, validate_hex_address_field, validate_optional_hex_address_field,
};
use crate::info::common::wire::{
    PortfolioSliceWire, VaultDetailsWire, VaultFollowerWire, VaultRelationshipDataWire,
    VaultRelationshipWire,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = crate::info::common::wire::VaultDetailsWire;
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
    VaultDetailsWire {
        name: "Test".to_string(),
        vault_address: "0xdfc24b077bc1425ad1dea75bcb6f8158e10df303".to_string(),
        leader: "0x677d831aef5328190852e24f13c46cac05f984e7".to_string(),
        description: "This community-owned vault provides liquidity to Hyperliquid through multiple market making strategies, performs liquidations, and accrues platform fees.".to_string(),
        portfolio: vec![(
            "day".to_string(),
            PortfolioSliceWire {
                account_value_history: vec![(1734397526634, "329265410.90790099".to_string())],
                pnl_history: vec![(1734397526634, "0.0".to_string())],
                vlm: "0.0".to_string(),
                extra: Default::default(),
            },
        )],
        apr: 0.36387129259090006,
        follower_state: None,
        leader_fraction: 0.0007904828725729887,
        leader_commission: 0,
        followers: vec![VaultFollowerWire {
            user: "0x005844b2ffb2e122cf4244be7dbcb4f84924907c".to_string(),
            vault_equity: "714491.71026243".to_string(),
            pnl: "3203.43026143".to_string(),
            all_time_pnl: "79843.74476743".to_string(),
            days_following: 388,
            vault_entry_time: 1700926145201,
            lockup_until: 1734824439201,
            extra: Default::default(),
        }],
        max_distributable: 94856870.164485,
        max_withdrawable: 742557.680863,
        is_closed: false,
        relationship: VaultRelationshipWire {
            type_: "parent".to_string(),
            data: Some(VaultRelationshipDataWire {
                child_addresses: vec![
                    "0x010461c14e146ac35fe42271bdc1134ee31c703a".to_string(),
                ],
                extra: Default::default(),
            }),
            extra: Default::default(),
        },
        allow_deposits: true,
        always_close_on_withdraw: false,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn vault_details_serializes_with_tuple_portfolio_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value["portfolio"],
            json!([["day",{
                "accountValueHistory":[[1734397526634u64,"329265410.90790099"]],
                "pnlHistory":[[1734397526634u64,"0.0"]],
                "vlm":"0.0"
            }]])
        );
    }

    #[test]
    fn vault_relationship_accepts_unknown_fields() {
        let value = json!({
            "type": "parent",
            "data": { "childAddresses": [], "future": true },
            "extraTop": "ok"
        });
        let parsed: VaultRelationshipWire = serde_json::from_value(value).unwrap();
        assert_eq!(parsed.extra.get("extraTop"), Some(&json!("ok")));
        assert_eq!(parsed.data.unwrap().extra.get("future"), Some(&json!(true)));
    }
}
