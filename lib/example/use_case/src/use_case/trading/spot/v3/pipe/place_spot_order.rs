use common_entity::fsm_v2::{CommandMetadata, SequencedCommand};
use common_entity::{
    CreatedEntityEvent, EntityError, ReplayableDomainEvent, ReplayableEntityChanges, TrackDiff,
};
use example_core_entity::{CreateSpotOrderV7Command, SpotOrderV7, SpotOrderV7Command};

/// 现货下单步骤的成功结果，内部 create event 是唯一业务真相。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderPipeChanges {
    pub order: CreatedEntityEvent<SpotOrderV7>,
}

impl PlaceSpotOrderPipeChanges {
    pub fn after(&self) -> &SpotOrderV7 {
        self.order.after()
    }
}

impl ReplayableEntityChanges for PlaceSpotOrderPipeChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        self.order.replayable_events()
    }
}

/// 在显式事件位置创建一个 `Pending` 现货订单。
pub fn place_spot_order(
    command: CreateSpotOrderV7Command,
    lsn: u64,
    command_index: u32,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<PlaceSpotOrderPipeChanges, EntityError> {
    let metadata = CommandMetadata::new_at(
        lsn,
        command_index,
        logical_ts,
        topic,
        partition,
        command.party_id.to_string(),
    );
    let order = SpotOrderV7::create(SequencedCommand::new(
        metadata,
        SpotOrderV7Command::Create(command),
    ))?;
    let mut event = order.track_create_event_at(lsn, command_index, logical_ts)?;
    event.t = topic;
    event.p = partition;
    event.domain_event_type = 1;

    let order = CreatedEntityEvent::try_new(order, event)
        .map_err(|_| EntityError::Custom("invalid created spot order event chain".to_owned()))?;
    Ok(PlaceSpotOrderPipeChanges { order })
}
