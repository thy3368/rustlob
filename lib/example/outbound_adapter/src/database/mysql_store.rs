use std::collections::HashMap;

use common_entity::stable_string_entity_id;
use example_core_use_case::entity::account::coin::Coin;
use example_core_use_case::entity::account::symbol::Symbol;
use example_core_use_case::entity::common::common_order::{
    CommonOrderExecution, CommonOrderTrigger, OrderBase,
};
use example_core_use_case::{
    BalanceV2, MarketRules, OrderGroupRelation, OrderSide, OrderTif, SpotOrderStatus, SpotOrderV7,
    UserId,
};
use mysql::prelude::Queryable;
use rust_decimal::Decimal;

use crate::shared::{StoreError, StoreSnapshot, balance_key, named_params};

pub(crate) const ACCOUNT_TABLE: &str = "example_place_order_accounts";
pub(crate) const MARKET_RULES_TABLE: &str = "example_place_order_market_rules";
pub(crate) const ORDER_TABLE: &str = "example_place_order_orders";
pub(crate) const EVENT_TABLE: &str = "example_place_order_events";

#[derive(Debug, Clone)]
pub struct MySqlStore {
    pub(crate) pool: mysql::Pool,
}

impl MySqlStore {
    pub fn new(database_url: &str) -> Result<Self, StoreError> {
        let pool = mysql::Pool::new(database_url).map_err(map_mysql_error)?;
        let store = Self { pool };
        store.ensure_schema()?;
        Ok(store)
    }

    pub fn ensure_schema(&self) -> Result<(), StoreError> {
        let mut conn = self.pool.get_conn().map_err(map_mysql_error)?;

        conn.query_drop(format!(
            "CREATE TABLE IF NOT EXISTS {ACCOUNT_TABLE} (
                account_numeric_id BIGINT UNSIGNED PRIMARY KEY,
                account_id VARCHAR(191) NOT NULL UNIQUE,
                available_base BIGINT UNSIGNED NOT NULL DEFAULT 0,
                frozen_base BIGINT UNSIGNED NOT NULL DEFAULT 0,
                available_quote BIGINT UNSIGNED NOT NULL,
                frozen_quote BIGINT UNSIGNED NOT NULL,
                version BIGINT UNSIGNED NOT NULL
            )"
        ))
        .map_err(map_mysql_error)?;

        conn.query_drop(format!(
            "CREATE TABLE IF NOT EXISTS {MARKET_RULES_TABLE} (
                symbol VARCHAR(64) PRIMARY KEY,
                min_qty BIGINT UNSIGNED NOT NULL
            )"
        ))
        .map_err(map_mysql_error)?;

        conn.query_drop(format!(
            "CREATE TABLE IF NOT EXISTS {ORDER_TABLE} (
                order_id VARCHAR(191) PRIMARY KEY,
                account_id VARCHAR(191) NOT NULL,
                asset BIGINT UNSIGNED NOT NULL DEFAULT 0,
                symbol VARCHAR(64) NOT NULL,
                side VARCHAR(16) NOT NULL DEFAULT 'buy',
                execution VARCHAR(16) NOT NULL DEFAULT 'limit',
                time_in_force VARCHAR(16) NOT NULL DEFAULT 'gtc',
                qty BIGINT UNSIGNED NOT NULL,
                price BIGINT UNSIGNED NOT NULL,
                created_sequence BIGINT UNSIGNED NOT NULL
            )"
        ))
        .map_err(map_mysql_error)?;

        conn.query_drop(format!(
            "CREATE TABLE IF NOT EXISTS {EVENT_TABLE} (
                id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
                entity_type TINYINT UNSIGNED NOT NULL,
                change_type TINYINT UNSIGNED NOT NULL,
                entity_id BIGINT UNSIGNED NOT NULL,
                previous_lsn BIGINT UNSIGNED NOT NULL,
                lsn BIGINT UNSIGNED NOT NULL,
                logical_ts BIGINT UNSIGNED NOT NULL,
                order_id VARCHAR(191) NULL,
                account_id VARCHAR(191) NULL,
                asset BIGINT UNSIGNED NULL,
                symbol VARCHAR(64) NULL,
                side VARCHAR(16) NULL,
                execution VARCHAR(16) NULL,
                time_in_force VARCHAR(16) NULL,
                qty BIGINT UNSIGNED NULL,
                price BIGINT UNSIGNED NULL,
                reserved_base BIGINT UNSIGNED NULL,
                reserved_quote BIGINT UNSIGNED NULL,
                available_base BIGINT UNSIGNED NULL,
                frozen_base BIGINT UNSIGNED NULL,
                available_quote BIGINT UNSIGNED NULL,
                frozen_quote BIGINT UNSIGNED NULL,
                published_at TIMESTAMP NULL DEFAULT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )"
        ))
        .map_err(map_mysql_error)?;

        Ok(())
    }

    pub fn seed_account_row(
        &self,
        account_id: &str,
        available_base: u64,
        frozen_base: u64,
        available_quote: u64,
        frozen_quote: u64,
        version: u64,
    ) -> Result<(), StoreError> {
        let mut conn = self.pool.get_conn().map_err(map_mysql_error)?;
        conn.exec_drop(
            format!(
                "INSERT INTO {ACCOUNT_TABLE} (
                    account_numeric_id, account_id, available_base, frozen_base, available_quote, frozen_quote, version
                 )
                 VALUES (
                    :account_numeric_id, :account_id, :available_base, :frozen_base, :available_quote, :frozen_quote, :version
                 )
                 ON DUPLICATE KEY UPDATE
                   account_id = VALUES(account_id),
                   available_base = VALUES(available_base),
                   frozen_base = VALUES(frozen_base),
                   available_quote = VALUES(available_quote),
                   frozen_quote = VALUES(frozen_quote),
                   version = VALUES(version)"
            ),
            named_params([
                ("account_numeric_id", mysql::Value::from(stable_string_entity_id(account_id))),
                ("account_id", mysql::Value::from(account_id)),
                ("available_base", mysql::Value::from(available_base)),
                ("frozen_base", mysql::Value::from(frozen_base)),
                ("available_quote", mysql::Value::from(available_quote)),
                ("frozen_quote", mysql::Value::from(frozen_quote)),
                ("version", mysql::Value::from(version)),
            ]),
        )
        .map_err(map_mysql_error)?;

        Ok(())
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), StoreError> {
        let mut conn = self.pool.get_conn().map_err(map_mysql_error)?;
        conn.exec_drop(
            format!(
                "INSERT INTO {MARKET_RULES_TABLE} (symbol, min_qty)
                 VALUES (:symbol, :min_qty)
                 ON DUPLICATE KEY UPDATE min_qty = VALUES(min_qty)"
            ),
            named_params([
                ("symbol", mysql::Value::from(market_rules.symbol)),
                ("min_qty", mysql::Value::from(market_rules.min_qty)),
            ]),
        )
        .map_err(map_mysql_error)?;

        Ok(())
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, StoreError> {
        let mut conn = self.pool.get_conn().map_err(map_mysql_error)?;

        let account_rows: Vec<(u64, String, u64, u64, u64, u64, u64)> = conn
            .query(format!(
                "SELECT account_numeric_id, account_id, available_base, frozen_base, available_quote, frozen_quote, version FROM {ACCOUNT_TABLE}"
            ))
            .map_err(map_mysql_error)?;
        let mut balances = HashMap::new();
        for (
            account_numeric_id,
            _account_id,
            available_base,
            frozen_base,
            available_quote,
            frozen_quote,
            version,
        ) in account_rows
        {
            balances.insert(
                balance_key(account_numeric_id, stable_string_entity_id("BTC")),
                BalanceV2::new(
                    account_numeric_id,
                    account_numeric_id,
                    stable_string_entity_id("BTC"),
                    available_base,
                    frozen_base,
                    version,
                    0,
                ),
            );
            balances.insert(
                balance_key(account_numeric_id, stable_string_entity_id("USDT")),
                BalanceV2::new(
                    account_numeric_id,
                    account_numeric_id,
                    stable_string_entity_id("USDT"),
                    available_quote,
                    frozen_quote,
                    version,
                    0,
                ),
            );
        }

        type OrderRow = (String, String, u32, String, String, String, String, u64, u64);
        let order_rows: Vec<OrderRow> = conn
            .query(format!(
                "SELECT order_id, account_id, asset, symbol, side, execution, time_in_force, qty, price FROM {ORDER_TABLE}"
            ))
            .map_err(map_mysql_error)?;
        let orders = order_rows
            .into_iter()
            .filter_map(
                |(
                    order_id,
                    account_id,
                    asset,
                    symbol,
                    side,
                    execution,
                    time_in_force,
                    qty,
                    price,
                )| {
                    let side = decode_side_mysql(side.as_str())?;
                    let (execution, tif) =
                        decode_execution_mysql(execution.as_str(), time_in_force.as_str())?;
                    let order_id = order_id.parse().ok()?;
                    if symbol != "BTCUSDT" && symbol != "BTC/USDT" {
                        return None;
                    }
                    let order = SpotOrderV7::from_record(OrderBase {
                        owner: UserId(stable_string_entity_id(&account_id)),
                        order_id,
                        symbol: Symbol::new(asset, Coin::BTC, Coin::USDT),
                        side,
                        limit_px: Decimal::from(price),
                        size: Decimal::from(qty),
                        orig_size: Decimal::from(qty),
                        execution,
                        tif,
                        trigger: CommonOrderTrigger::None,
                        trigger_px: None,
                        group_relation: OrderGroupRelation::Standalone,
                        is_position_tpsl: false,
                        reduce_only: false,
                        status: SpotOrderStatus::Open,
                        buy_quote_fee_reserve_limit: Decimal::ZERO,
                        buy_quote_fee_reserve_consumed: Decimal::ZERO,
                        cloid: None,
                        created_at: 0,
                        updated_at: 1,
                        version: 1,
                    })
                    .ok()?;
                    Some((order_id, order))
                },
            )
            .collect::<HashMap<_, _>>();

        let persisted_event_count = conn
            .query_first::<u64, _>(format!("SELECT COUNT(*) FROM {EVENT_TABLE}"))
            .map_err(map_mysql_error)?
            .unwrap_or(0) as usize;
        let published_event_count = conn
            .query_first::<u64, _>(format!(
                "SELECT COUNT(*) FROM {EVENT_TABLE} WHERE published_at IS NOT NULL"
            ))
            .map_err(map_mysql_error)?
            .unwrap_or(0) as usize;
        let next_order_sequence = conn
            .query_first::<u64, _>(format!(
                "SELECT COALESCE(MAX(created_sequence), 0) + 1 FROM {ORDER_TABLE}"
            ))
            .map_err(map_mysql_error)?
            .unwrap_or(1);

        Ok(StoreSnapshot {
            balances,
            orders,
            trades: HashMap::new(),
            persisted_event_count,
            published_event_count,
            broker_message_count: 0,
            next_order_sequence,
        })
    }
}

pub(crate) fn map_mysql_error(_error: mysql::Error) -> StoreError {
    StoreError::StoreUnavailable
}

fn decode_side_mysql(value: &str) -> Option<OrderSide> {
    match value {
        "buy" => Some(OrderSide::Buy),
        "sell" => Some(OrderSide::Sell),
        _ => None,
    }
}

fn decode_execution_mysql(
    execution: &str,
    time_in_force: &str,
) -> Option<(CommonOrderExecution, OrderTif)> {
    let tif = match time_in_force {
        "gtc" => OrderTif::Gtc,
        "ioc" => OrderTif::Ioc,
        "alo" => OrderTif::Alo,
        _ => return None,
    };
    let execution = match execution {
        "limit" => Some(CommonOrderExecution::Limit),
        "market" => Some(CommonOrderExecution::Market),
        _ => None,
    }?;
    Some((execution, tif))
}

pub(crate) fn event_string_field_mysql(
    event: &common_entity::EntityReplayableEvent,
    field_name: &str,
) -> Option<String> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        Some(String::from_utf8_lossy(change.new_value_bytes()).to_string())
    })
}

pub(crate) fn event_u64_field_mysql(
    event: &common_entity::EntityReplayableEvent,
    field_name: &str,
) -> Option<u64> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        std::str::from_utf8(change.new_value_bytes()).ok()?.parse::<u64>().ok()
    })
}
