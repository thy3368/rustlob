use std::collections::HashMap;

use common_entity::{
    EntityError, EntityReplayableEvent, LogAttr, LogEntry, PresequencedCommand, ReplayableChanges,
    Shard, StateDiff, UseCase,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{BalanceV2, SpotOrderStatus, SpotOrderV7};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::cancel_spot_order::{
    CancelSpotOrderV3Changes, CancelSpotOrderV3Cmd, CancelSpotOrderV3Error, CancelSpotOrderV3State,
    compute_cancel_changes,
};
use super::place_spot_order::{
    PlaceSpotOrderV3Changes, PlaceSpotOrderV3Cmd, PlaceSpotOrderV3Error, PlaceSpotOrderV3OrderCmd,
    PlaceSpotOrderV3State, SpotOrderV3Balances, compute_place_changes,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBlockCmd {
    pub items: Vec<LogEntry<SpotBlockCommand>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotBlockCommand {
    Place(PlaceSpotOrderV3Cmd),
    Cancel(CancelSpotOrderV3Cmd),
}

impl Shard for SpotBlockCmd {
    fn topic(&self) -> u8 {
        self.items.last().map_or(0, LogEntry::topic)
    }

    fn partition(&self) -> u8 {
        self.items.last().map_or(0, LogEntry::partition)
    }
}

impl LogAttr for SpotBlockCmd {
    fn lsn(&self) -> u64 {
        self.items.last().map_or(0, LogEntry::lsn)
    }

    fn logical_ts(&self) -> u64 {
        self.items.last().map_or(0, LogEntry::logical_ts)
    }
}

impl PresequencedCommand for SpotBlockCmd {
    fn contains_event_metadata(&self, event: &EntityReplayableEvent) -> bool {
        self.items.iter().any(|item| {
            item.topic() == event.t
                && item.partition() == event.p
                && item.lsn() == event.lsn
                && item.logical_ts() == event.logical_ts
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotBlockState {
    pub orders: Vec<SpotOrderV7>,
    pub balances: Vec<BalanceV2>,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotBlockChanges {
    pub item_results: Vec<SpotBlockItemResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotBlockItemResult {
    Applied {
        item: LogEntry<SpotBlockCommand>,
        changes: SpotBlockAppliedChanges,
    },
    Rejected {
        item: LogEntry<SpotBlockCommand>,
        error: SpotBlockItemError,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotBlockAppliedChanges {
    Place(PlaceSpotOrderV3Changes),
    Cancel(CancelSpotOrderV3Changes),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotBlockItemError {
    #[error(transparent)]
    Place(#[from] PlaceSpotOrderV3Error),
    #[error(transparent)]
    Cancel(#[from] CancelSpotOrderV3Error),
    #[error("order was not found")]
    OrderNotFound,
    #[error("balance was not found or is ambiguous")]
    BalanceNotFound,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotBlockError {
    #[error("spot block must contain at least one item")]
    EmptyItems,
    #[error("item LSNs must be strictly increasing")]
    NonIncreasingLsn,
    #[error("item logical timestamps must be strictly increasing")]
    NonIncreasingLogicalTimestamp,
    #[error("all items must use the same topic and partition")]
    MixedShard,
    #[error("place party id must match the item user identity")]
    PlaceIdentityMismatch,
    #[error("spot block state contains duplicate order identity")]
    DuplicateOrderIdentity,
    #[error("spot block state contains duplicate client order id")]
    DuplicateClientOrderId,
    #[error("spot block state contains duplicate balance identity")]
    DuplicateBalanceIdentity,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SpotBlockUseCase;

impl ReplayableChanges for SpotBlockChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::new();
        for result in &self.item_results {
            let SpotBlockItemResult::Applied { item, changes } = result else {
                continue;
            };
            let mut item_events = match changes {
                SpotBlockAppliedChanges::Place(changes) => changes.to_replayable_events()?,
                SpotBlockAppliedChanges::Cancel(changes) => changes.to_replayable_events()?,
            };
            for event in &mut item_events {
                event.t = item.topic();
                event.p = item.partition();
                event.lsn = item.lsn();
                event.logical_ts = item.logical_ts();
            }
            events.extend(item_events);
        }
        Ok(events)
    }
}

impl UseCase for SpotBlockUseCase {
    type Command = SpotBlockCmd;
    type StateGiven = SpotBlockState;
    type Error = SpotBlockError;
    type StateChanged = SpotBlockChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        let Some(first) = cmd.items.first() else {
            return Err(SpotBlockError::EmptyItems);
        };
        for pair in cmd.items.windows(2) {
            if pair[0].lsn() >= pair[1].lsn() {
                return Err(SpotBlockError::NonIncreasingLsn);
            }
            if pair[0].logical_ts() >= pair[1].logical_ts() {
                return Err(SpotBlockError::NonIncreasingLogicalTimestamp);
            }
        }
        for item in &cmd.items {
            if item.topic() != first.topic() || item.partition() != first.partition() {
                return Err(SpotBlockError::MixedShard);
            }
            if let SpotBlockCommand::Place(command) = item.payload() {
                let expected = common_entity::stable_string_entity_id(item.user_id());
                if place_orders(command).any(|order| order.party_id != expected) {
                    return Err(SpotBlockError::PlaceIdentityMismatch);
                }
            }
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        _cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        WorkingState::try_from(state.clone()).map(|_| ())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut working = WorkingState::try_from(state.clone())?;
        let mut item_results = Vec::with_capacity(cmd.items.len());
        for item in cmd.items.iter().cloned() {
            let result = working.apply(item.clone());
            item_results.push(match result {
                Ok(changes) => SpotBlockItemResult::Applied { item, changes },
                Err(error) => SpotBlockItemResult::Rejected { item, error },
            });
        }
        Ok(SpotBlockChanges { item_results })
    }
}

impl StateDiff for SpotBlockUseCase {
    type StateDiff = SpotBlockChanges;

    fn do_compute_state_diff(
        _state: Self::StateGiven,
        changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(changed)
    }
}

#[derive(Debug, Clone)]
struct WorkingState {
    orders: Vec<SpotOrderV7>,
    balances: Vec<BalanceV2>,
    order_by_oid: HashMap<u64, usize>,
    order_by_cloid: HashMap<String, usize>,
    balance_by_identity: HashMap<(u64, u64), usize>,
    next_trade_id: u64,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
}

impl TryFrom<SpotBlockState> for WorkingState {
    type Error = SpotBlockError;

    fn try_from(state: SpotBlockState) -> Result<Self, Self::Error> {
        let mut value = Self {
            orders: state.orders,
            balances: state.balances,
            order_by_oid: HashMap::new(),
            order_by_cloid: HashMap::new(),
            balance_by_identity: HashMap::new(),
            next_trade_id: state.trade_id_start,
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
        };
        value.rebuild_indexes()?;
        Ok(value)
    }
}

impl WorkingState {
    fn rebuild_indexes(&mut self) -> Result<(), SpotBlockError> {
        self.order_by_oid.clear();
        self.order_by_cloid.clear();
        self.balance_by_identity.clear();
        for (index, order) in self.orders.iter().enumerate() {
            let base = order.order_base();
            if self.order_by_oid.insert(base.order_id, index).is_some() {
                return Err(SpotBlockError::DuplicateOrderIdentity);
            }
            if let Some(cloid) = &base.cloid {
                if self.order_by_cloid.insert(cloid.clone(), index).is_some() {
                    return Err(SpotBlockError::DuplicateClientOrderId);
                }
            }
        }
        for (index, balance) in self.balances.iter().enumerate() {
            let key = (balance.account_id(), balance.coin_idx());
            if self.balance_by_identity.insert(key, index).is_some() {
                return Err(SpotBlockError::DuplicateBalanceIdentity);
            }
        }
        Ok(())
    }

    fn apply(
        &mut self,
        item: LogEntry<SpotBlockCommand>,
    ) -> Result<SpotBlockAppliedChanges, SpotBlockItemError> {
        match item.payload().clone() {
            SpotBlockCommand::Place(command) => self.apply_place(item, command),
            SpotBlockCommand::Cancel(command) => self.apply_cancel(item, command),
        }
    }

    fn apply_place(
        &mut self,
        item: LogEntry<SpotBlockCommand>,
        command: PlaceSpotOrderV3Cmd,
    ) -> Result<SpotBlockAppliedChanges, SpotBlockItemError> {
        let entry = common_entity::LogEntrySequencer::derive(&item, command);
        let parent = place_orders(entry.payload())
            .next()
            .ok_or(PlaceSpotOrderV3Error::InvalidGroup)?;
        let maker_orders = self
            .orders
            .iter()
            .filter(|order| {
                let base = order.order_base();
                base.symbol == parent.symbol
                    && base.side != parent.side
                    && matches!(
                        base.status,
                        SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
                    )
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut balances_by_order_id = HashMap::with_capacity(maker_orders.len() + 1);
        balances_by_order_id.insert(
            parent.order_id,
            self.balances_for(parent.party_id, parent.symbol)?,
        );
        for maker in &maker_orders {
            let base = maker.order_base();
            balances_by_order_id
                .insert(base.order_id, self.balances_for(base.owner.0, base.symbol)?);
        }
        let state = PlaceSpotOrderV3State {
            maker_orders,
            balances_by_order_id,
            trade_id_start: self.next_trade_id,
            maker_fee_bps: self.maker_fee_bps,
            taker_fee_bps: self.taker_fee_bps,
        };
        let changes = compute_place_changes(&entry, state)?;
        self.next_trade_id = self
            .next_trade_id
            .checked_add(changes.settled_trades.len() as u64)
            .ok_or(PlaceSpotOrderV3Error::TradeIdOverflow)?;
        for pair in &changes.updated_maker_orders {
            self.replace_order(pair.after.clone())?;
        }
        for order in &changes.created_orders {
            self.orders.push(order.clone());
        }
        for pair in &changes.updated_balances {
            self.replace_balance(pair.after.clone())?;
        }
        self.rebuild_indexes()
            .map_err(|_| SpotBlockItemError::OrderNotFound)?;
        Ok(SpotBlockAppliedChanges::Place(changes))
    }

    fn apply_cancel(
        &mut self,
        item: LogEntry<SpotBlockCommand>,
        command: CancelSpotOrderV3Cmd,
    ) -> Result<SpotBlockAppliedChanges, SpotBlockItemError> {
        let entry = common_entity::LogEntrySequencer::derive(&item, command);
        let order_index = match &entry.order_id {
            OrderId::Oid(oid) => self.order_by_oid.get(oid),
            OrderId::Cloid(cloid) => self.order_by_cloid.get(cloid),
        }
        .copied()
        .ok_or(SpotBlockItemError::OrderNotFound)?;
        let order = self.orders[order_index].clone();
        let base = order.order_base();
        let held_coin = if base.side == example_core_entity::OrderSide::Buy {
            base.symbol.quote
        } else {
            base.symbol.base
        };
        let balance = self
            .balance_for(base.owner.0, u64::from(held_coin.index()))?
            .clone();
        let changes = compute_cancel_changes(&entry, CancelSpotOrderV3State { order, balance })?;
        self.replace_order(changes.updated_order.after.clone())?;
        if let Some(pair) = &changes.updated_balance {
            self.replace_balance(pair.after.clone())?;
        }
        self.rebuild_indexes()
            .map_err(|_| SpotBlockItemError::OrderNotFound)?;
        Ok(SpotBlockAppliedChanges::Cancel(changes))
    }

    fn balances_for(
        &self,
        party_id: u64,
        symbol: example_core_entity::account::symbol::Symbol,
    ) -> Result<SpotOrderV3Balances, SpotBlockItemError> {
        Ok(SpotOrderV3Balances {
            base: self
                .balance_for(party_id, u64::from(symbol.base.index()))?
                .clone(),
            quote: self
                .balance_for(party_id, u64::from(symbol.quote.index()))?
                .clone(),
        })
    }

    fn balance_for(&self, party_id: u64, coin: u64) -> Result<&BalanceV2, SpotBlockItemError> {
        let mut matches = self
            .balances
            .iter()
            .filter(|balance| balance.party_id() == party_id && balance.coin_idx() == coin);
        let value = matches.next().ok_or(SpotBlockItemError::BalanceNotFound)?;
        if matches.next().is_some() {
            return Err(SpotBlockItemError::BalanceNotFound);
        }
        Ok(value)
    }

    fn replace_order(&mut self, order: SpotOrderV7) -> Result<(), SpotBlockItemError> {
        let index = self
            .order_by_oid
            .get(&order.order_base().order_id)
            .copied()
            .ok_or(SpotBlockItemError::OrderNotFound)?;
        self.orders[index] = order;
        Ok(())
    }

    fn replace_balance(&mut self, balance: BalanceV2) -> Result<(), SpotBlockItemError> {
        let index = self
            .balance_by_identity
            .get(&(balance.account_id(), balance.coin_idx()))
            .copied()
            .ok_or(SpotBlockItemError::BalanceNotFound)?;
        self.balances[index] = balance;
        Ok(())
    }
}

fn place_orders(command: &PlaceSpotOrderV3Cmd) -> impl Iterator<Item = &PlaceSpotOrderV3OrderCmd> {
    let (parent, children) = match command {
        PlaceSpotOrderV3Cmd::Single(order) => (order, &[][..]),
        PlaceSpotOrderV3Cmd::NormalTpsl { parent, children } => (parent, children.as_slice()),
    };
    std::iter::once(parent).chain(children.iter())
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, LogEntrySequencer};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::{OrderSide, OrderTif, OrderType};
    use rust_decimal::Decimal;

    use super::*;

    const USER: &str = "buyer";

    fn item(lsn: u64, payload: SpotBlockCommand) -> LogEntry<SpotBlockCommand> {
        LogEntrySequencer::from_metadata_with_topic(
            4,
            2,
            lsn,
            lsn + 100,
            9,
            lsn,
            USER.to_owned(),
            payload,
        )
    }

    fn place(order_id: u64) -> SpotBlockCommand {
        place_for(order_id, common_entity::stable_string_entity_id(USER))
    }

    fn place_for(order_id: u64, party_id: u64) -> SpotBlockCommand {
        SpotBlockCommand::Place(PlaceSpotOrderV3Cmd::Single(PlaceSpotOrderV3OrderCmd {
            party_id,
            order_id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side: OrderSide::Buy,
            price: Decimal::from(10),
            size: Decimal::from(2),
            order_type: OrderType::Limit { tif: OrderTif::Gtc },
            cloid: Some("block-order".to_owned()),
        }))
    }

    fn cancel(order_id: OrderId) -> SpotBlockCommand {
        SpotBlockCommand::Cancel(CancelSpotOrderV3Cmd { order_id })
    }

    fn state() -> SpotBlockState {
        let party = common_entity::stable_string_entity_id(USER);
        SpotBlockState {
            orders: Vec::new(),
            balances: vec![
                BalanceV2::new(party, party, u64::from(Coin::BTC.index()), 10, 0, 1, 1),
                BalanceV2::new(party, party, u64::from(Coin::USDT.index()), 100, 0, 1, 1),
            ],
            trade_id_start: 1,
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        }
    }

    #[test]
    fn rejects_invalid_presequencing_and_identity() {
        assert_eq!(
            SpotBlockUseCase.check_command(&SpotBlockCmd { items: vec![] }),
            Err(SpotBlockError::EmptyItems)
        );
        let duplicate = SpotBlockCmd {
            items: vec![item(2, place(7)), item(2, cancel(OrderId::Oid(7)))],
        };
        assert_eq!(
            SpotBlockUseCase.check_command(&duplicate),
            Err(SpotBlockError::NonIncreasingLsn)
        );

        let wrong_identity = item(2, place_for(7, 8));
        assert_eq!(
            SpotBlockUseCase.check_command(&SpotBlockCmd {
                items: vec![wrong_identity]
            }),
            Err(SpotBlockError::PlaceIdentityMismatch)
        );
    }

    #[test]
    fn place_then_cancel_preserves_both_order_events_and_release() {
        let command = SpotBlockCmd {
            items: vec![
                item(10, place(7)),
                item(11, cancel(OrderId::Cloid("block-order".into()))),
            ],
        };
        SpotBlockUseCase
            .check_command(&command)
            .expect("block metadata must be valid");
        let changes = SpotBlockUseCase
            .compute_state_diff(&command, state())
            .expect("place and cancel must both apply");
        assert!(
            changes
                .item_results
                .iter()
                .all(|item| matches!(item, SpotBlockItemResult::Applied { .. }))
        );

        let events = changes
            .to_replayable_events()
            .expect("changes must project");
        let order_type = <SpotOrderV7 as Entity>::entity_type();
        let order_events = events
            .iter()
            .filter(|event| event.entity_type == order_type)
            .collect::<Vec<_>>();
        assert_eq!(order_events.len(), 2);
        assert_eq!(order_events[0].lsn, 10);
        assert_eq!(order_events[1].lsn, 11);
        assert_eq!(order_events[1].previous_lsn, 10);
        assert!(events.iter().any(|event| {
            event.entity_type == <BalanceV2 as Entity>::entity_type() && event.lsn == 11
        }));
        assert!(events.iter().all(|event| event.t == 4 && event.p == 2));
    }

    #[test]
    fn rejected_item_does_not_stop_later_item() {
        let command = SpotBlockCmd {
            items: vec![item(10, cancel(OrderId::Oid(999))), item(11, place(7))],
        };
        let changes = SpotBlockUseCase
            .compute_state_diff(&command, state())
            .expect("item rejection is not a block error");
        assert!(matches!(
            changes.item_results[0],
            SpotBlockItemResult::Rejected { .. }
        ));
        assert!(matches!(
            changes.item_results[1],
            SpotBlockItemResult::Applied { .. }
        ));
    }

    #[test]
    fn log_entry_json_round_trip_preserves_metadata() {
        let original = item(42, place(7));
        let json = serde_json::to_vec(&original).expect("entry must serialize");
        let decoded: LogEntry<SpotBlockCommand> =
            serde_json::from_slice(&json).expect("entry must deserialize");
        assert_eq!(decoded, original);
    }
}
