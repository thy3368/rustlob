use std::collections::{BTreeMap, BTreeSet};

use example_core::{Balance, MarketRules, SpotOrder, SpotOrderTimeInForce};

use crate::entity::{AccountAssetKey, stable_hash_hex};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpotState {
    pub market_rules_by_symbol: BTreeMap<String, MarketRules>,
    pub asset_pairs_by_symbol: BTreeMap<String, SpotAssetPair>,
    pub trading_enabled_by_symbol: BTreeMap<String, bool>,
    pub balances: BTreeMap<AccountAssetKey, Balance>,
    pub orders: BTreeMap<String, SpotOrder>,
    pub settled_trade_ids: BTreeSet<String>,
    pub next_order_sequence_by_account: BTreeMap<String, u64>,
}

impl SpotState {
    pub fn commitment(&self) -> String {
        let market_rules = self
            .market_rules_by_symbol
            .iter()
            .map(|(symbol, rules)| format!("{symbol}:{}", market_rules_commitment(rules)))
            .collect::<Vec<_>>();
        let asset_pairs = self
            .asset_pairs_by_symbol
            .iter()
            .map(|(symbol, pair)| format!("{symbol}:{}", pair.commitment()))
            .collect::<Vec<_>>();
        let runtime = self
            .trading_enabled_by_symbol
            .iter()
            .map(|(symbol, enabled)| format!("{symbol}:{enabled}"))
            .collect::<Vec<_>>();
        let balances = self.balances.values().map(balance_commitment).collect::<Vec<_>>();
        let orders = self.orders.values().map(spot_order_commitment).collect::<Vec<_>>();
        let settled = self.settled_trade_ids.iter().cloned().collect::<Vec<_>>();
        let sequences = self
            .next_order_sequence_by_account
            .iter()
            .map(|(account_id, sequence)| format!("{account_id}:{sequence}"))
            .collect::<Vec<_>>();

        stable_hash_hex(&[
            stable_hash_hex(&market_rules).as_str(),
            stable_hash_hex(&asset_pairs).as_str(),
            stable_hash_hex(&runtime).as_str(),
            stable_hash_hex(&balances).as_str(),
            stable_hash_hex(&orders).as_str(),
            stable_hash_hex(&settled).as_str(),
            stable_hash_hex(&sequences).as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotAssetPair {
    pub base_asset_id: String,
    pub quote_asset_id: String,
}

impl SpotAssetPair {
    pub fn new(base_asset_id: impl Into<String>, quote_asset_id: impl Into<String>) -> Self {
        Self { base_asset_id: base_asset_id.into(), quote_asset_id: quote_asset_id.into() }
    }

    fn commitment(&self) -> String {
        stable_hash_hex(&[self.base_asset_id.as_str(), self.quote_asset_id.as_str()])
    }
}

fn market_rules_commitment(rules: &MarketRules) -> String {
    stable_hash_hex(&[rules.symbol.as_str(), rules.min_qty.to_string().as_str()])
}

fn balance_commitment(balance: &Balance) -> String {
    stable_hash_hex(&[
        balance.account_id.as_str(),
        balance.asset_id.as_str(),
        balance.available.to_string().as_str(),
        balance.frozen.to_string().as_str(),
        balance.version.to_string().as_str(),
    ])
}

fn spot_order_commitment(order: &SpotOrder) -> String {
    let execution = match order.execution {
        example_core::SpotOrderExecution::Market { aggressive_price } => {
            format!("market:{aggressive_price}")
        }
        example_core::SpotOrderExecution::Limit { price } => format!("limit:{price}"),
    };
    let status_reason = order.status_reason.map(|value| value.as_str()).unwrap_or_default();
    stable_hash_hex(&[
        order.order_id.as_str(),
        order.asset.to_string().as_str(),
        order.exchange_oid.map(|value| value.to_string()).unwrap_or_default().as_str(),
        order.account_id.as_str(),
        order.symbol.as_str(),
        order.side.as_str(),
        execution.as_str(),
        spot_order_tif(order.time_in_force),
        order.qty.to_string().as_str(),
        order.filled_qty.to_string().as_str(),
        order.status.as_str(),
        status_reason,
        order.reserved_base.to_string().as_str(),
        order.reserved_quote.to_string().as_str(),
        order.client_order_id.as_deref().unwrap_or_default(),
        order.version.to_string().as_str(),
    ])
}

fn spot_order_tif(value: SpotOrderTimeInForce) -> &'static str {
    match value {
        SpotOrderTimeInForce::Gtc => "gtc",
        SpotOrderTimeInForce::Ioc => "ioc",
        SpotOrderTimeInForce::Alo => "alo",
    }
}
