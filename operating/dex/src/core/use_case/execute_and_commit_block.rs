use std::sync::Arc;

use l1_core::{VmKind, VmRegistry};

use crate::adapter::evm_runtime::EvmRuntimeAdapter;
use crate::adapter::rust_vm_prep::RustVmPrepAdapter;

pub fn default_execute_and_commit_block_use_case() -> l1_core::ExecuteAndCommitBlockUseCase {
    let mut registry = VmRegistry::new();
    registry.register_runtime(VmKind::RustVm, Arc::new(RustVmPrepAdapter::new()));
    registry.register_runtime(VmKind::Evm, Arc::new(EvmRuntimeAdapter::new()));
    l1_core::ExecuteAndCommitBlockUseCase::with_vm_registry(registry)
}
