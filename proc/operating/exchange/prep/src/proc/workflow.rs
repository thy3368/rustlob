//! 永续合约交易工作流编排器
//!
//! 实现XPDL流程定义中的主流程路由逻辑

use std::sync::Arc;
use crate::proc::trading_prep_order_proc::*;

// ============================================================================
// 交易动作枚举 - 对应XPDL中的action字段
// ============================================================================

/// 交易动作类型
///
/// 对应XPDL流程定义中的action字段，用于路由到不同的业务活动
#[derive(Debug, Clone)]
pub enum TradeAction {
    /// 开仓
    OpenPosition(OpenPositionCommand),

    /// 平仓
    ClosePosition(ClosePositionCommand),

    /// 调整杠杆
    AdjustLeverage(SetLeverageCommand),

    /// 追加保证金（仅逐仓模式）
    AddMargin(AddMarginRequest),

    /// 减少保证金（仅逐仓模式）
    ReduceMargin(ReduceMarginRequest),

    /// 切换持仓模式（单向 ↔ 对冲）
    ChangePositionMode(SetPositionModeCommand),

    /// 变更保证金类型（逐仓 ↔ 全仓）
    ChangeMarginType(SetMarginTypeCommand),

    /// 撤销单个订单
    CancelOrder(CancelOrderCommand),

    /// 批量撤销订单
    CancelAllOrders(CancelAllOrdersCommand),

    /// 修改订单（价格/数量）
    ModifyOrder(ModifyOrderCommand),

    /// 设置自动追加保证金
    SetAutoAddMargin(SetAutoAddMarginRequest),
}

/// 工作流执行结果
#[derive(Debug, Clone)]
pub enum WorkflowResult {
    /// 开仓成功
    PositionOpened(OpenPositionResult),

    /// 平仓成功
    PositionClosed(ClosePositionResult),

    /// 杠杆调整成功
    LeverageAdjusted(SetLeverageResult),

    /// 保证金追加成功
    MarginAdded(MarginOperationResult),

    /// 保证金减少成功
    MarginReduced(MarginOperationResult),

    /// 持仓模式切换成功
    PositionModeChanged(SetPositionModeResult),

    /// 保证金类型变更成功
    MarginTypeChanged(SetMarginTypeResult),

    /// 订单撤销成功
    OrderCancelled(CancelOrderResult),

    /// 批量撤销成功
    AllOrdersCancelled(CancelAllOrdersResult),

    /// 订单修改成功
    OrderModified(ModifyOrderResult),

    /// 自动追加保证金设置成功
    AutoAddMarginSet,
}

// ============================================================================
// 保证金管理命令（XPDL中定义但代码中缺失）
// ============================================================================

/// 追加保证金请求
#[derive(Debug, Clone)]
pub struct AddMarginRequest {
    /// 交易员ID
    pub trader_id: String,

    /// 交易对
    pub symbol: Symbol,

    /// 持仓方向（对冲模式需要）
    pub position_side: PositionSide,

    /// 追加金额
    pub amount: Price,
}

impl AddMarginRequest {
    /// 创建追加保证金请求
    pub fn new(trader_id: String, symbol: Symbol, position_side: PositionSide, amount: Price) -> Self {
        Self {
            trader_id,
            symbol,
            position_side,
            amount,
        }
    }

    /// 验证请求有效性
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.amount.is_positive() {
            return Err("追加金额必须大于0");
        }
        Ok(())
    }
}

/// 减少保证金请求
#[derive(Debug, Clone)]
pub struct ReduceMarginRequest {
    /// 交易员ID
    pub trader_id: String,

    /// 交易对
    pub symbol: Symbol,

    /// 持仓方向（对冲模式需要）
    pub position_side: PositionSide,

    /// 减少金额
    pub amount: Price,
}

impl ReduceMarginRequest {
    /// 创建减少保证金请求
    pub fn new(trader_id: String, symbol: Symbol, position_side: PositionSide, amount: Price) -> Self {
        Self {
            trader_id,
            symbol,
            position_side,
            amount,
        }
    }

    /// 验证请求有效性
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.amount.is_positive() {
            return Err("减少金额必须大于0");
        }
        Ok(())
    }
}

/// 设置自动追加保证金请求
#[derive(Debug, Clone)]
pub struct SetAutoAddMarginRequest {
    /// 交易员ID
    pub trader_id: String,

    /// 交易对
    pub symbol: Symbol,

    /// 持仓方向（对冲模式需要）
    pub position_side: PositionSide,

    /// 是否启用自动追加
    pub auto_add_margin: bool,
}

impl SetAutoAddMarginRequest {
    /// 启用自动追加保证金
    pub fn enable(trader_id: String, symbol: Symbol, position_side: PositionSide) -> Self {
        Self {
            trader_id,
            symbol,
            position_side,
            auto_add_margin: true,
        }
    }

    /// 禁用自动追加保证金
    pub fn disable(trader_id: String, symbol: Symbol, position_side: PositionSide) -> Self {
        Self {
            trader_id,
            symbol,
            position_side,
            auto_add_margin: false,
        }
    }
}

/// 保证金操作结果
#[derive(Debug, Clone)]
pub struct MarginOperationResult {
    /// 交易对
    pub symbol: Symbol,

    /// 持仓方向
    pub position_side: PositionSide,

    /// 操作金额
    pub amount: Price,

    /// 新的持仓保证金
    pub new_margin: Price,

    /// 新的强平价格
    pub new_liquidation_price: Option<Price>,

    /// 操作时间戳
    pub timestamp: u64,
}

impl MarginOperationResult {
    /// 创建保证金操作结果
    pub fn new(
        symbol: Symbol,
        position_side: PositionSide,
        amount: Price,
        new_margin: Price,
        new_liquidation_price: Option<Price>,
    ) -> Self {
        Self {
            symbol,
            position_side,
            amount,
            new_margin,
            new_liquidation_price,
            timestamp: current_timestamp_ms(),
        }
    }
}

// ============================================================================
// 永续合约交易工作流编排器
// ============================================================================

/// 永续合约交易工作流
///
/// # 职责
/// - 接收交易动作请求
/// - 路由到对应的处理器
/// - 执行业务逻辑
/// - 返回统一的结果
///
/// # 架构
/// 实现XPDL中定义的主流程：
/// Start → RouteAction → [各个活动] → End
pub struct PerpetualTradingWorkflow {
    /// 命令处理器（开仓、平仓、撤单等）
    command_processor: Arc<dyn PerpOrderExchProc>,

    /// 查询处理器（查询订单、持仓等）
    query_processor: Arc<dyn PerpOrderExchQueryProc>,

    /// 保证金管理处理器
    margin_processor: Arc<dyn MarginManagementProc>,
}

impl PerpetualTradingWorkflow {
    /// 创建新的工作流实例
    pub fn new(
        command_processor: Arc<dyn PerpOrderExchProc>,
        query_processor: Arc<dyn PerpOrderExchQueryProc>,
        margin_processor: Arc<dyn MarginManagementProc>,
    ) -> Self {
        Self {
            command_processor,
            query_processor,
            margin_processor,
        }
    }

    /// 执行交易动作
    ///
    /// # 参数
    /// - `action`: 交易动作类型
    ///
    /// # 返回
    /// - `Ok(WorkflowResult)`: 执行成功
    /// - `Err(PrepCommandError)`: 执行失败
    ///
    /// # 示例
    /// ```ignore
    /// let workflow = PerpetualTradingWorkflow::new(processor, query, margin);
    ///
    /// // 开仓
    /// let action = TradeAction::OpenPosition(OpenPositionCommand::market_long(
    ///     Symbol::new("BTCUSDT"),
    ///     Quantity::from_f64(1.0)
    /// ));
    ///
    /// let result = workflow.execute(action).await?;
    /// ```
    pub fn execute(
        &self,
        action: TradeAction,
    ) -> Result<WorkflowResult, PrepCommandError> {
        // 对应XPDL中的RouteAction网关
        match action {
            // 开仓活动
            TradeAction::OpenPosition(cmd) => {
                let result = self.command_processor.open_position(cmd)?;
                Ok(WorkflowResult::PositionOpened(result))
            }

            // 平仓活动
            TradeAction::ClosePosition(cmd) => {
                let result = self.command_processor.close_position(cmd)?;
                Ok(WorkflowResult::PositionClosed(result))
            }

            // 调整杠杆活动
            TradeAction::AdjustLeverage(cmd) => {
                let result = self.command_processor.set_leverage(cmd)?;
                Ok(WorkflowResult::LeverageAdjusted(result))
            }

            // 追加保证金活动
            TradeAction::AddMargin(request) => {
                let result = self.margin_processor.add_margin(request)?;
                Ok(WorkflowResult::MarginAdded(result))
            }

            // 减少保证金活动
            TradeAction::ReduceMargin(request) => {
                let result = self.margin_processor.reduce_margin(request)?;
                Ok(WorkflowResult::MarginReduced(result))
            }

            // 切换持仓模式活动
            TradeAction::ChangePositionMode(cmd) => {
                let result = self.command_processor.set_position_mode(cmd)?;
                Ok(WorkflowResult::PositionModeChanged(result))
            }

            // 变更保证金类型活动
            TradeAction::ChangeMarginType(cmd) => {
                let result = self.command_processor.set_margin_type(cmd)?;
                Ok(WorkflowResult::MarginTypeChanged(result))
            }

            // 撤销订单活动
            TradeAction::CancelOrder(cmd) => {
                let result = self.command_processor.cancel_order(cmd)?;
                Ok(WorkflowResult::OrderCancelled(result))
            }

            // 批量撤销订单活动
            TradeAction::CancelAllOrders(cmd) => {
                let result = self.command_processor.cancel_all_orders(cmd)?;
                Ok(WorkflowResult::AllOrdersCancelled(result))
            }

            // 修改订单活动
            TradeAction::ModifyOrder(cmd) => {
                let result = self.command_processor.modify_order(cmd)?;
                Ok(WorkflowResult::OrderModified(result))
            }

            // 设置自动追加保证金活动
            TradeAction::SetAutoAddMargin(request) => {
                self.margin_processor.set_auto_add_margin(request)?;
                Ok(WorkflowResult::AutoAddMarginSet)
            }
        }
    }
}

// ============================================================================
// 保证金管理处理器 Trait
// ============================================================================

/// 保证金管理处理器
///
/// 负责处理保证金相关的操作（仅逐仓模式）
pub trait MarginManagementProc: Send + Sync {
    /// 追加保证金
    ///
    /// # 使用场景
    /// - 避免强平：当前价格接近强平价，追加保证金拉远强平线
    /// - 降低风险：市场波动加剧时主动增加保证金缓冲
    ///
    /// # 前置条件
    /// - 存在逐仓模式的持仓（全仓模式不支持）
    /// - 账户可用余额充足
    ///
    /// # 业务规则
    /// - 仅逐仓模式支持
    /// - 强平价格变化：追加后强平价格远离当前价
    /// - 资金转移：从可用余额转移到持仓保证金
    fn add_margin(
        &self,
        request: AddMarginRequest,
    ) -> Result<MarginOperationResult, PrepCommandError>;

    /// 减少保证金
    ///
    /// # 使用场景
    /// - 释放资金：行情有利时减少保证金，释放资金用于其他交易
    /// - 提高杠杆：通过减少保证金变相提高杠杆倍数
    ///
    /// # 前置条件
    /// - 存在逐仓模式的持仓
    /// - 减少后保证金仍满足最低要求
    /// - 不会导致立即触发强平
    ///
    /// # 业务规则
    /// - 仅逐仓模式支持
    /// - 最低保证金限制：不能减少到低于维持保证金要求
    /// - 强平价格变化：减少后强平价格靠近当前价
    fn reduce_margin(
        &self,
        request: ReduceMarginRequest,
    ) -> Result<MarginOperationResult, PrepCommandError>;

    /// 设置自动追加保证金
    ///
    /// # 使用场景
    /// - 自动风控：启用后自动追加保证金，避免因暂时波动被强平
    /// - 无人值守：策略自动运行时，避免因保证金不足导致强平
    ///
    /// # 业务规则
    /// - 仅逐仓模式支持
    /// - 自动触发：当保证金率接近维持保证金率时自动追加
    /// - 余额限制：仅从可用余额追加，不会超出账户余额
    fn set_auto_add_margin(
        &self,
        request: SetAutoAddMarginRequest,
    ) -> Result<(), PrepCommandError>;
}

/// 获取当前时间戳（毫秒）
fn current_timestamp_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_margin_request_validation() {
        let symbol = Symbol::new("BTCUSDT");
        let request = AddMarginRequest::new(
            "trader123".to_string(),
            symbol,
            PositionSide::Long,
            Price::from_f64(100.0),
        );

        assert!(request.validate().is_ok());

        // 无效金额
        let invalid_request = AddMarginRequest::new(
            "trader123".to_string(),
            symbol,
            PositionSide::Long,
            Price::from_raw(0),
        );

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_reduce_margin_request_validation() {
        let symbol = Symbol::new("ETHUSDT");
        let request = ReduceMarginRequest::new(
            "trader456".to_string(),
            symbol,
            PositionSide::Short,
            Price::from_f64(50.0),
        );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_set_auto_add_margin_request() {
        let symbol = Symbol::new("BTCUSDT");

        // 启用
        let enable_req = SetAutoAddMarginRequest::enable(
            "trader789".to_string(),
            symbol,
            PositionSide::Long,
        );
        assert!(enable_req.auto_add_margin);

        // 禁用
        let disable_req = SetAutoAddMarginRequest::disable(
            "trader789".to_string(),
            symbol,
            PositionSide::Long,
        );
        assert!(!disable_req.auto_add_margin);
    }
}
