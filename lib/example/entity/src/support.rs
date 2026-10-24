pub(crate) fn concat2(a: &str, b: &str) -> String {
    let mut out = String::with_capacity(a.len().saturating_add(b.len()));
    out.push_str(a);
    out.push_str(b);
    out
}

pub(crate) fn concat3(a: &str, b: &str, c: &str) -> String {
    let mut out = String::with_capacity(a.len().saturating_add(b.len()).saturating_add(c.len()));
    out.push_str(a);
    out.push_str(b);
    out.push_str(c);
    out
}

pub(crate) fn concat4(a: &str, b: &str, c: &str, d: &str) -> String {
    let mut out = String::with_capacity(
        a.len().saturating_add(b.len()).saturating_add(c.len()).saturating_add(d.len()),
    );
    out.push_str(a);
    out.push_str(b);
    out.push_str(c);
    out.push_str(d);
    out
}

pub(crate) fn concat5(a: &str, b: &str, c: &str, d: &str, e: &str) -> String {
    let mut out = String::with_capacity(
        a.len()
            .saturating_add(b.len())
            .saturating_add(c.len())
            .saturating_add(d.len())
            .saturating_add(e.len()),
    );
    out.push_str(a);
    out.push_str(b);
    out.push_str(c);
    out.push_str(d);
    out.push_str(e);
    out
}

/// 对带类型域的业务身份字段进行确定性编码；每个字段保留长度边界。
pub(crate) fn derived_entity_id(entity_type: u8, fields: &[&[u8]]) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    let mut hash = (OFFSET ^ u64::from(entity_type)).wrapping_mul(PRIME);
    for field in fields {
        for byte in (field.len() as u64).to_le_bytes().into_iter().chain(field.iter().copied()) {
            hash = (hash ^ u64::from(byte)).wrapping_mul(PRIME);
        }
    }
    hash
}

#[cfg(test)]
mod identity_tests {
    use super::derived_entity_id;

    #[test]
    fn derived_identity_preserves_type_and_field_boundaries() {
        assert_eq!(derived_entity_id(6, &[b"a", b"bc"]), derived_entity_id(6, &[b"a", b"bc"]));
        assert_ne!(derived_entity_id(6, &[b"a", b"bc"]), derived_entity_id(6, &[b"ab", b"c"]));
        assert_ne!(derived_entity_id(6, &[b"a", b"bc"]), derived_entity_id(7, &[b"a", b"bc"]));
    }
}
