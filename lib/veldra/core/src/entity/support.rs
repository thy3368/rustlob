use cmd_handler::ReplayFieldChange;

pub(crate) const FIELD_TYPE_STRING: u8 = 0;
pub(crate) const FIELD_TYPE_INT: u8 = 1;
pub(crate) const FIELD_TYPE_BOOL: u8 = 2;

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

pub(crate) fn stable_hash_hex(parts: &[impl AsRef<str>]) -> String {
    let mut hash = FNV_OFFSET_BASIS;
    for part in parts {
        for byte in part.as_ref().as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash ^= u64::from(0xff_u8);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}")
}

pub(crate) fn stable_positive_i64(value: &str) -> i64 {
    let mut hash = FNV_OFFSET_BASIS;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    (hash & i64::MAX as u64) as i64
}

pub(crate) fn string_field(name: &str, value: &str) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.as_bytes(),
        FIELD_TYPE_STRING,
    )
}

pub(crate) fn int_field(name: &str, value: u64) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.to_string().as_bytes(),
        FIELD_TYPE_INT,
    )
}

pub(crate) fn updated_int_field(name: &str, old_value: u64, new_value: u64) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        old_value.to_string().as_bytes(),
        new_value.to_string().as_bytes(),
        FIELD_TYPE_INT,
    )
}
