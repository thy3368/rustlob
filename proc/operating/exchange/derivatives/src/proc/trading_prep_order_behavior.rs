//! 币安永续合约订单命令 - 开仓/平仓
//!
//! 币安永续合约交易的核心订单命令
//! 遵循Clean Architecture和低延迟优化标准

use std::fmt;

// ============================================================================
// 从 base_types 导入核心基础类型（Clean Architecture - 统一类型管理）
// ============================================================================
pub use base_types::exchange::prep::prep_order::{OrderStatus, OrderType};
use base_types::exchange::spot::spot_types::TimeInForce;
// ============================================================================
// 从 account crate 导入领域实体
// ============================================================================
use base_types::{AssetId, PrepPosition};
pub use base_types::{
    OrderId, OrderSide, PositionId, PositionSide, PrepTrade, Price, Quantity, TradeId, TradingPair,
};
// ============================================================================
// 核心枚举类型
// ============================================================================

pub enum PerpCmdAny {
    OpenPositionCmd(OpenPositionCmd),
    ClosePositionCmd(ClosePositionCmd),
    CancelOrderCmd(CancelOrderCmd),
    ModifyOrderCmd(ModifyOrderCmd),
    CancelAllOrdersCmd(CancelAllOrdersCmd),
    SetLeverageCmd(SetLeverageCmd),
    SetMarginTypeCmd(SetMarginTypeCmd),
    SetPositionModeCmd(SetPositionModeCmd),
}

// ============================================================================
// 开仓订单命令
// ============================================================================

/// 开仓订单命令
#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub struct OpenPositionCmd {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 订单方向（Buy=做多, Sell=做空）
    pub side: OrderSide,
    /// 订单类型
    pub order_type: OrderType,
    /// 数量
    pub quantity: Quantity,
    /// 限价单价格（市价单为None）
    pub price: Option<Price>,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 订单有效期
    pub time_in_force: TimeInForce,
    /// 杠杆倍数（1-125）
    pub leverage: u8,
}

impl OpenPositionCmd {
    /// 创建市价做多订单
    ///
    /// # 示例
    /// ```
    /// use prep_proc::proc::trading_prep_order_behavior::{OpenPositionCmd, TradingPair, AssetId, Quantity};
    ///
    /// // 开多BTC，数量1.0
    /// let cmd = OpenPositionCmd::market_long(
    ///     TradingPair::BTC_USDT,
    ///     Quantity::from_f64(1.0)
    /// );
    /// ```
    #[inline]
    pub fn market_long(trading_pair: TradingPair, quantity: Quantity) -> Self {
        Self {
            trading_pair,
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
            leverage: 1,
        }
    }

    /// 创建市价做空订单
    #[inline]
    pub fn market_short(trading_pair: TradingPair, quantity: Quantity) -> Self {
        Self {
            trading_pair,
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
            leverage: 1,
        }
    }

    /// 创建限价做多订单
    #[inline]
    pub fn limit_long(trading_pair: TradingPair, quantity: Quantity, price: Price) -> Self {
        Self {
            trading_pair,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            quantity,
            price: Some(price),
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
            leverage: 1,
        }
    }

    /// 创建限价做空订单
    #[inline]
    pub fn limit_short(trading_pair: TradingPair, quantity: Quantity, price: Price) -> Self {
        Self {
            trading_pair,
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            quantity,
            price: Some(price),
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
            leverage: 1,
        }
    }

    /// 设置杠杆倍数
    #[inline]
    pub fn with_leverage(mut self, leverage: u8) -> Self {
        self.leverage = leverage;
        self
    }

    /// 设置订单有效期
    #[inline]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }

    /// 使用单向持仓模式
    #[inline]
    pub fn both_side(mut self) -> Self {
        self.position_side = PositionSide::Both;
        self
    }

    /// 验证订单有效性
    #[inline]
    pub fn validate(&self) -> Result<(), &'static str> {
        // 验证数量
        if !self.quantity.is_positive() {
            return Err("数量必须大于0");
        }

        // 验证限价单价格
        if self.order_type == OrderType::Limit {
            match self.price {
                Some(p) if p.is_positive() => {}
                Some(_) => return Err("限价单价格必须大于0"),
                None => return Err("限价单必须指定价格"),
            }
        }

        // 验证杠杆
        if self.leverage == 0 || self.leverage > 125 {
            return Err("杠杆倍数必须在1-125之间");
        }

        Ok(())
    }
}

// ============================================================================
// 平仓订单命令
// ============================================================================

/// 平仓订单命令
#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ClosePositionCmd {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 订单方向（平多用Sell，平空用Buy）
    pub side: OrderSide,
    /// 订单类型
    pub order_type: OrderType,
    /// 数量（None表示全部平仓）
    pub quantity: Option<Quantity>,
    /// 限价单价格
    pub price: Option<Price>,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 订单有效期
    pub time_in_force: TimeInForce,
}

impl ClosePositionCmd {
    /// 市价平多仓
    ///
    /// # 示例
    /// ```
    /// use prep_proc::proc::trading_prep_order_behavior::{ClosePositionCmd, TradingPair, Quantity};
    ///
    /// // 全部平多仓
    /// let cmd = ClosePositionCmd::market_close_long(
    ///     TradingPair::new("BTCUSDT"),
    ///     None  // None表示全部平仓
    /// );
    ///
    /// // 部分平多仓
    /// let cmd = ClosePositionCmd::market_close_long(
    ///     TradingPair::new("BTCUSDT"),
    ///     Some(Quantity::from_f64(0.5))
    /// );
    /// ```
    #[inline]
    pub fn market_close_long(trading_pair: TradingPair, quantity: Option<Quantity>) -> Self {
        Self {
            trading_pair,
            side: OrderSide::Sell, // 平多用卖
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 市价平空仓
    #[inline]
    pub fn market_close_short(trading_pair: TradingPair, quantity: Option<Quantity>) -> Self {
        Self {
            trading_pair,
            side: OrderSide::Buy, // 平空用买
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 限价平多仓
    #[inline]
    pub fn limit_close_long(trading_pair: TradingPair, quantity: Quantity, price: Price) -> Self {
        Self {
            trading_pair,
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            quantity: Some(quantity),
            price: Some(price),
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 限价平空仓
    #[inline]
    pub fn limit_close_short(trading_pair: TradingPair, quantity: Quantity, price: Price) -> Self {
        Self {
            trading_pair,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            quantity: Some(quantity),
            price: Some(price),
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 市价平仓（单向持仓模式）
    #[inline]
    pub fn market_close_both(
        trading_pair: TradingPair,
        side: OrderSide,
        quantity: Option<Quantity>,
    ) -> Self {
        Self {
            trading_pair,
            side,
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Both,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 设置订单有效期
    #[inline]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }

    /// 验证订单有效性
    #[inline]
    pub fn validate(&self) -> Result<(), &'static str> {
        // 验证数量（None表示全部平仓，允许）
        if let Some(qty) = self.quantity {
            if !qty.is_positive() {
                return Err("平仓数量必须大于0");
            }
        }

        // 验证限价单价格
        if self.order_type == OrderType::Limit {
            match self.price {
                Some(p) if p.is_positive() => {}
                Some(_) => return Err("限价单价格必须大于0"),
                None => return Err("限价单必须指定价格"),
            }
        }

        Ok(())
    }
}

// ============================================================================
// 命令错误类型
// ============================================================================

/// 订单命令错误（本地撮合引擎）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrepCmdError {
    /// 验证错误
    ValidationError(&'static str),
    /// 订单ID已存在（幂等性检查）
    DuplicateOrderId(String),
    /// 余额不足
    InsufficientBalance,
    /// 持仓不足（无法平仓）
    InsufficientPosition,
    /// 订单不存在
    OrderNotFound(String),
    /// 订单状态不允许操作（如已成交无法取消）
    InvalidOrderState(String),
    /// 撮合引擎内部错误
    MatchingEngineError(String),
    /// 风控拒绝
    RiskControlRejected(String),
    /// 未知错误
    Unknown(String),
}

impl fmt::Display for PrepCmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrepCmdError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            PrepCmdError::DuplicateOrderId(id) => write!(f, "订单ID重复: {}", id),
            PrepCmdError::InsufficientBalance => write!(f, "余额不足"),
            PrepCmdError::InsufficientPosition => write!(f, "持仓不足"),
            PrepCmdError::OrderNotFound(id) => write!(f, "订单不存在: {}", id),
            PrepCmdError::InvalidOrderState(msg) => write!(f, "订单状态不允许操作: {}", msg),
            PrepCmdError::MatchingEngineError(msg) => write!(f, "撮合引擎错误: {}", msg),
            PrepCmdError::RiskControlRejected(msg) => write!(f, "风控拒绝: {}", msg),
            PrepCmdError::Unknown(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}

impl std::error::Error for PrepCmdError {}

// ============================================================================
// 命令响应类型
// ============================================================================

/// 开仓命令响应（本地撮合结果）
#[derive(Debug, Clone)]
pub struct OpenPositionResult {
    /// 订单ID
    pub order_id: OrderId,
    /// 订单状态
    pub status: OrderStatus,
    /// 成交均价（如果已成交）
    pub avg_price: Option<Price>,
    /// 已成交数量
    pub filled_quantity: Quantity,
    /// 成交明细列表（按时间顺序）
    pub trades: Vec<PrepTrade>,
    /// 撮合序列号（用于追踪撮合顺序）
    pub match_seq: Option<u64>,
    /// 创建时间戳（毫秒）
    pub created_at: u64,
    /// 最后更新时间戳（毫秒）
    pub updated_at: u64,
}

impl OpenPositionResult {
    /// 创建待撮合状态的响应
    pub fn pending(order_id: OrderId) -> Self {
        let now = current_timestamp_ms();
        Self {
            order_id,
            status: OrderStatus::Pending,
            avg_price: None,
            filled_quantity: Quantity::from_raw(0),
            trades: Vec::new(),
            match_seq: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建已接受状态的响应（进入订单簿）
    pub fn accepted(order_id: OrderId) -> Self {
        let now = current_timestamp_ms();
        Self {
            order_id,
            status: OrderStatus::Submitted,
            avg_price: None,
            filled_quantity: Quantity::from_raw(0),
            trades: Vec::new(),
            match_seq: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建已成交状态的响应
    pub fn filled(order_id: OrderId, trades: Vec<PrepTrade>, match_seq: u64) -> Self {
        let now = current_timestamp_ms();

        // 计算成交均价和总量
        let (avg_price, filled_quantity) = Self::calculate_avg_price_and_quantity(&trades);

        Self {
            order_id,
            status: OrderStatus::Filled,
            avg_price: Some(avg_price),
            filled_quantity,
            trades,
            match_seq: Some(match_seq),
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建部分成交状态的响应
    pub fn partially_filled(order_id: OrderId, trades: Vec<PrepTrade>, match_seq: u64) -> Self {
        let now = current_timestamp_ms();

        // 计算成交均价和总量
        let (avg_price, filled_quantity) = Self::calculate_avg_price_and_quantity(&trades);

        Self {
            order_id,
            status: OrderStatus::PartiallyFilled,
            avg_price: Some(avg_price),
            filled_quantity,
            trades,
            match_seq: Some(match_seq),
            created_at: now,
            updated_at: now,
        }
    }

    /// 计算成交均价和总数量
    fn calculate_avg_price_and_quantity(trades: &[PrepTrade]) -> (Price, Quantity) {
        if trades.is_empty() {
            return (Price::from_raw(0), Quantity::from_raw(0));
        }

        let mut total_notional = 0.0;
        let mut total_quantity = 0.0;

        for trade in trades {
            let notional = trade.price.to_f64() * trade.quantity.to_f64();
            total_notional += notional;
            total_quantity += trade.quantity.to_f64();
        }

        let avg_price = if total_quantity > 0.0 {
            Price::from_f64(total_notional / total_quantity)
        } else {
            Price::from_raw(0)
        };

        (avg_price, Quantity::from_f64(total_quantity))
    }
}

/// 平仓命令响应（本地撮合结果）
#[derive(Debug, Clone)]
pub struct ClosePositionResult {
    /// 订单ID
    pub order_id: OrderId,
    /// 订单状态
    pub status: OrderStatus,
    /// 成交均价（如果已成交）
    pub avg_price: Option<Price>,
    /// 已平仓数量
    pub closed_quantity: Quantity,
    /// 成交明细列表（按时间顺序）
    pub trades: Vec<PrepTrade>,
    /// 平仓盈亏
    pub realized_pnl: Option<Price>,
    /// 撮合序列号（用于追踪撮合顺序）
    pub match_seq: Option<u64>,
    /// 创建时间戳（毫秒）
    pub created_at: u64,
    /// 最后更新时间戳（毫秒）
    pub updated_at: u64,
}

impl ClosePositionResult {
    /// 创建待撮合状态的响应
    pub fn pending(order_id: OrderId) -> Self {
        let now = current_timestamp_ms();
        Self {
            order_id,
            status: OrderStatus::Pending,
            avg_price: None,
            closed_quantity: Quantity::from_raw(0),
            trades: Vec::new(),
            realized_pnl: None,
            match_seq: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建已接受状态的响应（进入订单簿）
    pub fn accepted(order_id: OrderId) -> Self {
        let now = current_timestamp_ms();
        Self {
            order_id,
            status: OrderStatus::Submitted,
            avg_price: None,
            closed_quantity: Quantity::from_raw(0),
            trades: Vec::new(),
            realized_pnl: None,
            match_seq: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建已成交状态的响应
    pub fn filled(
        order_id: OrderId,
        trades: Vec<PrepTrade>,
        realized_pnl: Price,
        match_seq: u64,
    ) -> Self {
        let now = current_timestamp_ms();

        // 计算成交均价和总量
        let (avg_price, closed_quantity) = Self::calculate_avg_price_and_quantity(&trades);

        Self {
            order_id,
            status: OrderStatus::Filled,
            avg_price: Some(avg_price),
            closed_quantity,
            trades,
            realized_pnl: Some(realized_pnl),
            match_seq: Some(match_seq),
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建部分成交状态的响应
    pub fn partially_filled(
        order_id: OrderId,
        trades: Vec<PrepTrade>,
        realized_pnl: Price,
        match_seq: u64,
    ) -> Self {
        let now = current_timestamp_ms();

        // 计算成交均价和总量
        let (avg_price, closed_quantity) = Self::calculate_avg_price_and_quantity(&trades);

        Self {
            order_id,
            status: OrderStatus::PartiallyFilled,
            avg_price: Some(avg_price),
            closed_quantity,
            trades,
            realized_pnl: Some(realized_pnl),
            match_seq: Some(match_seq),
            created_at: now,
            updated_at: now,
        }
    }

    /// 计算成交均价和总数量
    fn calculate_avg_price_and_quantity(trades: &[PrepTrade]) -> (Price, Quantity) {
        if trades.is_empty() {
            return (Price::from_raw(0), Quantity::from_raw(0));
        }

        let mut total_notional = 0.0;
        let mut total_quantity = 0.0;

        for trade in trades {
            let notional = trade.price.to_f64() * trade.quantity.to_f64();
            total_notional += notional;
            total_quantity += trade.quantity.to_f64();
        }

        let avg_price = if total_quantity > 0.0 {
            Price::from_f64(total_notional / total_quantity)
        } else {
            Price::from_raw(0)
        };

        (avg_price, Quantity::from_f64(total_quantity))
    }
}

/// 取消订单命令
#[derive(Debug, Clone)]
pub struct CancelOrderCmd {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易对
    pub trading_pair: TradingPair,
}

impl CancelOrderCmd {
    /// 创建取消订单命令
    pub fn new(order_id: OrderId, trading_pair: TradingPair) -> Self {
        Self { order_id, trading_pair }
    }
}

/// 取消订单响应
#[derive(Debug, Clone)]
pub struct CancelOrderResult {
    /// 订单ID
    pub order_id: OrderId,
    /// 是否成功取消
    pub cancelled: bool,
    /// 订单状态
    pub status: OrderStatus,
    /// 取消时间戳（毫秒）
    pub cancelled_at: u64,
}

impl CancelOrderResult {
    /// 创建成功取消的响应
    pub fn success(order_id: OrderId) -> Self {
        Self {
            order_id,
            cancelled: true,
            status: OrderStatus::Cancelled,
            cancelled_at: current_timestamp_ms(),
        }
    }

    /// 创建取消失败的响应（订单已成交等）
    pub fn failed(order_id: OrderId, status: OrderStatus) -> Self {
        Self { order_id, cancelled: false, status, cancelled_at: current_timestamp_ms() }
    }
}

/// 查询订单命令
#[derive(Debug, Clone)]
pub struct QueryOrderCmd {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易对
    pub trading_pair: TradingPair,
}

impl QueryOrderCmd {
    /// 创建查询订单命令
    pub fn new(order_id: OrderId, trading_pair: TradingPair) -> Self {
        Self { order_id, trading_pair }
    }
}

/// 订单查询结果
#[derive(Debug, Clone, Default)]
pub struct OrderQueryResult {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易对
    pub trading_pair: TradingPair,
    /// 订单方向
    pub side: OrderSide,
    /// 订单类型
    pub order_type: OrderType,
    /// 订单状态
    pub status: OrderStatus,
    /// 订单数量
    pub quantity: Quantity,
    /// 订单价格（限价单）
    pub price: Option<Price>,
    /// 已成交数量
    pub filled_quantity: Quantity,
    /// 成交均价
    pub avg_price: Option<Price>,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 创建时间戳（毫秒）
    pub created_at: u64,
    /// 更新时间戳（毫秒）
    pub updated_at: u64,
}

/// 查询持仓命令
#[derive(Debug, Clone)]
pub struct QueryPositionCmd {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 持仓方向
    pub position_side: PositionSide,
}

impl QueryPositionCmd {
    /// 创建查询持仓命令
    pub fn new(trading_pair: TradingPair, position_side: PositionSide) -> Self {
        Self { trading_pair, position_side }
    }

    /// 查询多头持仓
    pub fn long(trading_pair: TradingPair) -> Self {
        Self { trading_pair, position_side: PositionSide::Long }
    }

    /// 查询空头持仓
    pub fn short(trading_pair: TradingPair) -> Self {
        Self { trading_pair, position_side: PositionSide::Short }
    }

    /// 查询单向持仓
    pub fn both(trading_pair: TradingPair) -> Self {
        Self { trading_pair, position_side: PositionSide::Both }
    }
}

// ============================================================================
// 订单管理命令
// ============================================================================

/// 修改订单命令
#[derive(Debug, Clone)]
pub struct ModifyOrderCmd {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易对
    pub trading_pair: TradingPair,
    /// 新价格（None表示不修改）
    pub new_price: Option<Price>,
    /// 新数量（None表示不修改）
    pub new_quantity: Option<Quantity>,
}

impl ModifyOrderCmd {
    /// 创建修改订单命令
    pub fn new(order_id: OrderId, trading_pair: TradingPair) -> Self {
        Self { order_id, trading_pair, new_price: None, new_quantity: None }
    }

    /// 修改价格
    pub fn with_price(mut self, price: Price) -> Self {
        self.new_price = Some(price);
        self
    }

    /// 修改数量
    pub fn with_quantity(mut self, quantity: Quantity) -> Self {
        self.new_quantity = Some(quantity);
        self
    }

    /// 验证命令
    pub fn validate(&self) -> Result<(), &'static str> {
        // 至少要修改一项
        if self.new_price.is_none() && self.new_quantity.is_none() {
            return Err("必须指定新价格或新数量");
        }

        // 验证新价格
        if let Some(price) = self.new_price {
            if !price.is_positive() {
                return Err("新价格必须大于0");
            }
        }

        // 验证新数量
        if let Some(qty) = self.new_quantity {
            if !qty.is_positive() {
                return Err("新数量必须大于0");
            }
        }

        Ok(())
    }
}

/// 修改订单响应
#[derive(Debug, Clone)]
pub struct ModifyOrderResult {
    /// 订单ID
    pub order_id: OrderId,
    /// 是否成功修改
    pub modified: bool,
    /// 新价格
    pub new_price: Option<Price>,
    /// 新数量
    pub new_quantity: Option<Quantity>,
    /// 修改时间戳（毫秒）
    pub modified_at: u64,
}

impl ModifyOrderResult {
    /// 创建成功修改的响应
    pub fn success(
        order_id: OrderId,
        new_price: Option<Price>,
        new_quantity: Option<Quantity>,
    ) -> Self {
        Self {
            order_id,
            modified: true,
            new_price,
            new_quantity,
            modified_at: current_timestamp_ms(),
        }
    }

    /// 创建修改失败的响应
    pub fn failed(order_id: OrderId) -> Self {
        Self {
            order_id,
            modified: false,
            new_price: None,
            new_quantity: None,
            modified_at: current_timestamp_ms(),
        }
    }
}

/// 批量取消订单命令
#[derive(Debug, Clone)]
pub struct CancelAllOrdersCmd {
    /// 交易对（None表示取消所有交易对的订单）
    pub trading_pair: Option<TradingPair>,
    /// 持仓方向（None表示取消所有方向的订单）
    pub position_side: Option<PositionSide>,
}

impl CancelAllOrdersCmd {
    /// 取消所有订单
    pub fn all() -> Self {
        Self { trading_pair: None, position_side: None }
    }

    /// 取消指定交易对的所有订单
    pub fn by_symbol(trading_pair: TradingPair) -> Self {
        Self { trading_pair: Some(trading_pair), position_side: None }
    }

    /// 取消指定持仓方向的所有订单
    pub fn by_position_side(position_side: PositionSide) -> Self {
        Self { trading_pair: None, position_side: Some(position_side) }
    }

    /// 取消指定交易对和持仓方向的订单
    pub fn by_symbol_and_side(trading_pair: TradingPair, position_side: PositionSide) -> Self {
        Self { trading_pair: Some(trading_pair), position_side: Some(position_side) }
    }
}

/// 批量取消订单响应
#[derive(Debug, Clone)]
pub struct CancelAllOrdersResult {
    /// 成功取消的订单数量
    pub cancelled_count: usize,
    /// 取消失败的订单数量
    pub failed_count: usize,
    /// 成功取消的订单ID列表
    pub cancelled_order_ids: Vec<OrderId>,
    /// 取消时间戳（毫秒）
    pub cancelled_at: u64,
}

impl CancelAllOrdersResult {
    /// 创建批量取消响应
    pub fn new(cancelled_order_ids: Vec<OrderId>, failed_count: usize) -> Self {
        let cancelled_count = cancelled_order_ids.len();
        Self {
            cancelled_count,
            failed_count,
            cancelled_order_ids,
            cancelled_at: current_timestamp_ms(),
        }
    }

    /// 创建空响应（没有订单可取消）
    pub fn empty() -> Self {
        Self {
            cancelled_count: 0,
            failed_count: 0,
            cancelled_order_ids: Vec::new(),
            cancelled_at: current_timestamp_ms(),
        }
    }
}

// ============================================================================
// 第一优先级核心命令 - 账户配置和查询
// ============================================================================

/// 获取当前时间戳（毫秒）
fn current_timestamp_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

/// 保证金类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarginType {
    /// 全仓模式（共享保证金）
    Cross = 1,
    /// 逐仓模式（独立保证金）
    Isolated = 2,
}

impl MarginType {
    pub const fn as_str(self) -> &'static str {
        match self {
            MarginType::Cross => "CROSSED",
            MarginType::Isolated => "ISOLATED",
        }
    }
}

/// 1. 设置杠杆命令
///
/// # 使用场景
/// - **首次交易前必需**：每个交易对首次交易前必须设置杠杆
/// - **调整风险水平**：盈利后降低杠杆锁定利润，或市场波动时降低风险
/// - **优化资金利用率**：提高杠杆释放保证金，用于开新仓位
/// - **策略切换**：不同策略需要不同杠杆（网格3-5倍，日内10-20倍）
///
/// # 保证金影响
/// - 降低杠杆：🔒 锁定更多保证金，可用余额减少，强平价格变远（更安全）
/// - 提高杠杆：🔓 释放保证金，可用余额增加，强平价格变近（风险增加）
///
/// # 示例
/// ```ignore
/// // 首次交易BTCUSDT前设置20倍杠杆
/// let cmd = SetLeverageCommand::new(Symbol::new("BTCUSDT"), 20);
/// let result = engine.set_leverage(cmd)?;
///
/// // 盈利后降低杠杆到5倍，锁定利润
/// let cmd = SetLeverageCommand::new(Symbol::new("BTCUSDT"), 5);
/// let result = engine.set_leverage(cmd)?;
/// // result.position_margin_change > 0 表示锁定了更多保证金
/// ```
#[derive(Debug, Clone)]
pub struct SetLeverageCmd {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 杠杆倍数（1-125）
    pub leverage: u8,
}

impl SetLeverageCmd {
    /// 创建设置杠杆命令
    pub fn new(trading_pair: TradingPair, leverage: u8) -> Self {
        Self { trading_pair, leverage }
    }

    /// 验证杠杆有效性
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.leverage == 0 || self.leverage > 125 {
            return Err("杠杆倍数必须在1-125之间");
        }
        Ok(())
    }
}

/// 设置杠杆结果
#[derive(Debug, Clone)]
pub struct SetLeverageResult {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 旧杠杆倍数
    pub old_leverage: u8,
    /// 新杠杆倍数
    pub new_leverage: u8,
    /// 仓位保证金变化（正数=锁定更多，负数=释放）
    pub position_margin_change: Price,
    /// 新的可用余额
    pub available_balance: Price,
    /// 新的强平价格（如果有持仓）
    pub liquidation_price: Option<Price>,
    /// 最大可开仓数量（基于新杠杆）
    pub max_open_quantity: Quantity,
}

/// 2. 设置保证金类型命令
///
/// # 使用场景
/// - **风险隔离**：使用逐仓模式隔离不同交易对的风险，一个爆仓不影响其他
/// - **资金共享**：使用全仓模式共享保证金，提高资金利用率
/// - **策略组合**：稳健策略用逐仓，激进策略用全仓
/// - **新手保护**：新手建议用逐仓，限制单次最大亏损
///
/// # 全仓 vs 逐仓对比
/// | 特性 | 全仓 (Cross) | 逐仓 (Isolated) |
/// |------|--------------|-----------------|
/// | 保证金 | 所有持仓共享账户余额 | 每个持仓独立保证金 |
/// | 风险 | 一个爆仓可能影响全部 | 风险隔离，最多亏损该仓位保证金 |
/// | 资金利用率 | 高（共享余额） | 低（独立锁定） |
/// | 适用场景 | 经验丰富，多品种套利 | 新手，高风险单边 |
///
/// # 示例
/// ```ignore
/// // 高风险交易用逐仓，限制最大亏损
/// let cmd = SetMarginTypeCommand::new(
///     Symbol::new("BTCUSDT"),
///     MarginType::Isolated
/// );
/// engine.set_margin_type(cmd)?;
///
/// // 稳健套利策略用全仓，提高资金效率
/// let cmd = SetMarginTypeCommand::new(
///     Symbol::new("ETHUSDT"),
///     MarginType::Cross
/// );
/// engine.set_margin_type(cmd)?;
/// ```
#[derive(Debug, Clone)]
pub struct SetMarginTypeCmd {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 保证金类型
    pub margin_type: MarginType,
}

impl SetMarginTypeCmd {
    /// 创建设置保证金类型命令
    pub fn new(trading_pair: TradingPair, margin_type: MarginType) -> Self {
        Self { trading_pair, margin_type }
    }
}

/// 设置保证金类型结果
#[derive(Debug, Clone)]
pub struct SetMarginTypeResult {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 新的保证金类型
    pub margin_type: MarginType,
    /// 是否成功
    pub success: bool,
}

/// 3. 设置持仓模式命令
///
/// # 使用场景
/// - **对冲策略**：同时持有多空仓位，用于套利或对冲风险
/// - **单向交易**：只做多或只做空，简化操作
/// - **网格交易**：对冲模式支持同时挂多空网格
/// - **趋势跟踪**：单向模式适合明确方向的趋势交易
///
/// # 单向 vs 对冲模式对比
/// | 特性 | 单向模式 (One-Way) | 对冲模式 (Hedge) |
/// |------|-------------------|------------------|
/// | 持仓方式 | 只能持有一个方向 | 可同时持有多空 |
/// | 开平仓 | 反向开仓=平仓 | 多空独立开平 |
/// | 适用策略 | 趋势跟踪，单边交易 | 套利，对冲，网格 |
/// | 复杂度 | 简单 | 复杂 |
/// | 手续费 | 较低（开平合并） | 较高（独立计算） |
///
/// # 示例
/// ```ignore
/// // 对冲模式：用于网格套利
/// let cmd = SetPositionModeCommand::hedge();
/// engine.set_position_mode(cmd)?;
/// // 可以同时开多单和空单
/// open_long(...);
/// open_short(...);
///
/// // 单向模式：用于趋势交易
/// let cmd = SetPositionModeCommand::one_way();
/// engine.set_position_mode(cmd)?;
/// // 开反向单会自动平仓
/// open_long(...);
/// open_short(...);  // 自动平掉多单
/// ```
///
/// # 注意
/// - ⚠️ 切换模式前必须平掉所有持仓
/// - ⚠️ 这是全局设置，影响所有交易对
/// - ⚠️ 切换后无法撤销，需谨慎操作
#[derive(Debug, Clone)]
pub struct SetPositionModeCmd {
    /// true=对冲模式（可同时持有多空），false=单向模式
    pub dual_side: bool,
}

impl SetPositionModeCmd {
    /// 创建对冲模式命令
    pub fn hedge() -> Self {
        Self { dual_side: true }
    }

    /// 创建单向模式命令
    pub fn one_way() -> Self {
        Self { dual_side: false }
    }
}

/// 设置持仓模式结果
#[derive(Debug, Clone)]
pub struct SetPositionModeResult {
    /// 是否为对冲模式
    pub dual_side: bool,
    /// 是否成功
    pub success: bool,
}

/// 4. 查询账户余额命令
///
/// # 使用场景
/// - **开仓前检查**：确认可用余额是否足够开仓
/// - **风控监控**：实时监控账户余额，防止过度杠杆
/// - **资金管理**：计算仓位大小，控制风险敞口
/// - **对账核对**：定期核对账户余额，发现异常
///
/// # 余额类型说明
/// - **总余额 (balance)**: 账户总资产
/// - **可用余额 (available_balance)**: 可用于开新仓的余额
/// - **仓位保证金 (position_margin)**: 已持仓锁定的保证金
/// - **挂单保证金 (order_margin)**: 未成交订单锁定的保证金
///
/// # 计算关系
/// ```text
/// 总余额 = 可用余额 + 仓位保证金 + 挂单保证金 + 未实现盈亏
/// 可用余额 = 总余额 - 仓位保证金 - 挂单保证金
/// ```
///
/// # 示例
/// ```ignore
/// // 查询USDT余额
/// let cmd = QueryAccountBalanceCommand::by_asset(Symbol::new("USDT"));
/// let balances = engine.query_account_balance(cmd)?;
///
/// for balance in balances {
///     println!("资产: {}", balance.asset.as_str());
///     println!("总余额: {}", balance.balance.to_f64());
///     println!("可用: {}", balance.available_balance.to_f64());
///     println!("仓位保证金: {}", balance.position_margin.to_f64());
/// }
///
/// // 查询所有资产余额
/// let cmd = QueryAccountBalanceCommand::all();
/// let balances = engine.query_account_balance(cmd)?;
/// ```
#[derive(Debug, Clone)]
pub struct QueryAccountBalanceCmd {
    /// 资产类型（None=查询所有）
    pub asset: Option<AssetId>,
}

impl QueryAccountBalanceCmd {
    /// 查询所有资产余额
    pub fn all() -> Self {
        Self { asset: None }
    }

    /// 查询指定资产余额
    pub fn by_asset(asset: AssetId) -> Self {
        Self { asset: Some(asset) }
    }
}

/// 账户余额信息
#[derive(Debug, Clone)]
pub struct AccountBalance {
    /// 资产类型（如USDT）
    pub asset: AssetId,
    /// 总余额
    pub balance: Price,
    /// 可用余额（可用于开新仓）
    pub available_balance: Price,
    /// 仓位保证金（已持仓锁定）
    pub position_margin: Price,
    /// 挂单保证金（未成交订单锁定）
    pub order_margin: Price,
    /// 未实现盈亏
    pub unrealized_pnl: Price,
}

impl AccountBalance {
    /// 创建账户余额
    pub fn new(
        asset: AssetId,
        balance: Price,
        available_balance: Price,
        position_margin: Price,
        order_margin: Price,
        unrealized_pnl: Price,
    ) -> Self {
        Self { asset, balance, available_balance, position_margin, order_margin, unrealized_pnl }
    }
}

/// 5. 查询账户信息命令
///
/// # 使用场景
/// - **全局风控**：监控总资产、总保证金、总盈亏
/// - **仓位总览**：查看所有持仓和资产分布
/// - **风险评估**：计算账户风险率、杠杆率
/// - **报表生成**：生成账户日报、月报
///
/// # 包含信息
/// - 账户总资产和可用余额
/// - 所有持仓列表（多空分开）
/// - 所有资产余额
/// - 总未实现盈亏
/// - 总保证金占用
///
/// # 示例
/// ```ignore
/// let cmd = QueryAccountInfoCommand::new();
/// let info = engine.query_account_info(cmd)?;
///
/// println!("总资产: {}", info.total_wallet_balance.to_f64());
/// println!("可用余额: {}", info.available_balance.to_f64());
/// println!("未实现盈亏: {}", info.total_unrealized_pnl.to_f64());
/// println!("持仓数量: {}", info.positions.len());
///
/// // 计算账户风险率
/// let risk_ratio = info.total_margin_balance.to_f64()
///     / info.total_wallet_balance.to_f64();
/// println!("风险率: {:.2}%", risk_ratio * 100.0);
/// ```
#[derive(Debug, Clone)]
pub struct QueryAccountInfoCmd {}

impl QueryAccountInfoCmd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for QueryAccountInfoCmd {
    fn default() -> Self {
        Self::new()
    }
}

/// 账户完整信息
#[derive(Debug, Clone)]
pub struct AccountInfo {
    /// 总钱包余额（含未实现盈亏）
    pub total_wallet_balance: Price,
    /// 总保证金余额
    pub total_margin_balance: Price,
    /// 总未实现盈亏
    pub total_unrealized_pnl: Price,
    /// 可用余额
    pub available_balance: Price,
    /// 所有持仓列表
    pub positions: Vec<PrepPosition>,
    /// 所有资产余额
    pub assets: Vec<AccountBalance>,
    /// 更新时间戳
    pub updated_at: u64,
}

impl AccountInfo {
    /// 创建账户信息
    pub fn new(
        total_wallet_balance: Price,
        total_margin_balance: Price,
        total_unrealized_pnl: Price,
        available_balance: Price,
        positions: Vec<PrepPosition>,
        assets: Vec<AccountBalance>,
    ) -> Self {
        Self {
            total_wallet_balance,
            total_margin_balance,
            total_unrealized_pnl,
            available_balance,
            positions,
            assets,
            updated_at: current_timestamp_ms(),
        }
    }

    /// 计算账户风险率
    pub fn risk_ratio(&self) -> f64 {
        if self.total_wallet_balance.raw() == 0 {
            return 0.0;
        }
        self.total_margin_balance.to_f64() / self.total_wallet_balance.to_f64()
    }

    /// 计算账户杠杆率
    pub fn leverage_ratio(&self) -> f64 {
        if self.total_margin_balance.raw() == 0 {
            return 0.0;
        }
        let total_notional: f64 =
            self.positions.iter().map(|p| p.entry_price.to_f64() * p.quantity.to_f64()).sum();
        total_notional / self.total_margin_balance.to_f64()
    }
}

/// 6. 查询标记价格命令
///
/// # 使用场景
/// - **计算未实现盈亏**：使用标记价格而非最新价，更准确
/// - **强平价格计算**：标记价格用于判断是否触发强平
/// - **资金费率查询**：获取当前和下次资金费率
/// - **风控监控**：监控标记价格与最新价的偏离度
///
/// # 标记价格 vs 最新价
/// | 价格类型 | 用途 | 特点 |
/// |---------|------|------|
/// | 最新价 (Last Price) | 订单成交 | 实时波动，可能被操纵 |
/// | 标记价格 (Mark Price) | 强平判断、盈亏计算 | 平滑处理，防止恶意爆仓 |
/// | 指数价格 (Index Price) | 标记价格基准 | 多交易所加权平均 |
///
/// # 资金费率说明
/// - 正费率：多头支付空头（做多成本高）
/// - 负费率：空头支付多头（做空成本高）
/// - 每8小时结算一次（00:00, 08:00, 16:00 UTC）
///
/// # 示例
/// ```ignore
/// // 查询BTCUSDT标记价格
/// let cmd = QueryMarkPriceCommand::by_symbol(Symbol::new("BTCUSDT"));
/// let mark_price = engine.query_mark_price(cmd)?;
///
/// println!("标记价格: {}", mark_price.mark_price.to_f64());
/// println!("指数价格: {}", mark_price.index_price.to_f64());
/// println!("资金费率: {:.4}%", mark_price.funding_rate.to_f64() * 100.0);
/// println!("下次结算: {}", mark_price.next_funding_time);
///
/// // 计算未实现盈亏
/// let unrealized_pnl = (mark_price.mark_price.to_f64() - entry_price)
///     * position_quantity;
/// ```
#[derive(Debug, Clone)]
pub struct QueryMarkPriceCmd {
    /// 交易对（None=查询所有）
    pub trading_pair: Option<TradingPair>,
}

impl QueryMarkPriceCmd {
    /// 查询所有交易对标记价格
    pub fn all() -> Self {
        Self { trading_pair: None }
    }

    /// 查询指定交易对标记价格
    pub fn by_symbol(trading_pair: TradingPair) -> Self {
        Self { trading_pair: Some(trading_pair) }
    }
}

/// 标记价格信息
#[derive(Debug, Clone)]
pub struct MarkPriceInfo {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 标记价格（用于强平和盈亏计算）
    pub mark_price: Price,
    /// 指数价格（多交易所加权平均）
    pub index_price: Price,
    /// 当前资金费率
    pub funding_rate: Price,
    /// 下次资金费率结算时间（毫秒时间戳）
    pub next_funding_time: u64,
    /// 预估下次资金费率
    pub estimated_settle_price: Price,
    /// 更新时间戳
    pub timestamp: u64,
}

impl MarkPriceInfo {
    /// 创建标记价格信息
    pub fn new(
        trading_pair: TradingPair,
        mark_price: Price,
        index_price: Price,
        funding_rate: Price,
        next_funding_time: u64,
        estimated_settle_price: Price,
    ) -> Self {
        Self {
            trading_pair,
            mark_price,
            index_price,
            funding_rate,
            next_funding_time,
            estimated_settle_price,
            timestamp: current_timestamp_ms(),
        }
    }

    /// 判断资金费率方向
    pub fn funding_direction(&self) -> &'static str {
        if self.funding_rate.raw() > 0 {
            "多头支付空头"
        } else if self.funding_rate.raw() < 0 {
            "空头支付多头"
        } else {
            "无资金费率"
        }
    }
}

/// 7. 查询历史资金费率命令
///
/// # 使用场景
/// - **趋势分析**：分析历史资金费率趋势，判断市场情绪
/// - **成本预估**：预估持仓期间的资金费用成本
/// - **策略回测**：回测资金费率套利策略
/// - **市场研究**：研究资金费率与价格走势的关系
///
/// # 资金费率趋势分析
/// - 持续正费率且偏高：市场过度看多，考虑做空套利
/// - 持续负费率且偏低：市场过度看空，考虑做多套利
/// - 费率波动剧烈：市场情绪不稳定，谨慎操作
///
/// # 示例
/// ```ignore
/// // 查询BTCUSDT最近100次资金费率
/// let cmd = QueryFundingRateHistoryCommand::new(Symbol::new("BTCUSDT"))
///     .with_limit(100);
/// let history = engine.query_funding_rate_history(cmd)?;
///
/// // 计算平均资金费率
/// let avg_rate: f64 = history.iter()
///     .map(|r| r.funding_rate.to_f64())
///     .sum::<f64>() / history.len() as f64;
///
/// println!("平均资金费率: {:.4}%", avg_rate * 100.0);
///
/// // 判断市场情绪
/// if avg_rate > 0.001 {
///     println!("市场过度看多，考虑做空套利");
/// } else if avg_rate < -0.001 {
///     println!("市场过度看空，考虑做多套利");
/// }
///
/// // 预估7天持仓成本
/// let position_value = 50000.0;  // 1 BTC @ 50000
/// let estimated_cost = position_value * avg_rate * (7 * 3);  // 7天×3次/天
/// println!("预估7天资金费用: {} USDT", estimated_cost);
/// ```
#[derive(Debug, Clone)]
pub struct QueryFundingRateHistoryCmd {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 开始时间（毫秒时间戳，可选）
    pub start_time: Option<u64>,
    /// 结束时间（毫秒时间戳，可选）
    pub end_time: Option<u64>,
    /// 返回数量限制（默认100，最大1000）
    pub limit: usize,
}

impl QueryFundingRateHistoryCmd {
    /// 创建查询历史资金费率命令
    pub fn new(trading_pair: TradingPair) -> Self {
        Self { trading_pair, start_time: None, end_time: None, limit: 100 }
    }

    /// 设置时间范围
    pub fn with_time_range(mut self, start_time: u64, end_time: u64) -> Self {
        self.start_time = Some(start_time);
        self.end_time = Some(end_time);
        self
    }

    /// 设置开始时间
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置返回数量限制
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit.min(1000); // 最大1000
        self
    }
}

/// 历史资金费率记录
#[derive(Debug, Clone)]
pub struct FundingRateRecord {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 资金费率
    pub funding_rate: Price,
    /// 结算时间（毫秒时间戳）
    pub funding_time: u64,
}

impl FundingRateRecord {
    /// 创建资金费率记录
    pub fn new(trading_pair: TradingPair, funding_rate: Price, funding_time: u64) -> Self {
        Self { trading_pair, funding_rate, funding_time }
    }

    /// 判断费率方向
    pub fn direction(&self) -> &'static str {
        if self.funding_rate.raw() > 0 {
            "多头支付空头"
        } else if self.funding_rate.raw() < 0 {
            "空头支付多头"
        } else {
            "无资金费率"
        }
    }

    /// 判断费率是否偏高（绝对值 > 0.1%）
    pub fn is_high(&self) -> bool {
        self.funding_rate.to_f64().abs() > 0.001
    }
}

/// 8. 查询资金费用收支记录命令
///
/// # 使用场景
/// - **费用统计**：查看持仓期间实际支付/收取的资金费用
/// - **盈亏分析**：计算扣除资金费用后的真实盈亏
/// - **对账审计**：核对资金费用扣费明细
/// - **策略评估**：评估套利策略的资金费用收益
///
/// # 资金费用计算
/// ```text
/// 资金费用 = 持仓名义价值 × 资金费率
/// 持仓名义价值 = 标记价格 × 持仓数量
///
/// 正费率时：
/// - 多头持仓：支付费用（income为负）
/// - 空头持仓：收取费用（income为正）
///
/// 负费率时：
/// - 多头持仓：收取费用（income为正）
/// - 空头持仓：支付费用（income为负）
/// ```
///
/// # 示例
/// ```ignore
/// // 查询BTCUSDT的资金费用记录
/// let cmd = QueryFundingFeeCommand::by_symbol(Symbol::new("BTCUSDT"))
///     .with_limit(50);
/// let fees = engine.query_funding_fee(cmd)?;
///
/// // 统计总收支
/// let total_income: f64 = fees.iter()
///     .map(|f| f.income.to_f64())
///     .sum();
///
/// if total_income > 0.0 {
///     println!("总收入: {} USDT", total_income);
/// } else {
///     println!("总支出: {} USDT", total_income.abs());
/// }
///
/// // 计算真实盈亏
/// let position = query_position(Symbol::new("BTCUSDT"))?;
/// let unrealized_pnl = position.unrealized_pnl.to_f64();
/// let real_pnl = unrealized_pnl + total_income;
///
/// println!("未实现盈亏: {}", unrealized_pnl);
/// println!("资金费用: {}", total_income);
/// println!("真实盈亏: {}", real_pnl);
/// ```
#[derive(Debug, Clone)]
pub struct QueryFundingFeeCmd {
    /// 交易对（可选，None=查询所有）
    pub trading_pair: Option<TradingPair>,
    /// 开始时间（毫秒时间戳，可选）
    pub start_time: Option<u64>,
    /// 结束时间（毫秒时间戳，可选）
    pub end_time: Option<u64>,
    /// 返回数量限制（默认100）
    pub limit: usize,
}

impl QueryFundingFeeCmd {
    /// 创建查询所有交易对的资金费用命令
    pub fn all() -> Self {
        Self { trading_pair: None, start_time: None, end_time: None, limit: 100 }
    }

    /// 创建查询指定交易对的资金费用命令
    pub fn by_symbol(trading_pair: TradingPair) -> Self {
        Self { trading_pair: Some(trading_pair), start_time: None, end_time: None, limit: 100 }
    }

    /// 设置时间范围
    pub fn with_time_range(mut self, start_time: u64, end_time: u64) -> Self {
        self.start_time = Some(start_time);
        self.end_time = Some(end_time);
        self
    }

    /// 设置开始时间
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置返回数量限制
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl Default for QueryFundingFeeCmd {
    fn default() -> Self {
        Self::all()
    }
}

/// 资金费用记录
#[derive(Debug, Clone)]
pub struct FundingFeeRecord {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 费用金额（正数=收入，负数=支出）
    pub income: Price,
    /// 资产类型（通常是USDT）
    pub asset: TradingPair,
    /// 结算时间（毫秒时间戳）
    pub time: u64,
    /// 交易ID
    pub tran_id: String,
}

impl FundingFeeRecord {
    /// 创建资金费用记录
    pub fn new(
        trading_pair: TradingPair,
        income: Price,
        asset: TradingPair,
        time: u64,
        tran_id: String,
    ) -> Self {
        Self { trading_pair, income, asset, time, tran_id }
    }

    /// 是否为收入
    pub fn is_income(&self) -> bool {
        self.income.raw() > 0
    }

    /// 是否为支出
    pub fn is_expense(&self) -> bool {
        self.income.raw() < 0
    }

    /// 获取绝对金额
    pub fn abs_amount(&self) -> Price {
        Price::from_raw(self.income.raw().abs())
    }
}

/// 查询成交记录命令
#[derive(Debug, Clone)]
pub struct QueryTradesCmd {
    /// 按订单ID查询（可选）
    pub order_id: Option<OrderId>,
    /// 按交易对查询（可选）
    pub trading_pair: Option<TradingPair>,
    /// 开始时间戳（毫秒，可选）
    pub start_time: Option<u64>,
    /// 结束时间戳（毫秒，可选）
    pub end_time: Option<u64>,
    /// 最大返回数量（默认100）
    pub limit: usize,
}

impl QueryTradesCmd {
    /// 创建查询成交记录命令
    pub fn new() -> Self {
        Self { order_id: None, trading_pair: None, start_time: None, end_time: None, limit: 100 }
    }

    /// 按订单ID查询
    pub fn by_order_id(order_id: OrderId) -> Self {
        Self {
            order_id: Some(order_id),
            trading_pair: None,
            start_time: None,
            end_time: None,
            limit: 100,
        }
    }

    /// 按交易对查询
    pub fn by_symbol(trading_pair: TradingPair) -> Self {
        Self {
            order_id: None,
            trading_pair: Some(trading_pair),
            start_time: None,
            end_time: None,
            limit: 100,
        }
    }

    /// 按时间范围查询
    pub fn by_time_range(start_time: u64, end_time: u64) -> Self {
        Self {
            order_id: None,
            trading_pair: None,
            start_time: Some(start_time),
            end_time: Some(end_time),
            limit: 100,
        }
    }

    /// 设置返回数量限制
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// 设置订单ID过滤
    pub fn with_order_id(mut self, order_id: OrderId) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /// 设置交易对过滤
    pub fn with_symbol(mut self, trading_pair: TradingPair) -> Self {
        self.trading_pair = Some(trading_pair);
        self
    }

    /// 设置开始时间
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }
}

impl Default for QueryTradesCmd {
    fn default() -> Self {
        Self::new()
    }
}

/// 成交记录查询结果
#[derive(Debug, Clone)]
pub struct TradesQueryResult {
    /// 成交记录列表（按时间降序）
    pub trades: Vec<PrepTrade>,
    /// 总成交数量
    pub total_count: usize,
    /// 是否有更多数据
    pub has_more: bool,
}

impl TradesQueryResult {
    /// 创建成交查询结果
    pub fn new(trades: Vec<PrepTrade>, total_count: usize, has_more: bool) -> Self {
        Self { trades, total_count, has_more }
    }

    /// 创建空结果
    pub fn empty() -> Self {
        Self { trades: Vec::new(), total_count: 0, has_more: false }
    }
}

/// 查询订单簿命令
#[derive(Debug, Clone)]
pub struct QueryOrderBookCmd {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 深度档位（如20表示20档深度）
    pub depth: u32,
}

impl QueryOrderBookCmd {
    /// 创建查询订单簿命令
    pub fn new(trading_pair: TradingPair, depth: u32) -> Self {
        Self { trading_pair, depth }
    }

    /// 默认20档深度
    pub fn default_depth(trading_pair: TradingPair) -> Self {
        Self::new(trading_pair, 20)
    }
}

/// 订单簿价格档位
#[derive(Debug, Clone)]
pub struct PriceLevel {
    /// 价格
    pub price: Price,
    /// 该价格的总数量
    pub quantity: Quantity,
    /// 该档位的订单数量
    pub order_count: usize,
}

impl PriceLevel {
    /// 创建价格档位
    pub fn new(price: Price, quantity: Quantity, order_count: usize) -> Self {
        Self { price, quantity, order_count }
    }
}

/// 订单簿快照
#[derive(Debug, Clone)]
pub struct OrderBookSnapshot {
    /// 交易对
    pub trading_pair: TradingPair,
    /// 买盘（按价格从高到低排序）
    pub bids: Vec<PriceLevel>,
    /// 卖盘（按价格从低到高排序）
    pub asks: Vec<PriceLevel>,
    /// 最佳买价
    pub best_bid: Option<Price>,
    /// 最佳卖价
    pub best_ask: Option<Price>,
    /// 快照时间戳（毫秒）
    pub timestamp: u64,
}

impl OrderBookSnapshot {
    /// 创建订单簿快照
    pub fn new(trading_pair: TradingPair, bids: Vec<PriceLevel>, asks: Vec<PriceLevel>) -> Self {
        let best_bid = bids.first().map(|level| level.price);
        let best_ask = asks.first().map(|level| level.price);

        Self { trading_pair, bids, asks, best_bid, best_ask, timestamp: current_timestamp_ms() }
    }

    /// 创建空订单簿
    pub fn empty(trading_pair: TradingPair) -> Self {
        Self {
            trading_pair,
            bids: Vec::new(),
            asks: Vec::new(),
            best_bid: None,
            best_ask: None,
            timestamp: current_timestamp_ms(),
        }
    }

    /// 获取买卖价差
    pub fn spread(&self) -> Option<Price> {
        match (self.best_bid, self.best_ask) {
            (Some(bid), Some(ask)) => Some(Price::from_raw(ask.raw() - bid.raw())),
            _ => None,
        }
    }

    /// 获取中间价
    pub fn mid_price(&self) -> Option<Price> {
        match (self.best_bid, self.best_ask) {
            (Some(bid), Some(ask)) => Some(Price::from_raw((bid.raw() + ask.raw()) / 2)),
            _ => None,
        }
    }
}

// ============================================================================
// 订单处理器 Trait - CQRS Command Handler
// ============================================================================

/// 永续合约订单处理器（本地撮合引擎）
///
/// 遵循CQRS模式的命令处理接口，用于本地订单簿（LOB）撮合
/// - Command: 开仓/平仓/查询命令
/// - Result: 撮合执行结果
/// - Error: 命令执行错误
///
/// # 职责
/// - 验证订单命令有效性
/// - 风控检查（余额、持仓、杠杆等）
/// - 在本地订单簿中撮合订单
/// - 管理持仓状态和计算盈亏
/// - 返回撮合结果
///
/// # 本地撮合流程
/// 1. 命令验证 → 2. 风控检查 → 3. 订单簿撮合 → 4. 持仓更新 → 5. 返回结果
///
/// # 示例
/// ```ignore
/// use std::collections::HashMap;
///
/// struct LocalMatchingEngine {
///     order_book: OrderBook,
///     positions: HashMap<Symbol, PositionInfo>,
///     balance: Price,
/// }
///
/// impl PerpOrderProc for LocalMatchingEngine {
///     fn handle_open_position(&self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
///         // 1. 验证命令
///         cmd.validate().map_err(PrepCommandError::ValidationError)?;
///
///         // 2. 风控检查 - 检查保证金是否充足
///         let required_margin = self.calculate_required_margin(&cmd)?;
///         if self.balance < required_margin {
///             return Err(PrepCommandError::InsufficientBalance);
///         }
///
///         // 3. 生成订单ID
///         let order_id: OrderId = 123456789;
///
///         // 4. 在订单簿中撮合
///         let match_result = self.order_book.match_order(&cmd)?;
///
///         // 5. 更新持仓
///         self.update_position(&cmd, &match_result)?;
///
///         // 6. 返回结果
///         match match_result.status {
///             MatchStatus::FullyFilled => {
///                 Ok(OpenPositionResult::filled(
///                     order_id,
///                     match_result.avg_price,
///                     match_result.filled_qty,
///                     match_result.seq,
///                 ))
///             }
///             MatchStatus::PartiallyFilled => {
///                 Ok(OpenPositionResult::partially_filled(
///                     order_id,
///                     match_result.avg_price,
///                     match_result.filled_qty,
///                     match_result.seq,
///                 ))
///             }
///             MatchStatus::NoMatch => {
///                 Ok(OpenPositionResult::accepted(order_id))
///             }
///         }
///     }
///
///     fn handle_close_position(&self, cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
///         // 1. 验证命令
///         cmd.validate().map_err(PrepCommandError::ValidationError)?;
///
///         // 2. 检查持仓
///         let position = self.positions.get(&cmd.trading_pair)
///             .ok_or(PrepCommandError::InsufficientPosition)?;
///
///         if !position.has_position() {
///             return Err(PrepCommandError::InsufficientPosition);
///         }
///
///         // 3. 撮合平仓订单
///         let match_result = self.order_book.match_order(&cmd)?;
///
///         // 4. 计算盈亏
///         let pnl = self.calculate_pnl(position, &match_result)?;
///
///         // 5. 更新持仓
///         self.update_position_on_close(&cmd, &match_result)?;
///
///         // 6. 返回结果
///         Ok(ClosePositionResult::filled(
///             123456789,  // 示例 OrderId
///             match_result.avg_price,
///             match_result.filled_qty,
///             pnl,
///             match_result.seq,
///         ))
///     }
///
///     fn cancel_order(&self, cmd: CancelOrderCommand) -> Result<CancelOrderResult, PrepCommandError> {
///         // 从订单簿中移除订单
///         self.order_book.cancel_order(&cmd.order_id)?;
///         Ok(CancelOrderResult::success(cmd.order_id))
///     }
///
///     fn query_order(&self, cmd: QueryOrderCommand) -> Result<OrderQueryResult, PrepCommandError> {
///         // 从订单簿查询订单状态
///         self.order_book.get_order(&cmd.order_id)
///             .ok_or_else(|| PrepCommandError::OrderNotFound(cmd.order_id.to_string()))
///     }
///
///     fn query_position(&self, cmd: QueryPositionCommand) -> Result<PositionInfo, PrepCommandError> {
///         // 查询当前持仓
///         Ok(self.positions.get(&cmd.trading_pair)
///             .cloned()
///             .unwrap_or_else(|| PositionInfo::empty(cmd.trading_pair, cmd.position_side)))
///     }
/// }
/// ```

pub trait PerpOrderExchBehavior: Send + Sync {
    /// 处理开仓命令（本地撮合）
    ///
    /// # 参数
    /// - `cmd`: 开仓命令
    ///
    /// # 返回
    /// - `Ok(OpenPositionResult)`: 撮合成功，返回订单结果
    /// - `Err(PrepCommandError)`: 撮合失败，返回错误信息
    ///
    /// # 错误
    /// - `ValidationError`: 命令验证失败
    /// - `InsufficientBalance`: 保证金不足
    /// - `DuplicateOrderId`: 订单ID重复
    /// - `RiskControlRejected`: 风控拒绝
    /// - `MatchingEngineError`: 撮合引擎内部错误
    fn open_position(&self, cmd: OpenPositionCmd) -> Result<OpenPositionResult, PrepCmdError>;

    /// 处理平仓命令（本地撮合）
    ///
    /// # 参数
    /// - `cmd`: 平仓命令
    ///
    /// # 返回
    /// - `Ok(ClosePositionResult)`: 撮合成功，返回订单结果（含盈亏）
    /// - `Err(PrepCommandError)`: 撮合失败，返回错误信息
    ///
    /// # 错误
    /// - `ValidationError`: 命令验证失败
    /// - `InsufficientPosition`: 持仓不足
    /// - `MatchingEngineError`: 撮合引擎内部错误
    fn close_position(&self, cmd: ClosePositionCmd) -> Result<ClosePositionResult, PrepCmdError>;

    /// 处理取消订单命令
    ///
    /// # 参数
    /// - `cmd`: 取消订单命令
    ///
    /// # 返回
    /// - `Ok(CancelOrderResult)`: 取消成功或失败的结果
    /// - `Err(PrepCommandError)`: 请求失败
    ///
    /// # 错误
    /// - `OrderNotFound`: 订单不存在
    /// - `InvalidOrderState`: 订单状态不允许取消（已成交等）
    ///
    /// # 注意
    /// - 已成交的订单无法取消，返回 `CancelOrderResult::failed`
    /// - 已取消的订单重复取消，返回成功
    fn cancel_order(&self, cmd: CancelOrderCmd) -> Result<CancelOrderResult, PrepCmdError>;

    /// 修改订单（价格和/或数量）
    ///
    /// # 参数
    /// - `cmd`: 修改订单命令
    ///
    /// # 返回
    /// - `Ok(ModifyOrderResult)`: 修改成功或失败的结果
    /// - `Err(PrepCommandError)`: 请求失败
    ///
    /// # 错误
    /// - `OrderNotFound`: 订单不存在
    /// - `InvalidOrderState`: 订单状态不允许修改（已成交等）
    /// - `ValidationError`: 新价格或新数量无效
    ///
    /// # 注意
    /// - 只能修改未成交或部分成交的订单
    /// - 修改订单会重新进入订单簿撮合队列
    /// - 至少要修改价格或数量中的一项
    fn modify_order(&self, cmd: ModifyOrderCmd) -> Result<ModifyOrderResult, PrepCmdError>;

    /// 批量取消订单
    ///
    /// # 参数
    /// - `cmd`: 批量取消订单命令
    ///
    /// # 返回
    /// - `Ok(CancelAllOrdersResult)`: 批量取消结果（含成功/失败数量）
    /// - `Err(PrepCommandError)`: 请求失败
    ///
    /// # 取消范围
    /// - `symbol = None, position_side = None`: 取消所有订单
    /// - `symbol = Some(s), position_side = None`: 取消指定交易对的所有订单
    /// - `symbol = None, position_side = Some(p)`: 取消指定持仓方向的所有订单
    /// - `symbol = Some(s), position_side = Some(p)`:
    ///   取消指定交易对和方向的订单
    ///
    /// # 注意
    /// - 已成交的订单无法取消
    /// - 返回成功取消的订单数量和失败的订单数量
    fn cancel_all_orders(
        &self,
        cmd: CancelAllOrdersCmd,
    ) -> Result<CancelAllOrdersResult, PrepCmdError>;

    // ========================================================================
    // 第一优先级核心方法 - 账户配置和查询
    // ========================================================================

    /// 设置杠杆倍数
    ///
    /// # 使用场景
    /// - 首次交易前必需设置
    /// - 调整风险水平（盈利后降低杠杆）
    /// - 优化资金利用率（提高杠杆释放保证金）
    ///
    /// # 参数
    /// - `cmd`: 设置杠杆命令
    ///
    /// # 返回
    /// - `Ok(SetLeverageResult)`: 设置成功，返回保证金变化信息
    /// - `Err(PrepCommandError)`: 设置失败
    ///
    /// # 错误
    /// - `ValidationError`: 杠杆倍数无效（必须1-125）
    /// - `InsufficientBalance`: 降低杠杆时可用余额不足
    ///
    /// # 注意
    /// - 降低杠杆会锁定更多保证金
    /// - 提高杠杆会释放保证金但增加强平风险
    fn set_leverage(&self, cmd: SetLeverageCmd) -> Result<SetLeverageResult, PrepCmdError>;

    /// 设置保证金类型
    ///
    /// # 使用场景
    /// - 风险隔离：逐仓模式隔离不同交易对风险
    /// - 资金共享：全仓模式提高资金利用率
    ///
    /// # 参数
    /// - `cmd`: 设置保证金类型命令
    ///
    /// # 返回
    /// - `Ok(SetMarginTypeResult)`: 设置成功
    /// - `Err(PrepCommandError)`: 设置失败
    ///
    /// # 错误
    /// - `InvalidOrderState`: 有持仓时无法切换保证金类型
    ///
    /// # 注意
    /// - 必须在无持仓时设置
    /// - 每个交易对独立设置
    fn set_margin_type(&self, cmd: SetMarginTypeCmd) -> Result<SetMarginTypeResult, PrepCmdError>;

    /// 设置持仓模式
    ///
    /// # 使用场景
    /// - 对冲策略：同时持有多空仓位
    /// - 单向交易：简化操作，只做一个方向
    ///
    /// # 参数
    /// - `cmd`: 设置持仓模式命令
    ///
    /// # 返回
    /// - `Ok(SetPositionModeResult)`: 设置成功
    /// - `Err(PrepCommandError)`: 设置失败
    ///
    /// # 错误
    /// - `InvalidOrderState`: 有持仓时无法切换模式
    ///
    /// # 注意
    /// - ⚠️ 全局设置，影响所有交易对
    /// - ⚠️ 必须在无持仓时设置
    /// - ⚠️ 切换后无法撤销
    fn set_position_mode(
        &self,
        cmd: SetPositionModeCmd,
    ) -> Result<SetPositionModeResult, PrepCmdError>;
}

pub trait PerpOrderExchQueryProc: Send + Sync {
    /// 查询订单状态（从本地订单簿）
    ///
    /// # 参数
    /// - `cmd`: 查询订单命令
    ///
    /// # 返回
    /// - `Ok(OrderQueryResult)`: 订单详细信息
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 错误
    /// - `OrderNotFound`: 订单不存在
    fn query_order(&self, cmd: QueryOrderCmd) -> Result<OrderQueryResult, PrepCmdError>;

    /// 查询持仓信息（从本地持仓管理器）
    ///
    /// # 参数
    /// - `cmd`: 查询持仓命令
    ///
    /// # 返回
    /// - `Ok(PositionInfo)`: 持仓详细信息（无持仓返回空持仓）
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 注意
    /// - 无持仓时返回 `PositionInfo::empty()`，而不是返回错误
    /// - 可通过 `has_position()` 判断是否有持仓
    fn query_position(&self, cmd: QueryPositionCmd) -> Result<PrepPosition, PrepCmdError>;

    /// 查询订单簿深度
    ///
    /// # 参数
    /// - `cmd`: 查询订单簿命令
    ///
    /// # 返回
    /// - `Ok(OrderBookSnapshot)`: 订单簿快照（含买卖盘深度）
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 快照内容
    /// - 买盘档位（按价格从高到低排序）
    /// - 卖盘档位（按价格从低到高排序）
    /// - 最佳买价和最佳卖价
    /// - 快照时间戳
    ///
    /// # 注意
    /// - 返回的是快照数据，可能与实时订单簿有延迟
    /// - 深度档位数量由命令参数指定
    fn query_order_book(&self, cmd: QueryOrderBookCmd) -> Result<OrderBookSnapshot, PrepCmdError>;

    /// 查询成交记录
    ///
    /// # 参数
    /// - `cmd`: 查询成交记录命令
    ///
    /// # 返回
    /// - `Ok(TradesQueryResult)`: 成交记录列表
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 查询条件
    /// - 支持按订单ID、交易对、时间范围过滤
    /// - 支持限制返回数量（分页）
    /// - 成交记录按时间降序排列（最新的在前）
    ///
    /// # 用途
    /// - 查看订单的成交明细
    /// - 统计交易手续费
    /// - 对账和调试
    /// - 生成交易报表
    ///
    /// # 注意
    /// - 返回的是历史成交记录
    /// - 建议设置合理的limit避免性能问题
    fn query_trades(&self, cmd: QueryTradesCmd) -> Result<TradesQueryResult, PrepCmdError>;

    /// 查询账户余额
    ///
    /// # 使用场景
    /// - 开仓前检查可用余额
    /// - 风控监控账户余额
    /// - 资金管理和仓位计算
    ///
    /// # 参数
    /// - `cmd`: 查询账户余额命令
    ///
    /// # 返回
    /// - `Ok(Vec<AccountBalance>)`: 账户余额列表
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 注意
    /// - 可查询单个资产或所有资产
    /// - 包含可用余额、仓位保证金、挂单保证金
    fn query_account_balance(
        &self,
        cmd: QueryAccountBalanceCmd,
    ) -> Result<Vec<AccountBalance>, PrepCmdError>;

    /// 查询账户完整信息
    ///
    /// # 使用场景
    /// - 全局风控监控
    /// - 仓位总览
    /// - 风险评估
    /// - 报表生成
    ///
    /// # 参数
    /// - `cmd`: 查询账户信息命令
    ///
    /// # 返回
    /// - `Ok(AccountInfo)`: 账户完整信息
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 包含信息
    /// - 总资产、可用余额、总盈亏
    /// - 所有持仓列表
    /// - 所有资产余额
    /// - 风险率、杠杆率计算
    fn query_account_info(&self, cmd: QueryAccountInfoCmd) -> Result<AccountInfo, PrepCmdError>;

    /// 查询标记价格
    ///
    /// # 使用场景
    /// - 计算未实现盈亏（使用标记价格）
    /// - 强平价格计算
    /// - 资金费率查询
    /// - 风控监控
    ///
    /// # 参数
    /// - `cmd`: 查询标记价格命令
    ///
    /// # 返回
    /// - `Ok(Vec<MarkPriceInfo>)`: 标记价格列表
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 注意
    /// - 标记价格用于强平判断，不是最新成交价
    /// - 包含资金费率和下次结算时间
    /// - 可查询单个交易对或所有交易对
    fn query_mark_price(&self, cmd: QueryMarkPriceCmd) -> Result<Vec<MarkPriceInfo>, PrepCmdError>;

    /// 查询历史资金费率
    ///
    /// # 使用场景
    /// - 趋势分析：分析历史费率趋势，判断市场情绪
    /// - 成本预估：预估持仓期间的资金费用成本
    /// - 策略回测：回测资金费率套利策略
    ///
    /// # 参数
    /// - `cmd`: 查询历史资金费率命令
    ///
    /// # 返回
    /// - `Ok(Vec<FundingRateRecord>)`: 历史资金费率列表
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 注意
    /// - 返回按时间降序排列（最新的在前）
    /// - 最多返回1000条记录
    /// - 可用于计算平均费率预估持仓成本
    fn query_funding_rate_history(
        &self,
        cmd: QueryFundingRateHistoryCmd,
    ) -> Result<Vec<FundingRateRecord>, PrepCmdError>;

    /// 查询资金费用收支记录
    ///
    /// # 使用场景
    /// - 费用统计：查看实际支付/收取的资金费用
    /// - 盈亏分析：计算扣除资金费用后的真实盈亏
    /// - 对账审计：核对资金费用扣费明细
    /// - 策略评估：评估套利策略的资金费用收益
    ///
    /// # 参数
    /// - `cmd`: 查询资金费用记录命令
    ///
    /// # 返回
    /// - `Ok(Vec<FundingFeeRecord>)`: 资金费用记录列表
    /// - `Err(PrepCommandError)`: 查询失败
    ///
    /// # 注意
    /// - income为正表示收入，为负表示支出
    /// - 可查询单个交易对或所有交易对
    /// - 用于计算真实盈亏（未实现盈亏 + 资金费用）
    fn query_funding_fee(
        &self,
        cmd: QueryFundingFeeCmd,
    ) -> Result<Vec<FundingFeeRecord>, PrepCmdError>;
}
// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {}
