use crate::{BlockStateChanges, Receipt, VmCapability, VmKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmExecutionOutput {
    pub vm_kind: VmKind,
    pub capability: VmCapability,
    pub state_changes: BlockStateChanges,
    pub receipts: Vec<Receipt>,
    pub gas_used: u64,
    pub product_events: Vec<ProductEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductEvent {
    pub product_type: String,
    pub event_type: String,
    pub payload: Vec<u8>,
}
