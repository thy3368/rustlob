use crate::info::common::validate::ensure_type;
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct ResponseWire {
        #[serde(rename = "borrowYearlyRate")]
        pub borrow_yearly_rate: String,
        #[serde(rename = "supplyYearlyRate")]
        pub supply_yearly_rate: String,
        pub balance: String,
        pub utilization: String,
        #[serde(rename = "oraclePx")]
        pub oracle_px: String,
        pub ltv: String,
        #[serde(rename = "totalSupplied")]
        pub total_supplied: String,
        #[serde(rename = "totalBorrowed")]
        pub total_borrowed: String,
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
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "borrowLendReserveState")?;
    let _ = request.token;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    reply::ResponseWire {
        borrow_yearly_rate: "0.05".to_string(),
        supply_yearly_rate: "0.0008245002".to_string(),
        balance: "3245939.4732256099".to_string(),
        utilization: "0.018322226".to_string(),
        oracle_px: "1.0".to_string(),
        ltv: "0.0".to_string(),
        total_supplied: "3306509.7335290499".to_string(),
        total_borrowed: "60582.61869494".to_string(),
    }
}
