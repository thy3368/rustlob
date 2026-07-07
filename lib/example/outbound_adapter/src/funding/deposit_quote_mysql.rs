use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCaseOutbound;
use example_core::{Balance, DepositQuoteCmd, DepositQuoteState};
use mysql::params;
use mysql::prelude::Queryable;

use crate::shared::{
    ACCOUNT_TABLE, DepositQuoteOutboundError, EVENT_TABLE, MySqlStore, event_string_field_mysql,
    event_u64_field_mysql, map_mysql_error,
};

#[derive(Debug, Clone)]
pub struct MySqlDepositQuoteOutbound {
    store: MySqlStore,
}

impl MySqlDepositQuoteOutbound {
    pub fn from_store(store: MySqlStore) -> Self {
        Self { store }
    }
}

impl CommandUseCaseOutbound for MySqlDepositQuoteOutbound {
    type Command = DepositQuoteCmd;
    type State = DepositQuoteState;
    type Error = DepositQuoteOutboundError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let mut conn = self.store.pool.get_conn().map_err(map_mysql_error)?;

        let account_row: Option<(String, u64, u64)> = conn
            .exec_first(
                format!(
                    "SELECT account_id, available_quote, version
                     FROM {ACCOUNT_TABLE}
                     WHERE account_id = :account_id"
                ),
                params! { "account_id" => cmd.party_id.as_str() },
            )
            .map_err(map_mysql_error)?;
        let (account_id, available_quote, version) =
            account_row.ok_or(DepositQuoteOutboundError::BalanceNotFound)?;

        Ok(DepositQuoteState {
            quote_balance: Balance {
                account_id,
                asset_id: "USDT".to_string(),
                available: available_quote,
                frozen: 0,
                entry_notional: None,
                identifier: None,
                version,
            },
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
            if event.is_updated() {
                conn.exec_drop(
                    format!(
                        "UPDATE {ACCOUNT_TABLE}
                         SET available_quote = COALESCE(:available_quote, available_quote),
                             frozen_quote = COALESCE(:frozen_quote, frozen_quote),
                             version = :version
                         WHERE account_id = :account_id"
                    ),
                    params! {
                        "account_id" => event_string_field_mysql(event, "account_id")
                            .ok_or(DepositQuoteOutboundError::EventDecodeFailed)?,
                        "available_quote" => event_u64_field_mysql(event, "available"),
                        "frozen_quote" => event_u64_field_mysql(event, "frozen"),
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
