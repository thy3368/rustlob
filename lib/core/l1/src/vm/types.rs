use crate::PendingRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VmCapability(pub String);

impl VmCapability {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<&str> for VmCapability {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VmKind {
    Evm,
    RustVm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmExecutionInput<Tx> {
    pub vm_kind: VmKind,
    pub capability: VmCapability,
    pub transaction: Tx,
}

impl VmExecutionInput<PendingRequest> {
    pub fn from_pending_request(
        vm_kind: VmKind,
        capability: impl Into<VmCapability>,
        transaction: PendingRequest,
    ) -> Self {
        Self { vm_kind, capability: capability.into(), transaction }
    }
}
