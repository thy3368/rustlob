use std::sync::Arc;

use l1_adapter::EvmRuntimeAdapter;
use l1_core::{VmKind, VmRegistry};

use crate::adapter::rust_vm_runtime::RustVmRuntimeAdapter;

pub fn default_execute_and_commit_block_use_case() -> l1_core::ExecuteAndCommitBlockUseCase {
    let mut registry = VmRegistry::new();
    registry.register_runtime(VmKind::RustVm, Arc::new(RustVmRuntimeAdapter::new()));
    registry.register_runtime(VmKind::Evm, Arc::new(EvmRuntimeAdapter::new()));
    l1_core::ExecuteAndCommitBlockUseCase::with_vm_registry(registry)
}
