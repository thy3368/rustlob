#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VmKind {
    Evm,
    RustVm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmExecutionInput<Tx> {
    pub transaction: Tx,
}
