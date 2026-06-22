use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommonExchangeFields {
    pub nonce: u64,
    pub signature: SignatureWire,
    #[serde(rename = "vaultAddress", skip_serializing_if = "Option::is_none")]
    pub vault_address: Option<String>,
    #[serde(rename = "expiresAfter", skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureWire {
    pub r: String,
    pub s: String,
    pub v: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonObjectWire(pub BTreeMap<String, JsonValue>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExchangeRequestEnvelopeWire<TAction> {
    pub action: TAction,
    #[serde(flatten)]
    pub common: CommonExchangeFields,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ExchangeActionTypeProbe {
    pub action: ExchangeActionTypeField,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ExchangeActionTypeField {
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeErrorResponseWire {
    pub status: &'static str,
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeResponseWire<TData> {
    pub status: &'static str,
    pub response: ExchangeResponseEnvelopeWire<TData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeResponseEnvelopeWire<TData> {
    #[serde(rename = "type")]
    pub type_: &'static str,
    pub data: TData,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeStatusesDataWire<TStatus> {
    pub statuses: Vec<TStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeStatusDataWire<TStatus> {
    pub status: TStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeEmptyResponseWire {
    pub status: &'static str,
    pub response: ExchangeEmptyResponseEnvelopeWire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExchangeEmptyResponseEnvelopeWire {
    #[serde(rename = "type")]
    pub type_: &'static str,
}

pub fn ok_default_response() -> ExchangeEmptyResponseWire {
    ExchangeEmptyResponseWire {
        status: "ok",
        response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
    }
}

pub fn ok_status_response<TStatus>(
    type_: &'static str,
    status: TStatus,
) -> ExchangeResponseWire<ExchangeStatusDataWire<TStatus>> {
    ExchangeResponseWire {
        status: "ok",
        response: ExchangeResponseEnvelopeWire { type_, data: ExchangeStatusDataWire { status } },
    }
}

pub fn ok_statuses_response<TStatus>(
    type_: &'static str,
    statuses: Vec<TStatus>,
) -> ExchangeResponseWire<ExchangeStatusesDataWire<TStatus>> {
    ExchangeResponseWire {
        status: "ok",
        response: ExchangeResponseEnvelopeWire {
            type_,
            data: ExchangeStatusesDataWire { statuses },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statuses_template_serializes_to_expected_shape() {
        let response = ExchangeResponseWire {
            status: "ok",
            response: ExchangeResponseEnvelopeWire {
                type_: "order",
                data: ExchangeStatusesDataWire { statuses: vec!["success"] },
            },
        };

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "order",
    "data": {
      "statuses": [
        "success"
      ]
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn status_template_serializes_to_expected_shape() {
        let response = ExchangeResponseWire {
            status: "ok",
            response: ExchangeResponseEnvelopeWire {
                type_: "twapCancel",
                data: ExchangeStatusDataWire { status: "success" },
            },
        };

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "twapCancel",
    "data": {
      "status": "success"
    }
  }
}"#;

        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_template_serializes_to_expected_shape() {
        let response = ExchangeEmptyResponseWire {
            status: "ok",
            response: ExchangeEmptyResponseEnvelopeWire { type_: "default" },
        };

        let actual = serde_json::to_string_pretty(&response).expect("response serializes");
        let expected = r#"{
  "status": "ok",
  "response": {
    "type": "default"
  }
}"#;

        assert_eq!(actual, expected);
    }
}
