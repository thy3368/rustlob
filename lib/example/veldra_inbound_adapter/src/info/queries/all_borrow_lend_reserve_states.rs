use crate::info::common::validate::ensure_type;
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ReserveStateWire =
        crate::info::queries::borrow_lend_reserve_state::reply::ResponseWire;
    pub type ResponseWire = Vec<(u32, ReserveStateWire)>;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
}
pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "allBorrowLendReserveStates")?;
    Ok(stub_response())
}
pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![
        (0, crate::info::queries::borrow_lend_reserve_state::stub_response()),
        (
            150,
            reply::ReserveStateWire {
                borrow_yearly_rate: "0.05".to_string(),
                supply_yearly_rate: "0.0".to_string(),
                balance: "11318.09684696".to_string(),
                utilization: "0.0".to_string(),
                oracle_px: "23.99".to_string(),
                ltv: "0.5".to_string(),
                total_supplied: "11318.09684696".to_string(),
                total_borrowed: "0.0".to_string(),
            },
        ),
    ]
}
