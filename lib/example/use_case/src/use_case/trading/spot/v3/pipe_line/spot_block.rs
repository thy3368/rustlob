use std::collections::HashMap;

use common_entity::{
    LogAttr, LogEntry, PresequencedCommand, ReplayableDomainEvent, ReplayableEntityChanges, Shard,
    UseCaseV3,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{
    BalanceV2, CreateSpotOrderV7Command, OrderGroupRelation, OrderSide, OrderType, SpotOrderStatus,
    SpotOrderV7,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::cancel_spot_order::{
    CancelSpotOrderFsmV3Changes as CancelSpotOrderV3Changes,
    CancelSpotOrderFsmV3Cmd as CancelSpotOrderV3Cmd,
    CancelSpotOrderFsmV3Error as CancelSpotOrderV3Error,
    CancelSpotOrderFsmV3State as CancelSpotOrderV3State, CancelSpotOrderFsmV3UseCase,
};
use crate::use_case::trading::spot::v3::pipe::match_spot_order::MatchSpotOrderPipeChanges;
use crate::use_case::trading::spot::v3::pipe::place_spot_order_pipeline_v2::{
    PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error, place_spot_order_pipeline_v2,
};
use crate::use_case::trading::spot::v3::pipe::settle_matched_spot_trades::{
    SettleTradesPipeChanges, SpotOrderSettlementBalances,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceSpotOrderV3OrderCmd {
    pub party_id: u64,
    pub order_id: u64,
    pub symbol: example_core_entity::account::symbol::Symbol,
    pub side: OrderSide,
    pub price: Decimal,
    pub size: Decimal,
    pub order_type: OrderType,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceSpotOrderV3Cmd {
    Single(PlaceSpotOrderV3OrderCmd),
    NormalTpsl {
        parent: PlaceSpotOrderV3OrderCmd,
        children: Vec<PlaceSpotOrderV3OrderCmd>,
    },
}

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
    fn contains_event_metadata(&self, event: &ReplayableDomainEvent) -> bool {
        self.items.iter().any(|item| {
            item.topic() == event.t
                && item.partition() == event.p
                && item.lsn() == event.lsn
                && item.logical_ts() == event.logical_ts
        })
    }

    fn allows_empty_events(&self) -> bool {
        true
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
    Place(PlaceSpotOrderPipelineV2Changes),
    Cancel(CancelSpotOrderV3Changes),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotBlockItemError {
    #[error(transparent)]
    Place(#[from] PlaceSpotOrderPipelineV2Error),
    #[error(transparent)]
    Cancel(#[from] CancelSpotOrderV3Error),
    #[error("order was not found")]
    OrderNotFound,
    #[error("balance was not found or is ambiguous")]
    BalanceNotFound,
    #[error("TPSL and grouped orders are not supported")]
    GroupedOrderNotSupported,
    #[error("order id already exists")]
    DuplicateOrderIdentity,
    #[error("client order id already exists")]
    DuplicateClientOrderId,
    #[error("trade id range overflows")]
    TradeIdOverflow,
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

impl ReplayableEntityChanges for SpotBlockChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let mut events = Vec::new();
        for result in &self.item_results {
            let SpotBlockItemResult::Applied { changes, .. } = result else {
                continue;
            };
            let item_events = match changes {
                SpotBlockAppliedChanges::Place(changes) => changes.replayable_events(),
                SpotBlockAppliedChanges::Cancel(changes) => changes.replayable_events(),
            };
            events.extend(item_events);
        }
        events
    }
}

impl UseCaseV3 for SpotBlockUseCase {
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
        let mut staged = self.clone();
        let changes = match item.payload().clone() {
            SpotBlockCommand::Place(command) => staged.apply_place(item, command),
            SpotBlockCommand::Cancel(command) => staged.apply_cancel(item, command),
        }?;
        *self = staged;
        Ok(changes)
    }

    fn apply_place(
        &mut self,
        item: LogEntry<SpotBlockCommand>,
        command: PlaceSpotOrderV3Cmd,
    ) -> Result<SpotBlockAppliedChanges, SpotBlockItemError> {
        let PlaceSpotOrderV3Cmd::Single(order) = command else {
            return Err(SpotBlockItemError::GroupedOrderNotSupported);
        };
        if self.order_by_oid.contains_key(&order.order_id) {
            return Err(SpotBlockItemError::DuplicateOrderIdentity);
        }
        if order
            .cloid
            .as_ref()
            .is_some_and(|cloid| self.order_by_cloid.contains_key(cloid))
        {
            return Err(SpotBlockItemError::DuplicateClientOrderId);
        }
        let maker_orders = self
            .orders
            .iter()
            .filter(|maker| {
                let base = maker.order_base();
                base.symbol == order.symbol
                    && base.side != order.side
                    && matches!(
                        base.status,
                        SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
                    )
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut balances_by_order_id = HashMap::with_capacity(maker_orders.len() + 1);
        balances_by_order_id.insert(
            order.order_id,
            self.balances_for(order.party_id, order.symbol)?,
        );
        for maker in &maker_orders {
            let base = maker.order_base();
            balances_by_order_id
                .insert(base.order_id, self.balances_for(base.owner.0, base.symbol)?);
        }
        let command = CreateSpotOrderV7Command {
            party_id: order.party_id,
            order_id: order.order_id,
            symbol: order.symbol,
            side: order.side,
            price: order.price,
            size: order.size,
            order_type: order.order_type,
            relation: OrderGroupRelation::Standalone,
            cloid: order.cloid,
            fee_bps: self.taker_fee_bps,
        };
        let changes = place_spot_order_pipeline_v2(
            command,
            &maker_orders,
            &balances_by_order_id,
            self.next_trade_id,
            self.maker_fee_bps,
            self.taker_fee_bps,
            item.lsn(),
            item.logical_ts(),
            item.topic(),
            item.partition(),
        )?;
        let trade_count = matched_changes(&changes).map_or(0, match_trade_count);
        self.next_trade_id = self
            .next_trade_id
            .checked_add(trade_count)
            .ok_or(SpotBlockItemError::TradeIdOverflow)?;
        self.project_place_changes(&changes)?;
        self.rebuild_indexes().map_err(map_index_error)?;
        Ok(SpotBlockAppliedChanges::Place(changes))
    }

    fn apply_cancel(
        &mut self,
        item: LogEntry<SpotBlockCommand>,
        command: CancelSpotOrderV3Cmd,
    ) -> Result<SpotBlockAppliedChanges, SpotBlockItemError> {
        let entry = common_entity::LogEntrySequencer::derive(&item, command);
        let order_index = match &entry.order_id {
            OrderId::Oid(oid) => self.order_by_oid.get(&oid),
            OrderId::Cloid(cloid) => self.order_by_cloid.get(cloid.as_str()),
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
        let changes = CancelSpotOrderFsmV3UseCase
            .validate_and_compute(&entry, &CancelSpotOrderV3State { order, balance })
            .map_err(|error| match error {
                common_entity::UseCaseComputeError::Business(error) => error,
                common_entity::UseCaseComputeError::Contract(_) => CancelSpotOrderV3Error::Entity(
                    common_entity::EntityError::Custom("cancel transition contract failed".into()),
                ),
            })?;
        self.replace_order(changes.order.after().clone())?;
        if let Some(balance) = &changes.balance {
            self.replace_balance(balance.after().clone())?;
        }
        self.rebuild_indexes()
            .map_err(|_| SpotBlockItemError::OrderNotFound)?;
        Ok(SpotBlockAppliedChanges::Cancel(changes))
    }

    fn balances_for(
        &self,
        party_id: u64,
        symbol: example_core_entity::account::symbol::Symbol,
    ) -> Result<SpotOrderSettlementBalances, SpotBlockItemError> {
        Ok(SpotOrderSettlementBalances {
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

    fn project_place_changes(
        &mut self,
        changes: &PlaceSpotOrderPipelineV2Changes,
    ) -> Result<(), SpotBlockItemError> {
        let frozen = frozen_changes(changes);
        let taker = matched_changes(changes)
            .and_then(matched_taker)
            .unwrap_or_else(|| frozen.order.after());
        self.orders.push(taker.clone());
        self.replace_balance(frozen.balance.after().clone())?;

        if let Some(matched) = matched_changes(changes) {
            for maker in matched_makers(matched) {
                self.replace_order(maker.after().clone())?;
            }
        }
        if let Some(settled) = settled_changes(changes) {
            for balance in &settled.updated_balances {
                self.replace_balance(balance.after().clone())?;
            }
        }
        for balance in unfrozen_changes(changes) {
            self.replace_balance(balance.after().clone())?;
        }
        Ok(())
    }
}

fn frozen_changes(
    changes: &PlaceSpotOrderPipelineV2Changes,
) -> &crate::use_case::trading::spot::v3::FreezeSpotOrderPipeChanges {
    match changes {
        PlaceSpotOrderPipelineV2Changes::Resting { frozen, .. }
        | PlaceSpotOrderPipelineV2Changes::Rejected { frozen, .. }
        | PlaceSpotOrderPipelineV2Changes::PartiallyFilled { frozen, .. }
        | PlaceSpotOrderPipelineV2Changes::Filled { frozen, .. }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { frozen, .. } => frozen,
    }
}

fn matched_changes(
    changes: &PlaceSpotOrderPipelineV2Changes,
) -> Option<&MatchSpotOrderPipeChanges> {
    match changes {
        PlaceSpotOrderPipelineV2Changes::Resting { .. } => None,
        PlaceSpotOrderPipelineV2Changes::Rejected { matched, .. }
        | PlaceSpotOrderPipelineV2Changes::PartiallyFilled { matched, .. }
        | PlaceSpotOrderPipelineV2Changes::Filled { matched, .. }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { matched, .. } => {
            Some(matched)
        }
    }
}

fn matched_taker(changes: &MatchSpotOrderPipeChanges) -> Option<&SpotOrderV7> {
    match changes {
        MatchSpotOrderPipeChanges::Resting => None,
        MatchSpotOrderPipeChanges::PartiallyFilled { taker, .. }
        | MatchSpotOrderPipeChanges::Filled { taker, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { taker, .. }
        | MatchSpotOrderPipeChanges::Rejected { taker } => Some(taker.after()),
    }
}

fn matched_makers(
    changes: &MatchSpotOrderPipeChanges,
) -> &[common_entity::UpdatedEntityEvents<SpotOrderV7>] {
    match changes {
        MatchSpotOrderPipeChanges::PartiallyFilled { makers, .. }
        | MatchSpotOrderPipeChanges::Filled { makers, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { makers, .. } => makers,
        MatchSpotOrderPipeChanges::Resting | MatchSpotOrderPipeChanges::Rejected { .. } => &[],
    }
}

fn match_trade_count(changes: &MatchSpotOrderPipeChanges) -> u64 {
    match changes {
        MatchSpotOrderPipeChanges::PartiallyFilled { trades, .. }
        | MatchSpotOrderPipeChanges::Filled { trades, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { trades, .. } => trades.len() as u64,
        MatchSpotOrderPipeChanges::Resting | MatchSpotOrderPipeChanges::Rejected { .. } => 0,
    }
}

fn settled_changes(changes: &PlaceSpotOrderPipelineV2Changes) -> Option<&SettleTradesPipeChanges> {
    match changes {
        PlaceSpotOrderPipelineV2Changes::PartiallyFilled { settled, .. }
        | PlaceSpotOrderPipelineV2Changes::Filled { settled, .. }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { settled, .. } => {
            Some(settled)
        }
        PlaceSpotOrderPipelineV2Changes::Resting { .. }
        | PlaceSpotOrderPipelineV2Changes::Rejected { .. } => None,
    }
}

fn unfrozen_changes(
    changes: &PlaceSpotOrderPipelineV2Changes,
) -> &[common_entity::UpdatedEntityEvents<BalanceV2>] {
    match changes {
        PlaceSpotOrderPipelineV2Changes::Rejected { unfrozen, .. }
        | PlaceSpotOrderPipelineV2Changes::PartiallyFilled { unfrozen, .. }
        | PlaceSpotOrderPipelineV2Changes::Filled { unfrozen, .. }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill { unfrozen, .. } => {
            &unfrozen.updated_balances
        }
        PlaceSpotOrderPipelineV2Changes::Resting { .. } => &[],
    }
}

fn map_index_error(error: SpotBlockError) -> SpotBlockItemError {
    match error {
        SpotBlockError::DuplicateOrderIdentity => SpotBlockItemError::DuplicateOrderIdentity,
        SpotBlockError::DuplicateClientOrderId => SpotBlockItemError::DuplicateClientOrderId,
        SpotBlockError::DuplicateBalanceIdentity => SpotBlockItemError::BalanceNotFound,
        _ => SpotBlockItemError::OrderNotFound,
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
    fn partially_filled_taker_releases_filled_maker_fee_reserve_after_settlement() {
        // Rule: taker 部分成交时，已完全成交 maker 的剩余费率预留仍必须在结算后解冻。
        // Given: 较小买方 maker 先挂单，较大卖方 taker 随后只成交一部分。
        let maker = common_entity::stable_string_entity_id("maker");
        let taker = common_entity::stable_string_entity_id("taker");
        let symbol = Symbol::new(1, Coin::BTC, Coin::USDT);
        let entry = |user: &str, lsn, payload| {
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
        };
        let place_order = |party_id, order_id, side, size, cloid: &str| {
            SpotBlockCommand::Place(PlaceSpotOrderV3Cmd::Single(PlaceSpotOrderV3OrderCmd {
                party_id,
                order_id,
                symbol,
                side,
                price: Decimal::from(10),
                size: Decimal::from(size),
                order_type: OrderType::Limit { tif: OrderTif::Gtc },
                cloid: Some(cloid.to_owned()),
            }))
        };
        let command = SpotBlockCmd {
            items: vec![
                entry(
                    "maker",
                    10,
                    place_order(maker, 7, OrderSide::Buy, 1, "maker-buy"),
                ),
                entry(
                    "taker",
                    11,
                    place_order(taker, 8, OrderSide::Sell, 2, "taker-sell"),
                ),
            ],
        };
        let given = SpotBlockState {
            orders: Vec::new(),
            balances: vec![
                BalanceV2::new(maker, maker, u64::from(Coin::BTC.index()), 0, 0, 1, 1),
                BalanceV2::new(maker, maker, u64::from(Coin::USDT.index()), 100, 0, 1, 1),
                BalanceV2::new(taker, taker, u64::from(Coin::BTC.index()), 10, 0, 1, 1),
                BalanceV2::new(taker, taker, u64::from(Coin::USDT.index()), 0, 0, 1, 1),
            ],
            trade_id_start: 1,
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        };

        // When: 同一 block 依次执行 maker 挂单和 taker 撮合。
        let changes = SpotBlockUseCase
            .validate_and_compute(&command, &given)
            .expect("partial fill must apply");

        // Then: pipe_line 保留部分成交分支、maker 终态及其解冻业务事实。
        let SpotBlockItemResult::Applied {
            changes:
                SpotBlockAppliedChanges::Place(PlaceSpotOrderPipelineV2Changes::PartiallyFilled {
                    matched,
                    settled,
                    unfrozen,
                    ..
                }),
            ..
        } = &changes.item_results[1]
        else {
            panic!(
                "expected partially-filled pipe_line: {:?}",
                changes.item_results[1]
            );
        };
        let MatchSpotOrderPipeChanges::PartiallyFilled { taker, makers, .. } = matched else {
            panic!("expected partially-filled match");
        };
        assert_eq!(taker.after().status(), SpotOrderStatus::PartiallyFilled);
        assert_eq!(makers.len(), 1);
        assert_eq!(makers[0].after().status(), SpotOrderStatus::Filled);
        assert_eq!(unfrozen.updated_balances.len(), 1);
        assert_eq!(unfrozen.updated_balances[0].after().frozen(), Decimal::ZERO);

        let settled_last = settled
            .replayable_events()
            .into_iter()
            .map(|event| event.command_index)
            .max()
            .expect("settlement must emit events");
        let unfreeze_event = unfrozen.replayable_events()[0];
        assert_eq!(unfreeze_event.command_index, settled_last + 1);

        // And: 事件元数据保持连续，spot_block 内存状态也消费该解冻结果。
        assert!(changes.replayable_events().windows(2).all(|events| {
            (events[0].lsn, events[0].command_index) < (events[1].lsn, events[1].command_index)
        }));
        assert_eq!(unfreeze_event.lsn, 11);
        let mut projected = WorkingState::try_from(given).expect("given state must be valid");
        for entry in command.items {
            projected.apply(entry).expect("block item must project");
        }
        assert_eq!(
            projected
                .balance_for(maker, u64::from(Coin::USDT.index()))
                .expect("maker quote balance must remain indexed")
                .frozen(),
            Decimal::ZERO
        );
    }

    #[test]
    fn partially_filled_taker_emits_no_unfreeze_when_filled_maker_has_no_reserve_left() {
        // Rule: Filled maker 的冻结资金已被结算完全消耗时，不得生成零金额解冻事件。
        // Given: 较小卖方 maker 冻结的 base 数量恰好全部成交。
        let maker = common_entity::stable_string_entity_id("maker");
        let taker = common_entity::stable_string_entity_id("taker");
        let symbol = Symbol::new(1, Coin::BTC, Coin::USDT);
        let entry = |user: &str, lsn, party_id, order_id, side, size, cloid: &str| {
            LogEntrySequencer::from_metadata_with_topic(
                4,
                2,
                lsn,
                lsn + 100,
                9,
                lsn,
                user.to_owned(),
                SpotBlockCommand::Place(PlaceSpotOrderV3Cmd::Single(PlaceSpotOrderV3OrderCmd {
                    party_id,
                    order_id,
                    symbol,
                    side,
                    price: Decimal::from(10),
                    size: Decimal::from(size),
                    order_type: OrderType::Limit { tif: OrderTif::Gtc },
                    cloid: Some(cloid.to_owned()),
                })),
            )
        };
        let items = vec![
            entry("maker", 10, maker, 7, OrderSide::Sell, 1, "maker-sell"),
            entry("taker", 11, taker, 8, OrderSide::Buy, 2, "taker-buy"),
        ];
        let mut projected = WorkingState::try_from(SpotBlockState {
            orders: Vec::new(),
            balances: vec![
                BalanceV2::new(maker, maker, u64::from(Coin::BTC.index()), 1, 0, 1, 1),
                BalanceV2::new(maker, maker, u64::from(Coin::USDT.index()), 0, 0, 1, 1),
                BalanceV2::new(taker, taker, u64::from(Coin::BTC.index()), 0, 0, 1, 1),
                BalanceV2::new(taker, taker, u64::from(Coin::USDT.index()), 100, 0, 1, 1),
            ],
            trade_id_start: 1,
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        })
        .expect("given state must be valid");

        // When: maker 挂单后被更大的 taker 完全吃掉。
        projected.apply(items[0].clone()).expect("maker must rest");
        let second = projected.apply(items[1].clone()).expect("taker must match");

        // Then: 分支仍为 taker PartiallyFilled，但 maker 无剩余冻结额可解冻。
        let SpotBlockAppliedChanges::Place(PlaceSpotOrderPipelineV2Changes::PartiallyFilled {
            matched,
            unfrozen,
            ..
        }) = second
        else {
            panic!("expected partially-filled pipe_line");
        };
        let MatchSpotOrderPipeChanges::PartiallyFilled { makers, .. } = matched else {
            panic!("expected partially-filled match");
        };
        assert_eq!(makers[0].after().status(), SpotOrderStatus::Filled);
        assert!(unfrozen.updated_balances.is_empty());
        assert!(unfrozen.replayable_events().is_empty());
        assert_eq!(
            projected
                .balance_for(maker, u64::from(Coin::BTC.index()))
                .expect("maker base balance must remain indexed")
                .frozen(),
            Decimal::ZERO
        );
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
            .validate_and_compute(&command, &state())
            .expect("place and cancel must both apply");
        assert!(
            changes
                .item_results
                .iter()
                .all(|item| matches!(item, SpotBlockItemResult::Applied { .. })),
            "unexpected item result: {:?}",
            changes.item_results
        );

        let events = changes.replayable_events();
        let order_type = <SpotOrderV7 as Entity>::entity_type();
        let order_events = events
            .iter()
            .filter(|event| event.entity_type == order_type)
            .collect::<Vec<_>>();
        assert_eq!(order_events.len(), 3);
        assert_eq!(order_events[0].lsn, 10);
        assert_eq!(order_events[1].lsn, 10);
        assert_eq!(order_events[1].previous_lsn, order_events[0].lsn);
        assert_eq!(
            order_events[1].previous_command_index,
            order_events[0].command_index
        );
        assert_eq!(order_events[2].lsn, 11);
        assert_eq!(order_events[2].previous_lsn, order_events[1].lsn);
        assert_eq!(
            order_events[2].previous_command_index,
            order_events[1].command_index
        );
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
            .validate_and_compute(&command, &state())
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
    fn grouped_and_duplicate_orders_reject_only_the_current_item() {
        let PlaceSpotOrderV3Cmd::Single(parent) = (match place(7) {
            SpotBlockCommand::Place(command) => command,
            SpotBlockCommand::Cancel(_) => unreachable!(),
        }) else {
            unreachable!()
        };
        let grouped = SpotBlockCommand::Place(PlaceSpotOrderV3Cmd::NormalTpsl {
            parent: parent.clone(),
            children: vec![PlaceSpotOrderV3OrderCmd {
                order_id: 8,
                side: OrderSide::Sell,
                order_type: OrderType::Trigger {
                    is_market: true,
                    trigger_price: 11,
                    tpsl: example_core_entity::OrderTriggerRole::TakeProfit,
                },
                cloid: Some("child".to_owned()),
                ..parent.clone()
            }],
        });
        let command = SpotBlockCmd {
            items: vec![
                item(10, grouped),
                item(11, place(7)),
                item(12, place(7)),
                item(13, {
                    let mut duplicate_cloid = parent;
                    duplicate_cloid.order_id = 9;
                    SpotBlockCommand::Place(PlaceSpotOrderV3Cmd::Single(duplicate_cloid))
                }),
                item(14, {
                    let mut later = match place(10) {
                        SpotBlockCommand::Place(PlaceSpotOrderV3Cmd::Single(order)) => order,
                        _ => unreachable!(),
                    };
                    later.cloid = Some("later".to_owned());
                    SpotBlockCommand::Place(PlaceSpotOrderV3Cmd::Single(later))
                }),
            ],
        };
        let changes = SpotBlockUseCase
            .validate_and_compute(&command, &state())
            .expect("item rejections must not fail the block");

        assert!(matches!(
            changes.item_results[0],
            SpotBlockItemResult::Rejected {
                error: SpotBlockItemError::GroupedOrderNotSupported,
                ..
            }
        ));
        assert!(matches!(
            changes.item_results[1],
            SpotBlockItemResult::Applied { .. }
        ));
        assert!(matches!(
            changes.item_results[2],
            SpotBlockItemResult::Rejected {
                error: SpotBlockItemError::DuplicateOrderIdentity,
                ..
            }
        ));
        assert!(matches!(
            changes.item_results[3],
            SpotBlockItemResult::Rejected {
                error: SpotBlockItemError::DuplicateClientOrderId,
                ..
            }
        ));
        assert!(matches!(
            changes.item_results[4],
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
