pub use crate::common::validate::{validate_cloid, validate_hex_address};
use crate::exchange::common::wire::CommonExchangeFields;
use crate::exchange::error::SharedFieldError;

pub fn validate_envelope_common(common: &CommonExchangeFields) -> Result<(), SharedFieldError> {
    validate_common_fields(
        common.nonce,
        common.expires_after,
        &common.signature.r,
        &common.signature.s,
        common.signature.v,
        common.vault_address.as_deref(),
    )
}

pub fn validate_common_fields(
    nonce: u64,
    expires_after: Option<u64>,
    signature_r: &str,
    signature_s: &str,
    signature_v: u64,
    vault_address: Option<&str>,
) -> Result<(), SharedFieldError> {
    if nonce == 0 {
        return Err(SharedFieldError::InvalidNonce);
    }
    if let Some(expires_after) = expires_after {
        if expires_after == 0 {
            return Err(SharedFieldError::InvalidExpiresAfter);
        }
    }
    validate_signature(signature_r, signature_s, signature_v)?;
    if let Some(vault_address) = vault_address {
        validate_hex_address(vault_address).map_err(|_| SharedFieldError::InvalidVaultAddress)?;
    }
    Ok(())
}

pub fn validate_signature(r: &str, s: &str, v: u64) -> Result<(), SharedFieldError> {
    if !(crate::common::validate::validate_prefixed_hex_with_len(r, 64).is_ok()
        && crate::common::validate::validate_prefixed_hex_with_len(s, 64).is_ok()
        && v <= 255)
    {
        return Err(SharedFieldError::InvalidSignature);
    }
    Ok(())
}

pub fn validate_hyperliquid_chain(value: &str) -> Result<(), ()> {
    if matches!(value, "Mainnet" | "Testnet") { Ok(()) } else { Err(()) }
}

pub fn validate_signature_chain_id(value: &str) -> Result<(), ()> {
    let Some(rest) = value.strip_prefix("0x") else {
        return Err(());
    };
    if rest.is_empty() || !rest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(());
    }
    Ok(())
}
