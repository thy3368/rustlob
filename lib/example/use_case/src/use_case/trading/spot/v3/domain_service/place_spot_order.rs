use common_entity::fsm_v2::{CommandMetadata, SequencedCommand};
use common_entity::{CreatedEntityEvent, EntityError, EventPosition, TrackDiff};
use example_core_entity::{CreateSpotOrderV7Command, SpotOrderV7, SpotOrderV7Command};

/// 仅创建一个 `Pending` 现货订单及其 replay create event。
pub fn place_spot_order(
    command: CreateSpotOrderV7Command,
    lsn: u64,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<CreatedEntityEvent<SpotOrderV7>, EntityError> {
    place_spot_order_at(
        command,
        EventPosition::new(lsn, 0),
        logical_ts,
        topic,
        partition,
    )
}

/// 在显式事件位置创建一个 `Pending` 现货订单。
pub fn place_spot_order_at(
    command: CreateSpotOrderV7Command,
    position: EventPosition,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<CreatedEntityEvent<SpotOrderV7>, EntityError> {
    let metadata = CommandMetadata::new_at(
        position,
        logical_ts,
        topic,
        partition,
        command.party_id.to_string(),
    );
    let order = SpotOrderV7::create(SequencedCommand::new(
        metadata,
        SpotOrderV7Command::Create(command),
    ))?;
    let mut event = order.track_create_event_at(position, logical_ts)?;
    event.t = topic;
    event.p = partition;
    event.domain_event_type = 1;

    CreatedEntityEvent::try_new(order, event)
        .map_err(|_| EntityError::Custom("invalid created spot order event chain".to_owned()))
}
