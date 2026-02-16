use std::sync::Arc;

use diff::ChangeLogEntry;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

pub struct SThreadSpotTradePipeline {
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
}

impl SThreadSpotTradePipeline {
    pub fn new(trade_behavior: Arc<SpotTradeBehaviorV2Impl>) -> Self {
        Self { trade_behavior }
    }

    pub fn handle_new_order(&self, cmd: NewOrderCmd) -> Result<(), SpotCmdErrorAny> {
        // 步骤1: 收单处理
        let (balance_change_log, order_change_log) =
            match self.trade_behavior.handle_acquiring2(cmd) {
                Ok((balance_log, order_log)) => (balance_log, order_log),
                Err(e) => {
                    tracing::error!("MThread Pipeline [收单]: 处理失败: {:?}", e);
                    return Err(e);
                }
            };

        tracing::info!("MThread Pipeline [收单]: 成功, order_id={}", order_change_log.entity_id());

        // 步骤2: 撮合处理
        let (order_change_logs_opt, trade_change_logs_opt) =
            match self.trade_behavior.handle_match3(order_change_log) {
                Ok((order_logs, trade_logs)) => (order_logs, trade_logs),
                Err(e) => {
                    tracing::error!("MThread Pipeline [撮合]: 处理失败: {:?}", e);
                    return Err(e);
                }
            };

        let trade_count = trade_change_logs_opt.as_ref().map(|v| v.len()).unwrap_or(0);
        tracing::info!("MThread Pipeline [撮合]: 完成, {} 笔成交", trade_count);

        // 步骤3: 结算处理
        let mut settlement_balance_logs: Vec<ChangeLogEntry> = Vec::new();
        if let Some(ref trade_logs) = trade_change_logs_opt {
            for trade_log in trade_logs {
                let trade_id = match trade_log.entity_id().parse::<u64>() {
                    Ok(id) => id,
                    Err(e) => {
                        tracing::error!(
                            "MThread Pipeline [结算]: 解析 trade_id 失败: entity_id={}, error={}",
                            trade_log.entity_id(),
                            e
                        );
                        continue;
                    }
                };

                match self.trade_behavior.handle_settlement2(trade_id) {
                    Ok(balance_logs) => {
                        settlement_balance_logs.extend(balance_logs);
                    }
                    Err(e) => {
                        tracing::error!(
                            "MThread Pipeline [结算]: trade_id={} 结算失败: {:?}",
                            trade_id,
                            e
                        );
                        continue;
                    }
                }
            }
        }

        //todo k线聚合&推送
        tracing::info!(
            "MThread Pipeline [结算]: 完成, 生成 {} 条余额变更日志",
            settlement_balance_logs.len()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pipeline_creation() {
        // TODO: 添加测试用例
    }
}
