use alloy_primitives::{Address, B256};

use crate::{Account, StorageKey, StorageValue};

pub trait StateReader {
    type Error;

    fn account(&self, address: Address) -> Result<Option<Account>, Self::Error>;
    fn storage(&self, address: Address, key: StorageKey) -> Result<StorageValue, Self::Error>;
}

pub trait StateWriter {
    type Error;

    fn set_account(
        &mut self,
        address: Address,
        account: Option<Account>,
    ) -> Result<(), Self::Error>;
    fn set_storage(
        &mut self,
        address: Address,
        key: StorageKey,
        value: StorageValue,
    ) -> Result<(), Self::Error>;
    fn set_storage_root(&mut self, address: Address, storage_root: B256)
    -> Result<(), Self::Error>;
}
