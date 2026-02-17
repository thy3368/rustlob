use std::sync::Arc;

use diff::ChangeLogEntry;
use push::k_line::k_line_service::KLineBehaviorV2Imp;
use push::push::push_service::PushBehaviorV2Imp;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

pub struct SThreadSpotTradePipeline {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    push_service: Arc<PushBehaviorV2Imp>,
    kline_service: Arc<KLineBehaviorV2Imp>,
}

impl SThreadSpotTradePipeline {
    pub fn new(
        trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
        push_service: Arc<PushBehaviorV2Imp>,
        kline_service: Arc<KLineBehaviorV2Imp>,
    ) -> Self {
        Self { trade_behavior, push_service, kline_service }
    }

    pub fn handle_new_order(&self, cmd: NewOrderCmd) -> Result<(), SpotCmdErrorAny> {
        // Step 1: Order acquisition
        let (balance_change_log, order_change_log) =
            self.trade_behavior.handle_acquiring2(cmd).map_err(|e| {
                tracing::error!("Pipeline [acquire]: Failed - {:?}", e);
                e
            })?;

        tracing::info!("Pipeline [acquire]: Success, order_id={}", order_change_log.entity_id());

        // Step 2: Order matching
        let (order_change_logs_opt, trade_change_logs_opt) =
            self.trade_behavior.handle_match3(order_change_log).map_err(|e| {
                tracing::error!("Pipeline [match]: Failed - {:?}", e);
                e
            })?;

        let trade_count = trade_change_logs_opt.as_ref().map(|v| v.len()).unwrap_or(0);
        tracing::info!("Pipeline [match]: Complete, {} trades", trade_count);

        // Step 3: Settlement + K-line processing (batch optimized)
        // Note: Settlement and K-line processing are done together to avoid double iteration
        let mut settlement_balance_logs = Vec::new();
        let mut kline_trade_logs = Vec::new();

        if let Some(ref trade_logs) = trade_change_logs_opt {
            for trade_log in trade_logs {
                let trade_id = trade_log
                    .entity_id()
                    .parse::<u64>()
                    .map_err(|e| {
                        tracing::error!(
                            "Pipeline [settle]: Parse trade_id failed - entity_id={}, error={}",
                            trade_log.entity_id(),
                            e
                        );
                    })
                    .ok();

                // Always add to kline logs regardless of settlement result
                kline_trade_logs.push(trade_log.clone());

                if let Some(id) = trade_id {
                    match self.trade_behavior.handle_settlement2(id) {
                        Ok(balance_logs) => {
                            settlement_balance_logs.extend(balance_logs);
                        }
                        Err(e) => {
                            tracing::error!("Pipeline [settle]: trade_id={} failed - {:?}", id, e);
                        }
                    }
                }
            }
        }

        // Batch send trade logs to K-line service
        if !kline_trade_logs.is_empty() {
            self.kline_service.handle_events(&kline_trade_logs);
        }

        // Publish change logs to queue (batch optimized)
        let balance_logs_count = settlement_balance_logs.len();
        let all_logs = Self::combine_logs(
            order_change_logs_opt,
            trade_change_logs_opt,
            settlement_balance_logs,
        );

        if !all_logs.is_empty() {
            self.push_service.handle_events(&all_logs);
        }

        tracing::info!(
            "Pipeline [settle]: Complete, generated {} balance logs",
            balance_logs_count
        );

        Ok(())
    }

    /// Combine all change logs into a single vector
    fn combine_logs(
        order_change_logs_opt: Option<Vec<ChangeLogEntry>>,
        trade_change_logs_opt: Option<Vec<ChangeLogEntry>>,
        balance_change_logs: Vec<ChangeLogEntry>,
    ) -> Vec<ChangeLogEntry> {
        let mut all_logs = Vec::new();

        if let Some(order_logs) = order_change_logs_opt {
            all_logs.extend(order_logs);
        }
        if let Some(trade_logs) = trade_change_logs_opt {
            all_logs.extend(trade_logs);
        }
        all_logs.extend(balance_change_logs);

        all_logs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pipeline_creation() {
        // TODO: 添加测试用例
    }
}
