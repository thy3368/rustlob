use std::collections::HashMap;

use example_core::{MarketRules, StoredOrder, TradingAccount};
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
                symbol VARCHAR(64) NOT NULL,
                qty BIGINT UNSIGNED NOT NULL,
                price BIGINT UNSIGNED NOT NULL,
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
                symbol VARCHAR(64) NULL,
                qty BIGINT UNSIGNED NULL,
                price BIGINT UNSIGNED NULL,
                reserved_quote BIGINT UNSIGNED NULL,
                available_quote BIGINT UNSIGNED NULL,
                frozen_quote BIGINT UNSIGNED NULL,
                published_at TIMESTAMP NULL DEFAULT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )"
        ))
        .map_err(map_mysql_error)?;

        Ok(())
    }

    pub fn seed_account(&self, account: TradingAccount) -> Result<(), StoreError> {
        let mut conn = self.pool.get_conn().map_err(map_mysql_error)?;
        conn.exec_drop(
            format!(
                "INSERT INTO {ACCOUNT_TABLE} (account_id, available_quote, frozen_quote, version)
                 VALUES (:account_id, :available_quote, :frozen_quote, :version)
                 ON DUPLICATE KEY UPDATE
                   available_quote = VALUES(available_quote),
                   frozen_quote = VALUES(frozen_quote),
                   version = VALUES(version)"
            ),
            params! {
                "account_id" => account.account_id,
                "available_quote" => account.available_quote,
                "frozen_quote" => account.frozen_quote,
                "version" => account.version,
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

        let account_rows: Vec<(String, u64, u64, u64)> = conn
            .query(format!(
                "SELECT account_id, available_quote, frozen_quote, version FROM {ACCOUNT_TABLE}"
            ))
            .map_err(map_mysql_error)?;
        let accounts = account_rows
            .into_iter()
            .map(|(account_id, available_quote, frozen_quote, version)| {
                (
                    account_id.clone(),
                    TradingAccount { account_id, available_quote, frozen_quote, version },
                )
            })
            .collect::<HashMap<_, _>>();

        let order_rows: Vec<(String, String, String, u64, u64, u64)> = conn
            .query(format!(
                "SELECT order_id, account_id, symbol, qty, price, reserved_quote FROM {ORDER_TABLE}"
            ))
            .map_err(map_mysql_error)?;
        let orders = order_rows
            .into_iter()
            .map(|(order_id, account_id, symbol, qty, price, reserved_quote)| {
                (
                    order_id.clone(),
                    StoredOrder { order_id, account_id, symbol, qty, price, reserved_quote },
                )
            })
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
            accounts,
            orders,
            persisted_event_count,
            published_event_count,
            next_order_sequence,
        })
    }
}

pub(crate) fn map_mysql_error(_error: mysql::Error) -> StoreError {
    StoreError::StoreUnavailable
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
