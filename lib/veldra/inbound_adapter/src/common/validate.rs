pub fn validate_prefixed_hex_with_len(value: &str, hex_len: usize) -> Result<(), ()> {
    if is_prefixed_hex_with_len(value, hex_len) { Ok(()) } else { Err(()) }
}

pub fn validate_hex_address(value: &str) -> Result<(), ()> {
    validate_prefixed_hex_with_len(value, 40)
}

pub fn validate_cloid(value: &str) -> Result<(), ()> {
    validate_prefixed_hex_with_len(value, 32)
}

fn is_prefixed_hex_with_len(value: &str, hex_len: usize) -> bool {
    let Some(rest) = value.strip_prefix("0x") else {
        return false;
    };
    rest.len() == hex_len && rest.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_address_requires_lowercase_prefix_and_40_hex_digits() {
        assert!(validate_hex_address("0x1111111111111111111111111111111111111111").is_ok());
        assert!(validate_hex_address("0xABCDEFabcdef1234567890ABCDEFabcdef123456").is_ok());
        assert!(validate_hex_address("1111111111111111111111111111111111111111").is_err());
        assert!(validate_hex_address("0X1111111111111111111111111111111111111111").is_err());
        assert!(validate_hex_address("0x111111111111111111111111111111111111111").is_err());
        assert!(validate_hex_address("0x111111111111111111111111111111111111111g").is_err());
    }

    #[test]
    fn cloid_requires_lowercase_prefix_and_32_hex_digits() {
        assert!(validate_cloid("0x1234567890abcdef1234567890abcdef").is_ok());
        assert!(validate_cloid("0x1234567890abcdef1234567890abcde").is_err());
        assert!(validate_cloid("1234567890abcdef1234567890abcdef").is_err());
        assert!(validate_cloid("0x1234567890abcdef1234567890abcdeg").is_err());
    }
}
