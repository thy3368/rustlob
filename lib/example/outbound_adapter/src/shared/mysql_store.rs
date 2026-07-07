use std::collections::HashMap;

use example_core::{
    Balance, MarketRules, SpotOrder, SpotOrderExecution, SpotOrderSide, SpotOrderTimeInForce,
};
use mysql::params;
use mysql::prelude::Queryable;

use super::StoreSnapshot;
use crate::shared::StoreError;

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
                account_id VARCHAR(191) PRIMARY KEY,
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
                reserved_base BIGINT UNSIGNED NOT NULL DEFAULT 0,
                reserved_quote BIGINT UNSIGNED NOT NULL,
                created_sequence BIGINT UNSIGNED NOT NULL
            )"
        ))
        .map_err(map_mysql_error)?;

        conn.query_drop(format!(
            "CREATE TABLE IF NOT EXISTS {EVENT_TABLE} (
                id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
                entity_type TINYINT UNSIGNED NOT NULL,
                change_type TINYINT UNSIGNED NOT NULL,
                entity_id BIGINT NOT NULL,
                old_version BIGINT UNSIGNED NOT NULL,
                new_version BIGINT UNSIGNED NOT NULL,
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
                    account_id, available_base, frozen_base, available_quote, frozen_quote, version
                 )
                 VALUES (
                    :account_id, :available_base, :frozen_base, :available_quote, :frozen_quote, :version
                 )
                 ON DUPLICATE KEY UPDATE
                   available_base = VALUES(available_base),
                   frozen_base = VALUES(frozen_base),
                   available_quote = VALUES(available_quote),
                   frozen_quote = VALUES(frozen_quote),
                   version = VALUES(version)"
            ),
            params! {
                "account_id" => account_id,
                "available_base" => available_base,
                "frozen_base" => frozen_base,
                "available_quote" => available_quote,
                "frozen_quote" => frozen_quote,
                "version" => version,
            },
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
            params! {
                "symbol" => market_rules.symbol,
                "min_qty" => market_rules.min_qty,
            },
        )
        .map_err(map_mysql_error)?;

        Ok(())
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, StoreError> {
        let mut conn = self.pool.get_conn().map_err(map_mysql_error)?;

        let account_rows: Vec<(String, u64, u64, u64, u64, u64)> = conn
            .query(format!(
                "SELECT account_id, available_base, frozen_base, available_quote, frozen_quote, version FROM {ACCOUNT_TABLE}"
            ))
            .map_err(map_mysql_error)?;
        let mut balances = HashMap::new();
        for (account_id, available_base, frozen_base, available_quote, frozen_quote, version) in
            account_rows
        {
            balances.insert(
                format!("{account_id}:BTC"),
                Balance::new(
                    account_id.clone(),
                    "BTC".to_string(),
                    available_base,
                    frozen_base,
                    version,
                ),
            );
            balances.insert(
                format!("{account_id}:USDT"),
                Balance::new(
                    account_id.clone(),
                    "USDT".to_string(),
                    available_quote,
                    frozen_quote,
                    version,
                ),
            );
        }

        type OrderRow = (String, String, u32, String, String, String, String, u64, u64, u64, u64);
        let order_rows: Vec<OrderRow> = conn
            .query(format!(
                "SELECT order_id, account_id, asset, symbol, side, execution, time_in_force, qty, price, reserved_base, reserved_quote FROM {ORDER_TABLE}"
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
                    reserved_base,
                    reserved_quote,
                )| {
                    let side = decode_side_mysql(side.as_str())?;
                    let execution = decode_execution_mysql(execution.as_str(), price)?;
                    let time_in_force = decode_time_in_force_mysql(time_in_force.as_str())?;
                    let order = SpotOrder::new(
                        order_id.clone(),
                        asset,
                        None,
                        account_id,
                        symbol,
                        side,
                        execution,
                        time_in_force,
                        qty,
                        reserved_base,
                        reserved_quote,
                        None,
                    );
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
            reservations: HashMap::new(),
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

fn decode_side_mysql(value: &str) -> Option<SpotOrderSide> {
    match value {
        "buy" => Some(SpotOrderSide::Buy),
        "sell" => Some(SpotOrderSide::Sell),
        _ => None,
    }
}

fn decode_execution_mysql(value: &str, price: u64) -> Option<SpotOrderExecution> {
    match value {
        "market" => Some(SpotOrderExecution::Market { aggressive_price: price }),
        "limit" => Some(SpotOrderExecution::Limit { price }),
        _ => None,
    }
}

fn decode_time_in_force_mysql(value: &str) -> Option<SpotOrderTimeInForce> {
    match value {
        "gtc" => Some(SpotOrderTimeInForce::Gtc),
        "ioc" => Some(SpotOrderTimeInForce::Ioc),
        "alo" => Some(SpotOrderTimeInForce::Alo),
        _ => None,
    }
}

pub(crate) fn event_string_field_mysql(
    event: &cmd_handler::EntityReplayableEvent,
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
    event: &cmd_handler::EntityReplayableEvent,
    field_name: &str,
) -> Option<u64> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }

        std::str::from_utf8(change.new_value_bytes()).ok()?.parse::<u64>().ok()
    })
}
