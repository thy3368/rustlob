use serde::de::DeserializeOwned;

pub trait JsonRequestError: Sized {
    fn malformed_json() -> Self;
    fn invalid_json_shape(message: String) -> Self;
}

pub fn parse_json_request<T, E>(body: &[u8]) -> Result<T, E>
where
    T: DeserializeOwned,
    E: JsonRequestError,
{
    serde_json::from_slice(body).map_err(classify_json_error)
}

pub fn classify_json_error<E>(error: serde_json::Error) -> E
where
    E: JsonRequestError,
{
    match error.classify() {
        serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
            E::malformed_json()
        }
        serde_json::error::Category::Data | serde_json::error::Category::Io => {
            E::invalid_json_shape(error.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum ProbeError {
        MalformedJson,
        InvalidJsonShape(String),
    }

    impl JsonRequestError for ProbeError {
        fn malformed_json() -> Self {
            Self::MalformedJson
        }

        fn invalid_json_shape(message: String) -> Self {
            Self::InvalidJsonShape(message)
        }
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct ProbeWire {
        value: u64,
    }

    #[test]
    fn parses_valid_json() {
        let request: ProbeWire = parse_json_request::<ProbeWire, ProbeError>(br#"{ "value": 7 }"#)
            .expect("valid json should parse");

        assert_eq!(request, ProbeWire { value: 7 });
    }

    #[test]
    fn maps_syntax_error_to_malformed_json() {
        let error = parse_json_request::<ProbeWire, ProbeError>(br#"{ "value": }"#)
            .expect_err("syntax error should fail");

        assert_eq!(error, ProbeError::MalformedJson);
    }

    #[test]
    fn maps_shape_error_to_invalid_json_shape() {
        let missing_field_error = parse_json_request::<ProbeWire, ProbeError>(br#"{}"#)
            .expect_err("missing field should fail");
        assert!(matches!(missing_field_error, ProbeError::InvalidJsonShape(_)));

        let wrong_type_error =
            parse_json_request::<ProbeWire, ProbeError>(br#"{ "value": "bad" }"#)
                .expect_err("wrong type should fail");
        assert!(matches!(wrong_type_error, ProbeError::InvalidJsonShape(_)));
    }
}
