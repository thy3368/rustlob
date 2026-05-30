#[cfg(test)]
use cmd_handler::EntityReplayableEvent;
use cmd_handler::ReplayFieldChange;

pub const ACCOUNT_ENTITY_TYPE: u8 = 1;
pub const ORDER_ENTITY_TYPE: u8 = 2;

pub(crate) const FIELD_TYPE_STRING: u8 = 0;
pub(crate) const FIELD_TYPE_INT: u8 = 1;

pub(crate) fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
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

#[cfg(test)]
pub(crate) fn field_as_u64(event: &EntityReplayableEvent, field_name: &str) -> Option<u64> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        std::str::from_utf8(change.new_value_bytes()).ok()?.parse::<u64>().ok()
    })
}
