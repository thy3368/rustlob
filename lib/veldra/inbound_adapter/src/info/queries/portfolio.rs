use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::common::wire::PortfolioSliceWire;
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::info::common::wire::PortfolioPeriodWire>;
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
    ensure_type(&request.type_, "portfolio")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![
        (
            "day".to_string(),
            PortfolioSliceWire {
                account_value_history: vec![
                    (1741886630493, "0.0".to_string()),
                    (1741895270493, "0.0".to_string()),
                ],
                pnl_history: vec![
                    (1741886630493, "0.0".to_string()),
                    (1741895270493, "0.0".to_string()),
                ],
                vlm: "0.0".to_string(),
                extra: Default::default(),
            },
        ),
        (
            "week".to_string(),
            PortfolioSliceWire {
                account_value_history: vec![],
                pnl_history: vec![],
                vlm: "0.0".to_string(),
                extra: Default::default(),
            },
        ),
    ]
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn portfolio_serializes_to_tuple_list_shape() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(
            value,
            json!([
                ["day",{
                    "accountValueHistory":[[1741886630493u64,"0.0"],[1741895270493u64,"0.0"]],
                    "pnlHistory":[[1741886630493u64,"0.0"],[1741895270493u64,"0.0"]],
                    "vlm":"0.0"
                }],
                ["week",{
                    "accountValueHistory":[],
                    "pnlHistory":[],
                    "vlm":"0.0"
                }]
            ])
        );
    }
}
