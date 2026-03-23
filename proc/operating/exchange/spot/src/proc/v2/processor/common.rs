use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::{AccountId, AssetId};
use db_repo::MySqlDbRepo;
use diff::ChangeLog;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;

#[derive(Debug, Clone)]
pub struct SettlementResult {
    pub balance_logs: Vec<ChangeLog>,
    pub success: bool,
}

impl SettlementResult {
    pub fn success(balance_logs: Vec<ChangeLog>) -> Self {
        Self { balance_logs, success: true }
    }

    pub fn failure() -> Self {
        Self { balance_logs: Vec::new(), success: false }
    }
}

pub trait SettlementHandler: Send + Sync {
    fn reconstruct_trade(&self, trade_log: &ChangeLog) -> Result<SpotTrade, SpotCmdErrorAny>;
    fn settle_trade(&self, trade: &SpotTrade) -> Result<SettlementResult, SpotCmdErrorAny>;
    fn publish_balance_logs(&self, logs: &[ChangeLog]);
}

pub struct DefaultSettlementHandler {
    balance_repo: Arc<MySqlDbRepo<Balance>>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl DefaultSettlementHandler {
    pub fn new(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self { balance_repo, event_publisher }
    }
}

impl SettlementHandler for DefaultSettlementHandler {
    fn reconstruct_trade(&self, trade_log: &ChangeLog) -> Result<SpotTrade, SpotCmdErrorAny> {
        tracing::warn!(entity_id = %trade_log.entity_id(), "Trade reconstruction not implemented yet");
        Err(SpotCmdErrorAny::Common(CommonError::Internal {
            message: "Trade reconstruction not implemented".to_string(),
        }))
    }

    fn settle_trade(&self, trade: &SpotTrade) -> Result<SettlementResult, SpotCmdErrorAny> {
        tracing::debug!(
            trade_id = %trade.trade_id,
            taker_order_id = %trade.taker_order_id,
            maker_order_id = %trade.maker_order_id,
            "Starting trade settlement"
        );

        let mut balance_logs = Vec::new();
        let taker_logs = self.settle_taker_side(trade)?;
        balance_logs.extend(taker_logs);
        let maker_logs = self.settle_maker_side(trade)?;
        balance_logs.extend(maker_logs);

        tracing::info!(trade_id = %trade.trade_id, balance_log_count = balance_logs.len(), "Trade settlement completed");

        Ok(SettlementResult::success(balance_logs))
    }

    fn publish_balance_logs(&self, logs: &[ChangeLog]) {
        if let Err(e) = self.event_publisher.publish_balance_logs(logs) {
            tracing::error!(error = ?e, "Failed to publish balance logs");
        }
    }
}

impl DefaultSettlementHandler {
    fn settle_taker_side(&self, trade: &SpotTrade) -> Result<Vec<ChangeLog>, SpotCmdErrorAny> {
        tracing::warn!("Taker settlement not implemented yet");
        Ok(Vec::new())
    }

    fn settle_maker_side(&self, trade: &SpotTrade) -> Result<Vec<ChangeLog>, SpotCmdErrorAny> {
        tracing::warn!("Maker settlement not implemented yet");
        Ok(Vec::new())
    }

    #[inline]
    pub fn build_balance_id(&self, account_id: AccountId, asset_id: AssetId) -> String {
        format!("{}:{}", account_id.0, u32::from(asset_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_result() {
        let result = SettlementResult::success(vec![]);
        assert!(result.success);
        assert_eq!(result.balance_logs.len(), 0);

        let result = SettlementResult::failure();
        assert!(!result.success);
    }
}
