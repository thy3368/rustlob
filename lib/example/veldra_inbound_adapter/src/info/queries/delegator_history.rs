use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::common::wire::{DelegatorDelegateWire, DelegatorDeltaWire, DelegatorHistoryWire};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::info::common::wire::DelegatorHistoryWire>;
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
    ensure_type(&request.type_, "delegatorHistory")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![DelegatorHistoryWire {
        time: 1735380381353,
        hash: "0x55492465cb523f90815a041a226ba90147008d4b221a24ae8dc35a0dbede4ea4".to_string(),
        delta: DelegatorDeltaWire {
            delegate: Some(DelegatorDelegateWire {
                validator: "0x5ac99df645f3414876c816caa18b2d234024b487".to_string(),
                amount: "10000.0".to_string(),
                is_undelegate: false,
                extra: Default::default(),
            }),
            extra: Default::default(),
        },
    }]
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn delegator_history_serializes_delegate_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value,
            json!([{
                "time": 1735380381353u64,
                "hash": "0x55492465cb523f90815a041a226ba90147008d4b221a24ae8dc35a0dbede4ea4",
                "delta": {
                    "delegate": {
                        "validator": "0x5ac99df645f3414876c816caa18b2d234024b487",
                        "amount": "10000.0",
                        "isUndelegate": false
                    }
                }
            }])
        );
    }

    #[test]
    fn delegator_history_delta_accepts_unknown_fields() {
        let value = json!({
            "delegate": {
                "validator": "0x5ac99df645f3414876c816caa18b2d234024b487",
                "amount": "10000.0",
                "isUndelegate": false
            },
            "futureAction": { "kind": "restake" }
        });
        let parsed: DelegatorDeltaWire = serde_json::from_value(value).unwrap();
        assert_eq!(parsed.extra.get("futureAction"), Some(&json!({ "kind": "restake" })));
    }
}
