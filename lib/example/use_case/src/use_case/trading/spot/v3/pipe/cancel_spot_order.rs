use common_entity::fsm_v2::{
    CommandMetadata as FsmCommandMetadata, Fsm as FsmV2, SequencedCommand,
};
use common_entity::{
    EntityError, PositionedEntity, ReplayableDomainEvent, ReplayableEntityChanges, Shard,
    UpdatedEntityEvents,
};
use example_core_entity::{
    BalanceV2, BalanceV2Command, BalanceV2Error, SpotOrderV7, SpotOrderV7Command,
};
use rust_decimal::Decimal;

/// 取消现货订单步骤产生的订单与可选余额变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderPipeChanges {
    pub order: UpdatedEntityEvents<SpotOrderV7>,
    pub balance: Option<UpdatedEntityEvents<BalanceV2>>,
}

impl ReplayableEntityChanges for CancelSpotOrderPipeChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let mut events = self.order.replayable_events();
        if let Some(balance) = &self.balance {
            events.extend(balance.replayable_events());
        }
        events
    }
}

/// 取消现货订单，并按同一日志序号生成订单与可选余额事件。
pub fn cancel_spot_order(
    order: &SpotOrderV7,
    balance: &BalanceV2,
    lsn: u64,
    logical_ts: u64,
    topic: u8,
    partition: u8,
) -> Result<CancelSpotOrderPipeChanges, EntityError> {
    let order_loaded_position = (order.lsn(), order.command_index());
    let order_metadata =
        FsmCommandMetadata::new_at(lsn, 0, logical_ts, topic, partition, order.party_id());
    let (order_after, order_event) = FsmV2::transition(
        order,
        SequencedCommand::new(order_metadata, SpotOrderV7Command::Cancel),
    )?;
    let release_requirement = order_after
        .terminal_unfreeze_delta(order)
        .map_err(|error| EntityError::Custom(error.to_string()))?;
    let order = UpdatedEntityEvents::try_new(
        order_after,
        order_loaded_position.0,
        order_loaded_position.1,
        vec![order_event],
    )
    .map_err(|error| EntityError::Custom(format!("invalid order event chain: {error:?}")))?;

    let release_amount = release_requirement.amount();
    let balance = if release_amount == Decimal::ZERO {
        None
    } else {
        if balance.coin_idx() != u64::from(release_requirement.currency().index()) {
            return Err(EntityError::Custom(
                "balance coin does not match the asset released by the order".to_owned(),
            ));
        }
        let loaded_position = (balance.lsn(), balance.command_index());
        let balance_metadata = FsmCommandMetadata::new_at(
            lsn,
            1,
            logical_ts,
            topic,
            partition,
            balance.party_id().to_string(),
        );
        let (after, event) = FsmV2::transition(
            balance,
            SequencedCommand::new(
                balance_metadata,
                BalanceV2Command::UnfreezeAndPay {
                    unfreeze_amount: release_amount,
                    payment_amount: Decimal::ZERO,
                },
            ),
        )
        .map_err(map_balance_error)?;
        Some(
            UpdatedEntityEvents::try_new(after, loaded_position.0, loaded_position.1, vec![event])
                .map_err(|error| {
                    EntityError::Custom(format!("invalid balance event chain: {error:?}"))
                })?,
        )
    };

    Ok(CancelSpotOrderPipeChanges { order, balance })
}

fn map_balance_error(error: BalanceV2Error) -> EntityError {
    match error {
        BalanceV2Error::Replay(error) => error,
        error => EntityError::Custom(format!("failed to release frozen balance: {error}")),
    }
}
