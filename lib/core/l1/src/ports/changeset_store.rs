use crate::{AccountChangeSet, CodeChangeSet, StorageChangeSet};

pub trait ChangeSetStore {
    type Error;

    fn account_changes(&self, block_number: u64) -> Result<Vec<AccountChangeSet>, Self::Error>;
    fn storage_changes(&self, block_number: u64) -> Result<Vec<StorageChangeSet>, Self::Error>;
    fn code_changes(&self, block_number: u64) -> Result<Vec<CodeChangeSet>, Self::Error>;
}
