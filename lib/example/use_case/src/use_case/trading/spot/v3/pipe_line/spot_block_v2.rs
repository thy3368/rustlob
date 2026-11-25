use std::collections::HashMap;

use common_entity::{
    EntityError, LogAttr, LogEntry, LogEntrySequencer, PresequencedCommand, ReplayableDomainEvent,
    ReplayableEntityChanges, Shard, UseCaseComputeError, UseCaseV3,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{
    BalanceV2, OrderSide, PlaceSpotOrderPipelineV2Cmd, SpotOrderStatus, SpotOrderV7,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    CancelSpotOrderFsmV3Cmd, CancelSpotOrderFsmV3Error, CancelSpotOrderFsmV3State,
    CancelSpotOrderFsmV3UseCase, PlaceSpotOrderPipelineV2State, PlaceSpotOrderPipelineV2UseCase,
};
use crate::use_case::trading::spot::v3::pipe::cancel_spot_order::CancelSpotOrderPipeChanges;
use crate::use_case::trading::spot::v3::pipe::match_spot_order::MatchSpotOrderPipeChanges;
use crate::use_case::trading::spot::v3::pipe::place_spot_order_pipeline_v2::{
    PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error,
};
use crate::use_case::trading::spot::v3::pipe::settle_matched_spot_trades::{
    SettleTradesPipeChanges, SpotOrderSettlementBalances,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotBlockCmdV2 {
    pub items: Vec<LogEntry<SpotBlockCommandV2>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotBlockCommandV2 {
    Place(PlaceSpotOrderPipelineV2Cmd),
    Cancel(CancelSpotOrderFsmV3Cmd),
}
impl Shard for SpotBlockCmdV2 {
    fn topic(&self) -> u8 {
        self.items.last().map_or(0, LogEntry::topic)
    }
    fn partition(&self) -> u8 {
        self.items.last().map_or(0, LogEntry::partition)
    }
}
impl LogAttr for SpotBlockCmdV2 {
    fn lsn(&self) -> u64 {
        self.items.last().map_or(0, LogEntry::lsn)
    }
    fn logical_ts(&self) -> u64 {
        self.items.last().map_or(0, LogEntry::logical_ts)
    }
}
impl PresequencedCommand for SpotBlockCmdV2 {
    fn contains_event_metadata(&self, e: &ReplayableDomainEvent) -> bool {
        self.items.iter().any(|i| {
            i.topic() == e.t
                && i.partition() == e.p
                && i.lsn() == e.lsn
                && i.logical_ts() == e.logical_ts
        })
    }
    fn allows_empty_events(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
///State4BuildBlock
pub struct SpotBlockStateV2 {
    pub orders: Vec<SpotOrderV7>,
    pub balances: Vec<BalanceV2>,
    pub next_trade_id: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotBlockCommandResultV2 {
    Place {
        command: LogEntry<PlaceSpotOrderPipelineV2Cmd>,
        result: Result<PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error>,
    },
    Cancel {
        command: LogEntry<CancelSpotOrderFsmV3Cmd>,
        result: Result<CancelSpotOrderPipeChanges, EntityError>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotBlockChangesV2 {
    pub command_results: Vec<SpotBlockCommandResultV2>,
    pub block_result: SpotBlockStateV2,
}
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotBlockErrorV2 {
    #[error("spot block must contain at least one command")]
    EmptyCommands,
    #[error("command LSNs must be strictly increasing")]
    NonIncreasingLsn,
    #[error("command logical timestamps must be strictly increasing")]
    NonIncreasingLogicalTimestamp,
    #[error("all commands must use the same topic and partition")]
    MixedShard,
    #[error("spot block state contains duplicate order identity")]
    DuplicateOrderIdentity,
    #[error("spot block state contains duplicate client order id")]
    DuplicateClientOrderId,
    #[error("spot block state contains duplicate balance identity")]
    DuplicateBalanceIdentity,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct SpotBlockUseCaseV2;
impl ReplayableEntityChanges for SpotBlockChangesV2 {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        self.command_results
            .iter()
            .flat_map(|r| match r {
                SpotBlockCommandResultV2::Place { result: Ok(c), .. } => c.replayable_events(),
                SpotBlockCommandResultV2::Cancel { result: Ok(c), .. } => c.replayable_events(),
                _ => Vec::new(),
            })
            .collect()
    }
}
impl UseCaseV3 for SpotBlockUseCaseV2 {
    type Command = SpotBlockCmdV2;
    type StateGiven = SpotBlockStateV2;
    type Error = SpotBlockErrorV2;
    type StateChanged = SpotBlockChangesV2;
    fn check_command(&self, c: &Self::Command) -> Result<(), Self::Error> {
        let first = c.items.first().ok_or(SpotBlockErrorV2::EmptyCommands)?;
        for p in c.items.windows(2) {
            if p[0].lsn() >= p[1].lsn() {
                return Err(SpotBlockErrorV2::NonIncreasingLsn);
            }
            if p[0].logical_ts() >= p[1].logical_ts() {
                return Err(SpotBlockErrorV2::NonIncreasingLogicalTimestamp);
            }
        }
        if c.items
            .iter()
            .any(|i| i.topic() != first.topic() || i.partition() != first.partition())
        {
            return Err(SpotBlockErrorV2::MixedShard);
        }
        Ok(())
    }
    fn validate_state_given(
        &self,
        _: &Self::Command,
        s: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        Working::try_from(s.clone()).map(|_| ())
    }
    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state_given: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut working = Working::try_from(state_given.clone())?;
        let command_results = cmd.items.iter().cloned().map(|i| working.apply(i)).collect();
        Ok(SpotBlockChangesV2 {
            command_results,
            block_result: working.s,
        })
    }
}

#[derive(Clone)]
struct Working {
    s: SpotBlockStateV2,
    oid: HashMap<u64, usize>,
    cloid: HashMap<String, usize>,
    bal: HashMap<(u64, u64), usize>,
}
impl TryFrom<SpotBlockStateV2> for Working {
    type Error = SpotBlockErrorV2;
    fn try_from(s: SpotBlockStateV2) -> Result<Self, Self::Error> {
        let mut w = Self {
            s,
            oid: HashMap::new(),
            cloid: HashMap::new(),
            bal: HashMap::new(),
        };
        w.index()?;
        Ok(w)
    }
}
impl Working {
    fn index(&mut self) -> Result<(), SpotBlockErrorV2> {
        self.oid.clear();
        self.cloid.clear();
        self.bal.clear();
        for (i, o) in self.s.orders.iter().enumerate() {
            let b = o.order_base();
            if self.oid.insert(b.order_id, i).is_some() {
                return Err(SpotBlockErrorV2::DuplicateOrderIdentity);
            }
            if let Some(c) = &b.cloid {
                if self.cloid.insert(c.clone(), i).is_some() {
                    return Err(SpotBlockErrorV2::DuplicateClientOrderId);
                }
            }
        }
        for (i, b) in self.s.balances.iter().enumerate() {
            if self.bal.insert((b.account_id(), b.coin_idx()), i).is_some() {
                return Err(SpotBlockErrorV2::DuplicateBalanceIdentity);
            }
        }
        Ok(())
    }
    fn apply(&mut self, log: LogEntry<SpotBlockCommandV2>) -> SpotBlockCommandResultV2 {
        match log.payload().clone() {
            SpotBlockCommandV2::Place(p) => {
                let command = LogEntrySequencer::derive(&log, p);
                let mut working = self.clone();
                let result = working.place(&command);
                if result.is_ok() {
                    *self = working
                }
                SpotBlockCommandResultV2::Place { command, result }
            }
            SpotBlockCommandV2::Cancel(p) => {
                let command = LogEntrySequencer::derive(&log, p);
                let mut n = self.clone();
                let result = n.cancel(&command);
                if result.is_ok() {
                    *self = n
                }
                SpotBlockCommandResultV2::Cancel { command, result }
            }
        }
    }
    fn place(
        &mut self,
        c: &LogEntry<PlaceSpotOrderPipelineV2Cmd>,
    ) -> Result<PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error> {
        let p = c.payload();
        let makers = self
            .s
            .orders
            .iter()
            .filter(|o| {
                let b = o.order_base();
                b.symbol == p.symbol
                    && b.side != p.side
                    && matches!(
                        b.status,
                        SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
                    )
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut balances = HashMap::with_capacity(makers.len() + 1);
        balances.insert(p.order_id, self.pair(p.party_id, p.symbol)?);
        for o in &makers {
            let b = o.order_base();
            balances.insert(b.order_id, self.pair(b.owner.0, b.symbol)?);
        }
        let state = PlaceSpotOrderPipelineV2State {
            maker_orders: makers,
            balances_by_order_id: balances,
            trade_id_start: self.s.next_trade_id,
            maker_fee_bps: self.s.maker_fee_bps,
            taker_fee_bps: self.s.taker_fee_bps,
        };
        let changes = PlaceSpotOrderPipelineV2UseCase
            .validate_and_compute(c, &state)
            .map_err(place_err)?;
        self.s.next_trade_id = self
            .s
            .next_trade_id
            .checked_add(matched(&changes).map_or(0, trades))
            .ok_or_else(|| {
                PlaceSpotOrderPipelineV2Error::InvalidInput("trade id range overflows".into())
            })?;
        self.project(&changes).map_err(as_place)?;
        self.index()
            .map_err(|e| PlaceSpotOrderPipelineV2Error::InvalidInput(e.to_string()))?;
        Ok(changes)
    }
    fn cancel(
        &mut self,
        c: &LogEntry<CancelSpotOrderFsmV3Cmd>,
    ) -> Result<CancelSpotOrderPipeChanges, EntityError> {
        let i = match &c.order_id {
            OrderId::Oid(x) => self.oid.get(x),
            OrderId::Cloid(x) => self.cloid.get(x),
        }
        .copied()
        .ok_or_else(|| EntityError::Custom("order was not found".into()))?;
        let order = self.s.orders[i].clone();
        let b = order.order_base();
        let coin = if b.side == OrderSide::Buy {
            b.symbol.quote
        } else {
            b.symbol.base
        };
        let balance = self.balance(b.owner.0, u64::from(coin.index()))?.clone();
        let changes = CancelSpotOrderFsmV3UseCase
            .validate_and_compute(c, &CancelSpotOrderFsmV3State { order, balance })
            .map_err(cancel_err)?;
        self.order(changes.order.after().clone())?;
        if let Some(b) = &changes.balance {
            self.balance_set(b.after().clone())?
        }
        self.index()
            .map_err(|e| EntityError::Custom(e.to_string()))?;
        Ok(changes)
    }
    fn balance(&self, p: u64, c: u64) -> Result<&BalanceV2, EntityError> {
        self.bal
            .get(&(p, c))
            .map(|i| &self.s.balances[*i])
            .ok_or_else(|| EntityError::Custom("balance was not found".into()))
    }
    fn pair(
        &self,
        p: u64,
        s: example_core_entity::account::symbol::Symbol,
    ) -> Result<SpotOrderSettlementBalances, PlaceSpotOrderPipelineV2Error> {
        Ok(SpotOrderSettlementBalances {
            base: self
                .balance(p, u64::from(s.base.index()))
                .map_err(as_place)?
                .clone(),
            quote: self
                .balance(p, u64::from(s.quote.index()))
                .map_err(as_place)?
                .clone(),
        })
    }
    fn order(&mut self, o: SpotOrderV7) -> Result<(), EntityError> {
        let i = self
            .oid
            .get(&o.order_base().order_id)
            .copied()
            .ok_or_else(|| EntityError::Custom("order was not found".into()))?;
        self.s.orders[i] = o;
        Ok(())
    }
    fn balance_set(&mut self, b: BalanceV2) -> Result<(), EntityError> {
        let i = self
            .bal
            .get(&(b.account_id(), b.coin_idx()))
            .copied()
            .ok_or_else(|| EntityError::Custom("balance was not found".into()))?;
        self.s.balances[i] = b;
        Ok(())
    }
    fn project(&mut self, c: &PlaceSpotOrderPipelineV2Changes) -> Result<(), EntityError> {
        let f = frozen(c);
        let taker = matched(c)
            .and_then(taker)
            .unwrap_or_else(|| f.order.after());
        self.s.orders.push(taker.clone());
        self.balance_set(f.balance.after().clone())?;
        if let Some(m) = matched(c) {
            for o in makers(m) {
                self.order(o.after().clone())?
            }
        }
        if let Some(x) = settled(c) {
            for b in &x.updated_balances {
                self.balance_set(b.after().clone())?
            }
        }
        for b in unfrozen(c) {
            self.balance_set(b.after().clone())?
        }
        Ok(())
    }
}
fn place_err(
    e: UseCaseComputeError<PlaceSpotOrderPipelineV2Error>,
) -> PlaceSpotOrderPipelineV2Error {
    match e {
        UseCaseComputeError::Business(e) => e,
        UseCaseComputeError::Contract(e) => {
            PlaceSpotOrderPipelineV2Error::InvalidInput(format!("{e:?}"))
        }
    }
}
fn cancel_err(e: UseCaseComputeError<CancelSpotOrderFsmV3Error>) -> EntityError {
    match e {
        UseCaseComputeError::Business(CancelSpotOrderFsmV3Error::Entity(e)) => e,
        UseCaseComputeError::Business(e) => EntityError::Custom(e.to_string()),
        UseCaseComputeError::Contract(e) => EntityError::Custom(format!("{e:?}")),
    }
}
fn as_place(e: EntityError) -> PlaceSpotOrderPipelineV2Error {
    PlaceSpotOrderPipelineV2Error::InvalidInput(e.to_string())
}
fn frozen(
    c: &PlaceSpotOrderPipelineV2Changes,
) -> &crate::use_case::trading::spot::v3::FreezeSpotOrderPipeChanges {
    match c {
        PlaceSpotOrderPipelineV2Changes::Resting { frozen, .. }
        | PlaceSpotOrderPipelineV2Changes::Rejected { frozen, .. }
        | PlaceSpotOrderPipelineV2Changes::PartiallyFilled { frozen, .. }
        | PlaceSpotOrderPipelineV2Changes::Filled { frozen, .. }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { frozen, .. } => frozen,
    }
}
fn matched(c: &PlaceSpotOrderPipelineV2Changes) -> Option<&MatchSpotOrderPipeChanges> {
    match c {
        PlaceSpotOrderPipelineV2Changes::Resting { .. } => None,
        PlaceSpotOrderPipelineV2Changes::Rejected { matched, .. }
        | PlaceSpotOrderPipelineV2Changes::PartiallyFilled { matched, .. }
        | PlaceSpotOrderPipelineV2Changes::Filled { matched, .. }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { matched, .. } => {
            Some(matched)
        }
    }
}
fn taker(c: &MatchSpotOrderPipeChanges) -> Option<&SpotOrderV7> {
    match c {
        MatchSpotOrderPipeChanges::Resting => None,
        MatchSpotOrderPipeChanges::PartiallyFilled { taker, .. }
        | MatchSpotOrderPipeChanges::Filled { taker, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { taker, .. }
        | MatchSpotOrderPipeChanges::Rejected { taker } => Some(taker.after()),
    }
}
fn makers(c: &MatchSpotOrderPipeChanges) -> &[common_entity::UpdatedEntityEvents<SpotOrderV7>] {
    match c {
        MatchSpotOrderPipeChanges::PartiallyFilled { makers, .. }
        | MatchSpotOrderPipeChanges::Filled { makers, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { makers, .. } => makers,
        _ => &[],
    }
}
fn trades(c: &MatchSpotOrderPipeChanges) -> u64 {
    match c {
        MatchSpotOrderPipeChanges::PartiallyFilled { trades, .. }
        | MatchSpotOrderPipeChanges::Filled { trades, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { trades, .. } => trades.len() as u64,
        _ => 0,
    }
}
fn settled(c: &PlaceSpotOrderPipelineV2Changes) -> Option<&SettleTradesPipeChanges> {
    match c {
        PlaceSpotOrderPipelineV2Changes::PartiallyFilled { settled, .. }
        | PlaceSpotOrderPipelineV2Changes::Filled { settled, .. }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { settled, .. } => {
            Some(settled)
        }
        _ => None,
    }
}
fn unfrozen(
    c: &PlaceSpotOrderPipelineV2Changes,
) -> &[common_entity::UpdatedEntityEvents<BalanceV2>] {
    match c {
        PlaceSpotOrderPipelineV2Changes::Rejected { unfrozen, .. }
        | PlaceSpotOrderPipelineV2Changes::PartiallyFilled { unfrozen, .. }
        | PlaceSpotOrderPipelineV2Changes::Filled { unfrozen, .. }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { unfrozen, .. } => {
            &unfrozen.updated_balances
        }
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, LogEntrySequencer};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::{OrderGroupRelation, OrderTif, OrderType};
    use rust_decimal::Decimal;

    use super::*;

    const USER: &str = "buyer";

    fn entry(lsn: u64, user: &str, payload: SpotBlockCommandV2) -> LogEntry<SpotBlockCommandV2> {
        LogEntrySequencer::from_metadata_with_topic(
            4,
            2,
            lsn,
            lsn + 100,
            9,
            lsn,
            user.to_owned(),
            payload,
        )
    }

    fn place(order_id: u64, user: &str) -> SpotBlockCommandV2 {
        SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
            party_id: common_entity::stable_string_entity_id(user),
            order_id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side: OrderSide::Buy,
            price: Decimal::from(10),
            size: Decimal::from(2),
            order_type: OrderType::Limit { tif: OrderTif::Gtc },
            relation: OrderGroupRelation::Standalone,
            cloid: Some(format!("order-{order_id}")),
            fee_bps: 10,
        })
    }

    fn cancel(order_id: u64) -> SpotBlockCommandV2 {
        SpotBlockCommandV2::Cancel(CancelSpotOrderFsmV3Cmd {
            order_id: OrderId::Oid(order_id),
        })
    }

    fn state() -> SpotBlockStateV2 {
        let party = common_entity::stable_string_entity_id(USER);
        SpotBlockStateV2 {
            orders: Vec::new(),
            balances: vec![
                BalanceV2::new(party, party, u64::from(Coin::BTC.index()), 10, 0, 1, 1),
                BalanceV2::new(party, party, u64::from(Coin::USDT.index()), 100, 0, 1, 1),
            ],
            next_trade_id: 1,
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        }
    }

    #[test]
    fn block_rejects_empty_non_increasing_and_mixed_shard_commands() {
        let use_case = SpotBlockUseCaseV2;
        assert_eq!(
            use_case.check_command(&SpotBlockCmdV2 { items: vec![] }),
            Err(SpotBlockErrorV2::EmptyCommands)
        );
        let repeated = entry(1, USER, place(1, USER));
        assert_eq!(
            use_case.check_command(&SpotBlockCmdV2 {
                items: vec![repeated.clone(), repeated]
            }),
            Err(SpotBlockErrorV2::NonIncreasingLsn)
        );
        let mixed = LogEntrySequencer::from_metadata_with_topic(
            5,
            2,
            2,
            102,
            9,
            2,
            USER.to_owned(),
            place(2, USER),
        );
        assert_eq!(
            use_case.check_command(&SpotBlockCmdV2 {
                items: vec![entry(1, USER, place(1, USER)), mixed]
            }),
            Err(SpotBlockErrorV2::MixedShard)
        );
    }

    #[test]
    fn block_rejects_duplicate_initial_balance_identity() {
        let command = SpotBlockCmdV2 {
            items: vec![entry(1, USER, place(1, USER))],
        };
        let mut given = state();
        given.balances.push(given.balances[0].clone());
        assert_eq!(
            SpotBlockUseCaseV2.validate_state_given(&command, &given),
            Err(SpotBlockErrorV2::DuplicateBalanceIdentity)
        );
    }

    #[test]
    fn failed_command_rolls_back_and_later_command_still_applies() {
        let command = SpotBlockCmdV2 {
            items: vec![
                entry(1, "intruder", place(7, USER)),
                entry(2, USER, place(8, USER)),
            ],
        };
        let changes = SpotBlockUseCaseV2
            .validate_and_compute(&command, &state())
            .unwrap();
        assert!(matches!(
            changes.command_results[0],
            SpotBlockCommandResultV2::Place { result: Err(_), .. }
        ));
        assert!(matches!(
            changes.command_results[1],
            SpotBlockCommandResultV2::Place { result: Ok(_), .. }
        ));
        assert_eq!(changes.block_result.orders.len(), 1);
        assert_eq!(changes.block_result.orders[0].entity_id(), 8);
    }

    #[test]
    fn place_then_cancel_preserves_typed_metadata_snapshot_and_replay_order() {
        let command = SpotBlockCmdV2 {
            items: vec![entry(10, USER, place(7, USER)), entry(11, USER, cancel(7))],
        };
        let changes = SpotBlockUseCaseV2
            .validate_and_compute(&command, &state())
            .unwrap();
        let SpotBlockCommandResultV2::Place {
            command: placed,
            result: Ok(_),
        } = &changes.command_results[0]
        else {
            panic!("place must succeed")
        };
        let SpotBlockCommandResultV2::Cancel {
            command: cancelled,
            result: Ok(_),
        } = &changes.command_results[1]
        else {
            panic!("cancel must succeed")
        };
        assert_eq!(
            (
                placed.topic(),
                placed.partition(),
                placed.lsn(),
                placed.logical_ts()
            ),
            (4, 2, 10, 110)
        );
        assert_eq!(
            (
                cancelled.topic(),
                cancelled.partition(),
                cancelled.lsn(),
                cancelled.logical_ts()
            ),
            (4, 2, 11, 111)
        );
        assert_eq!(changes.block_result.orders.len(), 1);
        assert_eq!(
            changes.block_result.orders[0].status(),
            SpotOrderStatus::Canceled
        );
        assert_eq!(changes.block_result.next_trade_id, 1);
        let events = changes.replayable_events();
        assert!(!events.is_empty());
        assert!(events.windows(2).all(|pair| pair[0].lsn <= pair[1].lsn));
        assert!(events.iter().all(|event| event.t == 4 && event.p == 2));
    }
}
