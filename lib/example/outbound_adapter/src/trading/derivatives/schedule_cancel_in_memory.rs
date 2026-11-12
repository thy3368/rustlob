use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

use common_entity::{Entity, EntityReplayableEvent, Fsm, LogEntry, StateSink, StateSource};
use example_core_use_case::{
    BalanceV2, BalanceV2Event, DueScheduledCancelSource, PerpOrderV3, PerpOrderV3Event,
    ScheduleHyperliquidCancelCmd, ScheduleHyperliquidCancelState, ScheduleHyperliquidCancelUseCase,
    ScheduledCancel, SpotOrderStatus, SpotOrderV7, SpotOrderV7Event, TriggerScheduledCancelCmd,
    TriggerScheduledCancelState, TriggerScheduledCancelUseCase, UserId,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::shared::event_string_field;

#[derive(Debug, Clone)]
struct State {
    schedules: HashMap<u64, ScheduledCancel>,
    spot_orders: HashMap<u64, SpotOrderV7>,
    perp_orders: HashMap<u64, PerpOrderV3>,
    balances: HashMap<u64, BalanceV2>,
    persisted_events: Vec<EntityReplayableEvent>,
    published_events: Vec<EntityReplayableEvent>,
}

impl State {
    fn new() -> Self {
        Self {
            schedules: HashMap::new(),
            spot_orders: HashMap::new(),
            perp_orders: HashMap::new(),
            balances: HashMap::new(),
            persisted_events: Vec::new(),
            published_events: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HyperliquidScheduleCancelInMemoryOutbound {
    state: Arc<Mutex<State>>,
}

impl Default for HyperliquidScheduleCancelInMemoryOutbound {
    fn default() -> Self {
        Self { state: Arc::new(Mutex::new(State::new())) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidScheduleCancelSnapshot {
    pub schedules: Vec<ScheduledCancel>,
    pub spot_orders: Vec<SpotOrderV7>,
    pub perp_orders: Vec<PerpOrderV3>,
    pub balances: Vec<BalanceV2>,
    pub persisted_event_count: usize,
    pub published_event_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HyperliquidScheduleCancelOutboundError {
    #[error("schedule cancel store unavailable")]
    StoreUnavailable,
    #[error("scheduled cancel state not found")]
    ScheduleNotFound,
    #[error("stale replay event for entity {entity_id}: expected version {expected}, got {actual}")]
    StaleEvent { entity_id: u64, expected: u64, actual: u64 },
    #[error("failed to decode schedule cancel replay event")]
    EventDecodeFailed,
    #[error("failed to replay schedule cancel event: {0}")]
    ReplayFailed(String),
}

impl HyperliquidScheduleCancelInMemoryOutbound {
    pub fn seed_spot_order(
        &self,
        order: SpotOrderV7,
    ) -> Result<(), HyperliquidScheduleCancelOutboundError> {
        self.lock()?.spot_orders.insert(order.order_base().order_id, order);
        Ok(())
    }

    pub fn seed_perp_order(
        &self,
        order: PerpOrderV3,
    ) -> Result<(), HyperliquidScheduleCancelOutboundError> {
        self.lock()?.perp_orders.insert(order.comon_order.order_id, order);
        Ok(())
    }

    pub fn seed_balance(
        &self,
        balance: BalanceV2,
    ) -> Result<(), HyperliquidScheduleCancelOutboundError> {
        self.lock()?.balances.insert(balance.entity_id(), balance);
        Ok(())
    }

    pub fn snapshot(
        &self,
    ) -> Result<HyperliquidScheduleCancelSnapshot, HyperliquidScheduleCancelOutboundError> {
        let state = self.lock()?;
        Ok(HyperliquidScheduleCancelSnapshot {
            schedules: state.schedules.values().cloned().collect(),
            spot_orders: state.spot_orders.values().cloned().collect(),
            perp_orders: state.perp_orders.values().cloned().collect(),
            balances: state.balances.values().cloned().collect(),
            persisted_event_count: state.persisted_events.len(),
            published_event_count: state.published_events.len(),
        })
    }

    fn lock(&self) -> Result<MutexGuard<'_, State>, HyperliquidScheduleCancelOutboundError> {
        self.state.lock().map_err(|_| HyperliquidScheduleCancelOutboundError::StoreUnavailable)
    }
}

impl DueScheduledCancelSource for HyperliquidScheduleCancelInMemoryOutbound {
    type Error = HyperliquidScheduleCancelOutboundError;

    fn due_user_ids(&self, now_ms: u64, limit: usize) -> Result<Vec<UserId>, Self::Error> {
        let state = self.lock()?;
        let mut due = state
            .schedules
            .values()
            .filter_map(|schedule| {
                schedule
                    .cancel_at_ms()
                    .filter(|deadline| *deadline <= now_ms)
                    .map(|deadline| (deadline, UserId(schedule.user_id())))
            })
            .collect::<Vec<_>>();
        due.sort_unstable_by_key(|(deadline, user_id)| (*deadline, user_id.0));
        due.truncate(limit);
        Ok(due.into_iter().map(|(_, user_id)| user_id).collect())
    }
}

impl StateSource<ScheduleHyperliquidCancelUseCase> for HyperliquidScheduleCancelInMemoryOutbound {
    type Error = HyperliquidScheduleCancelOutboundError;

    fn load_given_state(
        &self,
        cmd: &LogEntry<ScheduleHyperliquidCancelCmd>,
    ) -> Result<ScheduleHyperliquidCancelState, Self::Error> {
        let state = self.lock()?;
        Ok(ScheduleHyperliquidCancelState {
            scheduled_cancel: state
                .schedules
                .get(&cmd.user_id.0)
                .cloned()
                .unwrap_or_else(|| ScheduledCancel::new(cmd.user_id.0)),
        })
    }
}

impl StateSource<TriggerScheduledCancelUseCase> for HyperliquidScheduleCancelInMemoryOutbound {
    type Error = HyperliquidScheduleCancelOutboundError;

    fn load_given_state(
        &self,
        cmd: &LogEntry<TriggerScheduledCancelCmd>,
    ) -> Result<TriggerScheduledCancelState, Self::Error> {
        let state = self.lock()?;
        let scheduled_cancel = state
            .schedules
            .get(&cmd.user_id.0)
            .cloned()
            .ok_or(HyperliquidScheduleCancelOutboundError::ScheduleNotFound)?;
        Ok(TriggerScheduledCancelState {
            scheduled_cancel,
            spot_orders: state
                .spot_orders
                .values()
                .filter(|order| {
                    order.order_base().owner == cmd.user_id && !is_terminal(order.status())
                })
                .cloned()
                .collect(),
            spot_balances: state
                .balances
                .values()
                .filter(|balance| balance.party_id() == cmd.user_id.0)
                .cloned()
                .collect(),
            perp_orders: state
                .perp_orders
                .values()
                .filter(|order| {
                    order.comon_order.owner == cmd.user_id && !is_terminal(order.comon_order.status)
                })
                .cloned()
                .collect(),
        })
    }
}

fn is_terminal(status: SpotOrderStatus) -> bool {
    matches!(
        status,
        SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
    )
}

macro_rules! impl_sink {
    ($use_case:ty) => {
        impl StateSink<$use_case> for HyperliquidScheduleCancelInMemoryOutbound {
            type Error = HyperliquidScheduleCancelOutboundError;

            fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
                self.lock()?.persisted_events.extend(events.iter().cloned());
                Ok(())
            }

            fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
                let mut state = self.lock()?;
                let mut next = state.clone();
                replay_events(&mut next, events)?;
                *state = next;
                Ok(())
            }

            fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
                self.lock()?.published_events.extend(events.iter().cloned());
                Ok(())
            }
        }
    };
}

impl_sink!(ScheduleHyperliquidCancelUseCase);
impl_sink!(TriggerScheduledCancelUseCase);

fn replay_events(
    state: &mut State,
    events: &[EntityReplayableEvent],
) -> Result<(), HyperliquidScheduleCancelOutboundError> {
    for event in events {
        if event.entity_type == ScheduledCancel::entity_type() {
            replay_schedule(state, event)?;
        } else if event.entity_type == SpotOrderV7::entity_type() {
            let order = state
                .spot_orders
                .get_mut(&event.entity_id)
                .ok_or(HyperliquidScheduleCancelOutboundError::EventDecodeFailed)?;
            check_version(event, order.order_base().lsn)?;
            order
                .transition(SpotOrderV7Event::Cancel {
                    lsn: event.lsn,
                    timestamp: event.logical_ts,
                })
                .map_err(|error| {
                    HyperliquidScheduleCancelOutboundError::ReplayFailed(error.to_string())
                })?;
        } else if event.entity_type == PerpOrderV3::entity_type() {
            let order = state
                .perp_orders
                .get_mut(&event.entity_id)
                .ok_or(HyperliquidScheduleCancelOutboundError::EventDecodeFailed)?;
            check_version(event, order.comon_order.lsn)?;
            order.transition(PerpOrderV3Event::Cancel { timestamp: event.logical_ts }).map_err(
                |error| HyperliquidScheduleCancelOutboundError::ReplayFailed(error.to_string()),
            )?;
        } else if event.entity_type == BalanceV2::entity_type() {
            replay_balance(state, event)?;
        } else {
            return Err(HyperliquidScheduleCancelOutboundError::EventDecodeFailed);
        }
    }
    Ok(())
}

fn replay_schedule(
    state: &mut State,
    event: &EntityReplayableEvent,
) -> Result<(), HyperliquidScheduleCancelOutboundError> {
    let schedule = state
        .schedules
        .entry(event.entity_id)
        .or_insert_with(|| ScheduledCancel::new(event.entity_id));
    check_version(event, schedule.version())?;
    if event_string_field(event, "trigger_count").is_some() {
        schedule.trigger(event.logical_ts).map_err(|error| {
            HyperliquidScheduleCancelOutboundError::ReplayFailed(error.to_string())
        })?;
    } else {
        let deadline = event_string_field(event, "cancel_at_ms")
            .ok_or(HyperliquidScheduleCancelOutboundError::EventDecodeFailed)?;
        let deadline = if deadline.is_empty() {
            None
        } else {
            Some(
                deadline
                    .parse()
                    .map_err(|_| HyperliquidScheduleCancelOutboundError::EventDecodeFailed)?,
            )
        };
        schedule.replace(deadline, event.logical_ts).map_err(|error| {
            HyperliquidScheduleCancelOutboundError::ReplayFailed(error.to_string())
        })?;
    }
    Ok(())
}

fn replay_balance(
    state: &mut State,
    event: &EntityReplayableEvent,
) -> Result<(), HyperliquidScheduleCancelOutboundError> {
    let balance = state
        .balances
        .get_mut(&event.entity_id)
        .ok_or(HyperliquidScheduleCancelOutboundError::EventDecodeFailed)?;
    check_version(event, balance.version())?;
    let available = decimal_field(event, "available")?;
    let frozen = decimal_field(event, "frozen")?;
    let unfreeze = available
        .checked_sub(balance.available())
        .ok_or(HyperliquidScheduleCancelOutboundError::EventDecodeFailed)?;
    balance
        .transition(BalanceV2Event::UnfreezeAndPay {
            unfreeze_amount: unfreeze,
            payment_amount: Decimal::ZERO,
            timestamp: event.logical_ts,
        })
        .map_err(|error| HyperliquidScheduleCancelOutboundError::ReplayFailed(error.to_string()))?;
    if balance.available() != available || balance.frozen() != frozen {
        return Err(HyperliquidScheduleCancelOutboundError::EventDecodeFailed);
    }
    Ok(())
}

fn decimal_field(
    event: &EntityReplayableEvent,
    name: &'static str,
) -> Result<Decimal, HyperliquidScheduleCancelOutboundError> {
    let value = event_string_field(event, name)
        .ok_or(HyperliquidScheduleCancelOutboundError::EventDecodeFailed)?;
    Decimal::from_str(&value).map_err(|_| HyperliquidScheduleCancelOutboundError::EventDecodeFailed)
}

fn check_version(
    event: &EntityReplayableEvent,
    actual: u64,
) -> Result<(), HyperliquidScheduleCancelOutboundError> {
    if actual != event.previous_lsn {
        return Err(HyperliquidScheduleCancelOutboundError::StaleEvent {
            entity_id: event.entity_id,
            expected: event.previous_lsn,
            actual,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntrySequencer, UseCaseExecutor};
    use example_core_use_case::account::coin::Coin;
    use example_core_use_case::account::symbol::Symbol;
    use example_core_use_case::common::common_order::OrderBase;
    use example_core_use_case::{OrderSide, ScheduleHyperliquidCancelCmd};

    use super::*;

    const USER: UserId = UserId(7);
    const NOW_MS: u64 = 1_700_000_000_000;
    const DUE_MS: u64 = NOW_MS + 5_000;

    fn order_base(order_id: u64, side: OrderSide) -> OrderBase {
        OrderBase {
            owner: USER,
            order_id,
            symbol: Symbol::new(3, Coin::BTC, Coin::USDC),
            side,
            limit_px: Decimal::from(100),
            size: Decimal::from(2),
            orig_size: Decimal::from(2),
            status: SpotOrderStatus::Open,
            created_at: NOW_MS,
            updated_at: NOW_MS,
            ..OrderBase::default()
        }
    }

    #[test]
    fn setting_and_triggering_share_one_authoritative_store() {
        let outbound = HyperliquidScheduleCancelInMemoryOutbound::default();
        let spot = SpotOrderV7::from_record(order_base(11, OrderSide::Buy))
            .expect("spot fixture should be valid");
        let release = {
            let mut canceled = spot.clone();
            canceled
                .transition(SpotOrderV7Event::Cancel { lsn: 9, timestamp: DUE_MS })
                .expect("fixture should cancel");
            canceled.release_requirement().expect("fixture should release")
        };
        outbound.seed_spot_order(spot).expect("spot seed should succeed");
        outbound
            .seed_balance(BalanceV2::new(
                USER.0,
                70,
                u64::from(release.currency().index()),
                Decimal::ZERO,
                release.amount(),
                0,
                NOW_MS,
            ))
            .expect("balance seed should succeed");
        outbound
            .seed_perp_order(PerpOrderV3 {
                comon_order: order_base(12, OrderSide::Sell),
                reduce_only: false,
                is_liquidation: false,
            })
            .expect("perp seed should succeed");

        let schedule = LogEntrySequencer::from_metadata(
            1,
            NOW_MS,
            1,
            1,
            ScheduleHyperliquidCancelCmd {
                user_id: USER,
                cancel_at_ms: Some(DUE_MS),
                now_ms: NOW_MS,
            },
        );
        UseCaseExecutor
            .execute::<ScheduleHyperliquidCancelUseCase, _, _>(
                &ScheduleHyperliquidCancelUseCase,
                &schedule,
                &outbound,
                &outbound,
            )
            .expect("schedule should persist and replay");
        assert_eq!(outbound.due_user_ids(DUE_MS - 1, 10).expect("scan should work"), vec![]);
        assert_eq!(outbound.due_user_ids(DUE_MS, 10).expect("scan should work"), vec![USER]);

        let trigger = LogEntrySequencer::from_metadata(
            2,
            DUE_MS,
            0,
            USER.0,
            TriggerScheduledCancelCmd { user_id: USER, now_ms: DUE_MS },
        );
        UseCaseExecutor
            .execute::<TriggerScheduledCancelUseCase, _, _>(
                &TriggerScheduledCancelUseCase,
                &trigger,
                &outbound,
                &outbound,
            )
            .expect("due schedule should atomically cancel all orders");

        let snapshot = outbound.snapshot().expect("snapshot should work");
        assert_eq!(snapshot.schedules[0].cancel_at_ms(), None);
        assert_eq!(snapshot.schedules[0].trigger_count(), 1);
        assert_eq!(snapshot.spot_orders[0].status(), SpotOrderStatus::Canceled);
        assert_eq!(snapshot.perp_orders[0].comon_order.status, SpotOrderStatus::Canceled);
        assert_eq!(snapshot.balances[0].frozen(), Decimal::ZERO);
        assert_eq!(snapshot.persisted_event_count, 5);
        assert_eq!(snapshot.published_event_count, 5);
    }
}
