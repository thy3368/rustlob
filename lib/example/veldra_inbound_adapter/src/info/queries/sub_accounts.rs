use crate::hyperliquid_ws::{ClearinghouseStateWire, WsMarginSummaryWire};
use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::common::wire::{SpotBalanceWire, SpotStateWire, SubAccountWire};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::info::common::wire::SubAccountWire>;
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
    ensure_type(&request.type_, "subAccounts")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![SubAccountWire {
        name: "Test".to_string(),
        sub_account_user: "0x035605fc2f24d65300227189025e90a0d947f16c".to_string(),
        master: "0x8c967e73e6b15087c42a10d344cff4c96d877f1d".to_string(),
        clearinghouse_state: ClearinghouseStateWire {
            asset_positions: vec![],
            cross_margin_summary: Some(WsMarginSummaryWire {
                account_value: "29.78001".to_string(),
                total_ntl_pos: "0.0".to_string(),
                total_raw_usd: "29.78001".to_string(),
                total_margin_used: "0.0".to_string(),
                extra: Default::default(),
            }),
            margin_summary: Some(WsMarginSummaryWire {
                account_value: "29.78001".to_string(),
                total_ntl_pos: "0.0".to_string(),
                total_raw_usd: "29.78001".to_string(),
                total_margin_used: "0.0".to_string(),
                extra: Default::default(),
            }),
            withdrawable: Some("29.78001".to_string()),
            cross_maintenance_margin_used: Some("0.0".to_string()),
            time: Some(1733968369395),
            extra: Default::default(),
        },
        spot_state: SpotStateWire {
            balances: vec![SpotBalanceWire {
                coin: "USDC".to_string(),
                token: 0,
                total: "0.22".to_string(),
                hold: "0.0".to_string(),
                entry_ntl: "0.0".to_string(),
                extra: Default::default(),
            }],
            extra: Default::default(),
        },
        extra: Default::default(),
    }]
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn sub_accounts_serializes_to_expected_wire_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value,
            json!([{
                "name": "Test",
                "subAccountUser": "0x035605fc2f24d65300227189025e90a0d947f16c",
                "master": "0x8c967e73e6b15087c42a10d344cff4c96d877f1d",
                "clearinghouseState": {
                    "marginSummary": {
                        "accountValue": "29.78001",
                        "totalNtlPos": "0.0",
                        "totalRawUsd": "29.78001",
                        "totalMarginUsed": "0.0"
                    },
                    "crossMarginSummary": {
                        "accountValue": "29.78001",
                        "totalNtlPos": "0.0",
                        "totalRawUsd": "29.78001",
                        "totalMarginUsed": "0.0"
                    },
                    "crossMaintenanceMarginUsed": "0.0",
                    "withdrawable": "29.78001",
                    "assetPositions": [],
                    "time": 1733968369395u64
                },
                "spotState": {
                    "balances": [{
                        "coin": "USDC",
                        "token": 0,
                        "total": "0.22",
                        "hold": "0.0",
                        "entryNtl": "0.0"
                    }]
                }
            }])
        );
    }

    #[test]
    fn sub_accounts_accepts_unknown_spot_state_fields() {
        let value = json!({
            "name": "Test",
            "subAccountUser": "0x035605fc2f24d65300227189025e90a0d947f16c",
            "master": "0x8c967e73e6b15087c42a10d344cff4c96d877f1d",
            "clearinghouseState": { "assetPositions": [] },
            "spotState": { "balances": [], "newField": "future" }
        });
        let parsed: SubAccountWire = serde_json::from_value(value).unwrap();
        assert_eq!(parsed.spot_state.extra.get("newField"), Some(&json!("future")));
    }
}
