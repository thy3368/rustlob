use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderPlacedMessage {
    pub order_id: String,
    pub party_id: String,
    pub symbol: String,
    pub asset: u32,
    pub match_id: String,
    pub trace_id: Option<String>,
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotTradeMatchedMessage {
    pub trade_ids: Vec<String>,
    pub party_id: String,
    pub match_id: String,
    pub settlement_batch_id: String,
    pub trace_id: Option<String>,
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotPipelineMessage {
    SpotOrderPlaced(SpotOrderPlacedMessage),
    SpotTradeMatched(SpotTradeMatchedMessage),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotPipelineBrokerError {
    #[error("broker unavailable")]
    Unavailable,
}

pub trait SpotPipelineBroker: Send + Sync {
    fn publish(&self, message: SpotPipelineMessage) -> Result<(), SpotPipelineBrokerError>;
    fn pop(&self) -> Result<Option<SpotPipelineMessage>, SpotPipelineBrokerError>;
    fn len(&self) -> Result<usize, SpotPipelineBrokerError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemorySpotPipelineBroker {
    queue: Arc<Mutex<VecDeque<SpotPipelineMessage>>>,
}

impl SpotPipelineBroker for InMemorySpotPipelineBroker {
    fn publish(&self, message: SpotPipelineMessage) -> Result<(), SpotPipelineBrokerError> {
        let mut queue = self.queue.lock().map_err(|_| SpotPipelineBrokerError::Unavailable)?;
        queue.push_back(message);
        Ok(())
    }

    fn pop(&self) -> Result<Option<SpotPipelineMessage>, SpotPipelineBrokerError> {
        let mut queue = self.queue.lock().map_err(|_| SpotPipelineBrokerError::Unavailable)?;
        Ok(queue.pop_front())
    }

    fn len(&self) -> Result<usize, SpotPipelineBrokerError> {
        let queue = self.queue.lock().map_err(|_| SpotPipelineBrokerError::Unavailable)?;
        Ok(queue.len())
    }
}
