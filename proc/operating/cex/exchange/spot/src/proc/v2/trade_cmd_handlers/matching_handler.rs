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
use base_types::handler::handler_update::{
    ApplyCommandChanges, ChangeSet, CmdHandlerForUpdate,
};
use base_types::{Price, Quantity};
use db_repo::MySqlDbRepo;
use diff::ChangeLog;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

pub struct MatchCmd {
    pub taker_order: SpotOrder,
}

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

        self.insert_order_to_lob(&order)?;
        let match_result = self.try_match(&mut order)?;

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

    fn try_match(&self, _order: &mut SpotOrder) -> Result<MatchResult, SpotCmdErrorAny> {
        Ok(MatchResult::empty())
    }

    #[allow(dead_code)]
    fn create_trade(
        &self,
        _taker_order: &SpotOrder,
        _maker_order: &SpotOrder,
        _match_price: Price,
        _match_quantity: Quantity,
    ) -> SpotTrade {
        todo!("Implement trade creation")
    }

    #[allow(dead_code)]
    fn update_order_status(
        &self,
        _order: &mut SpotOrder,
        _filled_quantity: Quantity,
    ) -> Result<ChangeLog, SpotCmdErrorAny> {
        todo!("Implement order status update")
    }
}

impl ApplyCommandChanges<MatchCmd, (), MatchResult, ChangeLog, SpotCmdErrorAny>
    for MatchingHandler
{
    fn apply_command_and_collect_changes(
        &self,
        cmd: &MatchCmd,
        _state_set: (),
    ) -> Result<ChangeSet<MatchResult, ChangeLog>, SpotCmdErrorAny> {
        let result = self.match_order(cmd.taker_order.clone())?;
        let mut changelogs = Vec::with_capacity(result.order_logs.len() + result.trade_logs.len());
        changelogs.extend(result.order_logs.iter().cloned());
        changelogs.extend(result.trade_logs.iter().cloned());
        Ok(ChangeSet { writes: result, changelogs })
    }
}

//todo 实现撮合命令handler
// CmdHandlerForUpdate 里面应该用match cmd 代替 SpotOrder
impl CmdHandlerForUpdate<MatchCmd, (), MatchResult, ChangeLog, SpotCmdErrorAny>
    for MatchingHandler
{
    fn pre_check_command(&self, _cmd: &MatchCmd) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn load_state_set_for_update(&self, _cmd: &MatchCmd) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn validate_command_in_lock(
        &self,
        cmd: &MatchCmd,
        _state_set: &(),
    ) -> Result<(), SpotCmdErrorAny> {
        if cmd.taker_order.state.status
            != base_types::exchange::spot::spot_types::OrderStatus::Pending
        {
            return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                field: "taker_order.status",
                reason: "must be pending before matching",
            }));
        }
        Ok(())
    }

    fn persist_changelogs(&self, _changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn replay_changelogs_to_state(&self, _changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn publish_changelog(&self, _changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        Ok(())
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
