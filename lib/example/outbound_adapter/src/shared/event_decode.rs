use cmd_handler::EntityReplayableEvent;

pub(crate) fn event_string_field(
    event: &EntityReplayableEvent,
    field_name: &str,
) -> Option<String> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        Some(String::from_utf8_lossy(change.new_value_bytes()).to_string())
    })
}

pub(crate) fn event_u64_field(event: &EntityReplayableEvent, field_name: &str) -> Option<u64> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        std::str::from_utf8(change.new_value_bytes()).ok()?.parse::<u64>().ok()
    })
}

pub(crate) fn event_order_sequence(event: &EntityReplayableEvent) -> Option<u64> {
    event_u64_field(event, "order_sequence").or_else(|| {
        let order_id = event_string_field(event, "order_id")?;
        order_id.rsplit('-').next()?.parse::<u64>().ok()
    })
}

#[cfg(test)]
mod tests {
    use cmd_handler::{EntityReplayableEvent, ReplayFieldChange};
    use example_core::ORDER_ENTITY_TYPE;

    use super::event_order_sequence;

    fn string_field(name: &str, value: &str) -> ReplayFieldChange {
        ReplayFieldChange::new(
            ReplayFieldChange::field_name_from_str(name),
            &[],
            value.as_bytes(),
            0,
        )
    }

    fn int_field(name: &str, value: u64) -> ReplayFieldChange {
        ReplayFieldChange::new(
            ReplayFieldChange::field_name_from_str(name),
            &[],
            value.to_string().as_bytes(),
            1,
        )
    }

    #[test]
    fn event_order_sequence_prefers_explicit_field() {
        let mut event = EntityReplayableEvent::new_created(0, 0, 1, ORDER_ENTITY_TYPE);
        event.add_field_change(string_field("order_id", "trader-1-BTCUSDT-9"));
        event.add_field_change(int_field("order_sequence", 7));

        assert_eq!(event_order_sequence(&event), Some(7));
    }

    #[test]
    fn event_order_sequence_falls_back_to_order_id_suffix() {
        let mut event = EntityReplayableEvent::new_created(0, 0, 1, ORDER_ENTITY_TYPE);
        event.add_field_change(string_field("order_id", "trader-1-BTCUSDT-9"));

        assert_eq!(event_order_sequence(&event), Some(9));
    }
}
