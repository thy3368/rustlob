use crate::hyperliquid_ws::WsFillWire;
use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::hyperliquid_ws::WsFillWire>;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    user: String,
    #[serde(rename = "aggregateByTime")]
    aggregate_by_time: Option<bool>,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "userFills")?;
    validate_hex_address_field("user", &request.user)?;
    let _ = request.aggregate_by_time;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![WsFillWire {
        coin: "AVAX".to_string(),
        px: "18.435".to_string(),
        sz: "93.53".to_string(),
        side: "B".to_string(),
        time: 1681222254710,
        start_position: Some("26.86".to_string()),
        dir: Some("Open Long".to_string()),
        closed_pnl: Some("0.0".to_string()),
        hash: Some(
            "0xa166e3fa63c25663024b03f2e0da011a00307e4017465df020210d3d432e7cb8".to_string(),
        ),
        oid: Some(90542681),
        crossed: Some(false),
        fee: Some("0.01".to_string()),
        fee_token: Some("USDC".to_string()),
        builder_fee: Some("0.01".to_string()),
        tid: Some(118906512037719),
        extra: Default::default(),
    }]
}
