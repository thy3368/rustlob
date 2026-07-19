use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::MiFamilyOutbound;
use example_core::{
    Balance, MarketRules, ORDER_ENTITY_TYPE, PlaceSpotOrderV2TakerTemplateContextV3,
    SpotOrderV2CommandV3, SpotOrderV2GivenStateV3, SpotOrderV2UseCaseFamilyV3,
    build_place_spot_order_v2_taker_template_v3,
};
use mysql::params;
use mysql::prelude::Queryable;

use super::place_order_in_memory::{base_asset_id_for, quote_asset_id_for, symbol_for_asset};
use crate::shared::{
    ACCOUNT_TABLE, EVENT_TABLE, MARKET_RULES_TABLE, MySqlStore, ORDER_TABLE,
    PlaceOrderOutboundError, StoreSnapshot, event_string_field_mysql, event_u64_field_mysql,
    map_mysql_error,
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

    pub fn seed_balances(
        &self,
        base_account_id: &str,
        available_base: u64,
        frozen_base: u64,
        available_quote: u64,
        frozen_quote: u64,
        version: u64,
    ) -> Result<(), crate::StoreError> {
        self.store.seed_account_row(
            base_account_id,
            available_base,
            frozen_base,
            available_quote,
            frozen_quote,
            version,
        )
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), crate::StoreError> {
        self.store.seed_market_rules(market_rules)
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, crate::StoreError> {
        self.store.snapshot()
    }
}

const DEFAULT_FEE_ACCOUNT_ID: &str = "fee";
const DEFAULT_MAKER_FEE_BPS: u64 = 5;
const DEFAULT_TAKER_FEE_BPS: u64 = 10;

impl MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3> for MySqlPlaceOrderOutbound {
    type Error = PlaceOrderOutboundError;

    fn load_given_state(
        &self,
        cmd: &SpotOrderV2CommandV3,
    ) -> Result<SpotOrderV2GivenStateV3, Self::Error> {
        let SpotOrderV2CommandV3::Place(cmd) = cmd else {
            return Err(PlaceOrderOutboundError::UnsupportedCommandBranch);
        };
        let mut conn = self.store.pool.get_conn().map_err(map_mysql_error)?;
        let requested_symbol = symbol_for_asset(cmd.asset);

        let account_row: Option<(String, u64, u64, u64, u64, u64)> = conn
            .exec_first(
                format!(
                    "SELECT account_id, available_base, frozen_base, available_quote, frozen_quote, version
                     FROM {ACCOUNT_TABLE}
                     WHERE account_id = :account_id"
                ),
                params! { "account_id" => cmd.party_id.as_str() },
            )
            .map_err(map_mysql_error)?;
        let (account_id, available_base, frozen_base, available_quote, frozen_quote, version) =
            account_row.ok_or(PlaceOrderOutboundError::BalanceNotFound)?;

        let market_rules_row: Option<(String, u64)> = conn
            .exec_first(
                format!(
                    "SELECT symbol, min_qty
                     FROM {MARKET_RULES_TABLE}
                     WHERE symbol = :symbol"
                ),
                params! { "symbol" => requested_symbol },
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

        let base_asset_id = base_asset_id_for(symbol.as_str()).to_string();
        let quote_asset_id = quote_asset_id_for(symbol.as_str()).to_string();
        let settlement_balances = vec![
            Balance::new(
                account_id.clone(),
                base_asset_id.clone(),
                available_base,
                frozen_base,
                version,
            ),
            Balance::new(
                account_id.clone(),
                quote_asset_id.clone(),
                available_quote,
                frozen_quote,
                version,
            ),
            Balance::new(DEFAULT_FEE_ACCOUNT_ID.to_string(), quote_asset_id.clone(), 0, 0, 1),
        ];
        let order_id = format!("{}-{}-{}", cmd.party_id, symbol, next_order_sequence);
        let taker_order = build_place_spot_order_v2_taker_template_v3(
            cmd,
            PlaceSpotOrderV2TakerTemplateContextV3 {
                order_id,
                symbol,
                settlement_balances: &settlement_balances,
                base_asset_id: base_asset_id.clone(),
                quote_asset_id: quote_asset_id.clone(),
                maker_fee_bps: DEFAULT_MAKER_FEE_BPS,
                taker_fee_bps: DEFAULT_TAKER_FEE_BPS,
            },
        )?;

        Ok(SpotOrderV2GivenStateV3::Place {
            taker_order,
            maker_orders: Vec::new(),
            settlement_balances,
            base_asset_id,
            quote_asset_id,
            fee_account_id: DEFAULT_FEE_ACCOUNT_ID.to_string(),
            maker_fee_bps: DEFAULT_MAKER_FEE_BPS,
            taker_fee_bps: DEFAULT_TAKER_FEE_BPS,
        })
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut conn = self.store.pool.get_conn().map_err(map_mysql_error)?;

        for event in events {
            conn.exec_drop(
                format!(
                    "INSERT INTO {EVENT_TABLE} (
                        entity_type, change_type, entity_id, old_version, new_version,
                        order_id, account_id, asset, symbol, side, execution, time_in_force,
                        qty, price, reserved_base, reserved_quote,
                        available_base, frozen_base, available_quote, frozen_quote
                     ) VALUES (
                        :entity_type, :change_type, :entity_id, :old_version, :new_version,
                        :order_id, :account_id, :asset, :symbol, :side, :execution, :time_in_force,
                        :qty, :price, :reserved_base, :reserved_quote,
                        :available_base, :frozen_base, :available_quote, :frozen_quote
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
                    "asset" => event_u64_field_mysql(event, "asset"),
                    "symbol" => event_string_field_mysql(event, "symbol"),
                    "side" => event_string_field_mysql(event, "side"),
                    "execution" => event_string_field_mysql(event, "execution"),
                    "time_in_force" => event_string_field_mysql(event, "time_in_force"),
                    "qty" => event_u64_field_mysql(event, "qty"),
                    "price" => event_u64_field_mysql(event, "price"),
                    "reserved_base" => event_u64_field_mysql(event, "reserved_base"),
                    "reserved_quote" => event_u64_field_mysql(event, "reserved_quote"),
                    "available_base" => event_u64_field_mysql(event, "available"),
                    "frozen_base" => event_u64_field_mysql(event, "frozen"),
                    "available_quote" => event_u64_field_mysql(event, "available"),
                    "frozen_quote" => event_u64_field_mysql(event, "frozen"),
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
                            order_id, account_id, asset, symbol, side, execution, time_in_force,
                            qty, price, reserved_base, reserved_quote, created_sequence
                         ) VALUES (
                            :order_id, :account_id, :asset, :symbol, :side, :execution, :time_in_force,
                            :qty, :price, :reserved_base, :reserved_quote, :created_sequence
                         )
                         ON DUPLICATE KEY UPDATE
                            account_id = VALUES(account_id),
                            asset = VALUES(asset),
                            symbol = VALUES(symbol),
                            side = VALUES(side),
                            execution = VALUES(execution),
                            time_in_force = VALUES(time_in_force),
                            qty = VALUES(qty),
                            price = VALUES(price),
                            reserved_base = VALUES(reserved_base),
                            reserved_quote = VALUES(reserved_quote),
                            created_sequence = VALUES(created_sequence)"
                    ),
                    params! {
                        "order_id" => event_string_field_mysql(event, "order_id")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "account_id" => event_string_field_mysql(event, "account_id")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "asset" => event_u64_field_mysql(event, "asset")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "symbol" => event_string_field_mysql(event, "symbol")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "side" => event_string_field_mysql(event, "side")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "execution" => event_string_field_mysql(event, "execution")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "time_in_force" => event_string_field_mysql(event, "time_in_force")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "qty" => event_u64_field_mysql(event, "qty")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "price" => event_u64_field_mysql(event, "price")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "reserved_base" => event_u64_field_mysql(event, "reserved_base")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "reserved_quote" => event_u64_field_mysql(event, "reserved_quote")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "created_sequence" => 0_u64,
                    },
                )
                .map_err(map_mysql_error)?;
                continue;
            }

            if event.is_updated() {
                let asset_id = event_string_field_mysql(event, "asset_id")
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
                let (available_column, frozen_column) = if asset_id == "BTC" {
                    ("available_base", "frozen_base")
                } else {
                    ("available_quote", "frozen_quote")
                };
                conn.exec_drop(
                    format!(
                        "UPDATE {ACCOUNT_TABLE}
                         SET {available_column} = COALESCE(:available_value, {available_column}),
                             {frozen_column} = COALESCE(:frozen_value, {frozen_column}),
                             version = :version
                         WHERE account_id = :account_id"
                    ),
                    params! {
                        "account_id" => event_string_field_mysql(event, "account_id")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                        "available_value" => event_u64_field_mysql(event, "available"),
                        "frozen_value" => event_u64_field_mysql(event, "frozen"),
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
