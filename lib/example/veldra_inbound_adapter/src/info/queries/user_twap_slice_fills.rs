use serde_json::{Map, Number, Value};

use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = serde_json::Value;
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
    ensure_type(&request.type_, "userTwapSliceFills")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    Value::Array(vec![object([
        (
            "fill",
            object([
                ("closedPnl", string_value("0.0")),
                ("coin", string_value("AVAX")),
                ("crossed", Value::Bool(true)),
                ("dir", string_value("Open Long")),
                (
                    "hash",
                    string_value(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ),
                ),
                ("oid", u64_value(90_542_681)),
                ("px", string_value("18.435")),
                ("side", string_value("B")),
                ("startPosition", string_value("26.86")),
                ("sz", string_value("93.53")),
                ("time", u64_value(1_681_222_254_710)),
                ("fee", string_value("0.01")),
                ("feeToken", string_value("USDC")),
                ("tid", u64_value(118_906_512_037_719)),
            ]),
        ),
        ("twapId", u64_value(3_156)),
    ])])
}

fn object<const N: usize>(entries: [(&str, Value); N]) -> Value {
    let mut object = Map::new();
    for (name, value) in entries {
        object.insert(name.to_string(), value);
    }
    Value::Object(object)
}

fn string_value(value: impl Into<String>) -> Value {
    Value::String(value.into())
}

fn u64_value(value: u64) -> Value {
    Value::Number(Number::from(value))
}
