use alloy_primitives::B256;

use crate::VmKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeBlob {
    pub code_hash: B256,
    pub vm_kind: VmKind,
    pub bytes: Vec<u8>,
}
