use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use base_types_v2::Symbol;
use common_entity::Entity;
use example_core_entity::spot::{SpotTradeSettlementStatus, SpotTradeV3};
use example_core_entity::{BalanceV2, PartyId, SpotOrderV7, TradeBase};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;
use rocksdb::{DB, Options, WriteBatch};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::State4BuildBlock;

const PREFIX: &[u8] = b"rustlob:build-block:v1:";
const METADATA_KEY: &[u8] = b"rustlob:build-block:v1:metadata";
const ORDER_NAMESPACE: u8 = b'o';
const BALANCE_NAMESPACE: u8 = b'b';
const TRADE_NAMESPACE: u8 = b't';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityChanges<T> {
    pub created: Vec<T>,
    pub updated: Vec<T>,
    pub deleted: Vec<u64>,
}

impl<T> Default for EntityChanges<T> {
    fn default() -> Self {
        Self {
            created: Vec::new(),
            updated: Vec::new(),
            deleted: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildBlockStateDiff {
    pub orders: EntityChanges<SpotOrderV7>,
    pub balances: EntityChanges<BalanceV2>,
    pub trades: EntityChanges<SpotTradeV3>,
    pub next_trade_id: Option<u64>,
    pub maker_fee_bps: Option<u64>,
    pub taker_fee_bps: Option<u64>,
}

impl BuildBlockStateDiff {
    pub fn is_empty(&self) -> bool {
        self.orders.created.is_empty()
            && self.orders.updated.is_empty()
            && self.orders.deleted.is_empty()
            && self.balances.created.is_empty()
            && self.balances.updated.is_empty()
            && self.balances.deleted.is_empty()
            && self.trades.created.is_empty()
            && self.trades.updated.is_empty()
            && self.trades.deleted.is_empty()
            && self.next_trade_id.is_none()
            && self.maker_fee_bps.is_none()
            && self.taker_fee_bps.is_none()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BuildBlockWriteSummary {
    pub orders_created: usize,
    pub orders_updated: usize,
    pub orders_deleted: usize,
    pub balances_created: usize,
    pub balances_updated: usize,
    pub balances_deleted: usize,
    pub trades_created: usize,
    pub trades_updated: usize,
    pub trades_deleted: usize,
    pub metadata_updated: bool,
}

#[derive(Debug, Error)]
pub enum BuildBlockRocksDbError {
    #[error("RocksDB error: {0}")]
    RocksDb(#[from] rocksdb::Error),
    #[error("build-block state codec error: {0}")]
    Codec(#[from] bincode::Error),
    #[error("duplicate {entity} identity {id}")]
    DuplicateIdentity { entity: &'static str, id: u64 },
    #[error("{entity} identity changed at projection {projection}: {before} -> {after}")]
    IdentityChanged {
        entity: &'static str,
        projection: String,
        before: u64,
        after: u64,
    },
    #[error("trade {trade_id} has inconsistent projection copies")]
    InconsistentTradeProjection { trade_id: u64 },
    #[error("stored {entity} {id} is missing")]
    MissingEntity { entity: &'static str, id: u64 },
    #[error("stored build-block metadata has unsupported version {0}")]
    UnsupportedVersion(u8),
    #[error("stored order {order_id} is invalid: {message}")]
    InvalidOrder { order_id: u64, message: String },
    #[error("failed to rebuild LOB for order {order_id}: {message}")]
    Lob { order_id: u64, message: String },
    #[error("invalid build-block snapshot: {0}")]
    InvalidSnapshot(String),
}

pub struct BlockRocksDbRepo {
    db: DB,
}

impl BlockRocksDbRepo {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, BuildBlockRocksDbError> {
        let mut options = Options::default();
        options.create_if_missing(true);
        Ok(Self {
            db: DB::open(&options, path)?,
        })
    }

    pub fn persist_state_diff(
        &self,
        before: &State4BuildBlock,
        after: &State4BuildBlock,
    ) -> Result<BuildBlockWriteSummary, BuildBlockRocksDbError> {
        let before_normalized = normalize(before)?;
        let after_normalized = normalize(after)?;
        validate_projection_identity(&before_normalized, &after_normalized)?;
        let mut diff = diff_normalized(before, after, &before_normalized, &after_normalized);
        let initialized = self.db.get(METADATA_KEY)?.is_some();
        if !initialized {
            diff = full_snapshot_diff(after, &after_normalized);
        } else if diff.is_empty() {
            return Ok(BuildBlockWriteSummary::default());
        }

        let metadata = Metadata::from_normalized(after, &after_normalized);
        let metadata_bytes = bincode::serialize(&metadata)?;
        let mut encoded_orders = Vec::new();
        for order in diff.orders.created.iter().chain(&diff.orders.updated) {
            let id = Entity::entity_id(order);
            encoded_orders.push((
                entity_key(ORDER_NAMESPACE, id),
                bincode::serialize(&after_normalized.orders[&id])?,
            ));
        }
        let mut encoded_balances = Vec::new();
        for balance in diff.balances.created.iter().chain(&diff.balances.updated) {
            let id = Entity::entity_id(balance);
            encoded_balances.push((
                entity_key(BALANCE_NAMESPACE, id),
                bincode::serialize(&after_normalized.balances[&id])?,
            ));
        }
        let mut encoded_trades = Vec::new();
        for trade in diff.trades.created.iter().chain(&diff.trades.updated) {
            let id = Entity::entity_id(trade);
            encoded_trades.push((
                entity_key(TRADE_NAMESPACE, id),
                bincode::serialize(&after_normalized.trades[&id])?,
            ));
        }

        let mut batch = WriteBatch::default();
        for (key, value) in encoded_orders
            .into_iter()
            .chain(encoded_balances)
            .chain(encoded_trades)
        {
            batch.put(key, value);
        }
        for id in &diff.orders.deleted {
            batch.delete(entity_key(ORDER_NAMESPACE, *id));
        }
        for id in &diff.balances.deleted {
            batch.delete(entity_key(BALANCE_NAMESPACE, *id));
        }
        for id in &diff.trades.deleted {
            batch.delete(entity_key(TRADE_NAMESPACE, *id));
        }
        batch.put(METADATA_KEY, metadata_bytes);
        self.db.write(batch)?;
        Ok(summary(&diff))
    }

    pub fn load_state(&self) -> Result<Option<State4BuildBlock>, BuildBlockRocksDbError> {
        let Some(bytes) = self.db.get(METADATA_KEY)? else {
            return Ok(None);
        };
        let metadata: Metadata = bincode::deserialize(&bytes)?;
        if metadata.version != 1 {
            return Err(BuildBlockRocksDbError::UnsupportedVersion(metadata.version));
        }
        let mut orders = BTreeMap::new();
        let mut markets = BTreeSet::new();
        for &id in &metadata.order_ids {
            let record: StoredOrder = self.load_entity(ORDER_NAMESPACE, id, "order")?;
            markets.insert(record.symbol);
            if orders.insert(id, record).is_some() {
                return Err(BuildBlockRocksDbError::DuplicateIdentity {
                    entity: "order",
                    id,
                });
            }
        }
        let mut balances = BTreeMap::new();
        for &id in &metadata.balance_ids {
            let record: StoredBalance = self.load_entity(BALANCE_NAMESPACE, id, "balance")?;
            if balances.insert(id, record).is_some() {
                return Err(BuildBlockRocksDbError::DuplicateIdentity {
                    entity: "balance",
                    id,
                });
            }
        }
        let mut trades = BTreeMap::new();
        for &id in &metadata.trade_ids {
            let record: StoredTrade = self.load_entity(TRADE_NAMESPACE, id, "trade")?;
            if trades.insert(id, record).is_some() {
                return Err(BuildBlockRocksDbError::DuplicateIdentity {
                    entity: "trade",
                    id,
                });
            }
        }
        Ok(Some(rebuild_state(
            metadata,
            markets.into_iter().collect(),
            orders,
            balances,
            trades,
        )?))
    }

    fn load_entity<T: for<'de> Deserialize<'de>>(
        &self,
        namespace: u8,
        id: u64,
        entity: &'static str,
    ) -> Result<T, BuildBlockRocksDbError> {
        let bytes = self
            .db
            .get(entity_key(namespace, id))?
            .ok_or(BuildBlockRocksDbError::MissingEntity { entity, id })?;
        Ok(bincode::deserialize(&bytes)?)
    }
}

const CANONICAL_SNAPSHOT_VERSION: u8 = 1;

#[derive(Serialize, Deserialize)]
struct CanonicalSnapshot {
    version: u8,
    metadata: Metadata,
    markets: Vec<Symbol>,
    orders: BTreeMap<u64, StoredOrder>,
    balances: BTreeMap<u64, StoredBalance>,
    trades: BTreeMap<u64, StoredTrade>,
}

/// 将构块状态编码为与 map 插入顺序无关的版本化 canonical 快照。
pub fn encode_build_block_state(
    state: &State4BuildBlock,
) -> Result<Vec<u8>, BuildBlockRocksDbError> {
    let normalized = normalize(state)?;
    bincode::serialize(&CanonicalSnapshot {
        version: CANONICAL_SNAPSHOT_VERSION,
        metadata: Metadata::from_normalized(state, &normalized),
        markets: state.spot_symbol_lob.keys().copied().collect(),
        orders: normalized.orders,
        balances: normalized.balances,
        trades: normalized.trades,
    })
    .map_err(Into::into)
}

/// 从 canonical 快照恢复完整订单、余额、成交投影和 LOB。
pub fn decode_build_block_state(bytes: &[u8]) -> Result<State4BuildBlock, BuildBlockRocksDbError> {
    let snapshot: CanonicalSnapshot = bincode::deserialize(bytes)?;
    if snapshot.version != CANONICAL_SNAPSHOT_VERSION {
        return Err(BuildBlockRocksDbError::UnsupportedVersion(snapshot.version));
    }
    validate_snapshot_ids(&snapshot)?;
    rebuild_state(
        snapshot.metadata,
        snapshot.markets,
        snapshot.orders,
        snapshot.balances,
        snapshot.trades,
    )
}

fn validate_snapshot_ids(snapshot: &CanonicalSnapshot) -> Result<(), BuildBlockRocksDbError> {
    let actual = [
        snapshot.orders.keys().copied().collect::<Vec<_>>(),
        snapshot.balances.keys().copied().collect::<Vec<_>>(),
        snapshot.trades.keys().copied().collect::<Vec<_>>(),
    ];
    let expected = [
        &snapshot.metadata.order_ids,
        &snapshot.metadata.balance_ids,
        &snapshot.metadata.trade_ids,
    ];
    for (entity, (expected, actual)) in ["order", "balance", "trade"]
        .into_iter()
        .zip(expected.into_iter().zip(actual))
    {
        if expected != &actual {
            return Err(BuildBlockRocksDbError::InvalidSnapshot(format!(
                "{entity} identity index does not match records"
            )));
        }
    }
    Ok(())
}

fn rebuild_state(
    metadata: Metadata,
    markets: Vec<Symbol>,
    orders: BTreeMap<u64, StoredOrder>,
    balances: BTreeMap<u64, StoredBalance>,
    trades: BTreeMap<u64, StoredTrade>,
) -> Result<State4BuildBlock, BuildBlockRocksDbError> {
    if metadata.version != 1 {
        return Err(BuildBlockRocksDbError::UnsupportedVersion(metadata.version));
    }
    let mut state = State4BuildBlock {
        next_trade_id: metadata.next_trade_id,
        maker_fee_bps: metadata.maker_fee_bps,
        taker_fee_bps: metadata.taker_fee_bps,
        spot_symbol_lob: BTreeMap::new(),
        user_symbol_quote_balance: BTreeMap::new(),
        user_symbol_base_balance: BTreeMap::new(),
        user_symbol_trades: BTreeMap::new(),
        user_symbol_orders: BTreeMap::new(),
    };
    for symbol in markets {
        if state
            .spot_symbol_lob
            .insert(symbol, LocalLobBtree::new(symbol))
            .is_some()
        {
            return Err(BuildBlockRocksDbError::InvalidSnapshot(
                "duplicate market symbol".to_owned(),
            ));
        }
    }
    let mut projected_orders: BTreeMap<(PartyId, Symbol), Vec<(usize, SpotOrderV7)>> =
        BTreeMap::new();
    for (id, record) in orders {
        let order =
            SpotOrderV7::from_record_at(record.order, record.command_index).map_err(|error| {
                BuildBlockRocksDbError::InvalidOrder {
                    order_id: id,
                    message: error.to_string(),
                }
            })?;
        if Entity::entity_id(&order) != id {
            return Err(BuildBlockRocksDbError::InvalidSnapshot(format!(
                "order record identity {id} does not match entity"
            )));
        }
        projected_orders
            .entry((record.party_id, record.symbol))
            .or_default()
            .push((record.ordinal, order));
    }
    for (key, mut values) in projected_orders {
        values.sort_by_key(|(ordinal, _)| *ordinal);
        ensure_contiguous_ordinals("order", values.iter().map(|(ordinal, _)| *ordinal))?;
        state
            .user_symbol_orders
            .insert(key, values.into_iter().map(|(_, value)| value).collect());
    }
    for orders in state.user_symbol_orders.values() {
        for order in orders {
            let symbol = order.order_base().symbol;
            state
                .spot_symbol_lob
                .get_mut(&symbol)
                .ok_or_else(|| {
                    BuildBlockRocksDbError::InvalidSnapshot(
                        "order references an unknown market".to_owned(),
                    )
                })?
                .insert_order(order.clone())
                .map_err(|error| BuildBlockRocksDbError::Lob {
                    order_id: Entity::entity_id(order),
                    message: error.to_string(),
                })?;
        }
    }
    for (id, record) in balances {
        if Entity::entity_id(&record.balance) != id {
            return Err(BuildBlockRocksDbError::InvalidSnapshot(format!(
                "balance record identity {id} does not match entity"
            )));
        }
        let target = match record.kind {
            BalanceKind::Base => &mut state.user_symbol_base_balance,
            BalanceKind::Quote => &mut state.user_symbol_quote_balance,
        };
        if target
            .insert((record.party_id, record.symbol), record.balance)
            .is_some()
        {
            return Err(BuildBlockRocksDbError::InvalidSnapshot(
                "duplicate balance projection".to_owned(),
            ));
        }
    }
    let mut projected_trades: BTreeMap<(PartyId, Symbol), Vec<(usize, SpotTradeV3)>> =
        BTreeMap::new();
    for (id, record) in trades {
        let trade = record.trade.into_entity();
        if Entity::entity_id(&trade) != id {
            return Err(BuildBlockRocksDbError::InvalidSnapshot(format!(
                "trade record identity {id} does not match entity"
            )));
        }
        let mut seen = BTreeSet::new();
        for projection in record.projections {
            if !seen.insert((
                projection.party_id.clone(),
                projection.symbol,
                projection.ordinal,
            )) {
                return Err(BuildBlockRocksDbError::InconsistentTradeProjection { trade_id: id });
            }
            projected_trades
                .entry((projection.party_id, projection.symbol))
                .or_default()
                .push((projection.ordinal, trade.clone()));
        }
    }
    for (key, mut values) in projected_trades {
        values.sort_by_key(|(ordinal, _)| *ordinal);
        ensure_contiguous_ordinals("trade", values.iter().map(|(ordinal, _)| *ordinal))?;
        state
            .user_symbol_trades
            .insert(key, values.into_iter().map(|(_, value)| value).collect());
    }
    Ok(state)
}

fn ensure_contiguous_ordinals(
    entity: &'static str,
    ordinals: impl Iterator<Item = usize>,
) -> Result<(), BuildBlockRocksDbError> {
    if ordinals
        .enumerate()
        .any(|(expected, actual)| expected != actual)
    {
        return Err(BuildBlockRocksDbError::InvalidSnapshot(format!(
            "{entity} projection ordinals are not contiguous"
        )));
    }
    Ok(())
}

pub fn diff_build_block_state(
    before: &State4BuildBlock,
    after: &State4BuildBlock,
) -> Result<BuildBlockStateDiff, BuildBlockRocksDbError> {
    let before_normalized = normalize(before)?;
    let after_normalized = normalize(after)?;
    validate_projection_identity(&before_normalized, &after_normalized)?;
    Ok(diff_normalized(
        before,
        after,
        &before_normalized,
        &after_normalized,
    ))
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    version: u8,
    next_trade_id: u64,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
    order_ids: Vec<u64>,
    balance_ids: Vec<u64>,
    trade_ids: Vec<u64>,
}

impl Metadata {
    fn from_normalized(state: &State4BuildBlock, normalized: &Normalized) -> Self {
        Self {
            version: 1,
            next_trade_id: state.next_trade_id,
            maker_fee_bps: state.maker_fee_bps,
            taker_fee_bps: state.taker_fee_bps,
            order_ids: normalized.orders.keys().copied().collect(),
            balance_ids: normalized.balances.keys().copied().collect(),
            trade_ids: normalized.trades.keys().copied().collect(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoredOrder {
    party_id: PartyId,
    symbol: Symbol,
    ordinal: usize,
    order: example_core_entity::common::OrderBase,
    command_index: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum BalanceKind {
    Base,
    Quote,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoredBalance {
    party_id: PartyId,
    symbol: Symbol,
    kind: BalanceKind,
    balance: BalanceV2,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TradeDto {
    trade: TradeBase,
    settled: bool,
    lsn: u64,
    command_index: u32,
}

impl TradeDto {
    fn from_entity(trade: &SpotTradeV3) -> Self {
        Self {
            trade: trade.comon_trade.clone(),
            settled: trade.settlement_status == SpotTradeSettlementStatus::Settled,
            lsn: trade.lsn,
            command_index: trade.command_index,
        }
    }

    fn into_entity(self) -> SpotTradeV3 {
        SpotTradeV3 {
            replay_events: Vec::new(),
            comon_trade: self.trade,
            settlement_status: if self.settled {
                SpotTradeSettlementStatus::Settled
            } else {
                SpotTradeSettlementStatus::Unsettled
            },
            lsn: self.lsn,
            command_index: self.command_index,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TradeProjection {
    party_id: PartyId,
    symbol: Symbol,
    ordinal: usize,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoredTrade {
    trade: TradeDto,
    projections: Vec<TradeProjection>,
}

struct Normalized {
    orders: BTreeMap<u64, StoredOrder>,
    order_entities: BTreeMap<u64, SpotOrderV7>,
    balances: BTreeMap<u64, StoredBalance>,
    trades: BTreeMap<u64, StoredTrade>,
    order_positions: BTreeMap<(PartyId, Symbol, usize), u64>,
    balance_positions: BTreeMap<(BalanceKind, PartyId, Symbol), u64>,
    trade_positions: BTreeMap<(PartyId, Symbol, usize), u64>,
}

fn normalize(state: &State4BuildBlock) -> Result<Normalized, BuildBlockRocksDbError> {
    let mut normalized = Normalized {
        orders: BTreeMap::new(),
        order_entities: BTreeMap::new(),
        balances: BTreeMap::new(),
        trades: BTreeMap::new(),
        order_positions: BTreeMap::new(),
        balance_positions: BTreeMap::new(),
        trade_positions: BTreeMap::new(),
    };
    for ((party_id, symbol), orders) in &state.user_symbol_orders {
        for (ordinal, order) in orders.iter().enumerate() {
            let id = Entity::entity_id(order);
            let record = StoredOrder {
                party_id: party_id.clone(),
                symbol: *symbol,
                ordinal,
                order: order.order_base().clone(),
                command_index: Entity::command_index(order),
            };
            if normalized.orders.insert(id, record).is_some() {
                return Err(BuildBlockRocksDbError::DuplicateIdentity {
                    entity: "order",
                    id,
                });
            }
            normalized.order_entities.insert(id, order.clone());
            normalized
                .order_positions
                .insert((party_id.clone(), *symbol, ordinal), id);
        }
    }
    for (kind, balances) in [
        (BalanceKind::Base, &state.user_symbol_base_balance),
        (BalanceKind::Quote, &state.user_symbol_quote_balance),
    ] {
        for ((party_id, symbol), balance) in balances {
            let id = Entity::entity_id(balance);
            let record = StoredBalance {
                party_id: party_id.clone(),
                symbol: *symbol,
                kind,
                balance: balance.clone(),
            };
            if normalized.balances.insert(id, record).is_some() {
                return Err(BuildBlockRocksDbError::DuplicateIdentity {
                    entity: "balance",
                    id,
                });
            }
            normalized
                .balance_positions
                .insert((kind, party_id.clone(), *symbol), id);
        }
    }
    for ((party_id, symbol), trades) in &state.user_symbol_trades {
        for (ordinal, trade) in trades.iter().enumerate() {
            let id = Entity::entity_id(trade);
            let dto = TradeDto::from_entity(trade);
            let projection = TradeProjection {
                party_id: party_id.clone(),
                symbol: *symbol,
                ordinal,
            };
            match normalized.trades.get_mut(&id) {
                Some(existing) if existing.trade != dto => {
                    return Err(BuildBlockRocksDbError::InconsistentTradeProjection {
                        trade_id: id,
                    });
                }
                Some(existing) => existing.projections.push(projection),
                None => {
                    normalized.trades.insert(
                        id,
                        StoredTrade {
                            trade: dto,
                            projections: vec![projection],
                        },
                    );
                }
            }
            normalized
                .trade_positions
                .insert((party_id.clone(), *symbol, ordinal), id);
        }
    }
    Ok(normalized)
}

fn validate_projection_identity(
    before: &Normalized,
    after: &Normalized,
) -> Result<(), BuildBlockRocksDbError> {
    validate_positions("order", &before.order_positions, &after.order_positions)?;
    validate_positions(
        "balance",
        &before.balance_positions,
        &after.balance_positions,
    )?;
    validate_positions("trade", &before.trade_positions, &after.trade_positions)
}

fn validate_positions<K: Ord + std::fmt::Debug>(
    entity: &'static str,
    before: &BTreeMap<K, u64>,
    after: &BTreeMap<K, u64>,
) -> Result<(), BuildBlockRocksDbError> {
    for (position, before_id) in before {
        if let Some(after_id) = after.get(position) {
            if before_id != after_id
                && !after.values().any(|id| id == before_id)
                && !before.values().any(|id| id == after_id)
            {
                return Err(BuildBlockRocksDbError::IdentityChanged {
                    entity,
                    projection: format!("{position:?}"),
                    before: *before_id,
                    after: *after_id,
                });
            }
        }
    }
    Ok(())
}

fn diff_normalized(
    before: &State4BuildBlock,
    after: &State4BuildBlock,
    old: &Normalized,
    new: &Normalized,
) -> BuildBlockStateDiff {
    BuildBlockStateDiff {
        orders: changes(&old.orders, &new.orders, |id, _| {
            new.order_entities[&id].clone()
        }),
        balances: changes(&old.balances, &new.balances, |_, record| {
            record.balance.clone()
        }),
        trades: changes(&old.trades, &new.trades, |_, record| {
            record.trade.clone().into_entity()
        }),
        next_trade_id: (before.next_trade_id != after.next_trade_id).then_some(after.next_trade_id),
        maker_fee_bps: (before.maker_fee_bps != after.maker_fee_bps).then_some(after.maker_fee_bps),
        taker_fee_bps: (before.taker_fee_bps != after.taker_fee_bps).then_some(after.taker_fee_bps),
    }
}

fn changes<R: PartialEq, T>(
    old: &BTreeMap<u64, R>,
    new: &BTreeMap<u64, R>,
    entity: impl Fn(u64, &R) -> T,
) -> EntityChanges<T> {
    let old_ids: BTreeSet<_> = old.keys().copied().collect();
    let new_ids: BTreeSet<_> = new.keys().copied().collect();
    EntityChanges {
        created: new_ids
            .difference(&old_ids)
            .map(|id| entity(*id, &new[id]))
            .collect(),
        updated: new_ids
            .intersection(&old_ids)
            .filter(|id| old[*id] != new[*id])
            .map(|id| entity(*id, &new[id]))
            .collect(),
        deleted: old_ids.difference(&new_ids).copied().collect(),
    }
}

fn full_snapshot_diff(state: &State4BuildBlock, normalized: &Normalized) -> BuildBlockStateDiff {
    BuildBlockStateDiff {
        orders: EntityChanges {
            created: normalized.order_entities.values().cloned().collect(),
            ..Default::default()
        },
        balances: EntityChanges {
            created: normalized
                .balances
                .values()
                .map(|record| record.balance.clone())
                .collect(),
            ..Default::default()
        },
        trades: EntityChanges {
            created: normalized
                .trades
                .values()
                .map(|record| record.trade.clone().into_entity())
                .collect(),
            ..Default::default()
        },
        next_trade_id: Some(state.next_trade_id),
        maker_fee_bps: Some(state.maker_fee_bps),
        taker_fee_bps: Some(state.taker_fee_bps),
    }
}

fn summary(diff: &BuildBlockStateDiff) -> BuildBlockWriteSummary {
    BuildBlockWriteSummary {
        orders_created: diff.orders.created.len(),
        orders_updated: diff.orders.updated.len(),
        orders_deleted: diff.orders.deleted.len(),
        balances_created: diff.balances.created.len(),
        balances_updated: diff.balances.updated.len(),
        balances_deleted: diff.balances.deleted.len(),
        trades_created: diff.trades.created.len(),
        trades_updated: diff.trades.updated.len(),
        trades_deleted: diff.trades.deleted.len(),
        metadata_updated: true,
    }
}

fn entity_key(namespace: u8, id: u64) -> Vec<u8> {
    let mut key = Vec::with_capacity(PREFIX.len() + 9);
    key.extend_from_slice(PREFIX);
    key.push(namespace);
    key.extend_from_slice(&id.to_be_bytes());
    key
}

#[cfg(test)]
mod tests {
    use base_types_v2::Coin;
    use example_core_entity::account::symbol::Symbol as EntitySymbol;
    use example_core_entity::common::PartyId;
    use example_core_entity::{OrderSide, TradeBase};
    use rust_decimal::Decimal;
    use tempfile::TempDir;

    use super::*;

    fn empty_state() -> State4BuildBlock {
        State4BuildBlock {
            next_trade_id: 7,
            maker_fee_bps: 1,
            taker_fee_bps: 2,
            spot_symbol_lob: BTreeMap::new(),
            user_symbol_quote_balance: BTreeMap::new(),
            user_symbol_base_balance: BTreeMap::new(),
            user_symbol_trades: BTreeMap::new(),
            user_symbol_orders: BTreeMap::new(),
        }
    }

    fn trade(id: u64, price: i64) -> SpotTradeV3 {
        SpotTradeV3 {
            replay_events: Vec::new(),
            comon_trade: TradeBase {
                trade_id: id,
                symbol: EntitySymbol::new(1, Coin::BTC, Coin::USDT),
                taker_party: PartyId("10".to_owned()),
                maker_party: PartyId("20".to_owned()),
                taker_order_id: 100,
                maker_order_id: 200,
                taker_side: OrderSide::Buy,
                price: Decimal::new(price, 0),
                size: Decimal::ONE,
                taker_fee: Decimal::ZERO,
                maker_fee: Decimal::ZERO,
                created_at: 1,
                updated_at: 1,
                version: 1,
            },
            settlement_status: SpotTradeSettlementStatus::Unsettled,
            lsn: 1,
            command_index: 0,
        }
    }

    #[test]
    fn first_snapshot_loads_metadata_and_empty_diff_does_not_write() {
        let dir = TempDir::new().unwrap();
        let store = BlockRocksDbRepo::open(dir.path()).unwrap();
        let state = empty_state();

        let summary = store.persist_state_diff(&empty_state(), &state).unwrap();
        assert!(summary.metadata_updated);
        assert_eq!(store.load_state().unwrap(), Some(state.clone()));

        let summary = store.persist_state_diff(&state, &state).unwrap();
        assert_eq!(summary, BuildBlockWriteSummary::default());
    }

    #[test]
    fn shared_trade_is_stored_once_and_projection_order_is_restored() {
        let dir = TempDir::new().unwrap();
        let store = BlockRocksDbRepo::open(dir.path()).unwrap();
        let mut state = empty_state();
        let symbol = Symbol::new(1, Coin::BTC, Coin::USDT);
        let first = trade(8, 10);
        let second = trade(9, 11);
        state.user_symbol_trades.insert(
            ("maker".into(), symbol),
            vec![second.clone(), first.clone()],
        );
        state
            .user_symbol_trades
            .insert(("taker".into(), symbol), vec![first.clone()]);

        let summary = store.persist_state_diff(&empty_state(), &state).unwrap();
        assert_eq!(summary.trades_created, 2);
        let loaded = store.load_state().unwrap().unwrap();
        assert_eq!(loaded.user_symbol_trades, state.user_symbol_trades);
    }

    #[test]
    fn inconsistent_trade_copies_are_rejected_without_overwriting_snapshot() {
        let dir = TempDir::new().unwrap();
        let store = BlockRocksDbRepo::open(dir.path()).unwrap();
        let baseline = empty_state();
        store.persist_state_diff(&empty_state(), &baseline).unwrap();
        let mut invalid = baseline.clone();
        let symbol = Symbol::new(1, Coin::BTC, Coin::USDT);
        invalid
            .user_symbol_trades
            .insert(("maker".into(), symbol), vec![trade(8, 10)]);
        invalid
            .user_symbol_trades
            .insert(("taker".into(), symbol), vec![trade(8, 11)]);

        assert!(matches!(
            store.persist_state_diff(&baseline, &invalid),
            Err(BuildBlockRocksDbError::InconsistentTradeProjection { trade_id: 8 })
        ));
        assert_eq!(store.load_state().unwrap(), Some(baseline));
    }

    #[test]
    fn corrupt_metadata_returns_codec_error() {
        let dir = TempDir::new().unwrap();
        let store = BlockRocksDbRepo::open(dir.path()).unwrap();
        store.db.put(METADATA_KEY, b"invalid").unwrap();
        assert!(matches!(
            store.load_state(),
            Err(BuildBlockRocksDbError::Codec(_))
        ));
    }

    #[test]
    fn canonical_snapshot_round_trips_empty_market_and_is_stable() {
        let mut state = empty_state();
        let symbol = Symbol::new(1, Coin::BTC, Coin::USDT);
        state
            .spot_symbol_lob
            .insert(symbol, LocalLobBtree::new(symbol));

        let first = encode_build_block_state(&state).unwrap();
        let restored = decode_build_block_state(&first).unwrap();

        assert_eq!(restored, state);
        assert_eq!(encode_build_block_state(&restored).unwrap(), first);
    }

    #[test]
    fn canonical_snapshot_rejects_unknown_version_and_corrupt_data() {
        let bytes = encode_build_block_state(&empty_state()).unwrap();
        let mut snapshot: CanonicalSnapshot = bincode::deserialize(&bytes).unwrap();
        snapshot.version = 2;

        assert!(matches!(
            decode_build_block_state(&bincode::serialize(&snapshot).unwrap()),
            Err(BuildBlockRocksDbError::UnsupportedVersion(2))
        ));
        assert!(matches!(
            decode_build_block_state(b"invalid"),
            Err(BuildBlockRocksDbError::Codec(_))
        ));
    }
}
