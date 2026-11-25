use common_entity::{
    CreatedEntityEvent, EntityError, LogEntry, ReplayableDomainEvent, ReplayableEntityChanges,
    TrackDiff,
};
use example_core_entity::{PlaceSpotOrderPipelineV2Cmd, SpotOrderV7, SpotOrderV7Command};

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
    command: LogEntry<PlaceSpotOrderPipelineV2Cmd>,
) -> Result<PlaceSpotOrderPipeChanges, EntityError> {
    let lsn = command.lsn();
    let command_index = command.command_index();
    let logical_ts = command.logical_ts();
    let topic = command.topic();
    let partition = command.partition();
    let order = SpotOrderV7::create(SpotOrderV7Command::Create(command))?;
    let mut event = order.track_create_event_at(lsn, command_index, logical_ts)?;
    event.t = topic;
    event.p = partition;
    event.domain_event_type = 1;

    let order = CreatedEntityEvent::try_new(order, event)
        .map_err(|_| EntityError::Custom("invalid created spot order event chain".to_owned()))?;
    Ok(PlaceSpotOrderPipeChanges { order })
}
