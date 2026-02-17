use std::sync::Arc;

use diff::ChangeLogEntry;
use push::k_line::k_line_service::KLineBehaviorV2Imp;
use push::push::push_service::PushBehaviorV2Imp;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;
use crate::proc::v2::spot_trade_v2::SpotTradeBehaviorV2Impl;

/// 单线程现货交易流水线
///
/// 负责处理完整的订单生命周期：收单 → 撮合 → 结算 → 推送
/// 设计为单线程执行以避免锁竞争，适合超高频交易场景
pub struct SThreadSpotTradePipeline {
    /// 交易行为实现，包含收单、撮合、结算等核心业务逻辑
    trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
    /// 推送服务，负责将变更日志推送到客户端
    push_service: Arc<PushBehaviorV2Imp>,
    /// K线服务，负责聚合交易数据生成K线
    kline_service: Arc<KLineBehaviorV2Imp>,
}

impl SThreadSpotTradePipeline {
    /// 创建新的流水线实例
    ///
    /// # 参数
    /// - `trade_behavior`: 交易行为实现
    /// - `push_service`: 推送服务
    /// - `kline_service`: K线服务
    pub fn new(
        trade_behavior: Arc<SpotTradeBehaviorV2Impl>,
        push_service: Arc<PushBehaviorV2Imp>,
        kline_service: Arc<KLineBehaviorV2Imp>,
    ) -> Self {
        Self { trade_behavior, push_service, kline_service }
    }

    /// 处理新订单请求
    ///
    /// 完整的订单处理流程：
    /// 1. 收单处理：验证订单、冻结资金
    /// 2. 撮合处理：匹配买卖订单、生成交收记录
    /// 3. 结算处理：更新账户余额、释放冻结资金
    /// 4. 数据推送：将变更日志推送给K线服务和客户端
    ///
    /// # 参数
    /// - `cmd`: 新订单命令
    ///
    /// # 返回值
    /// - `Ok(())`: 处理成功
    /// - `Err(SpotCmdErrorAny)`: 处理失败
    pub fn handle_new_order(&self, cmd: NewOrderCmd) -> Result<(), SpotCmdErrorAny> {
        // ========== 步骤1：订单收单 ==========
        // 验证订单有效性，检查用户余额，冻结相应资金
        let (balance_change_log, order_change_log) =
            self.trade_behavior.handle_acquiring2(cmd).map_err(|e| {
                tracing::error!("Pipeline [收单]: 处理失败 - {:?}", e);
                e
            })?;

        tracing::info!("Pipeline [收单]: 成功, 订单ID={}", order_change_log.entity_id());

        // ========== 步骤2：订单撮合 ==========
        // 将订单加入撮合引擎，尝试与其他订单匹配成交
        let (order_change_logs_opt, trade_change_logs_opt) =
            self.trade_behavior.handle_match3(order_change_log).map_err(|e| {
                tracing::error!("Pipeline [撮合]: 处理失败 - {:?}", e);
                e
            })?;

        let trade_count = trade_change_logs_opt.as_ref().map(|v| v.len()).unwrap_or(0);
        tracing::info!("Pipeline [撮合]: 完成, 成交 {} 笔", trade_count);

        // ========== 步骤3：结算处理 + K线数据准备 ==========
        // 处理每笔成交的结算，同时准备K线服务所需的数据
        // 合并循环优化：避免对成交日志的重复遍历
        let mut settlement_balance_logs = Vec::new(); // 结算产生的余额变更日志
        let mut kline_trade_logs = Vec::new(); // 需要推送给K线服务的成交日志

        if let Some(ref trade_logs) = trade_change_logs_opt {
            for trade_log in trade_logs {
                // 解析成交ID，用于结算处理
                let trade_id = trade_log
                    .entity_id()
                    .parse::<u64>()
                    .map_err(|e| {
                        tracing::error!(
                            "Pipeline [结算]: 解析成交ID失败 - 实体ID={}, 错误={}",
                            trade_log.entity_id(),
                            e
                        );
                    })
                    .ok();

                // 无论结算是否成功，都推送给K线服务（K线应反映实际成交）
                kline_trade_logs.push(trade_log.clone());

                // 执行结算：更新买卖双方的账户余额
                if let Some(id) = trade_id {
                    match self.trade_behavior.handle_settlement2(id) {
                        Ok(balance_logs) => {
                            // 结算成功，收集余额变更日志
                            settlement_balance_logs.extend(balance_logs);
                        }
                        Err(e) => {
                            // 结算失败，记录错误但不中断流程
                            tracing::error!("Pipeline [结算]: 成交ID={} 结算失败 - {:?}", id, e);
                        }
                    }
                }
            }
        }

        // ========== 步骤4：K线数据处理 ==========
        // 批量推送成交日志到K线服务进行聚合
        if !kline_trade_logs.is_empty() {
            self.kline_service.handle_events(&kline_trade_logs);
        }

        // ========== 步骤5：客户端推送 ==========
        // 收集所有变更日志并批量推送给客户端
        let balance_logs_count = settlement_balance_logs.len();

        // 合并三类变更日志：订单变更、成交变更、余额变更
        let all_logs = Self::combine_logs(
            order_change_logs_opt,   // 订单状态变更
            trade_change_logs_opt,   // 成交记录
            settlement_balance_logs, // 余额变更
        );

        // 批量推送到客户端
        if !all_logs.is_empty() {
            self.push_service.handle_events(&all_logs);
        }

        tracing::info!("Pipeline [结算]: 完成, 生成 {} 条余额变更日志", balance_logs_count);

        Ok(())
    }

    /// 合并所有变更日志
    ///
    /// 将订单、成交、余额三类变更日志合并为一个向量，
    /// 用于统一推送给客户端
    ///
    /// # 参数
    /// - `order_change_logs_opt`: 订单变更日志（可选）
    /// - `trade_change_logs_opt`: 成交变更日志（可选）
    /// - `balance_change_logs`: 余额变更日志
    ///
    /// # 返回值
    /// 合并后的变更日志向量
    fn combine_logs(
        order_change_logs_opt: Option<Vec<ChangeLogEntry>>,
        trade_change_logs_opt: Option<Vec<ChangeLogEntry>>,
        balance_change_logs: Vec<ChangeLogEntry>,
    ) -> Vec<ChangeLogEntry> {
        let mut all_logs = Vec::new();

        // 添加订单变更日志
        if let Some(order_logs) = order_change_logs_opt {
            all_logs.extend(order_logs);
        }

        // 添加成交变更日志
        if let Some(trade_logs) = trade_change_logs_opt {
            all_logs.extend(trade_logs);
        }

        // 添加余额变更日志
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
