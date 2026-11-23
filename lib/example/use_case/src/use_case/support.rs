#[cfg(test)]
use common_entity::ReplayableDomainEvent;

pub const ACCOUNT_ENTITY_TYPE: u8 = 1;
pub const ORDER_ENTITY_TYPE: u8 = 2;

#[cfg(test)]
pub(crate) fn field_as_u64(event: &ReplayableDomainEvent, field_name: &str) -> Option<u64> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        std::str::from_utf8(change.new_value_bytes())
            .ok()?
            .parse::<u64>()
            .ok()
    })
}
