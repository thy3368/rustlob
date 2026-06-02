use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::CommandUseCaseOutbound;
use example_core::{
    ACCOUNT_ENTITY_TYPE, MarketRules, ORDER_ENTITY_TYPE, PlaceOrderCmd, PlaceOrderState,
    TradingAccount,
};
use mysql::params;
use mysql::prelude::Queryable;

use crate::shared::{
    ACCOUNT_TABLE, EVENT_TABLE, MARKET_RULES_TABLE, MySqlStore, ORDER_TABLE,
    PlaceOrderOutboundError, StoreSnapshot, event_order_sequence, event_string_field_mysql,
    event_u64_field_mysql, map_mysql_error,
};

#[derive(Debug, Clone)]
pub struct MySqlPlaceOrderOutbound {
    store: MySqlStore,
}

impl MySqlPlaceOrderOutbound {
    pub fn new(database_url: &str) -> Result<Self, crate::StoreError> {
        Ok(Self::from_store(MySqlStore::new(database_url)?))
    }

    pub fn from_store(store: MySqlStore) -> Self {
        Self { store }
    }

    pub fn ensure_schema(&self) -> Result<(), crate::StoreError> {
        self.store.ensure_schema()
    }

    pub fn seed_account(&self, account: TradingAccount) -> Result<(), crate::StoreError> {
        self.store.seed_account(account)
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), crate::StoreError> {
        self.store.seed_market_rules(market_rules)
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, crate::StoreError> {
        self.store.snapshot()
    }
}

impl CommandUseCaseOutbound for MySqlPlaceOrderOutbound {
    type Command = PlaceOrderCmd;
    type State = PlaceOrderState;
    type Error = PlaceOrderOutboundError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let mut conn = self.store.pool.get_conn().map_err(map_mysql_error)?;

        let account_row: Option<(String, u64, u64, u64)> = conn
            .exec_first(
                format!(
                    "SELECT account_id, available_quote, frozen_quote, version
                     FROM {ACCOUNT_TABLE}
                     WHERE account_id = :account_id"
                ),
                params! { "account_id" => cmd.party_id.as_str() },
            )
            .map_err(map_mysql_error)?;
        let (account_id, available_quote, frozen_quote, version) =
            account_row.ok_or(PlaceOrderOutboundError::AccountNotFound)?;

        let market_rules_row: Option<(String, u64)> = conn
            .exec_first(
                format!(
                    "SELECT symbol, min_qty
                     FROM {MARKET_RULES_TABLE}
                     WHERE symbol = :symbol"
                ),
                params! { "symbol" => cmd.symbol.as_str() },
            )
            .map_err(map_mysql_error)?;
        let (symbol, min_qty) =
            market_rules_row.ok_or(PlaceOrderOutboundError::MarketRulesNotFound)?;

        let next_order_sequence = conn
            .query_first::<u64, _>(format!(
                "SELECT COALESCE(MAX(created_sequence), 0) + 1 FROM {ORDER_TABLE}"
            ))
            .map_err(map_mysql_error)?
            .unwrap_or(1);

        Ok(PlaceOrderState {
            trading_enabled: true,
            next_order_sequence,
            account: TradingAccount { account_id, available_quote, frozen_quote, version },
            market_rules: MarketRules { symbol, min_qty },
        })
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut conn = self.store.pool.get_conn().map_err(map_mysql_error)?;

        for event in events {
            conn.exec_drop(
                format!(
                    "INSERT INTO {EVENT_TABLE} (
                        entity_type, change_type, entity_id, old_version, new_version,
                        order_id, account_id, symbol, qty, price, reserved_quote, available_quote, frozen_quote
                     ) VALUES (
                        :entity_type, :change_type, :entity_id, :old_version, :new_version,
                        :order_id, :account_id, :symbol, :qty, :price, :reserved_quote, :available_quote, :frozen_quote
                     )"
                ),
                params! {
                    "entity_type" => event.entity_type,
                    "change_type" => event.change_type,
                    "entity_id" => event.entity_id,
                    "old_version" => event.old_version,
                    "new_version" => event.new_version,
                    "order_id" => event_string_field_mysql(event, "order_id"),
                    "account_id" => event_string_field_mysql(event, "account_id"),
                    "symbol" => event_string_field_mysql(event, "symbol"),
                    "qty" => event_u64_field_mysql(event, "qty"),
                    "price" => event_u64_field_mysql(event, "price"),
                    "reserved_quote" => event_u64_field_mysql(event, "reserved_quote"),
                    "available_quote" => event_u64_field_mysql(event, "available_quote"),
                    "frozen_quote" => event_u64_field_mysql(event, "frozen_quote"),
                },
            )
            .map_err(map_mysql_error)?;
        }

        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut conn = self.store.pool.get_conn().map_err(map_mysql_error)?;

        for event in events {
            if event.entity_type == ORDER_ENTITY_TYPE && event.is_created() {
                conn.exec_drop(
                    format!(
                        "INSERT INTO {ORDER_TABLE} (
                            order_id, account_id, symbol, qty, price, reserved_quote, created_sequence
                         ) VALUES (
                            :order_id, :account_id, :symbol, :qty, :price, :reserved_quote, :created_sequence
                         )
                         ON DUPLICATE KEY UPDATE
                            account_id = VALUES(account_id),
                            symbol = VALUES(symbol),
                            qty = VALUES(qty),
                            price = VALUES(price),
                            reserved_quote = VALUES(reserved_quote),
                            created_sequence = VALUES(created_sequence)"
                    ),
                    params! {
                        "order_id" => event_string_field_mysql(event, "order_id")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "account_id" => event_string_field_mysql(event, "account_id")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "symbol" => event_string_field_mysql(event, "symbol")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "qty" => event_u64_field_mysql(event, "qty")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "price" => event_u64_field_mysql(event, "price")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "reserved_quote" => event_u64_field_mysql(event, "reserved_quote")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "created_sequence" => event_order_sequence(event)
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    },
                )
                .map_err(map_mysql_error)?;
                continue;
            }

            if event.entity_type == ACCOUNT_ENTITY_TYPE && event.is_updated() {
                conn.exec_drop(
                    format!(
                        "UPDATE {ACCOUNT_TABLE}
                         SET available_quote = :available_quote,
                             frozen_quote = :frozen_quote,
                             version = :version
                         WHERE account_id = :account_id"
                    ),
                    params! {
                        "account_id" => event_string_field_mysql(event, "account_id")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "available_quote" => event_u64_field_mysql(event, "available_quote")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "frozen_quote" => event_u64_field_mysql(event, "frozen_quote")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "version" => event.new_version,
                    },
                )
                .map_err(map_mysql_error)?;
            }
        }

        Ok(())
    }

    fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut conn = self.store.pool.get_conn().map_err(map_mysql_error)?;
        conn.query_drop(format!(
            "UPDATE {EVENT_TABLE}
             SET published_at = CURRENT_TIMESTAMP
             WHERE published_at IS NULL"
        ))
        .map_err(map_mysql_error)?;
        Ok(())
    }
}
