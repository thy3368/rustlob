use alloy_primitives::{Address, B256};
use db_repo::{KvStore, StorageError};
use l1_core::{
    Account, AccountChangeSet, BlockStateChanges, ChangeSetStore, CodeBlob, CodeChangeSet,
    CodeStore, StateReader, StateWriter, StorageChangeSet, StorageKey, StorageValue, VmKind,
};
use mdbx::MdbxKvStore;

pub struct MdbxStateStore {
    accounts: MdbxKvStore,
    storage: MdbxKvStore,
    codes: MdbxKvStore,
    account_changes: MdbxKvStore,
    storage_changes: MdbxKvStore,
    code_changes: MdbxKvStore,
}

impl MdbxStateStore {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, StorageError> {
        Ok(Self {
            accounts: MdbxKvStore::open(path.as_ref(), "l1_plain_accounts")?,
            storage: MdbxKvStore::open(path.as_ref(), "l1_plain_storage")?,
            codes: MdbxKvStore::open(path.as_ref(), "l1_code_store")?,
            account_changes: MdbxKvStore::open(path.as_ref(), "l1_account_changes")?,
            storage_changes: MdbxKvStore::open(path.as_ref(), "l1_storage_changes")?,
            code_changes: MdbxKvStore::open(path.as_ref(), "l1_code_changes")?,
        })
    }

    fn account_key(address: Address) -> [u8; 20] {
        address.0.0
    }

    fn storage_key(address: Address, key: StorageKey) -> [u8; 52] {
        let mut out = [0u8; 52];
        out[..20].copy_from_slice(&address.0.0);
        out[20..].copy_from_slice(key.as_slice());
        out
    }

    fn block_address_key(block_number: u64, address: Address) -> [u8; 28] {
        let mut out = [0u8; 28];
        out[..8].copy_from_slice(&block_number.to_be_bytes());
        out[8..].copy_from_slice(&address.0.0);
        out
    }

    fn block_storage_key(block_number: u64, address: Address, key: StorageKey) -> [u8; 60] {
        let mut out = [0u8; 60];
        out[..8].copy_from_slice(&block_number.to_be_bytes());
        out[8..28].copy_from_slice(&address.0.0);
        out[28..].copy_from_slice(key.as_slice());
        out
    }

    fn block_code_key(block_number: u64, code_hash: B256) -> [u8; 40] {
        let mut out = [0u8; 40];
        out[..8].copy_from_slice(&block_number.to_be_bytes());
        out[8..].copy_from_slice(code_hash.as_slice());
        out
    }

    fn decode_account(bytes: &[u8]) -> Option<Account> {
        if bytes.len() != 105 {
            return None;
        }

        let nonce = u64::from_be_bytes(bytes[0..8].try_into().ok()?);
        let balance = alloy_primitives::U256::from_be_slice(&bytes[8..40]);
        let code_hash = B256::from_slice(&bytes[40..72]);
        let storage_root = B256::from_slice(&bytes[72..104]);
        let vm_kind = match bytes[104] {
            0 => VmKind::Evm,
            1 => VmKind::RustVm,
            _ => return None,
        };

        Some(Account { nonce, balance, code_hash, storage_root, vm_kind })
    }

    fn encode_account(account: &Account) -> [u8; 105] {
        let mut out = [0u8; 105];
        out[0..8].copy_from_slice(&account.nonce.to_be_bytes());
        out[8..40].copy_from_slice(&account.balance.to_be_bytes::<32>());
        out[40..72].copy_from_slice(account.code_hash.as_slice());
        out[72..104].copy_from_slice(account.storage_root.as_slice());
        out[104] = match account.vm_kind {
            VmKind::Evm => 0,
            VmKind::RustVm => 1,
        };
        out
    }

    fn encode_code(code: &CodeBlob) -> Vec<u8> {
        let mut out = Vec::with_capacity(33 + code.bytes.len());
        out.push(match code.vm_kind {
            VmKind::Evm => 0,
            VmKind::RustVm => 1,
        });
        out.extend_from_slice(code.code_hash.as_slice());
        out.extend_from_slice(&code.bytes);
        out
    }

    fn decode_code(bytes: &[u8]) -> Option<CodeBlob> {
        if bytes.len() < 33 {
            return None;
        }

        let vm_kind = match bytes[0] {
            0 => VmKind::Evm,
            1 => VmKind::RustVm,
            _ => return None,
        };

        Some(CodeBlob {
            code_hash: B256::from_slice(&bytes[1..33]),
            vm_kind,
            bytes: bytes[33..].to_vec(),
        })
    }

    pub fn apply_block_state_changes(
        &mut self,
        block_number: u64,
        state_changes: &BlockStateChanges,
    ) -> Result<(), StorageError> {
        for delta in &state_changes.account_deltas {
            let change_key = Self::block_address_key(block_number, delta.address);
            let previous_root =
                delta.previous.as_ref().map(|account| account.storage_root).unwrap_or_default();
            self.account_changes.put(&change_key, previous_root.as_slice())?;
            self.set_account(delta.address, delta.current.clone())?;
        }

        for delta in &state_changes.storage_deltas {
            let change_key = Self::block_storage_key(block_number, delta.address, delta.key);
            self.storage_changes.put(&change_key, delta.previous.as_slice())?;
            self.set_storage(delta.address, delta.key, delta.current)?;
        }

        for delta in &state_changes.code_deltas {
            let change_key = Self::block_code_key(block_number, delta.code_hash);
            let previous_len =
                delta.previous.as_ref().map(|code| code.bytes.len() as u64).unwrap_or_default();
            self.code_changes.put(&change_key, &previous_len.to_be_bytes())?;
            if let Some(code) = &delta.current {
                self.put_code(code.clone())?;
            }
        }

        Ok(())
    }
}

impl StateReader for MdbxStateStore {
    type Error = StorageError;

    fn account(&self, address: Address) -> Result<Option<Account>, Self::Error> {
        let key = Self::account_key(address);
        let value = self.accounts.get(&key)?;
        Ok(value.as_deref().and_then(Self::decode_account))
    }

    fn storage(&self, address: Address, key: StorageKey) -> Result<StorageValue, Self::Error> {
        let key = Self::storage_key(address, key);
        let value = self.storage.get(&key)?;
        Ok(value
            .and_then(|bytes: Vec<u8>| bytes.get(..32).and_then(|slice| B256::try_from(slice).ok()))
            .unwrap_or_default())
    }
}

impl StateWriter for MdbxStateStore {
    type Error = StorageError;

    fn set_account(
        &mut self,
        address: Address,
        account: Option<Account>,
    ) -> Result<(), Self::Error> {
        let key = Self::account_key(address);
        match account {
            Some(account) => {
                let encoded = Self::encode_account(&account);
                self.accounts.put(&key, &encoded)
            }
            None => self.accounts.delete(&key),
        }
    }

    fn set_storage(
        &mut self,
        address: Address,
        key: StorageKey,
        value: StorageValue,
    ) -> Result<(), Self::Error> {
        let key = Self::storage_key(address, key);
        self.storage.put(&key, value.as_slice())
    }

    fn set_storage_root(
        &mut self,
        address: Address,
        storage_root: B256,
    ) -> Result<(), Self::Error> {
        let mut account = self.account(address)?.unwrap_or(Account {
            nonce: 0,
            balance: Default::default(),
            code_hash: Default::default(),
            storage_root: Default::default(),
            vm_kind: VmKind::Evm,
        });
        account.storage_root = storage_root;
        self.set_account(address, Some(account))
    }
}

impl CodeStore for MdbxStateStore {
    type Error = StorageError;

    fn code(&self, code_hash: B256) -> Result<Option<CodeBlob>, Self::Error> {
        let value = self.codes.get(code_hash.as_slice())?;
        Ok(value.as_deref().and_then(Self::decode_code))
    }

    fn put_code(&mut self, code: CodeBlob) -> Result<(), Self::Error> {
        let encoded = Self::encode_code(&code);
        self.codes.put(code.code_hash.as_slice(), &encoded)
    }
}

impl ChangeSetStore for MdbxStateStore {
    type Error = StorageError;

    fn account_changes(&self, block_number: u64) -> Result<Vec<AccountChangeSet>, Self::Error> {
        let key = Self::block_address_key(block_number, Address::ZERO);
        Ok(self.account_changes.get(&key)?.map(|_| Vec::new()).unwrap_or_default())
    }

    fn storage_changes(&self, block_number: u64) -> Result<Vec<StorageChangeSet>, Self::Error> {
        let key = Self::block_storage_key(block_number, Address::ZERO, B256::ZERO);
        Ok(self.storage_changes.get(&key)?.map(|_| Vec::new()).unwrap_or_default())
    }

    fn code_changes(&self, block_number: u64) -> Result<Vec<CodeChangeSet>, Self::Error> {
        let key = Self::block_code_key(block_number, B256::ZERO);
        Ok(self.code_changes.get(&key)?.map(|_| Vec::new()).unwrap_or_default())
    }
}
