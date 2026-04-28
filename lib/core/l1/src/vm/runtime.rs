use crate::{VmExecutionInput, VmExecutionOutput};

pub trait VmRuntime<Tx> {
    type Error;

    fn execute(&self, input: VmExecutionInput<Tx>) -> Result<VmExecutionOutput, Self::Error>;
}
