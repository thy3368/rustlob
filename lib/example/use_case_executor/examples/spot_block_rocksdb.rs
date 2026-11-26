use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

use base_types_v2::{Coin, Symbol};
use common_entity::{Entity, LogEntrySequencer, stable_string_entity_id};
use example_core_entity::{
    BalanceV2, OrderGroupRelation, OrderSide, OrderTif, OrderType, PlaceSpotOrderPipelineV2Cmd,
};
use example_outbound_adapter::database::{
    BlockRocksDbRepo, BuildBlockWriteSummary, State4BuildBlock,
};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;
use rust_decimal::Decimal;
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::{
    SpotBlockCommandExecutionV3, SpotBlockCommandV2, build_block_spot_v3,
};

const USER_ID: &str = "demo-user";
const SYMBOL_ID: u32 = 1;
const MAKER_FEE_BPS: u64 = 5;
const TAKER_FEE_BPS: u64 = 10;
const INITIAL_BALANCE: u64 = 1_000_000;
const ROCKSDB_PATH: &str = "/tmp/rustlob-spot-rocksdb";

#[derive(Debug)]
struct RunOutcome {
    order_id: u64,
    cloid: String,
    summary: BuildBlockWriteSummary,
}

fn main() -> Result<(), Box<dyn Error>> {
    let outcome = run(Path::new(ROCKSDB_PATH))?;
    println!(
        "order_id={} cloid={} orders(created={}, updated={}) balances(created={}, updated={}) trades(created={}, updated={})",
        outcome.order_id,
        outcome.cloid,
        outcome.summary.orders_created,
        outcome.summary.orders_updated,
        outcome.summary.balances_created,
        outcome.summary.balances_updated,
        outcome.summary.trades_created,
        outcome.summary.trades_updated,
    );
    Ok(())
}

fn run(path: &Path) -> Result<RunOutcome, Box<dyn Error>> {
    let repo = BlockRocksDbRepo::open(path)?;
    let before = repo.load_state()?.unwrap_or_else(empty_state);
    let mut after = before.clone();
    ensure_demo_state(&mut after);

    let order_id = next_order_id(&after)?;
    let lsn = next_lsn(&after)?;
    let logical_ts = next_logical_ts(&after)?;
    let mut cloid = String::from("demo-");
    cloid.push_str(&order_id.to_string());
    let command = LogEntrySequencer::from_metadata_with_topic(
        1,
        0,
        lsn,
        logical_ts,
        order_id,
        order_id,
        USER_ID.to_owned(),
        SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
            party_id: stable_string_entity_id(USER_ID),
            order_id,
            symbol: spot_symbol(),
            side: OrderSide::Buy,
            price: Decimal::from(100_u64),
            size: Decimal::ONE,
            order_type: OrderType::Limit { tif: OrderTif::Gtc },
            relation: OrderGroupRelation::Standalone,
            cloid: Some(cloid.clone()),
            fee_bps: TAKER_FEE_BPS,
        }),
    );

    let execution = build_block_spot_v3(&[command], &mut after)
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    for result in execution.command_results {
        match result {
            SpotBlockCommandExecutionV3::Place {
                result: Err(error), ..
            } => {
                eprintln!("现货下单失败，未写入 RocksDB: {error:?}");
                return Err("现货下单失败，未写入 RocksDB".into());
            }
            SpotBlockCommandExecutionV3::Cancel {
                result: Err(error), ..
            } => {
                eprintln!("现货撤单失败，未写入 RocksDB: {error:?}");
                return Err("现货撤单失败，未写入 RocksDB".into());
            }
            _ => {}
        }
    }

    let summary = repo.persist_state_diff(&before, &after)?;
    let reloaded = repo.load_state()?.ok_or("持久化后未能重新加载状态")?;
    let persisted = reloaded
        .user_symbol_orders
        .values()
        .flatten()
        .any(|order| order.order_base().order_id == order_id);
    if !persisted {
        return Err("重新加载后找不到新订单".into());
    }

    Ok(RunOutcome {
        order_id,
        cloid,
        summary,
    })
}

fn empty_state() -> State4BuildBlock {
    State4BuildBlock {
        next_trade_id: 1,
        maker_fee_bps: MAKER_FEE_BPS,
        taker_fee_bps: TAKER_FEE_BPS,
        spot_symbol_lob: BTreeMap::new(),
        user_symbol_quote_balance: BTreeMap::new(),
        user_symbol_base_balance: BTreeMap::new(),
        user_symbol_trades: BTreeMap::new(),
        user_symbol_orders: BTreeMap::new(),
    }
}

fn ensure_demo_state(state: &mut State4BuildBlock) {
    let symbol = spot_symbol();
    state
        .spot_symbol_lob
        .entry(symbol)
        .or_insert_with(|| LocalLobBtree::new(symbol));
    let key = (USER_ID.to_owned(), symbol);
    let party_id = stable_string_entity_id(USER_ID);
    state
        .user_symbol_base_balance
        .entry(key.clone())
        .or_insert_with(|| {
            BalanceV2::new(
                party_id,
                party_id,
                u64::from(symbol.base.index()),
                INITIAL_BALANCE,
                0_u64,
                0,
                0,
            )
        });
    state
        .user_symbol_quote_balance
        .entry(key)
        .or_insert_with(|| {
            BalanceV2::new(
                party_id,
                party_id,
                u64::from(symbol.quote.index()),
                INITIAL_BALANCE,
                0_u64,
                0,
                0,
            )
        });
}

fn spot_symbol() -> Symbol {
    Symbol::new(SYMBOL_ID, Coin::BTC, Coin::USDT)
}

fn next_order_id(state: &State4BuildBlock) -> Result<u64, Box<dyn Error>> {
    state
        .user_symbol_orders
        .values()
        .flatten()
        .map(|order| order.order_base().order_id)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(|| "order ID 已耗尽".into())
}

fn next_lsn(state: &State4BuildBlock) -> Result<u64, Box<dyn Error>> {
    let order_lsn = state.user_symbol_orders.values().flatten().map(Entity::lsn);
    let balance_lsn = state
        .user_symbol_base_balance
        .values()
        .chain(state.user_symbol_quote_balance.values())
        .map(Entity::lsn);
    let trade_lsn = state.user_symbol_trades.values().flatten().map(Entity::lsn);
    order_lsn
        .chain(balance_lsn)
        .chain(trade_lsn)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(|| "LSN 已耗尽".into())
}

fn next_logical_ts(state: &State4BuildBlock) -> Result<u64, Box<dyn Error>> {
    let order_ts = state
        .user_symbol_orders
        .values()
        .flatten()
        .map(|order| order.order_base().updated_at);
    let balance_ts = state
        .user_symbol_base_balance
        .values()
        .chain(state.user_symbol_quote_balance.values())
        .map(BalanceV2::updated_at);
    let trade_ts = state
        .user_symbol_trades
        .values()
        .flatten()
        .map(|trade| trade.comon_trade.updated_at);
    order_ts
        .chain(balance_ts)
        .chain(trade_ts)
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or_else(|| "逻辑时间已耗尽".into())
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn empty_database_round_trip_persists_demo_state_and_order() {
        let dir = TempDir::new().unwrap();
        let outcome = run(dir.path()).unwrap();
        let state = BlockRocksDbRepo::open(dir.path())
            .unwrap()
            .load_state()
            .unwrap()
            .unwrap();

        assert_eq!(outcome.order_id, 1);
        assert!(state.spot_symbol_lob.contains_key(&spot_symbol()));
        let key = (USER_ID.to_owned(), spot_symbol());
        let base_balance = state.user_symbol_base_balance.get(&key).unwrap();
        let quote_balance = state.user_symbol_quote_balance.get(&key).unwrap();
        assert_eq!(base_balance.coin_idx(), u64::from(Coin::BTC.index()));
        assert_eq!(quote_balance.coin_idx(), u64::from(Coin::USDT.index()));
        assert_eq!(state.user_symbol_orders.values().flatten().count(), 1);
    }

    #[test]
    fn repeated_run_uses_new_identity_and_preserves_existing_order() {
        let dir = TempDir::new().unwrap();
        let first = run(dir.path()).unwrap();
        let second = run(dir.path()).unwrap();
        let state = BlockRocksDbRepo::open(dir.path())
            .unwrap()
            .load_state()
            .unwrap()
            .unwrap();

        assert_ne!(first.order_id, second.order_id);
        assert_ne!(first.cloid, second.cloid);
        let ids: Vec<_> = state
            .user_symbol_orders
            .values()
            .flatten()
            .map(|order| order.order_base().order_id)
            .collect();
        assert!(ids.contains(&first.order_id));
        assert!(ids.contains(&second.order_id));
    }

    #[test]
    fn failed_execution_is_not_persisted() {
        let dir = TempDir::new().unwrap();
        let repo = BlockRocksDbRepo::open(dir.path()).unwrap();
        let before = empty_state();
        repo.persist_state_diff(&before, &before).unwrap();
        let mut candidate = before.clone();
        let invalid = LogEntrySequencer::from_metadata(
            1,
            1,
            1,
            1,
            USER_ID.to_owned(),
            SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
                party_id: stable_string_entity_id(USER_ID),
                order_id: 1,
                symbol: spot_symbol(),
                side: OrderSide::Buy,
                price: Decimal::from(100_u64),
                size: Decimal::ONE,
                order_type: OrderType::Limit { tif: OrderTif::Gtc },
                relation: OrderGroupRelation::Standalone,
                cloid: Some("will-fail".to_owned()),
                fee_bps: TAKER_FEE_BPS,
            }),
        );

        let execution = build_block_spot_v3(&[invalid], &mut candidate).unwrap();
        assert!(matches!(
            &execution.command_results[0],
            SpotBlockCommandExecutionV3::Place { result: Err(_), .. }
        ));
        assert_eq!(candidate, before);
        assert_eq!(repo.load_state().unwrap(), Some(before));
    }
}
