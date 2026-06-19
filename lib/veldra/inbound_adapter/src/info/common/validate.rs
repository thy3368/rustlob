use crate::info::common::wire::OidWire;
use crate::info::error::InfoHttpError;

pub fn ensure_type(actual: &str, expected: &str) -> Result<(), InfoHttpError> {
    if actual == expected {
        Ok(())
    } else {
        Err(InfoHttpError::validation(format!(
            "Unexpected `type` for `{expected}` handler: `{actual}`."
        )))
    }
}

pub fn validate_hex_address_field(field: &str, value: &str) -> Result<(), InfoHttpError> {
    validate_hex_address(value).map_err(|_| {
        InfoHttpError::validation(format!(
            "Invalid `{field}`. Expected a 42-character hexadecimal address."
        ))
    })
}

pub fn validate_optional_hex_address_field(
    field: &str,
    value: Option<&str>,
) -> Result<(), InfoHttpError> {
    if let Some(value) = value {
        validate_hex_address_field(field, value)?;
    }
    Ok(())
}

pub fn validate_non_empty_string_field(field: &str, value: &str) -> Result<(), InfoHttpError> {
    if value.trim().is_empty() {
        return Err(InfoHttpError::validation(format!(
            "Invalid `{field}`. Expected a non-empty string."
        )));
    }
    Ok(())
}

pub fn validate_positive_u64_field(field: &str, value: u64) -> Result<(), InfoHttpError> {
    if value == 0 {
        return Err(InfoHttpError::validation(format!(
            "Invalid `{field}`. Expected a positive integer."
        )));
    }
    Ok(())
}

pub fn validate_optional_positive_u64_field(
    field: &str,
    value: Option<u64>,
) -> Result<(), InfoHttpError> {
    if let Some(value) = value {
        validate_positive_u64_field(field, value)?;
    }
    Ok(())
}

pub fn validate_n_sig_figs(n_sig_figs: Option<u32>) -> Result<(), InfoHttpError> {
    if let Some(n_sig_figs) = n_sig_figs {
        if !matches!(n_sig_figs, 2..=5) {
            return Err(InfoHttpError::validation(
                "Invalid `nSigFigs`. Expected one of `2`, `3`, `4`, `5`.".to_string(),
            ));
        }
    }
    Ok(())
}

pub fn validate_mantissa(
    n_sig_figs: Option<u32>,
    mantissa: Option<u32>,
) -> Result<(), InfoHttpError> {
    if let Some(mantissa) = mantissa {
        if n_sig_figs != Some(5) || !matches!(mantissa, 1 | 2 | 5) {
            return Err(InfoHttpError::validation(
                "Invalid `mantissa`. It is only allowed when `nSigFigs` is `5`, and must be one of `1`, `2`, `5`.".to_string(),
            ));
        }
    }
    Ok(())
}

pub fn validate_oid_field(field: &str, oid: &OidWire) -> Result<(), InfoHttpError> {
    match oid {
        OidWire::Oid(value) => validate_positive_u64_field(field, *value),
        OidWire::Cloid(value) => validate_cloid(value).map_err(|_| {
            InfoHttpError::validation(format!(
                "Invalid `{field}`. Expected a positive u64 oid or 128-bit hexadecimal cloid."
            ))
        }),
    }
}

fn validate_hex_address(value: &str) -> Result<(), ()> {
    if is_prefixed_hex_with_len(value, 40) { Ok(()) } else { Err(()) }
}

fn validate_cloid(value: &str) -> Result<(), ()> {
    if is_prefixed_hex_with_len(value, 32) { Ok(()) } else { Err(()) }
}

fn is_prefixed_hex_with_len(value: &str, hex_len: usize) -> bool {
    let Some(rest) = value.strip_prefix("0x") else {
        return false;
    };
    rest.len() == hex_len && rest.bytes().all(|byte| byte.is_ascii_hexdigit())
}
