use std::collections::HashMap;

pub(crate) fn named_params<const N: usize>(entries: [(&str, mysql::Value); N]) -> mysql::Params {
    mysql::Params::Named(
        entries
            .into_iter()
            .map(|(name, value)| (name.as_bytes().to_vec(), value))
            .collect::<HashMap<_, _>>(),
    )
}

pub(crate) fn optional_u64_param(value: Option<u64>) -> mysql::Value {
    match value {
        Some(value) => mysql::Value::from(value),
        None => mysql::Value::NULL,
    }
}

pub(crate) fn optional_string_param(value: Option<String>) -> mysql::Value {
    match value {
        Some(value) => mysql::Value::from(value),
        None => mysql::Value::NULL,
    }
}
