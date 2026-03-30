//! 撮合引擎
//!
//! 负责订单撮合的核心逻辑：
//! - 订单匹配
//! - 成交生成
//! - 订单状态更新
//! - 变更日志生成
//!
//! # 设计原则
//! - 单一职责：只负责撮合逻辑，不处理余额
//! - 无状态：不持有订单簿状态
//! - 纯函数：撮合逻辑为纯函数
//! - 高性能：优化关键路径
//!
//! # 架构说明
//! 余额变更不在撮合引擎中处理，而是通过独立的 SettlementProcessor
//! 消费 trade_change_log 消息来处理，实现关注点分离

use std::sync::Arc;

use base_types::exchange::spot::spot_types::{OrderSide, SpotOrder, SpotTrade};
use base_types::handler::handler::CmdHandler;
use base_types::{Price, Quantity, Timestamp};
use db_repo::MySqlDbRepo;
use diff::{ChangeLog, Entity};
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::{NewOrderAck, NewOrderCmd};

/// 撮合结果
///
/// # 说明
/// 不包含 balance_logs，余额变更由 SettlementProcessor 处理
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// 成交列表
    pub trades: Vec<SpotTrade>,
    /// 订单变更日志
    pub order_logs: Vec<ChangeLog>,
    /// 成交变更日志（用于触发结算）
    pub trade_logs: Vec<ChangeLog>,
}

impl MatchResult {
    /// 创建空的撮合结果
    pub fn empty() -> Self {
        Self { trades: Vec::new(), order_logs: Vec::new(), trade_logs: Vec::new() }
    }

    /// 检查是否有成交
    pub fn has_trades(&self) -> bool {
        !self.trades.is_empty()
    }

    /// 获取成交数量
    pub fn trade_count(&self) -> usize {
        self.trades.len()
    }
}

/// 撮合引擎
///
/// # 职责
/// - 从订单簿中查找匹配订单
/// - 生成成交记录
/// - 更新订单状态
/// - 生成变更日志
///
/// # 不负责
/// - 余额更新（由 SettlementProcessor 处理）
/// - 资金冻结（由 OrderHandler 处理）
pub struct MatchingHandler {
    /// 订单簿仓储
    lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
    /// 订单仓储
    order_repo: Arc<MySqlDbRepo<SpotOrder>>,
    /// 成交仓储
    trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
}

impl MatchingHandler {
    /// 创建新的撮合引擎
    ///
    /// # 参数
    /// - `lob_repo`: 订单簿仓储
    /// - `order_repo`: 订单仓储
    /// - `trade_repo`: 成交仓储
    ///
    /// # 说明
    /// 不再需要 balance_repo，余额更新由 SettlementProcessor 处理
    pub fn new(
        lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
    ) -> Self {
        Self { lob_repo, order_repo, trade_repo }
    }

    /// 执行订单撮合
    ///
    /// # 流程
    /// 1. 将订单插入订单簿
    /// 2. 尝试匹配对手订单
    /// 3. 生成成交记录
    /// 4. 更新订单状态
    /// 5. 生成变更日志
    ///
    /// # 参数
    /// - `order`: 待撮合的订单
    ///
    /// # 返回
    /// - `Ok(MatchResult)`: 撮合结果（包含成交和订单日志）
    /// - `Err(SpotCmdErrorAny)`: 撮合失败错误
    ///
    /// # 注意
    /// 余额更新不在此处理，SettlementProcessor 会消费 trade_logs 并更新余额
    pub fn match_order(&self, mut order: SpotOrder) -> Result<MatchResult, SpotCmdErrorAny> {
        tracing::debug!(
            order_id = %order.order_id,
            symbol = ?order.trading_pair,
            side = ?order.side,
            price = %order.price.unwrap_or_default(),
            quantity = %order.total_base_qty,
            "Starting order matching"
        );

        // 1. 插入订单到订单簿
        self.insert_order_to_lob(&order)?;

        // 2. 尝试撮合
        let match_result = self.try_match(&mut order)?;

        // 3. 记录撮合结果
        if match_result.has_trades() {
            tracing::info!(
                order_id = %order.order_id,
                trade_count = match_result.trade_count(),
                "Order matched successfully"
            );
        } else {
            tracing::debug!(
                order_id = %order.order_id,
                "Order added to order book, no immediate match"
            );
        }

        Ok(match_result)
    }

    /// 将订单插入订单簿
    ///
    /// # 参数
    /// - `order`: 订单引用
    ///
    /// # 返回
    /// - `Ok(())`: 插入成功
    /// - `Err(SpotCmdErrorAny)`: 插入失败
    fn insert_order_to_lob(&self, order: &SpotOrder) -> Result<(), SpotCmdErrorAny> {
        self.lob_repo.add_order(order.trading_pair, order.clone()).map_err(|e| {
            tracing::error!(
                order_id = %order.order_id,
                symbol = ?order.trading_pair,
                error = ?e,
                "Failed to insert order to LOB"
            );
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("Failed to insert order to LOB: {}", e),
            })
        })
    }

    /// 尝试撮合订单
    ///
    /// # 参数
    /// - `order`: 可变订单引用
    ///
    /// # 返回
    /// - `Ok(MatchResult)`: 撮合结果
    /// - `Err(SpotCmdErrorAny)`: 撮合失败
    ///
    /// # 说明
    /// 只负责撮合和生成成交，不处理余额更新
    fn try_match(&self, order: &mut SpotOrder) -> Result<MatchResult, SpotCmdErrorAny> {
        // TODO: 实现完整的撮合逻辑
        // 1. 从订单簿获取对手方订单
        // 2. 价格匹配检查
        // 3. 生成成交
        // 4. 更新订单状态
        // 5. 生成变更日志

        // 当前返回空结果（占位实现）
        Ok(MatchResult::empty())
    }

    /// 生成成交记录
    ///
    /// # 参数
    /// - `taker_order`: Taker 订单
    /// - `maker_order`: Maker 订单
    /// - `match_price`: 成交价格
    /// - `match_quantity`: 成交数量
    ///
    /// # 返回
    /// 成交记录
    #[allow(dead_code)]
    fn create_trade(
        &self,
        taker_order: &SpotOrder,
        maker_order: &SpotOrder,
        match_price: Price,
        match_quantity: Quantity,
    ) -> SpotTrade {
        // TODO: 实现成交记录生成
        todo!("Implement trade creation")
    }

    /// 更新订单状态
    ///
    /// # 参数
    /// - `order`: 可变订单引用
    /// - `filled_quantity`: 已成交数量
    ///
    /// # 返回
    /// 订单变更日志
    #[allow(dead_code)]
    fn update_order_status(
        &self,
        order: &mut SpotOrder,
        filled_quantity: Quantity,
    ) -> Result<ChangeLog, SpotCmdErrorAny> {
        // TODO: 实现订单状态更新
        todo!("Implement order status update")
    }
}

impl CmdHandler<SpotOrder, MatchResult, SpotCmdErrorAny> for MatchingHandler {
    fn cmd_handle(&self, order: SpotOrder) -> Result<MatchResult, SpotCmdErrorAny> {
        return self.match_order(order);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_result_empty() {
        let result = MatchResult::empty();
        assert!(!result.has_trades());
        assert_eq!(result.trade_count(), 0);
    }

    // TODO: 添加更多测试
}
