use serde::de::DeserializeOwned;

use crate::exchange::error::ExchangeHttpError;

pub fn parse_json_request<T>(body: &[u8]) -> Result<T, ExchangeHttpError>
where
    T: DeserializeOwned,
{
    crate::common::parse::parse_json_request(body)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct ProbeWire {
        value: u64,
    }

    #[test]
    fn parses_valid_json() {
        let request: ProbeWire =
            parse_json_request(br#"{ "value": 7 }"#).expect("valid json should parse");

        assert_eq!(request, ProbeWire { value: 7 });
    }

    #[test]
    fn maps_syntax_error_to_malformed_json() {
        let error = parse_json_request::<ProbeWire>(br#"{ "value": }"#)
            .expect_err("syntax error should fail");

        assert!(matches!(error, ExchangeHttpError::MalformedJson));
    }

    #[test]
    fn maps_shape_error_to_invalid_json_shape() {
        let missing_field_error =
            parse_json_request::<ProbeWire>(br#"{}"#).expect_err("missing field should fail");
        assert!(matches!(missing_field_error, ExchangeHttpError::InvalidJsonShape(_)));

        let wrong_type_error = parse_json_request::<ProbeWire>(br#"{ "value": "bad" }"#)
            .expect_err("wrong type should fail");
        assert!(matches!(wrong_type_error, ExchangeHttpError::InvalidJsonShape(_)));
    }
}
