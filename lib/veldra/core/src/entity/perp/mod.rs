use std::collections::BTreeMap;

use example_core::{
    Balance, HyperliquidPerpFundingSettlement, HyperliquidPerpOrder, HyperliquidPerpPosition,
};

use crate::entity::{AccountAssetKey, AccountMarketKey, stable_hash_hex};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PerpState {
    pub orders: BTreeMap<String, HyperliquidPerpOrder>,
    pub positions: BTreeMap<AccountMarketKey, HyperliquidPerpPosition>,
    pub balances: BTreeMap<AccountAssetKey, Balance>,
    pub funding_settlements: BTreeMap<String, HyperliquidPerpFundingSettlement>,
    pub trading_enabled_by_market: BTreeMap<String, bool>,
    pub next_order_sequence_by_account: BTreeMap<String, u64>,
}

impl PerpState {
    pub fn commitment(&self) -> String {
        let orders = self.orders.values().map(perp_order_commitment).collect::<Vec<_>>();
        let positions = self.positions.values().map(perp_position_commitment).collect::<Vec<_>>();
        let balances = self.balances.values().map(balance_commitment).collect::<Vec<_>>();
        let funding = self
            .funding_settlements
            .values()
            .map(perp_funding_settlement_commitment)
            .collect::<Vec<_>>();
        let runtime = self
            .trading_enabled_by_market
            .iter()
            .map(|(market, enabled)| format!("{market}:{enabled}"))
            .collect::<Vec<_>>();
        let sequences = self
            .next_order_sequence_by_account
            .iter()
            .map(|(account_id, sequence)| format!("{account_id}:{sequence}"))
            .collect::<Vec<_>>();

        stable_hash_hex(&[
            stable_hash_hex(&orders).as_str(),
            stable_hash_hex(&positions).as_str(),
            stable_hash_hex(&balances).as_str(),
            stable_hash_hex(&funding).as_str(),
            stable_hash_hex(&runtime).as_str(),
            stable_hash_hex(&sequences).as_str(),
        ])
    }
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

fn perp_order_commitment(order: &HyperliquidPerpOrder) -> String {
    stable_hash_hex(&[format!("{order:?}")])
}

fn perp_position_commitment(position: &HyperliquidPerpPosition) -> String {
    stable_hash_hex(&[format!("{position:?}")])
}

fn perp_funding_settlement_commitment(settlement: &HyperliquidPerpFundingSettlement) -> String {
    stable_hash_hex(&[format!("{settlement:?}")])
}
