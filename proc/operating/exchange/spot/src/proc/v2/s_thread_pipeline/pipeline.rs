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

        // Step 3: Settlement
        let mut settlement_balance_logs = Vec::new();
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

                if let Some(id) = trade_id {
                    if let Err(e) = self.trade_behavior.handle_settlement2(id) {
                        tracing::error!("Pipeline [settle]: trade_id={} failed - {:?}", id, e);
                    } else {
                        settlement_balance_logs.push(trade_log.clone());
                    }
                }
            }
        }

        // Send trade logs to K-line service
        if let Some(ref trade_logs) = trade_change_logs_opt {
            for trade_log in trade_logs {
                self.kline_service.handle_event(trade_log.clone());
            }
        }

        // Publish change logs to queue
        let balance_logs_count = settlement_balance_logs.len();
        self.publish_change_logs(
            order_change_logs_opt,
            trade_change_logs_opt,
            settlement_balance_logs,
        );

        tracing::info!(
            "Pipeline [settle]: Complete, generated {} balance logs",
            balance_logs_count
        );

        Ok(())
    }

    /// Publish change logs to corresponding topics for PushService to subscribe and push
    fn publish_change_logs(
        &self,
        order_change_logs_opt: Option<Vec<ChangeLogEntry>>,
        trade_change_logs_opt: Option<Vec<ChangeLogEntry>>,
        balance_change_logs: Vec<ChangeLogEntry>,
    ) {
        // Publish OrderChangeLog
        if let Some(order_logs) = order_change_logs_opt {
            for log in order_logs {
                self.push_service.handle_event(log);
            }
        }

        // Publish TradeChangeLog
        if let Some(trade_logs) = trade_change_logs_opt {
            for log in trade_logs {
                self.push_service.handle_event(log);
            }
        }

        // Publish BalanceChangeLog
        for log in balance_change_logs {
            self.push_service.handle_event(log);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pipeline_creation() {
        // TODO: 添加测试用例
    }
}
