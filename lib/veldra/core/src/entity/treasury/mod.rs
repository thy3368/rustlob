use std::collections::{BTreeMap, BTreeSet};

use example_core::Balance;

use crate::entity::{AccountAssetKey, stable_hash_hex};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreasuryState {
    pub balances: BTreeMap<AccountAssetKey, Balance>,
    pub processed_transfer_ids: BTreeSet<String>,
    pub processed_withdraw_ids: BTreeSet<String>,
    pub withdraw_locks: BTreeMap<AccountAssetKey, WithdrawLockState>,
}

impl TreasuryState {
    pub fn commitment(&self) -> String {
        let balances = self.balances.values().map(balance_commitment).collect::<Vec<_>>();
        let transfers = self.processed_transfer_ids.iter().cloned().collect::<Vec<_>>();
        let withdraws = self.processed_withdraw_ids.iter().cloned().collect::<Vec<_>>();
        let locks =
            self.withdraw_locks.values().map(WithdrawLockState::commitment).collect::<Vec<_>>();

        stable_hash_hex(&[
            stable_hash_hex(&balances).as_str(),
            stable_hash_hex(&transfers).as_str(),
            stable_hash_hex(&withdraws).as_str(),
            stable_hash_hex(&locks).as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WithdrawLockState {
    pub account_id: String,
    pub asset: String,
    pub locked_amount: u64,
    pub reason: String,
    pub version: u64,
}

impl WithdrawLockState {
    fn commitment(&self) -> String {
        stable_hash_hex(&[
            self.account_id.as_str(),
            self.asset.as_str(),
            self.locked_amount.to_string().as_str(),
            self.reason.as_str(),
            self.version.to_string().as_str(),
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
