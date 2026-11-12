use std::collections::HashSet;

use common_entity::{
    EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::spot::{SpotOrderV7, SpotOrderV7Event};
use example_core_entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, PerpOrderV3, PerpOrderV3Error, PerpOrderV3Event,
    ScheduledCancel, ScheduledCancelError, SpotAmountConversionError, UserId,
};
use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerScheduledCancelCmd {
    pub user_id: UserId,
    pub now_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerScheduledCancelState {
    pub scheduled_cancel: ScheduledCancel,
    pub spot_orders: Vec<SpotOrderV7>,
    pub spot_balances: Vec<BalanceV2>,
    pub perp_orders: Vec<PerpOrderV3>,
}

/// 供 on-time adapter 查询已到期计划的 outbound port。
pub trait DueScheduledCancelSource {
    type Error;

    fn due_user_ids(&self, now_ms: u64, limit: usize) -> Result<Vec<UserId>, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerScheduledCancelChanges {
    pub changed_schedule: UpdatedEntityPair<ScheduledCancel>,
    pub canceled_spot_orders: Vec<UpdatedEntityPair<SpotOrderV7>>,
    pub changed_spot_balances: Vec<UpdatedEntityPair<BalanceV2>>,
    pub canceled_perp_orders: Vec<UpdatedEntityPair<PerpOrderV3>>,
    pub trigger_count_today: u8,
}

impl ReplayableChanges for TriggerScheduledCancelChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(
            1 + self.canceled_spot_orders.len()
                + self.changed_spot_balances.len()
                + self.canceled_perp_orders.len(),
        );
        events.push(self.changed_schedule.to_replayable_event()?);
        for pair in &self.canceled_spot_orders {
            events.push(pair.to_replayable_event()?);
        }
        for pair in &self.changed_spot_balances {
            events.push(pair.to_replayable_event()?);
        }
        for pair in &self.canceled_perp_orders {
            events.push(pair.to_replayable_event()?);
        }
        Ok(events)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TriggerScheduledCancelError {
    #[error("scheduled cancel state belongs to a different user")]
    UserMismatch,
    #[error("loaded order belongs to a different user")]
    OrderOwnerMismatch,
    #[error("loaded order ids must be unique")]
    DuplicateOrderId,
    #[error("command LSN must be newer than every loaded spot order")]
    StaleSpotOrder,
    #[error("spot release balance is missing")]
    MissingSpotBalance,
    #[error("multiple spot balances match one release asset")]
    AmbiguousSpotBalance,
    #[error(transparent)]
    Schedule(#[from] ScheduledCancelError),
    #[error(transparent)]
    SpotOrder(#[from] EntityError),
    #[error(transparent)]
    SpotAmount(#[from] SpotAmountConversionError),
    #[error(transparent)]
    BalanceV2(#[from] BalanceV2Error),
    #[error(transparent)]
    PerpOrder(#[from] PerpOrderV3Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TriggerScheduledCancelUseCase;

impl UseCase for TriggerScheduledCancelUseCase {
    type Command = LogEntry<TriggerScheduledCancelCmd>;
    type StateGiven = TriggerScheduledCancelState;
    type Error = TriggerScheduledCancelError;
    type StateChanged = TriggerScheduledCancelChanges;

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.scheduled_cancel.user_id() != cmd.user_id.0 {
            return Err(TriggerScheduledCancelError::UserMismatch);
        }
        let mut ids = HashSet::with_capacity(state.spot_orders.len() + state.perp_orders.len());
        for order in &state.spot_orders {
            if order.order_base().owner != cmd.user_id {
                return Err(TriggerScheduledCancelError::OrderOwnerMismatch);
            }
            if !ids.insert(order.order_base().order_id) {
                return Err(TriggerScheduledCancelError::DuplicateOrderId);
            }
            if cmd.lsn() <= order.order_base().lsn {
                return Err(TriggerScheduledCancelError::StaleSpotOrder);
            }
        }
        for order in &state.perp_orders {
            if order.comon_order.owner != cmd.user_id {
                return Err(TriggerScheduledCancelError::OrderOwnerMismatch);
            }
            if !ids.insert(order.comon_order.order_id) {
                return Err(TriggerScheduledCancelError::DuplicateOrderId);
            }
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let schedule_before = state.scheduled_cancel.clone();
        let mut schedule_after = schedule_before.clone();
        let trigger_count_today = schedule_after.trigger(cmd.now_ms)?;

        let mut balances_after = state.spot_balances.clone();
        let mut canceled_spot_orders = Vec::with_capacity(state.spot_orders.len());
        for before in &state.spot_orders {
            let mut after = before.clone();
            after.transition(SpotOrderV7Event::Cancel {
                lsn: cmd.lsn(),
                timestamp: cmd.logical_ts(),
            })?;
            let release = after.release_requirement()?;
            let coin_idx = u64::from(release.currency().index());
            let mut matching = balances_after.iter_mut().filter(|balance| {
                balance.party_id() == cmd.user_id.0 && balance.coin_idx() == coin_idx
            });
            let balance = matching.next().ok_or(TriggerScheduledCancelError::MissingSpotBalance)?;
            if matching.next().is_some() {
                return Err(TriggerScheduledCancelError::AmbiguousSpotBalance);
            }
            balance.transition(BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: release.amount(),
                payment_amount: Decimal::ZERO,
                timestamp: cmd.logical_ts(),
            })?;
            canceled_spot_orders.push(UpdatedEntityPair::new(before.clone(), after));
        }

        let changed_spot_balances = state
            .spot_balances
            .iter()
            .cloned()
            .zip(balances_after)
            .filter(|(before, after)| before != after)
            .map(|(before, after)| UpdatedEntityPair::new(before, after))
            .collect();

        let mut canceled_perp_orders = Vec::with_capacity(state.perp_orders.len());
        for before in &state.perp_orders {
            let mut after = before.clone();
            after.transition(PerpOrderV3Event::Cancel { timestamp: cmd.logical_ts() })?;
            canceled_perp_orders.push(UpdatedEntityPair::new(before.clone(), after));
        }

        Ok(TriggerScheduledCancelChanges {
            changed_schedule: UpdatedEntityPair::new(schedule_before, schedule_after),
            canceled_spot_orders,
            changed_spot_balances,
            canceled_perp_orders,
            trigger_count_today,
        })
    }
}

impl StateDiff for TriggerScheduledCancelUseCase {
    type StateDiff = TriggerScheduledCancelChanges;

    fn do_compute_state_diff(
        _state_given: Self::StateGiven,
        state_changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(state_changed)
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntrySequencer, ReplayableChanges};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::common_order::OrderBase;
    use example_core_entity::{OrderSide, SpotOrderStatus};

    use super::*;

    const USER: UserId = UserId(7);
    const NOW_MS: u64 = 1_700_000_010_000;
    const LOGICAL_TS: u64 = 1_700_000_010_000;

    fn command() -> LogEntry<TriggerScheduledCancelCmd> {
        LogEntrySequencer::from_metadata(
            9,
            LOGICAL_TS,
            7,
            1,
            TriggerScheduledCancelCmd { user_id: USER, now_ms: NOW_MS },
        )
    }

    fn armed_schedule() -> ScheduledCancel {
        let mut schedule = ScheduledCancel::new(USER.0);
        schedule.replace(Some(NOW_MS), NOW_MS - 5_000).expect("fixture should arm");
        schedule
    }

    fn order_base(id: u64, side: OrderSide) -> OrderBase {
        OrderBase {
            owner: USER,
            order_id: id,
            symbol: Symbol::new(3, Coin::BTC, Coin::USDC),
            side,
            limit_px: Decimal::from(100),
            size: Decimal::from(2),
            orig_size: Decimal::from(2),
            status: SpotOrderStatus::Open,
            created_at: LOGICAL_TS - 10,
            updated_at: LOGICAL_TS - 10,
            ..OrderBase::default()
        }
    }

    fn state_with_spot_and_perp() -> TriggerScheduledCancelState {
        let spot = SpotOrderV7::from_record(order_base(11, OrderSide::Buy))
            .expect("spot fixture should be valid");
        let release = {
            let mut canceled = spot.clone();
            canceled
                .transition(SpotOrderV7Event::Cancel { lsn: 9, timestamp: LOGICAL_TS })
                .expect("fixture should cancel");
            canceled.release_requirement().expect("fixture should release")
        };
        let balance = BalanceV2::new(
            USER.0,
            70,
            u64::from(release.currency().index()),
            Decimal::ZERO,
            release.amount(),
            0,
            LOGICAL_TS - 10,
        );
        let perp = PerpOrderV3 {
            comon_order: order_base(12, OrderSide::Sell),
            reduce_only: false,
            is_liquidation: false,
        };
        TriggerScheduledCancelState {
            scheduled_cancel: armed_schedule(),
            spot_orders: vec![spot],
            spot_balances: vec![balance],
            perp_orders: vec![perp],
        }
    }

    // competitor: hyperliquid; feature: scheduleCancel trigger; state: GREEN; target: use_case
    #[test]
    fn due_trigger_cancels_all_orders_releases_spot_funds_and_increments_count() {
        let state = state_with_spot_and_perp();
        let changes = TriggerScheduledCancelUseCase
            .compute_state_changed(&command(), &state)
            .expect("到期死人开关应原子撤销全部订单");

        assert_eq!(changes.changed_schedule.after.cancel_at_ms(), None);
        assert_eq!(changes.trigger_count_today, 1);
        assert_eq!(changes.canceled_spot_orders[0].after.status(), SpotOrderStatus::Canceled);
        assert_eq!(
            changes.canceled_perp_orders[0].after.comon_order.status,
            SpotOrderStatus::Canceled
        );
        assert_eq!(changes.changed_spot_balances[0].after.frozen(), Decimal::ZERO);
        assert!(changes.changed_spot_balances[0].after.available() > Decimal::ZERO);
        assert_eq!(changes.to_replayable_events().expect("events should project").len(), 4);
    }

    #[test]
    fn not_due_rejects_without_returning_partial_changes() {
        let mut state = state_with_spot_and_perp();
        state
            .scheduled_cancel
            .replace(Some(NOW_MS + 1), NOW_MS - 5_000)
            .expect("fixture should rearm");

        assert_eq!(
            TriggerScheduledCancelUseCase.compute_state_changed(&command(), &state),
            Err(TriggerScheduledCancelError::Schedule(ScheduledCancelError::NotDue))
        );
    }

    #[test]
    fn missing_spot_balance_rejects_the_whole_trigger() {
        let mut state = state_with_spot_and_perp();
        state.spot_balances.clear();
        assert_eq!(
            TriggerScheduledCancelUseCase.compute_state_changed(&command(), &state),
            Err(TriggerScheduledCancelError::MissingSpotBalance)
        );
    }
}
