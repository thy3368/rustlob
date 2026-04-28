use crate::{BlockStateChanges, ProductEvent};

pub trait ProductProjectionUpdater {
    type Error;

    fn apply(
        &mut self,
        block_number: u64,
        state_changes: &BlockStateChanges,
        product_events: &[ProductEvent],
    ) -> Result<(), Self::Error>;

    fn revert(&mut self, block_number: u64) -> Result<(), Self::Error>;
}
