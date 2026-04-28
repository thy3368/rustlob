use alloy_primitives::{B256, U256};

use crate::VmKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub nonce: u64,
    pub balance: U256,
    pub code_hash: B256,
    pub storage_root: B256,
    pub vm_kind: VmKind,
}
