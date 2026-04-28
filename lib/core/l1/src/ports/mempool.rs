use alloy_primitives::Address;

use crate::PendingRequest;

pub trait MempoolPort: Send + Sync {
    fn add_requests(&self, requests: Vec<PendingRequest>) -> Result<(), MempoolError>;

    fn fetch_requests(&self, limit: usize) -> Result<Vec<PendingRequest>, MempoolError>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn clear(&self);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MempoolError {
    PoolFull,
    StorageError(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolEntry {
    pub request: PendingRequest,
    pub sender: Option<Address>,
}
