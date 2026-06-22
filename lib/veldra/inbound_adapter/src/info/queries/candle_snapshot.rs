use crate::hyperliquid_ws::CandleWire;
use crate::info::common::validate::{
    ensure_type, validate_non_empty_string_field, validate_positive_u64_field,
};
use crate::info::common::wire::CandleSnapshotReqWire;
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::hyperliquid_ws::CandleWire>;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    req: CandleSnapshotReqWire,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "candleSnapshot")?;
    validate_non_empty_string_field("req.coin", &request.req.coin)?;
    validate_non_empty_string_field("req.interval", &request.req.interval)?;
    validate_positive_u64_field("req.startTime", request.req.start_time)?;
    validate_positive_u64_field("req.endTime", request.req.end_time)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![CandleWire {
        t: 1681923600000,
        close_time: 1681924499999,
        s: "BTC".to_string(),
        i: "15m".to_string(),
        o: "29295.0".to_string(),
        c: "29258.0".to_string(),
        h: "29309.0".to_string(),
        l: "29250.0".to_string(),
        v: "0.98639".to_string(),
        num_trades: 189,
        extra: Default::default(),
    }]
}
