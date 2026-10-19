/// Stable FNV-1a identity for string business keys used by replay events.
pub fn stable_string_entity_id(value: &str) -> u64 {
    value.bytes().fold(0xcbf29ce484222325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(test)]
mod tests {
    use super::stable_string_entity_id;

    #[test]
    fn string_ids_are_stable() {
        assert_eq!(stable_string_entity_id(""), 0xcbf29ce484222325);
        assert_eq!(stable_string_entity_id("a"), 0xaf63dc4c8601ec8c);
        assert_eq!(stable_string_entity_id("account-1"), stable_string_entity_id("account-1"));
        assert_ne!(stable_string_entity_id("account-1"), stable_string_entity_id("account-2"));
    }
}
