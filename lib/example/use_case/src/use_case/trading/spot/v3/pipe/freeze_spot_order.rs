use common_entity::fsm_v2::Fsm as FsmV2;
use common_entity::{
    Entity, EntityError, LogEntrySequencer, ReplayableDomainEvent, ReplayableEntityChanges, Shard,
    UpdatedEntityEvents,
};
use example_core_entity::{
    BalanceV2, BalanceV2Command, BalanceV2Error, SpotOrderV7, SpotOrderV7Command,
};

/// 冻结现货订单步骤产生的订单与余额变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeSpotOrderPipeChanges {
    pub order: UpdatedEntityEvents<SpotOrderV7>,
    pub balance: UpdatedEntityEvents<BalanceV2>,
}

impl ReplayableEntityChanges for FreezeSpotOrderPipeChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let mut events = self.order.replayable_events();
        events.extend(self.balance.replayable_events());
        events
    }
}

/// 从指定 `command_index` 起连续生成订单和余额冻结事件。
pub fn freeze_spot_order_from(
    order: &SpotOrderV7,
    balance: &BalanceV2,
    lsn: u64,
    command_index: u32,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<FreezeSpotOrderPipeChanges, EntityError> {
    if balance.party_id() != &order.order_base().owner {
        return Err(EntityError::Custom(
            "balance owner does not match the order owner".to_owned(),
        ));
    }
    let order_loaded_position = (order.lsn(), order.command_index());
    let order_command = LogEntrySequencer::from_metadata_with_topic_at(
        topic,
        partition,
        lsn,
        command_index,
        logical_ts,
        0,
        0,
        order.party_id(),
        SpotOrderV7Command::Freeze,
    );
    let (order_after, order_event) = FsmV2::transition(order, order_command)?;
    let order_changes = UpdatedEntityEvents::try_new(
        order_after,
        order_loaded_position.0,
        order_loaded_position.1,
        vec![order_event],
    )
    .map_err(|error| EntityError::Custom(format!("invalid order event chain: {error:?}")))?;
    let requirement = order_changes
        .after()
        .taker_freeze_amount()
        .map_err(|error| EntityError::Custom(format!("failed to derive freeze amount: {error}")))?;
    if balance.coin_idx() != u64::from(requirement.currency().index()) {
        return Err(EntityError::Custom(
            "balance coin does not match the asset frozen by the order".to_owned(),
        ));
    }

    let balance_loaded_position = (balance.lsn(), balance.command_index());
    let balance_command = LogEntrySequencer::from_metadata_with_topic_at(
        topic,
        partition,
        lsn,
        command_index
            .checked_add(1)
            .ok_or_else(|| EntityError::Custom("freeze command index overflow".to_owned()))?,
        logical_ts,
        0,
        0,
        balance.party_id().to_string(),
        BalanceV2Command::Freeze {
            amount: requirement.amount(),
        },
    );
    let (balance_after, balance_event) =
        FsmV2::transition(balance, balance_command).map_err(map_balance_error)?;
    let balance_changes = UpdatedEntityEvents::try_new(
        balance_after,
        balance_loaded_position.0,
        balance_loaded_position.1,
        vec![balance_event],
    )
    .map_err(|error| EntityError::Custom(format!("invalid balance event chain: {error:?}")))?;

    Ok(FreezeSpotOrderPipeChanges {
        order: order_changes,
        balance: balance_changes,
    })
}

fn map_balance_error(error: BalanceV2Error) -> EntityError {
    match error {
        BalanceV2Error::Replay(error) => error,
        error => EntityError::Custom(format!("failed to freeze balance: {error}")),
    }
}
