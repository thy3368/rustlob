use alloy_primitives::{Address, B256, BlockNumber};

use crate::BlockBody;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<Tx> {
    pub header: BlockHeader,
    pub body: BlockBody<Tx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub parent_hash: B256,
    pub transactions_root: B256,
    pub receipts_root: B256,
    pub state_root: B256,
    pub beneficiary: Address,
    pub number: BlockNumber,
    pub timestamp: u64,
}
