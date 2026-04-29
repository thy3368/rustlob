#[cfg(test)]
mod tests {
    use alloy_primitives::{Address, B256, U256};
    use db_repo::StorageError;
    use crate::MdbxStateStore;
    use l1_core::{
        Account, AccountDelta, BlockStateChanges, CodeBlob, CodeDelta, StateReader, StorageDelta,
        VmKind,
    };


    fn temp_path(name: &str) -> std::path::PathBuf {
        let nanos =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("rustlob-{}-{}", name, nanos))
    }

    #[test]
    fn applies_block_state_changes_to_plain_state() -> Result<(), StorageError> {
        let path = temp_path("l1-state-store");
        let mut store = MdbxStateStore::open(&path)?;
        let address = Address::repeat_byte(0x11);
        let storage_key = B256::repeat_byte(0x22);
        let storage_value = B256::repeat_byte(0x33);
        let code_hash = B256::repeat_byte(0x44);

        let account = Account {
            nonce: 7,
            balance: U256::from(99u64),
            code_hash,
            storage_root: B256::repeat_byte(0x55),
            vm_kind: VmKind::Evm,
        };

        let code = CodeBlob { code_hash, vm_kind: VmKind::Evm, bytes: vec![1, 2, 3, 4] };

        let changes = BlockStateChanges {
            account_deltas: vec![AccountDelta {
                address,
                previous: None,
                current: Some(account.clone()),
            }],
            storage_deltas: vec![StorageDelta {
                address,
                key: storage_key,
                previous: B256::ZERO,
                current: storage_value,
            }],
            code_deltas: vec![CodeDelta { code_hash, previous: None, current: Some(code.clone()) }],
        };

        store.apply_block_state_changes(1, &changes)?;

        assert_eq!(store.account(address)?, Some(account));
        assert_eq!(store.storage(address, storage_key)?, storage_value);
        assert_eq!(l1_core::CodeStore::code(&store, code_hash)?, Some(code));
        Ok(())
    }
}
