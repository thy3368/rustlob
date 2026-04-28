use alloy_primitives::{Address, B256};

use crate::{Account, CodeBlob, StorageKey, StorageValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountDelta {
    pub address: Address,
    pub previous: Option<Account>,
    pub current: Option<Account>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageDelta {
    pub address: Address,
    pub key: StorageKey,
    pub previous: StorageValue,
    pub current: StorageValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeDelta {
    pub code_hash: B256,
    pub previous: Option<CodeBlob>,
    pub current: Option<CodeBlob>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockStateChanges {
    pub account_deltas: Vec<AccountDelta>,
    pub storage_deltas: Vec<StorageDelta>,
    pub code_deltas: Vec<CodeDelta>,
}
