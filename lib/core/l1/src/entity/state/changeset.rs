use alloy_primitives::{Address, B256};

use crate::{Account, CodeBlob, StorageKey, StorageValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountChangeSet {
    pub block_number: u64,
    pub address: Address,
    pub previous: Option<Account>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageChangeSet {
    pub block_number: u64,
    pub address: Address,
    pub key: StorageKey,
    pub previous: StorageValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeChangeSet {
    pub block_number: u64,
    pub code_hash: B256,
    pub previous: Option<CodeBlob>,
}
