use crate::info::common::validate::ensure_type;
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct ResponseWire {
        #[serde(rename = "isAligned")]
        pub is_aligned: bool,
        #[serde(rename = "firstAlignedTime")]
        pub first_aligned_time: u64,
        #[serde(rename = "evmMintedSupply")]
        pub evm_minted_supply: String,
        #[serde(rename = "dailyAmountOwed")]
        pub daily_amount_owed: Vec<(String, String)>,
        #[serde(rename = "predictedRate")]
        pub predicted_rate: String,
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    token: u32,
}
pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire =
        serde_json::from_slice(body).map_err(InfoHttpError::from_json_error)?;
    ensure_type(&request.type_, "alignedQuoteTokenInfo")?;
    let _ = request.token;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    reply::ResponseWire {
        is_aligned: true,
        first_aligned_time: 1758949452538,
        evm_minted_supply: "0.0".to_string(),
        daily_amount_owed: vec![
            ("2025-10-04".to_string(), "0.0".to_string()),
            ("2025-10-05".to_string(), "0.0".to_string()),
        ],
        predicted_rate: "0.01".to_string(),
    }
}
