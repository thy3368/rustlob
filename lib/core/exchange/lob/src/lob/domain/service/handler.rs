//! 订单命令处理器
//!
//! 采用分层命令模式设计：
//! - SpotCommand: 核心现货订单（由 MatchingService 直接处理）
//! - AlgoCommand: 算法交易订单（由 AlgoService 处理，内部调用 SpotCommand）
//! - ConditionalCommand: 条件订单（由 ConditionalService 处理）

use crate::lob::domain::entity::lob_types::{OrderId, Price, Quantity, Side, Trade, TraderId};
use account::BalanceError;

// ============================================================================
// 核心现货命令 (SpotCommand)
// ============================================================================

/// 现货订单命令
///
/// 核心订单类型，由 MatchingService 直接处理
#[derive(Debug, Clone)]
pub enum SpotCommand {
    /// 限价单
    LimitOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// 市价单
    MarketOrder {
        trader: TraderId,
        side: Side,
        quantity: Quantity,
    },

    /// 冰山单
    IcebergOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        total_quantity: Quantity,
        display_quantity: Quantity,
    },

    /// 取消订单
    CancelOrder { order_id: OrderId },

    /// 修改订单
    ModifyOrder {
        order_id: OrderId,
        new_price: Option<Price>,
        new_quantity: Option<Quantity>,
    },

    /// 批量取消订单
    CancelAllOrders {
        trader: TraderId,
        side: Option<Side>,
    },
}

/// 现货命令执行结果
#[derive(Debug, Clone)]
pub enum SpotCommandResult {
    /// 限价单结果
    LimitOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
    },

    /// 市价单结果
    MarketOrder { trades: Vec<Trade> },

    /// 冰山单结果
    IcebergOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
        remaining_total: Quantity,
        current_display: Quantity,
    },

    /// 取消订单结果
    CancelOrder { success: bool },

    /// 修改订单结果
    ModifyOrder {
        order_id: OrderId,
        success: bool,
        new_price: Option<Price>,
        new_quantity: Option<Quantity>,
    },

    /// 批量取消订单结果
    CancelAllOrders {
        cancelled_count: usize,
        order_ids: Vec<OrderId>,
    },

    /// 账户检查失败（余额不足等）
    AccountCheckFailed { error: BalanceError },

    /// 未实现
    NotImplemented,
}

/// 现货订单处理器
///
/// 核心订单处理接口，处理 SpotCommand
pub trait SpotOrderHandler: Send + Sync {
    fn handle(&mut self, cmd: SpotCommand) -> SpotCommandResult;
}

// ============================================================================
// 算法交易命令 (AlgoCommand)
// ============================================================================

/// 紧急程度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrgencyLevel {
    Low,
    Medium,
    High,
}

/// 算法交易命令
///
/// 由 AlgoService 处理，内部拆分为多个 SpotCommand
#[derive(Debug, Clone)]
pub enum AlgoCommand {
    /// TWAP订单 (Time-Weighted Average Price)
    /// 按时间均匀分配订单
    Twap {
        trader: TraderId,
        side: Side,
        total_quantity: Quantity,
        duration_secs: u64,
        interval_secs: u64,
    },

    /// VWAP订单 (Volume-Weighted Average Price)
    /// 按市场成交量分配订单
    Vwap {
        trader: TraderId,
        side: Side,
        total_quantity: Quantity,
        target_vwap: Option<Price>,
    },

    /// POV订单 (Percentage of Volume)
    /// 按市场成交量百分比参与
    Pov {
        trader: TraderId,
        side: Side,
        total_quantity: Quantity,
        participation_rate: u32, // basis points (1/10000)
    },

    /// 实施缺口订单 (Implementation Shortfall)
    /// 最小化执行成本与决策价格的差异
    ImplementationShortfall {
        trader: TraderId,
        side: Side,
        total_quantity: Quantity,
        urgency: UrgencyLevel,
    },
}

/// 算法命令执行结果
#[derive(Debug, Clone)]
pub enum AlgoCommandResult {
    /// TWAP结果
    Twap {
        parent_order_id: OrderId,
        child_orders: Vec<OrderId>,
        total_traded: Quantity,
        avg_price: Option<Price>,
    },

    /// VWAP结果
    Vwap {
        parent_order_id: OrderId,
        child_orders: Vec<OrderId>,
        total_traded: Quantity,
        achieved_vwap: Option<Price>,
    },

    /// POV结果
    Pov {
        parent_order_id: OrderId,
        child_orders: Vec<OrderId>,
        total_traded: Quantity,
        actual_participation_rate: u32,
    },

    /// 实施缺口结果
    ImplementationShortfall {
        parent_order_id: OrderId,
        child_orders: Vec<OrderId>,
        total_traded: Quantity,
        shortfall_bps: i32, // basis points, 可为负
    },

    /// 未实现
    NotImplemented,
}

/// 算法订单处理器
pub trait AlgoOrderHandler: Send + Sync {
    fn handle(&mut self, cmd: AlgoCommand) -> AlgoCommandResult;
}

// ============================================================================
// 条件订单命令 (ConditionalCommand)
// ============================================================================

/// 钉住类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegType {
    /// 钉住最优价 (Primary Peg)
    Primary,
    /// 钉住对手价 (Market Peg)
    Market,
    /// 钉住中间价 (Midpoint Peg)
    Midpoint,
}

/// 拍卖类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuctionType {
    Opening,
    Closing,
    Intraday,
}

/// 条件订单命令
///
/// 由 ConditionalService 处理，触发后转为 SpotCommand
#[derive(Debug, Clone)]
pub enum ConditionalCommand {
    // ========== 止损订单 ==========
    /// 止损市价单
    /// 当市价达到触发价时，转为市价单
    StopMarket {
        trader: TraderId,
        side: Side,
        stop_price: Price,
        quantity: Quantity,
    },

    /// 止损限价单
    /// 当市价达到触发价时，转为限价单
    StopLimit {
        trader: TraderId,
        side: Side,
        stop_price: Price,
        limit_price: Price,
        quantity: Quantity,
    },

    /// 追踪止损单
    /// 止损价随市价变化而移动
    TrailingStop {
        trader: TraderId,
        side: Side,
        trail_amount: Price,
        quantity: Quantity,
    },

    /// 追踪止损百分比单
    TrailingStopPercent {
        trader: TraderId,
        side: Side,
        trail_percent: u32, // basis points
        quantity: Quantity,
    },

    // ========== 时间条件订单 ==========
    /// FOK订单 (Fill-Or-Kill)
    /// 立即全部成交，否则全部取消
    FillOrKill {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// IOC订单 (Immediate-Or-Cancel)
    /// 立即成交，未成交部分自动取消
    ImmediateOrCancel {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// GTD订单 (Good-Till-Date)
    /// 有效至指定时间
    GoodTillDate {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        expire_timestamp: u64,
    },

    // ========== 组合订单 ==========
    /// OCO订单 (One-Cancels-Other)
    /// 一个成交则取消另一个
    Oco {
        trader: TraderId,
        order1: Box<ConditionalCommand>,
        order2: Box<ConditionalCommand>,
    },

    /// 括号订单 (Bracket Order)
    /// 入场单 + 止盈单 + 止损单
    Bracket {
        trader: TraderId,
        side: Side,
        entry_price: Price,
        entry_quantity: Quantity,
        take_profit_price: Price,
        stop_loss_price: Price,
    },

    // ========== 高级订单 ==========
    /// 隐藏订单
    /// 完全不显示在订单簿中
    Hidden {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// 钉住订单
    /// 价格自动跟随市场最优价
    Pegged {
        trader: TraderId,
        side: Side,
        offset: i32,
        quantity: Quantity,
        peg_type: PegType,
    },

    /// 最小成交量订单
    MinimumQuantity {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        min_quantity: Quantity,
    },
}

/// 条件命令执行结果
#[derive(Debug, Clone)]
pub enum ConditionalCommandResult {
    // ========== 止损订单结果 ==========
    StopMarket {
        order_id: OrderId,
        triggered: bool,
        trades: Vec<Trade>,
    },

    StopLimit {
        order_id: OrderId,
        triggered: bool,
        trades: Vec<Trade>,
    },

    TrailingStop {
        order_id: OrderId,
        current_stop_price: Price,
        triggered: bool,
        trades: Vec<Trade>,
    },

    TrailingStopPercent {
        order_id: OrderId,
        current_stop_price: Price,
        triggered: bool,
        trades: Vec<Trade>,
    },

    // ========== 时间条件订单结果 ==========
    FillOrKill {
        filled: bool,
        trades: Vec<Trade>,
    },

    ImmediateOrCancel {
        order_id: OrderId,
        trades: Vec<Trade>,
        cancelled_quantity: Quantity,
    },

    GoodTillDate {
        order_id: OrderId,
        trades: Vec<Trade>,
    },

    // ========== 组合订单结果 ==========
    Oco {
        executed_order: Box<ConditionalCommandResult>,
        cancelled_order_id: Option<OrderId>,
    },

    Bracket {
        entry_order_id: OrderId,
        take_profit_order_id: OrderId,
        stop_loss_order_id: OrderId,
        entry_trades: Vec<Trade>,
        exit_trades: Vec<Trade>,
    },

    // ========== 高级订单结果 ==========
    Hidden {
        order_id: OrderId,
        trades: Vec<Trade>,
    },

    Pegged {
        order_id: OrderId,
        current_price: Price,
        trades: Vec<Trade>,
    },

    MinimumQuantity {
        order_id: OrderId,
        trades: Vec<Trade>,
        all_fills_meet_minimum: bool,
    },

    /// 未实现
    NotImplemented,
}

/// 条件订单处理器
pub trait ConditionalOrderHandler: Send + Sync {
    fn handle(&mut self, cmd: ConditionalCommand) -> ConditionalCommandResult;
}

// ============================================================================
// 做市商命令 (MarketMakerCommand) - 可选扩展
// ============================================================================

/// 做市商命令
#[derive(Debug, Clone)]
pub enum MarketMakerCommand {
    /// 双向报价
    TwoWayQuote {
        trader: TraderId,
        bid_price: Price,
        bid_quantity: Quantity,
        ask_price: Price,
        ask_quantity: Quantity,
    },

    /// 拍卖订单
    AuctionOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        auction_type: AuctionType,
    },
}

/// 做市商命令结果
#[derive(Debug, Clone)]
pub enum MarketMakerCommandResult {
    TwoWayQuote {
        bid_order_id: OrderId,
        ask_order_id: OrderId,
        bid_trades: Vec<Trade>,
        ask_trades: Vec<Trade>,
    },

    AuctionOrder {
        order_id: OrderId,
        auction_price: Option<Price>,
        trades: Vec<Trade>,
    },

    NotImplemented,
}

/// 做市商处理器
pub trait MarketMakerHandler: Send + Sync {
    fn handle(&mut self, cmd: MarketMakerCommand) -> MarketMakerCommandResult;
}
