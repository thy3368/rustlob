use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::handler::handler_update::{
    ChangeSet, CmdHandlerForUpdate, HandlerLatencyMetrics,
};
use db_repo::MySqlDbRepo;
use diff::ChangeLog;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;

#[derive(Debug, Clone)]
pub struct SettlementCmd {
    pub trades: Vec<SpotTrade>,
}

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

pub struct DefaultSettlementHandler {
    _balance_repo: Arc<MySqlDbRepo<Balance>>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl DefaultSettlementHandler {
    pub fn new(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            _balance_repo: balance_repo,
            event_publisher,
        }
    }

    fn settle_trade(&self, trade: &SpotTrade) -> Result<SettlementResult, SpotCmdErrorAny> {
        tracing::debug!(
            trade_id = %trade.trade_id,
            taker_order_id = %trade.taker_order_id,
            maker_order_id = %trade.maker_order_id,
            "Starting trade settlement"
        );

        let mut balance_logs = self.settle_taker_side(trade)?;
        balance_logs.extend(self.settle_maker_side(trade)?);

        tracing::info!(
            trade_id = %trade.trade_id,
            balance_log_count = balance_logs.len(),
            "Trade settlement completed"
        );

        Ok(SettlementResult::success(balance_logs))
    }

    fn publish_balance_logs(&self, logs: &[ChangeLog]) {
        if let Err(e) = self.event_publisher.publish_balance_logs(logs) {
            tracing::error!(error = ?e, "Failed to publish balance logs");
        }
    }
}

impl CmdHandlerForUpdate<SettlementCmd, (), SettlementResult, ChangeLog, SpotCmdErrorAny>
    for DefaultSettlementHandler
{
    fn pre_check_command(&self, _cmd: &SettlementCmd) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn load_state_set_for_update(&self, _cmd: &SettlementCmd) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &SettlementCmd,
        _state_set: &(),
    ) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn apply_command_and_collect_changes(
        &self,
        cmd: &SettlementCmd,
        _state_set: (),
    ) -> Result<ChangeSet<SettlementResult, ChangeLog>, SpotCmdErrorAny> {
        let mut balance_logs = Vec::new();
        for trade in &cmd.trades {
            let result = self.settle_trade(trade)?;
            balance_logs.extend(result.balance_logs);
        }

        Ok(ChangeSet {
            writes: SettlementResult::success(balance_logs.clone()),
            changelogs: balance_logs,
        })
    }

    fn persist_changelogs(&self, _changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn replay_changelogs_to_state(&self, _changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn publish_changelog(&self, changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        self.publish_balance_logs(changelogs);
        Ok(())
    }

    fn observe_latency(&self, metrics: &HandlerLatencyMetrics) {
        tracing::debug!(
            total_ns = metrics.total_ns,
            pre_check_ns = metrics.pre_check_ns,
            load_state_ns = metrics.load_state_ns,
            validate_in_lock_ns = metrics.validate_in_lock_ns,
            apply_changes_ns = metrics.apply_changes_ns,
            persist_changelogs_ns = metrics.persist_changelogs_ns,
            replay_changelogs_ns = metrics.replay_changelogs_ns,
            publish_changelog_ns = metrics.publish_changelog_ns,
            changelog_count = metrics.changelog_count,
            "settlement handler latency"
        );
    }
}

impl DefaultSettlementHandler {
    fn settle_taker_side(&self, _trade: &SpotTrade) -> Result<Vec<ChangeLog>, SpotCmdErrorAny> {
        tracing::warn!("Taker settlement not implemented yet");
        Ok(Vec::new())
    }

    fn settle_maker_side(&self, _trade: &SpotTrade) -> Result<Vec<ChangeLog>, SpotCmdErrorAny> {
        tracing::warn!("Maker settlement not implemented yet");
        Ok(Vec::new())
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
