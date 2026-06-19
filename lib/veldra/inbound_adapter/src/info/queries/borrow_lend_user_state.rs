use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::common::wire::{
    BorrowLendPositionWire, BorrowLendStateWire, BorrowLendUserStateWire,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = crate::info::common::wire::BorrowLendUserStateWire;
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
    ensure_type(&request.type_, "borrowLendUserState")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    BorrowLendUserStateWire {
        token_to_state: vec![
            (
                0,
                BorrowLendStateWire {
                    borrow: BorrowLendPositionWire {
                        basis: "0.0".to_string(),
                        value: "0.0".to_string(),
                        extra: Default::default(),
                    },
                    supply: BorrowLendPositionWire {
                        basis: "44.69295862".to_string(),
                        value: "44.69692314".to_string(),
                        extra: Default::default(),
                    },
                    extra: Default::default(),
                },
            ),
            (
                1105,
                BorrowLendStateWire {
                    borrow: BorrowLendPositionWire {
                        basis: "0.0".to_string(),
                        value: "0.0".to_string(),
                        extra: Default::default(),
                    },
                    supply: BorrowLendPositionWire {
                        basis: "0.0".to_string(),
                        value: "0.0".to_string(),
                        extra: Default::default(),
                    },
                    extra: Default::default(),
                },
            ),
        ],
        health: "healthy".to_string(),
        health_factor: None,
        extra: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn borrow_lend_user_state_serializes_to_tuple_list_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value["tokenToState"],
            json!([
                [0, {
                    "borrow": { "basis": "0.0", "value": "0.0" },
                    "supply": { "basis": "44.69295862", "value": "44.69692314" }
                }],
                [1105, {
                    "borrow": { "basis": "0.0", "value": "0.0" },
                    "supply": { "basis": "0.0", "value": "0.0" }
                }]
            ])
        );
    }

    #[test]
    fn borrow_lend_user_state_accepts_unknown_fields() {
        let value = json!({
            "tokenToState": [],
            "health": "healthy",
            "healthFactor": null,
            "future": "field"
        });
        let parsed: BorrowLendUserStateWire = serde_json::from_value(value).unwrap();
        assert_eq!(parsed.extra.get("future"), Some(&json!("field")));
    }
}
