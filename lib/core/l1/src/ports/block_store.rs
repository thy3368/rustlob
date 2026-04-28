use alloy_primitives::{B256, BlockNumber};

use crate::{BlockBody, BlockHeader, Receipt};

pub trait BlockStore<Tx> {
    type Error;

    fn header(&self, block_number: BlockNumber) -> Result<Option<BlockHeader>, Self::Error>;
    fn body(&self, block_hash: B256) -> Result<Option<BlockBody<Tx>>, Self::Error>;
    fn receipts(&self, block_hash: B256) -> Result<Vec<Receipt>, Self::Error>;
}
