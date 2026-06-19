use crate::hyperliquid_ws::{WsBookWire, WsLevelWire};
use crate::info::common::validate::{
    ensure_type, validate_mantissa, validate_n_sig_figs, validate_non_empty_string_field,
};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = crate::hyperliquid_ws::WsBookWire;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    coin: String,
    #[serde(rename = "nSigFigs")]
    n_sig_figs: Option<u32>,
    mantissa: Option<u32>,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire =
        serde_json::from_slice(body).map_err(InfoHttpError::from_json_error)?;
    ensure_type(&request.type_, "l2Book")?;
    validate_non_empty_string_field("coin", &request.coin)?;
    validate_n_sig_figs(request.n_sig_figs)?;
    validate_mantissa(request.n_sig_figs, request.mantissa)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    WsBookWire {
        coin: "BTC".to_string(),
        time: 1754450974231,
        levels: vec![
            vec![
                WsLevelWire {
                    px: "113377.0".to_string(),
                    sz: "7.6699".to_string(),
                    n: 17,
                    extra: Default::default(),
                },
                WsLevelWire {
                    px: "113376.0".to_string(),
                    sz: "4.13714".to_string(),
                    n: 8,
                    extra: Default::default(),
                },
            ],
            vec![WsLevelWire {
                px: "113397.0".to_string(),
                sz: "0.11543".to_string(),
                n: 3,
                extra: Default::default(),
            }],
        ],
        extra: Default::default(),
    }
}
