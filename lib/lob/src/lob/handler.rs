use crate::lob::{OrderId, Price, Quantity, Side, Trade, TraderId};

/// 钉住订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegType {
    /// 钉住最优价（Primary Peg）
    Primary,
    /// 钉住对手价（Market Peg）
    Market,
    /// 钉住中间价（Midpoint Peg）
    Midpoint,
}

/// 拍卖类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuctionType {
    /// 开盘拍卖
    Opening,
    /// 收盘拍卖
    Closing,
    /// 盘中拍卖
    Intraday,
}

/// 紧急程度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrgencyLevel {
    /// 低紧急度
    Low,
    /// 中等紧急度
    Medium,
    /// 高紧急度
    High,
}

/// 订单命令枚举
///
/// 定义所有支持的订单类型命令
#[derive(Debug, Clone)]
pub enum Command {
    // ========== 基础订单类型 ==========
    /// 限价单命令 ✅ 已实现
    LimitOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// 市价单命令 ✅ 已实现
    MarketOrder {
        trader: TraderId,
        side: Side,
        quantity: Quantity,
    },

    /// 冰山单命令 ✅ 已实现
    IcebergOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        total_quantity: Quantity,
        display_quantity: Quantity,
    },

    /// 取消订单命令 ✅ 已实现
    CancelOrder { order_id: OrderId },

    // ========== 时间条件订单 (Time-In-Force) ==========
    /// FOK订单 (Fill-Or-Kill) 🔧 待实现
    /// 立即全部成交，否则全部取消
    FillOrKillOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// IOC订单 (Immediate-Or-Cancel) 🔧 待实现
    /// 立即成交，未成交部分自动取消
    ImmediateOrCancelOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// AON订单 (All-Or-None) 🔧 待实现
    /// 必须全部成交，否则保留在订单簿
    AllOrNoneOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// GTD订单 (Good-Till-Date) 🔧 待实现
    /// 有效至指定时间
    GoodTillDateOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        expire_time: u64, // Unix timestamp
    },

    // ========== 止损订单 (Stop Orders) ==========
    /// 止损市价单 (Stop Market) 🔧 待实现
    /// 当市价达到触发价时，转为市价单
    StopMarketOrder {
        trader: TraderId,
        side: Side,
        stop_price: Price,
        quantity: Quantity,
    },

    /// 止损限价单 (Stop Limit) 🔧 待实现
    /// 当市价达到触发价时，转为限价单
    StopLimitOrder {
        trader: TraderId,
        side: Side,
        stop_price: Price,
        limit_price: Price,
        quantity: Quantity,
    },

    /// 追踪止损单 (Trailing Stop) 🔧 待实现
    /// 止损价随市价变化而移动
    TrailingStopOrder {
        trader: TraderId,
        side: Side,
        trail_amount: Price,
        quantity: Quantity,
    },

    /// 追踪止损百分比单 🔧 待实现
    TrailingStopPercentOrder {
        trader: TraderId,
        side: Side,
        trail_percent: u32, // basis points (1/10000)
        quantity: Quantity,
    },

    // ========== 订单修改命令 ==========
    /// 修改订单 🔧 待实现
    ModifyOrder {
        order_id: OrderId,
        new_price: Option<Price>,
        new_quantity: Option<Quantity>,
    },

    /// 取消并替换订单 (Cancel-Replace) 🔧 待实现
    CancelReplaceOrder {
        old_order_id: OrderId,
        new_price: Price,
        new_quantity: Quantity,
    },

    /// 批量取消订单 🔧 待实现
    CancelAllOrders {
        trader: TraderId,
        side: Option<Side>,
    },

    // ========== 高级订单类型 ==========
    /// 隐藏订单 (Hidden Order) 🔧 待实现
    /// 完全不显示在订单簿中
    HiddenOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    },

    /// 钉住订单 (Pegged Order) 🔧 待实现
    /// 价格自动跟随市场最优价
    PeggedOrder {
        trader: TraderId,
        side: Side,
        offset: i32,
        quantity: Quantity,
        peg_type: PegType,
    },

    /// 最小成交量订单 🔧 待实现
    MinimumQuantityOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        min_quantity: Quantity,
    },

    /// 双向报价 (Two-Way Quote) 🔧 待实现
    TwoWayQuote {
        trader: TraderId,
        bid_price: Price,
        bid_quantity: Quantity,
        ask_price: Price,
        ask_quantity: Quantity,
    },

    // ========== 算法交易订单 ==========
    /// TWAP订单 (Time-Weighted Average Price) 🔧 待实现
    TwapOrder {
        trader: TraderId,
        side: Side,
        total_quantity: Quantity,
        duration: u64,
        interval: u64,
    },

    /// VWAP订单 (Volume-Weighted Average Price) 🔧 待实现
    VwapOrder {
        trader: TraderId,
        side: Side,
        total_quantity: Quantity,
        target_vwap: Option<Price>,
    },

    /// POV订单 (Percentage of Volume) 🔧 待实现
    PovOrder {
        trader: TraderId,
        side: Side,
        total_quantity: Quantity,
        participation_rate: u32,
    },

    /// 实施缺口订单 (Implementation Shortfall) 🔧 待实现
    ImplementationShortfallOrder {
        trader: TraderId,
        side: Side,
        total_quantity: Quantity,
        urgency: UrgencyLevel,
    },

    // ========== 条件订单 ==========
    /// OCO订单 (One-Cancels-Other) 🔧 待实现
    OcoOrder {
        trader: TraderId,
        order1: Box<Command>,
        order2: Box<Command>,
    },

    /// 括号订单 (Bracket Order) 🔧 待实现
    BracketOrder {
        trader: TraderId,
        side: Side,
        entry_price: Price,
        entry_quantity: Quantity,
        take_profit_price: Price,
        stop_loss_price: Price,
    },

    // ========== 交易所特定订单 ==========
    /// 拍卖订单 🔧 待实现
    AuctionOrder {
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        auction_type: AuctionType,
    },

    /// 做市商双边报价 🔧 待实现
    MarketMakerQuote {
        trader: TraderId,
        bid_price: Price,
        bid_quantity: Quantity,
        ask_price: Price,
        ask_quantity: Quantity,
        mm_id: String,
    },
}

/// 命令执行结果
///
/// 封装不同命令的执行结果
#[derive(Debug, Clone)]
pub enum CommandResult {
    // ========== 基础订单类型结果 ==========
    /// 限价单结果 ✅
    LimitOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
    },

    ToDo {

    },
    

    /// 市价单结果 ✅
    MarketOrder { trades: Vec<Trade> },

    /// 冰山单结果 ✅
    IcebergOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
        remaining_total: Quantity,
        current_display: Quantity,
    },

    /// 取消订单结果 ✅
    CancelOrder { success: bool },

    // ========== 时间条件订单结果 ==========
    /// FOK订单结果 🔧
    FillOrKillOrder { filled: bool, trades: Vec<Trade> },

    /// IOC订单结果 🔧
    ImmediateOrCancelOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
        cancelled_quantity: Quantity,
    },

    /// AON订单结果 🔧
    AllOrNoneOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
        filled: bool,
    },

    /// GTD订单结果 🔧
    GoodTillDateOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
    },

    // ========== 止损订单结果 ==========
    /// 止损市价单结果 🔧
    StopMarketOrder {
        order_id: OrderId,
        triggered: bool,
        trades: Vec<Trade>,
    },

    /// 止损限价单结果 🔧
    StopLimitOrder {
        order_id: OrderId,
        triggered: bool,
        trades: Vec<Trade>,
    },

    /// 追踪止损单结果 🔧
    TrailingStopOrder {
        order_id: OrderId,
        current_stop_price: Price,
        triggered: bool,
        trades: Vec<Trade>,
    },

    /// 追踪止损百分比单结果 🔧
    TrailingStopPercentOrder {
        order_id: OrderId,
        current_stop_price: Price,
        triggered: bool,
        trades: Vec<Trade>,
    },

    // ========== 订单修改结果 ==========
    /// 修改订单结果 🔧
    ModifyOrder {
        order_id: OrderId,
        success: bool,
        new_price: Option<Price>,
        new_quantity: Option<Quantity>,
    },

    /// 取消并替换订单结果 🔧
    CancelReplaceOrder {
        old_order_id: OrderId,
        new_order_id: OrderId,
        success: bool,
        trades: Vec<Trade>,
    },

    /// 批量取消订单结果 🔧
    CancelAllOrders {
        cancelled_count: usize,
        order_ids: Vec<OrderId>,
    },

    // ========== 高级订单类型结果 ==========
    /// 隐藏订单结果 🔧
    HiddenOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
    },

    /// 钉住订单结果 🔧
    PeggedOrder {
        order_id: OrderId,
        current_price: Price,
        trades: Vec<Trade>,
    },

    /// 最小成交量订单结果 🔧
    MinimumQuantityOrder {
        order_id: OrderId,
        trades: Vec<Trade>,
        all_fills_meet_minimum: bool,
    },

    /// 双向报价结果 🔧
    TwoWayQuote {
        bid_order_id: OrderId,
        ask_order_id: OrderId,
        bid_trades: Vec<Trade>,
        ask_trades: Vec<Trade>,
    },

    // ========== 算法交易订单结果 ==========
    /// TWAP订单结果 🔧
    TwapOrder {
        parent_order_id: OrderId,
        child_orders: Vec<OrderId>,
        total_traded: Quantity,
        avg_price: Option<Price>,
    },

    /// VWAP订单结果 🔧
    VwapOrder {
        parent_order_id: OrderId,
        child_orders: Vec<OrderId>,
        total_traded: Quantity,
        vwap: Option<Price>,
    },

    /// POV订单结果 🔧
    PovOrder {
        parent_order_id: OrderId,
        child_orders: Vec<OrderId>,
        total_traded: Quantity,
        actual_participation_rate: u32,
    },

    /// 实施缺口订单结果 🔧
    ImplementationShortfallOrder {
        parent_order_id: OrderId,
        child_orders: Vec<OrderId>,
        total_traded: Quantity,
        implementation_shortfall: i64, // 可以为负（成本）
    },

    // ========== 条件订单结果 ==========
    /// OCO订单结果 🔧
    OcoOrder {
        executed_order: Box<CommandResult>,
        cancelled_order_id: Option<OrderId>,
    },

    /// 括号订单结果 🔧
    BracketOrder {
        entry_order_id: OrderId,
        take_profit_order_id: OrderId,
        stop_loss_order_id: OrderId,
        entry_trades: Vec<Trade>,
        exit_trades: Vec<Trade>,
    },

    // ========== 交易所特定订单结果 ==========
    /// 拍卖订单结果 🔧
    AuctionOrder {
        order_id: OrderId,
        auction_price: Option<Price>,
        trades: Vec<Trade>,
    },

    /// 做市商双边报价结果 🔧
    MarketMakerQuote {
        bid_order_id: OrderId,
        ask_order_id: OrderId,
        bid_trades: Vec<Trade>,
        ask_trades: Vec<Trade>,
        spread: Price,
    },
}

/// 订单命令处理器trait
///
/// 定义所有订单类型的处理接口，支持：
/// - 限价单 (Limit Order)
/// - 市价单 (Market Order)
/// - 取消单 (Cancel Order)
/// - 冰山单 (Iceberg Order)
///
/// 遵循Clean Architecture原则：
/// - 处理器专注于业务逻辑，不依赖仓储层
/// - 纯函数式命令处理，输入命令返回结果
/// - 仓储操作由用例层(Use Case Layer)负责
pub trait OrderCommandHandler: Send + Sync {
    /// 统一的命令处理API
    ///
    /// # 参数
    /// - `command`: 订单命令
    ///
    /// # 返回
    /// - `CommandResult`: 命令执行结果
    fn handle(&mut self, command: Command) -> CommandResult;

    /// 获取处理器名称
    fn handler_name(&self) -> &'static str;
}
