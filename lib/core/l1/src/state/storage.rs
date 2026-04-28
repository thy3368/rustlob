use alloy_primitives::{Address, B256};

pub type StorageKey = B256;
pub type StorageValue = B256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageSlot {
    pub address: Address,
    pub key: StorageKey,
    pub value: StorageValue,
}
