use std::collections::HashMap;
use std::sync::Arc;

use crate::{VmExecutionInput, VmExecutionOutput, VmKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmRuntimeError {
    UnregisteredVmKind(VmKind),
    UnsupportedCapability { vm_kind: VmKind, capability: String },
    ExecutionFailed(String),
}

pub trait VmRuntime<Tx>: Send + Sync {
    fn execute(&self, input: VmExecutionInput<Tx>) -> Result<VmExecutionOutput, VmRuntimeError>;
}

pub trait VmRuntimeResolver<Tx>: Send + Sync {
    fn execute(&self, input: VmExecutionInput<Tx>) -> Result<VmExecutionOutput, VmRuntimeError>;
}

#[derive(Default)]
pub struct VmRegistry<Tx> {
    runtimes: HashMap<VmKind, Arc<dyn VmRuntime<Tx>>>,
}

impl<Tx> VmRegistry<Tx> {
    pub fn new() -> Self {
        Self { runtimes: HashMap::new() }
    }

    pub fn register_runtime(
        &mut self,
        vm_kind: VmKind,
        runtime: Arc<dyn VmRuntime<Tx>>,
    ) -> Option<Arc<dyn VmRuntime<Tx>>> {
        self.runtimes.insert(vm_kind, runtime)
    }
}

impl<Tx> VmRuntimeResolver<Tx> for VmRegistry<Tx> {
    fn execute(&self, input: VmExecutionInput<Tx>) -> Result<VmExecutionOutput, VmRuntimeError> {
        let runtime = self
            .runtimes
            .get(&input.vm_kind)
            .ok_or(VmRuntimeError::UnregisteredVmKind(input.vm_kind))?;
        runtime.execute(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BlockStateChanges, ProductEvent, Receipt, VmCapability};
    use alloy_primitives::Bloom;

    struct StubRuntime;

    impl VmRuntime<String> for StubRuntime {
        fn execute(&self, input: VmExecutionInput<String>) -> Result<VmExecutionOutput, VmRuntimeError> {
            Ok(VmExecutionOutput {
                vm_kind: input.vm_kind,
                capability: input.capability.clone(),
                state_changes: BlockStateChanges::default(),
                receipts: vec![Receipt {
                    success: true,
                    cumulative_gas_used: 7,
                    logs: vec![],
                    bloom: Bloom::ZERO,
                }],
                gas_used: 7,
                product_events: vec![ProductEvent {
                    product_type: format!("{:?}", input.vm_kind),
                    event_type: input.capability.0,
                    payload: input.transaction.into_bytes(),
                }],
            })
        }
    }

    #[test]
    fn registry_dispatches_registered_runtime() {
        let mut registry = VmRegistry::new();
        registry.register_runtime(VmKind::RustVm, Arc::new(StubRuntime));

        let output = registry
            .execute(VmExecutionInput {
                vm_kind: VmKind::RustVm,
                capability: VmCapability::new("dex.prep.place_order"),
                transaction: "payload".to_string(),
            })
            .unwrap();

        assert_eq!(output.gas_used, 7);
        assert_eq!(output.product_events.len(), 1);
        assert_eq!(output.product_events[0].event_type, "dex.prep.place_order");
    }

    #[test]
    fn registry_rejects_unregistered_runtime() {
        let registry = VmRegistry::<String>::new();
        let error = registry
            .execute(VmExecutionInput {
                vm_kind: VmKind::Evm,
                capability: VmCapability::new("evm.deploy"),
                transaction: "payload".to_string(),
            })
            .unwrap_err();

        assert_eq!(error, VmRuntimeError::UnregisteredVmKind(VmKind::Evm));
    }
}
