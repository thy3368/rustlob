//! 永续合约命令定义
//!
//! 统一委托模型，与主流交易所一致 (Binance, OKX, Bybit)
//!
//! ## 优先级分层
//!
//! - P0: 核心交易 (LimitOrder, MarketOrder, CancelOrder)
//! - P1: 风险控制 (SetLeverage, AdjustMargin, Liquidate, SetStopLoss,
//!   SetTakeProfit)
//! - P2: 高级功能 (SwitchMarginMode, SettleFundingRate, ADL, TrailingStop)
//! - P3: 扩展功能 (FlashClose, ReversePosition, BatchCancelOrders)

use crate::domain::entity::{
    Leverage, Margin, MarginMode, OrderId, OrderStatus, PositionId, PositionMode, PositionSide,
    Price, Quantity, Side, TimeInForce, Trade, TraderId,
};

// ============================================================================
// P0 - 核心交易命令（统一委托模型）
// ============================================================================

/// 永续合约命令
#[derive(Debug, Clone)]
pub enum Command {
    // ==================== P0: 核心交易 ====================
    /// 限价委托
    ///
    /// 统一模型：
    /// - 开多：`side=Buy, position_side=Long, reduce_only=false`
    /// - 平多：`side=Sell, position_side=Long, reduce_only=true`
    /// - 开空：`side=Sell, position_side=Short, reduce_only=false`
    /// - 平空：`side=Buy, position_side=Short, reduce_only=true`
    LimitOrder {
        /// 交易者ID
        trader: TraderId,
        /// 订单方向
        side: Side,
        /// 委托价格
        price: Price,
        /// 委托数量
        quantity: Quantity,
        /// 持仓方向
        position_side: PositionSide,
        /// 只减仓
        reduce_only: bool,
        /// 有效期
        time_in_force: TimeInForce,
    },

    /// 市价委托
    MarketOrder {
        /// 交易者ID
        trader: TraderId,
        /// 订单方向
        side: Side,
        /// 委托数量
        quantity: Quantity,
        /// 持仓方向
        position_side: PositionSide,
        /// 只减仓
        reduce_only: bool,
    },

    /// 取消委托
    CancelOrder {
        /// 订单ID
        order_id: OrderId,
    },

    // ==================== P1: 风险控制 ====================
    /// 设置杠杆
    SetLeverage {
        /// 交易者ID
        trader: TraderId,
        /// 杠杆倍数
        leverage: Leverage,
        /// 持仓方向（None=双边）
        position_side: Option<PositionSide>,
    },

    /// 调整保证金（仅逐仓）
    AdjustMargin {
        /// 交易者ID
        trader: TraderId,
        /// 仓位ID
        position_id: PositionId,
        /// 金额（正=追加，负=减少）
        amount: i64,
    },

    /// 强制平仓（系统触发）
    Liquidate {
        /// 仓位ID
        position_id: PositionId,
        /// 标记价格
        mark_price: Price,
        /// 破产价格
        bankruptcy_price: Price,
    },

    /// 设置止损
    SetStopLoss {
        /// 交易者ID
        trader: TraderId,
        /// 仓位ID
        position_id: PositionId,
        /// 触发价格
        trigger_price: Price,
        /// 平仓价格（None=市价）
        close_price: Option<Price>,
    },

    /// 设置止盈
    SetTakeProfit {
        /// 交易者ID
        trader: TraderId,
        /// 仓位ID
        position_id: PositionId,
        /// 触发价格
        trigger_price: Price,
        /// 平仓价格（None=市价）
        close_price: Option<Price>,
    },

    // ==================== P2: 高级功能 ====================
    /// 切换保证金模式
    SwitchMarginMode {
        /// 交易者ID
        trader: TraderId,
        /// 目标模式
        mode: MarginMode,
    },

    /// 切换持仓模式
    SwitchPositionMode {
        /// 交易者ID
        trader: TraderId,
        /// 目标模式
        mode: PositionMode,
    },

    /// 资金费率结算（系统触发）
    SettleFundingRate {
        /// 仓位ID
        position_id: PositionId,
        /// 费率（正=多付空）
        funding_rate: i64,
        /// 标记价格
        mark_price: Price,
    },

    /// 自动减仓（系统触发）
    ADL {
        /// 被减仓仓位ID
        position_id: PositionId,
        /// 对手方仓位ID
        counterparty_position_id: PositionId,
        /// 减仓数量
        quantity: Quantity,
        /// 成交价格
        price: Price,
    },

    /// 追踪止损
    TrailingStop {
        /// 交易者ID
        trader: TraderId,
        /// 仓位ID
        position_id: PositionId,
        /// 回调比例（基点 1/10000）
        callback_rate: u32,
        /// 激活价格
        activation_price: Option<Price>,
    },

    // ==================== P3: 扩展功能 ====================
    /// 闪电平仓
    FlashClose {
        /// 交易者ID
        trader: TraderId,
        /// 仓位ID
        position_id: PositionId,
    },

    /// 反向开仓
    ReversePosition {
        /// 交易者ID
        trader: TraderId,
        /// 仓位ID
        position_id: PositionId,
        /// 新仓数量
        new_quantity: Quantity,
        /// 价格（None=市价）
        price: Option<Price>,
    },

    /// 批量取消
    BatchCancelOrders {
        /// 交易者ID
        trader: TraderId,
        /// 订单ID列表
        order_ids: Vec<OrderId>,
    },

    /// 全部取消
    CancelAllOrders {
        /// 交易者ID
        trader: TraderId,
        /// 持仓方向筛选
        position_side: Option<PositionSide>,
    },
}

// ============================================================================
// 命令结果
// ============================================================================

/// 错误码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// 保证金不足
    InsufficientMargin = 1001,
    /// 余额不足
    InsufficientBalance = 1002,
    /// 订单不存在
    OrderNotFound = 1003,
    /// 仓位不存在
    PositionNotFound = 1004,
    /// 无效数量
    InvalidQuantity = 1005,
    /// 无效价格
    InvalidPrice = 1006,
    /// 无效杠杆
    InvalidLeverage = 1007,
    /// 持仓模式不匹配
    PositionModeMismatch = 1008,
    /// 只减仓被拒
    ReduceOnlyRejected = 1009,
    /// 超最大挂单数
    MaxOpenOrdersExceeded = 1010,
    /// 超最大持仓量
    MaxPositionSizeExceeded = 1011,
    /// 会触发强平
    WouldTriggerLiquidation = 1012,
    /// 系统错误
    SystemError = 9999,
}

/// 命令结果
#[derive(Debug, Clone)]
pub enum CommandResult {
    // ==================== P0: 核心交易结果 ====================
    /// 限价委托结果
    LimitOrder {
        /// 订单ID
        order_id: OrderId,
        /// 成交列表
        trades: Vec<Trade>,
        /// 剩余数量
        remaining_quantity: Quantity,
        /// 状态
        status: OrderStatus,
    },

    /// 市价委托结果
    MarketOrder {
        /// 订单ID
        order_id: OrderId,
        /// 成交列表
        trades: Vec<Trade>,
        /// 均价
        avg_price: Price,
        /// 成交数量
        filled_quantity: Quantity,
    },

    /// 取消结果
    CancelOrder {
        /// 订单ID
        order_id: OrderId,
        /// 是否成功
        success: bool,
        /// 取消数量
        cancelled_quantity: Quantity,
    },

    // ==================== P1: 风险控制结果 ====================
    /// 设置杠杆结果
    SetLeverage {
        /// 交易者ID
        trader: TraderId,
        /// 原杠杆
        old_leverage: Leverage,
        /// 新杠杆
        new_leverage: Leverage,
        /// 是否成功
        success: bool,
    },

    /// 调整保证金结果
    AdjustMargin {
        /// 仓位ID
        position_id: PositionId,
        /// 原保证金
        old_margin: Margin,
        /// 新保证金
        new_margin: Margin,
        /// 新强平价
        new_liquidation_price: Price,
    },

    /// 强平结果
    Liquidate {
        /// 仓位ID
        position_id: PositionId,
        /// 成交列表
        trades: Vec<Trade>,
        /// 亏损
        loss: u64,
        /// 保险基金贡献
        insurance_fund_contribution: u64,
    },

    /// 止损结果
    SetStopLoss {
        /// 仓位ID
        position_id: PositionId,
        /// 触发价
        trigger_price: Price,
        /// 是否成功
        success: bool,
    },

    /// 止盈结果
    SetTakeProfit {
        /// 仓位ID
        position_id: PositionId,
        /// 触发价
        trigger_price: Price,
        /// 是否成功
        success: bool,
    },

    // ==================== P2: 高级功能结果 ====================
    /// 切换保证金模式结果
    SwitchMarginMode {
        /// 交易者ID
        trader: TraderId,
        /// 原模式
        old_mode: MarginMode,
        /// 新模式
        new_mode: MarginMode,
        /// 是否成功
        success: bool,
    },

    /// 切换持仓模式结果
    SwitchPositionMode {
        /// 交易者ID
        trader: TraderId,
        /// 原模式
        old_mode: PositionMode,
        /// 新模式
        new_mode: PositionMode,
        /// 是否成功
        success: bool,
    },

    /// 资金费率结算结果
    SettleFundingRate {
        /// 仓位ID
        position_id: PositionId,
        /// 资金费（正=支付，负=收取）
        funding_fee: i64,
        /// 新保证金
        new_margin: Margin,
    },

    /// 自动减仓结果
    ADL {
        /// 仓位ID
        position_id: PositionId,
        /// 对手方仓位ID
        counterparty_position_id: PositionId,
        /// 数量
        quantity: Quantity,
        /// 价格
        price: Price,
        /// 已实现盈亏
        realized_pnl: i64,
    },

    /// 追踪止损结果
    TrailingStop {
        /// 仓位ID
        position_id: PositionId,
        /// 当前触发价
        current_trigger_price: Price,
        /// 是否成功
        success: bool,
    },

    // ==================== P3: 扩展功能结果 ====================
    /// 闪电平仓结果
    FlashClose {
        /// 仓位ID
        position_id: PositionId,
        /// 成交列表
        trades: Vec<Trade>,
        /// 已实现盈亏
        realized_pnl: i64,
    },

    /// 反向开仓结果
    ReversePosition {
        /// 原仓位ID
        old_position_id: PositionId,
        /// 新仓位ID
        new_position_id: PositionId,
        /// 平仓成交
        close_trades: Vec<Trade>,
        /// 开仓成交
        open_trades: Vec<Trade>,
        /// 已实现盈亏
        realized_pnl: i64,
    },

    /// 批量取消结果
    BatchCancelOrders {
        /// 成功
        cancelled: Vec<OrderId>,
        /// 失败
        failed: Vec<OrderId>,
    },

    /// 全部取消结果
    CancelAllOrders {
        /// 取消数量
        cancelled_count: usize,
        /// 订单列表
        order_ids: Vec<OrderId>,
    },

    /// 错误
    Error {
        /// 错误码
        code: ErrorCode,
        /// 错误信息
        message: String,
    },
}

// ============================================================================
// 命令处理器
// ============================================================================

/// 永续合约命令处理器
///
/// 遵循 Clean Architecture：
/// 处理器专注业务逻辑，不依赖具体仓储实现
pub trait PrepCommandHandler: Send + Sync {
    /// 处理命令
    fn handle(&mut self, command: Command) -> CommandResult;

    /// 处理器名称
    fn handler_name(&self) -> &'static str;
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_long() {
        // 开多 = Buy + Long + reduce_only=false
        let cmd = Command::LimitOrder {
            trader: 1,
            side: Side::Buy,
            price: 50000_00,
            quantity: 100,
            position_side: PositionSide::Long,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        };

        match cmd {
            Command::LimitOrder { side, position_side, reduce_only, .. } => {
                assert_eq!(side, Side::Buy);
                assert_eq!(position_side, PositionSide::Long);
                assert!(!reduce_only);
            }
            _ => panic!("Expected LimitOrder"),
        }
    }

    #[test]
    fn test_close_long() {
        // 平多 = Sell + Long + reduce_only=true
        let cmd = Command::LimitOrder {
            trader: 1,
            side: Side::Sell,
            price: 51000_00,
            quantity: 100,
            position_side: PositionSide::Long,
            reduce_only: true,
            time_in_force: TimeInForce::GTC,
        };

        match cmd {
            Command::LimitOrder { side, position_side, reduce_only, .. } => {
                assert_eq!(side, Side::Sell);
                assert_eq!(position_side, PositionSide::Long);
                assert!(reduce_only);
            }
            _ => panic!("Expected LimitOrder"),
        }
    }

    #[test]
    fn test_open_short() {
        // 开空 = Sell + Short + reduce_only=false
        let cmd = Command::MarketOrder {
            trader: 1,
            side: Side::Sell,
            quantity: 50,
            position_side: PositionSide::Short,
            reduce_only: false,
        };

        match cmd {
            Command::MarketOrder { side, position_side, reduce_only, .. } => {
                assert_eq!(side, Side::Sell);
                assert_eq!(position_side, PositionSide::Short);
                assert!(!reduce_only);
            }
            _ => panic!("Expected MarketOrder"),
        }
    }

    #[test]
    fn test_one_way_mode() {
        // 单向持仓用 Both
        let cmd = Command::LimitOrder {
            trader: 1,
            side: Side::Buy,
            price: 50000_00,
            quantity: 100,
            position_side: PositionSide::Both,
            reduce_only: false,
            time_in_force: TimeInForce::GTC,
        };

        match cmd {
            Command::LimitOrder { position_side, .. } => {
                assert_eq!(position_side, PositionSide::Both);
            }
            _ => panic!("Expected LimitOrder"),
        }
    }

    #[test]
    fn test_time_in_force_default() {
        assert_eq!(TimeInForce::default(), TimeInForce::GTC);
    }
}
