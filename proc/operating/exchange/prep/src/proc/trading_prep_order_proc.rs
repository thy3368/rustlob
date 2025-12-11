//! 币安永续合约订单命令 - 开仓/平仓
//!
//! 币安永续合约交易的核心订单命令
//! 遵循Clean Architecture和低延迟优化标准

use std::fmt;

// ============================================================================
// 核心枚举类型
// ============================================================================

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Side {
    /// 买入
    Buy = 1,
    /// 卖出
    Sell = 2,
}

impl Side {
    /// 获取相反方向
    #[inline(always)]
    pub const fn opposite(self) -> Self {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }

    /// 转换为字符串
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        }
    }
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OrderType {
    /// 市价单
    Market = 1,
    /// 限价单
    Limit = 2,
}

impl OrderType {
    /// 转换为字符串
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
        }
    }
}

/// 持仓方向（币安对冲模式）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PositionSide {
    /// 单向持仓模式
    Both = 1,
    /// 多头持仓
    Long = 2,
    /// 空头持仓
    Short = 3,
}

impl PositionSide {
    /// 转换为字符串
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            PositionSide::Both => "BOTH",
            PositionSide::Long => "LONG",
            PositionSide::Short => "SHORT",
        }
    }
}

/// 订单有效期类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeInForce {
    /// 成交为止（一直有效）
    GTC = 1,
    /// 立即成交或取消
    IOC = 2,
    /// 全部成交或取消
    FOK = 3,
    /// 只做Maker（Post only）
    GTX = 4,
}

impl TimeInForce {
    /// 转换为字符串
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
            TimeInForce::GTX => "GTX",
        }
    }
}

// ============================================================================
// 值对象 - 固定精度算术
// ============================================================================

/// 价格（8位小数精度）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Price(i64);

impl Price {
    const SCALE: i64 = 100_000_000;

    /// 从原始值创建
    #[inline(always)]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    /// 从浮点数创建
    #[inline(always)]
    pub fn from_f64(value: f64) -> Self {
        Self((value * Self::SCALE as f64) as i64)
    }

    /// 获取原始值
    #[inline(always)]
    pub const fn raw(self) -> i64 {
        self.0
    }

    /// 转换为浮点数
    #[inline(always)]
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / Self::SCALE as f64
    }

    /// 是否为正数
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.8}", self.to_f64())
    }
}

/// 数量（8位小数精度）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Quantity(i64);

impl Quantity {
    const SCALE: i64 = 100_000_000;

    /// 从原始值创建
    #[inline(always)]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    /// 从浮点数创建
    #[inline(always)]
    pub fn from_f64(value: f64) -> Self {
        Self((value * Self::SCALE as f64) as i64)
    }

    /// 获取原始值
    #[inline(always)]
    pub const fn raw(self) -> i64 {
        self.0
    }

    /// 转换为浮点数
    #[inline(always)]
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / Self::SCALE as f64
    }

    /// 是否为正数
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    /// 是否为零
    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.8}", self.to_f64())
    }
}

/// 交易对符号
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol([u8; 16]);

impl Symbol {
    /// 创建新的交易对符号
    #[inline]
    pub fn new(s: &str) -> Self {
        let mut data = [0u8; 16];
        let bytes = s.as_bytes();
        let len = bytes.len().min(16);
        data[..len].copy_from_slice(&bytes[..len]);
        Self(data)
    }

    /// 转换为字符串
    #[inline]
    pub fn as_str(&self) -> &str {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(16);
        unsafe { std::str::from_utf8_unchecked(&self.0[..len]) }
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// 开仓订单命令
// ============================================================================

/// 开仓订单命令
#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub struct OpenPositionCommand {
    /// 交易对
    pub symbol: Symbol,
    /// 订单方向（Buy=做多, Sell=做空）
    pub side: Side,
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

impl OpenPositionCommand {
    /// 创建市价做多订单
    ///
    /// # 示例
    /// ```
    /// use prep_proc::proc::trading_prep_order_proc::{OpenPositionCommand, Symbol, Quantity};
    ///
    /// // 开多BTC，数量1.0
    /// let cmd = OpenPositionCommand::market_long(
    ///     Symbol::new("BTCUSDT"),
    ///     Quantity::from_f64(1.0)
    /// );
    /// ```
    #[inline]
    pub fn market_long(symbol: Symbol, quantity: Quantity) -> Self {
        Self {
            symbol,
            side: Side::Buy,
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
    pub fn market_short(symbol: Symbol, quantity: Quantity) -> Self {
        Self {
            symbol,
            side: Side::Sell,
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
    pub fn limit_long(symbol: Symbol, quantity: Quantity, price: Price) -> Self {
        Self {
            symbol,
            side: Side::Buy,
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
    pub fn limit_short(symbol: Symbol, quantity: Quantity, price: Price) -> Self {
        Self {
            symbol,
            side: Side::Sell,
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
pub struct ClosePositionCommand {
    /// 交易对
    pub symbol: Symbol,
    /// 订单方向（平多用Sell，平空用Buy）
    pub side: Side,
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

impl ClosePositionCommand {
    /// 市价平多仓
    ///
    /// # 示例
    /// ```
    /// use prep_proc::proc::trading_prep_order_proc::{ClosePositionCommand, Symbol, Quantity};
    ///
    /// // 全部平多仓
    /// let cmd = ClosePositionCommand::market_close_long(
    ///     Symbol::new("BTCUSDT"),
    ///     None  // None表示全部平仓
    /// );
    ///
    /// // 部分平多仓
    /// let cmd = ClosePositionCommand::market_close_long(
    ///     Symbol::new("BTCUSDT"),
    ///     Some(Quantity::from_f64(0.5))
    /// );
    /// ```
    #[inline]
    pub fn market_close_long(symbol: Symbol, quantity: Option<Quantity>) -> Self {
        Self {
            symbol,
            side: Side::Sell,  // 平多用卖
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 市价平空仓
    #[inline]
    pub fn market_close_short(symbol: Symbol, quantity: Option<Quantity>) -> Self {
        Self {
            symbol,
            side: Side::Buy,  // 平空用买
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 限价平多仓
    #[inline]
    pub fn limit_close_long(symbol: Symbol, quantity: Quantity, price: Price) -> Self {
        Self {
            symbol,
            side: Side::Sell,
            order_type: OrderType::Limit,
            quantity: Some(quantity),
            price: Some(price),
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 限价平空仓
    #[inline]
    pub fn limit_close_short(symbol: Symbol, quantity: Quantity, price: Price) -> Self {
        Self {
            symbol,
            side: Side::Buy,
            order_type: OrderType::Limit,
            quantity: Some(quantity),
            price: Some(price),
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// 市价平仓（单向持仓模式）
    #[inline]
    pub fn market_close_both(symbol: Symbol, side: Side, quantity: Option<Quantity>) -> Self {
        Self {
            symbol,
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
pub enum PrepCommandError {
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

impl fmt::Display for PrepCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrepCommandError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            PrepCommandError::DuplicateOrderId(id) => write!(f, "订单ID重复: {}", id),
            PrepCommandError::InsufficientBalance => write!(f, "余额不足"),
            PrepCommandError::InsufficientPosition => write!(f, "持仓不足"),
            PrepCommandError::OrderNotFound(id) => write!(f, "订单不存在: {}", id),
            PrepCommandError::InvalidOrderState(msg) => write!(f, "订单状态不允许操作: {}", msg),
            PrepCommandError::MatchingEngineError(msg) => write!(f, "撮合引擎错误: {}", msg),
            PrepCommandError::RiskControlRejected(msg) => write!(f, "风控拒绝: {}", msg),
            PrepCommandError::Unknown(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}

impl std::error::Error for PrepCommandError {}

// ============================================================================
// 命令响应类型
// ============================================================================

/// 订单ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId(String);

impl OrderId {
    /// 创建新的订单ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// 生成随机订单ID
    pub fn generate() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(format!("ORD-{}", timestamp))
    }

    /// 获取字符串表示
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 成交ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TradeId(String);

impl TradeId {
    /// 创建新的成交ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// 生成随机成交ID
    pub fn generate() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(format!("TRD-{}", timestamp))
    }

    /// 获取字符串表示
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TradeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 成交记录（单次撮合成交）
#[derive(Debug, Clone)]
pub struct Trade {
    /// 成交ID
    pub trade_id: TradeId,
    /// 关联订单ID
    pub order_id: OrderId,
    /// 交易对
    pub symbol: Symbol,
    /// 订单方向
    pub side: Side,
    /// 成交价格
    pub price: Price,
    /// 成交数量
    pub quantity: Quantity,
    /// 手续费
    pub fee: Price,
    /// 手续费资产（通常是USDT）
    pub fee_asset: Symbol,
    /// 是否为Maker（流动性提供方）
    pub is_maker: bool,
    /// 成交时间戳（毫秒）
    pub timestamp: u64,
}

impl Trade {
    /// 创建新的成交记录
    pub fn new(
        trade_id: TradeId,
        order_id: OrderId,
        symbol: Symbol,
        side: Side,
        price: Price,
        quantity: Quantity,
        fee: Price,
        fee_asset: Symbol,
        is_maker: bool,
    ) -> Self {
        Self {
            trade_id,
            order_id,
            symbol,
            side,
            price,
            quantity,
            fee,
            fee_asset,
            is_maker,
            timestamp: current_timestamp_ms(),
        }
    }

    /// 计算成交金额（价格 * 数量）
    pub fn notional(&self) -> Price {
        // 简化计算：使用浮点数计算后转回定点数
        let value = self.price.to_f64() * self.quantity.to_f64();
        Price::from_f64(value)
    }
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OrderStatus {
    /// 等待提交
    Pending = 1,
    /// 已提交
    Submitted = 2,
    /// 部分成交
    PartiallyFilled = 3,
    /// 完全成交
    Filled = 4,
    /// 已取消
    Cancelled = 5,
    /// 已拒绝
    Rejected = 6,
}

impl OrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            OrderStatus::Pending => "PENDING",
            OrderStatus::Submitted => "SUBMITTED",
            OrderStatus::PartiallyFilled => "PARTIALLY_FILLED",
            OrderStatus::Filled => "FILLED",
            OrderStatus::Cancelled => "CANCELLED",
            OrderStatus::Rejected => "REJECTED",
        }
    }

    /// 是否为最终状态
    pub const fn is_final(self) -> bool {
        matches!(
            self,
            OrderStatus::Filled | OrderStatus::Cancelled | OrderStatus::Rejected
        )
    }
}

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
    pub trades: Vec<Trade>,
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
    pub fn filled(
        order_id: OrderId,
        trades: Vec<Trade>,
        match_seq: u64,
    ) -> Self {
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
    pub fn partially_filled(
        order_id: OrderId,
        trades: Vec<Trade>,
        match_seq: u64,
    ) -> Self {
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
    fn calculate_avg_price_and_quantity(trades: &[Trade]) -> (Price, Quantity) {
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
    pub trades: Vec<Trade>,
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
        trades: Vec<Trade>,
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
        trades: Vec<Trade>,
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
    fn calculate_avg_price_and_quantity(trades: &[Trade]) -> (Price, Quantity) {
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
pub struct CancelOrderCommand {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易对
    pub symbol: Symbol,
}

impl CancelOrderCommand {
    /// 创建取消订单命令
    pub fn new(order_id: OrderId, symbol: Symbol) -> Self {
        Self { order_id, symbol }
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
        Self {
            order_id,
            cancelled: false,
            status,
            cancelled_at: current_timestamp_ms(),
        }
    }
}

/// 查询订单命令
#[derive(Debug, Clone)]
pub struct QueryOrderCommand {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易对
    pub symbol: Symbol,
}

impl QueryOrderCommand {
    /// 创建查询订单命令
    pub fn new(order_id: OrderId, symbol: Symbol) -> Self {
        Self { order_id, symbol }
    }
}

/// 订单查询结果
#[derive(Debug, Clone)]
pub struct OrderQueryResult {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易对
    pub symbol: Symbol,
    /// 订单方向
    pub side: Side,
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
pub struct QueryPositionCommand {
    /// 交易对
    pub symbol: Symbol,
    /// 持仓方向
    pub position_side: PositionSide,
}

impl QueryPositionCommand {
    /// 创建查询持仓命令
    pub fn new(symbol: Symbol, position_side: PositionSide) -> Self {
        Self {
            symbol,
            position_side,
        }
    }

    /// 查询多头持仓
    pub fn long(symbol: Symbol) -> Self {
        Self {
            symbol,
            position_side: PositionSide::Long,
        }
    }

    /// 查询空头持仓
    pub fn short(symbol: Symbol) -> Self {
        Self {
            symbol,
            position_side: PositionSide::Short,
        }
    }

    /// 查询单向持仓
    pub fn both(symbol: Symbol) -> Self {
        Self {
            symbol,
            position_side: PositionSide::Both,
        }
    }
}

/// 持仓信息
#[derive(Debug, Clone)]
pub struct PositionInfo {
    /// 交易对
    pub symbol: Symbol,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 持仓数量（正数表示多头，负数表示空头）
    pub quantity: Quantity,
    /// 持仓均价
    pub entry_price: Price,
    /// 标记价格（用于计算未实现盈亏）
    pub mark_price: Price,
    /// 未实现盈亏
    pub unrealized_pnl: Price,
    /// 已实现盈亏
    pub realized_pnl: Price,
    /// 杠杆倍数
    pub leverage: u8,
    /// 保证金
    pub margin: Price,
    /// 强平价格
    pub liquidation_price: Option<Price>,
    /// 更新时间戳（毫秒）
    pub updated_at: u64,
}

impl PositionInfo {
    /// 创建空持仓
    pub fn empty(symbol: Symbol, position_side: PositionSide) -> Self {
        Self {
            symbol,
            position_side,
            quantity: Quantity::from_raw(0),
            entry_price: Price::from_raw(0),
            mark_price: Price::from_raw(0),
            unrealized_pnl: Price::from_raw(0),
            realized_pnl: Price::from_raw(0),
            leverage: 1,
            margin: Price::from_raw(0),
            liquidation_price: None,
            updated_at: current_timestamp_ms(),
        }
    }

    /// 是否有持仓
    pub fn has_position(&self) -> bool {
        self.quantity.is_positive()
    }

    /// 是否为多头
    pub fn is_long(&self) -> bool {
        self.position_side == PositionSide::Long && self.quantity.is_positive()
    }

    /// 是否为空头
    pub fn is_short(&self) -> bool {
        self.position_side == PositionSide::Short && self.quantity.is_positive()
    }
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
// 订单管理命令
// ============================================================================

/// 修改订单命令
#[derive(Debug, Clone)]
pub struct ModifyOrderCommand {
    /// 订单ID
    pub order_id: OrderId,
    /// 交易对
    pub symbol: Symbol,
    /// 新价格（None表示不修改）
    pub new_price: Option<Price>,
    /// 新数量（None表示不修改）
    pub new_quantity: Option<Quantity>,
}

impl ModifyOrderCommand {
    /// 创建修改订单命令
    pub fn new(order_id: OrderId, symbol: Symbol) -> Self {
        Self {
            order_id,
            symbol,
            new_price: None,
            new_quantity: None,
        }
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
pub struct CancelAllOrdersCommand {
    /// 交易对（None表示取消所有交易对的订单）
    pub symbol: Option<Symbol>,
    /// 持仓方向（None表示取消所有方向的订单）
    pub position_side: Option<PositionSide>,
}

impl CancelAllOrdersCommand {
    /// 取消所有订单
    pub fn all() -> Self {
        Self {
            symbol: None,
            position_side: None,
        }
    }

    /// 取消指定交易对的所有订单
    pub fn by_symbol(symbol: Symbol) -> Self {
        Self {
            symbol: Some(symbol),
            position_side: None,
        }
    }

    /// 取消指定持仓方向的所有订单
    pub fn by_position_side(position_side: PositionSide) -> Self {
        Self {
            symbol: None,
            position_side: Some(position_side),
        }
    }

    /// 取消指定交易对和持仓方向的订单
    pub fn by_symbol_and_side(symbol: Symbol, position_side: PositionSide) -> Self {
        Self {
            symbol: Some(symbol),
            position_side: Some(position_side),
        }
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

/// 查询成交记录命令
#[derive(Debug, Clone)]
pub struct QueryTradesCommand {
    /// 按订单ID查询（可选）
    pub order_id: Option<OrderId>,
    /// 按交易对查询（可选）
    pub symbol: Option<Symbol>,
    /// 开始时间戳（毫秒，可选）
    pub start_time: Option<u64>,
    /// 结束时间戳（毫秒，可选）
    pub end_time: Option<u64>,
    /// 最大返回数量（默认100）
    pub limit: usize,
}

impl QueryTradesCommand {
    /// 创建查询成交记录命令
    pub fn new() -> Self {
        Self {
            order_id: None,
            symbol: None,
            start_time: None,
            end_time: None,
            limit: 100,
        }
    }

    /// 按订单ID查询
    pub fn by_order_id(order_id: OrderId) -> Self {
        Self {
            order_id: Some(order_id),
            symbol: None,
            start_time: None,
            end_time: None,
            limit: 100,
        }
    }

    /// 按交易对查询
    pub fn by_symbol(symbol: Symbol) -> Self {
        Self {
            order_id: None,
            symbol: Some(symbol),
            start_time: None,
            end_time: None,
            limit: 100,
        }
    }

    /// 按时间范围查询
    pub fn by_time_range(start_time: u64, end_time: u64) -> Self {
        Self {
            order_id: None,
            symbol: None,
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
    pub fn with_symbol(mut self, symbol: Symbol) -> Self {
        self.symbol = Some(symbol);
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

impl Default for QueryTradesCommand {
    fn default() -> Self {
        Self::new()
    }
}

/// 成交记录查询结果
#[derive(Debug, Clone)]
pub struct TradesQueryResult {
    /// 成交记录列表（按时间降序）
    pub trades: Vec<Trade>,
    /// 总成交数量
    pub total_count: usize,
    /// 是否有更多数据
    pub has_more: bool,
}

impl TradesQueryResult {
    /// 创建成交查询结果
    pub fn new(trades: Vec<Trade>, total_count: usize, has_more: bool) -> Self {
        Self {
            trades,
            total_count,
            has_more,
        }
    }

    /// 创建空结果
    pub fn empty() -> Self {
        Self {
            trades: Vec::new(),
            total_count: 0,
            has_more: false,
        }
    }
}

/// 查询订单簿命令
#[derive(Debug, Clone)]
pub struct QueryOrderBookCommand {
    /// 交易对
    pub symbol: Symbol,
    /// 深度档位（如20表示20档深度）
    pub depth: u32,
}

impl QueryOrderBookCommand {
    /// 创建查询订单簿命令
    pub fn new(symbol: Symbol, depth: u32) -> Self {
        Self { symbol, depth }
    }

    /// 默认20档深度
    pub fn default_depth(symbol: Symbol) -> Self {
        Self::new(symbol, 20)
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
        Self {
            price,
            quantity,
            order_count,
        }
    }
}

/// 订单簿快照
#[derive(Debug, Clone)]
pub struct OrderBookSnapshot {
    /// 交易对
    pub symbol: Symbol,
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
    pub fn new(symbol: Symbol, bids: Vec<PriceLevel>, asks: Vec<PriceLevel>) -> Self {
        let best_bid = bids.first().map(|level| level.price);
        let best_ask = asks.first().map(|level| level.price);

        Self {
            symbol,
            bids,
            asks,
            best_bid,
            best_ask,
            timestamp: current_timestamp_ms(),
        }
    }

    /// 创建空订单簿
    pub fn empty(symbol: Symbol) -> Self {
        Self {
            symbol,
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
            (Some(bid), Some(ask)) => {
                Some(Price::from_raw(ask.raw() - bid.raw()))
            }
            _ => None,
        }
    }

    /// 获取中间价
    pub fn mid_price(&self) -> Option<Price> {
        match (self.best_bid, self.best_ask) {
            (Some(bid), Some(ask)) => {
                Some(Price::from_raw((bid.raw() + ask.raw()) / 2))
            }
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
///     fn handle_open_position(&mut self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
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
///         let order_id = OrderId::generate();
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
///     fn handle_close_position(&mut self, cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
///         // 1. 验证命令
///         cmd.validate().map_err(PrepCommandError::ValidationError)?;
///
///         // 2. 检查持仓
///         let position = self.positions.get(&cmd.symbol)
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
///             OrderId::generate(),
///             match_result.avg_price,
///             match_result.filled_qty,
///             pnl,
///             match_result.seq,
///         ))
///     }
///
///     fn cancel_order(&mut self, cmd: CancelOrderCommand) -> Result<CancelOrderResult, PrepCommandError> {
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
///         Ok(self.positions.get(&cmd.symbol)
///             .cloned()
///             .unwrap_or_else(|| PositionInfo::empty(cmd.symbol, cmd.position_side)))
///     }
/// }
/// ```


pub trait PerpOrderExchangeProc: Send + Sync {
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
    fn handle_open_position(
        &mut self,
        cmd: OpenPositionCommand,
    ) -> Result<OpenPositionResult, PrepCommandError>;

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
    fn handle_close_position(
        &mut self,
        cmd: ClosePositionCommand,
    ) -> Result<ClosePositionResult, PrepCommandError>;

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
    fn cancel_order(
        &mut self,
        cmd: CancelOrderCommand,
    ) -> Result<CancelOrderResult, PrepCommandError>;

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
    fn query_order(
        &self,
        cmd: QueryOrderCommand,
    ) -> Result<OrderQueryResult, PrepCommandError>;

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
    fn query_position(
        &self,
        cmd: QueryPositionCommand,
    ) -> Result<PositionInfo, PrepCommandError>;

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
    fn modify_order(
        &mut self,
        cmd: ModifyOrderCommand,
    ) -> Result<ModifyOrderResult, PrepCommandError>;

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
    /// - `symbol = Some(s), position_side = Some(p)`: 取消指定交易对和方向的订单
    ///
    /// # 注意
    /// - 已成交的订单无法取消
    /// - 返回成功取消的订单数量和失败的订单数量
    fn cancel_all_orders(
        &mut self,
        cmd: CancelAllOrdersCommand,
    ) -> Result<CancelAllOrdersResult, PrepCommandError>;

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
    fn query_order_book(
        &self,
        cmd: QueryOrderBookCommand,
    ) -> Result<OrderBookSnapshot, PrepCommandError>;

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
    fn query_trades(
        &self,
        cmd: QueryTradesCommand,
    ) -> Result<TradesQueryResult, PrepCommandError>;
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_position_market_long() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(1.0))
            .with_leverage(10);

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Buy);
        assert_eq!(cmd.position_side, PositionSide::Long);
        assert_eq!(cmd.order_type, OrderType::Market);
        assert_eq!(cmd.leverage, 10);
        assert!(cmd.price.is_none());
    }

    #[test]
    fn test_open_position_market_short() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = OpenPositionCommand::market_short(symbol, Quantity::from_f64(2.0))
            .with_leverage(5);

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Sell);
        assert_eq!(cmd.position_side, PositionSide::Short);
        assert_eq!(cmd.leverage, 5);
    }

    #[test]
    fn test_open_position_limit_long() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = OpenPositionCommand::limit_long(
            symbol,
            Quantity::from_f64(0.5),
            Price::from_f64(50000.0),
        )
        .with_leverage(20);

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.order_type, OrderType::Limit);
        assert!(cmd.price.is_some());
        assert_eq!(cmd.leverage, 20);
    }

    #[test]
    fn test_open_position_limit_short() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = OpenPositionCommand::limit_short(
            symbol,
            Quantity::from_f64(3.0),
            Price::from_f64(3000.0),
        );

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Sell);
        assert_eq!(cmd.position_side, PositionSide::Short);
    }

    #[test]
    fn test_close_position_market_long() {
        let symbol = Symbol::new("BTCUSDT");

        // 全部平仓
        let cmd = ClosePositionCommand::market_close_long(symbol, None);
        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Sell);
        assert_eq!(cmd.position_side, PositionSide::Long);
        assert!(cmd.quantity.is_none());

        // 部分平仓
        let cmd = ClosePositionCommand::market_close_long(symbol, Some(Quantity::from_f64(0.5)));
        assert!(cmd.validate().is_ok());
        assert!(cmd.quantity.is_some());
    }

    #[test]
    fn test_close_position_market_short() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = ClosePositionCommand::market_close_short(symbol, None);

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Buy);  // 平空用买
        assert_eq!(cmd.position_side, PositionSide::Short);
    }

    #[test]
    fn test_close_position_limit_long() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = ClosePositionCommand::limit_close_long(
            symbol,
            Quantity::from_f64(1.0),
            Price::from_f64(55000.0),
        );

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.order_type, OrderType::Limit);
        assert!(cmd.price.is_some());
    }

    #[test]
    fn test_close_position_limit_short() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = ClosePositionCommand::limit_close_short(
            symbol,
            Quantity::from_f64(2.0),
            Price::from_f64(2900.0),
        );

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Buy);
    }

    #[test]
    fn test_validation_invalid_quantity() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_raw(0));
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_leverage() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(1.0))
            .with_leverage(200);  // 超过125
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_validation_limit_order_missing_price() {
        let symbol = Symbol::new("BTCUSDT");
        let mut cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(1.0));
        cmd.order_type = OrderType::Limit;
        cmd.price = None;
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_side_opposite() {
        assert_eq!(Side::Buy.opposite(), Side::Sell);
        assert_eq!(Side::Sell.opposite(), Side::Buy);
    }

    #[test]
    fn test_value_objects() {
        let price = Price::from_f64(50000.12345678);
        assert!((price.to_f64() - 50000.12345678).abs() < 0.00000001);

        let qty = Quantity::from_f64(1.5);
        assert!((qty.to_f64() - 1.5).abs() < 0.00000001);
        assert!(qty.is_positive());
        assert!(!qty.is_zero());
    }

    #[test]
    fn test_symbol() {
        let symbol = Symbol::new("BTCUSDT");
        assert_eq!(symbol.as_str(), "BTCUSDT");
    }

    #[test]
    fn test_memory_alignment() {
        use std::mem::align_of;
        assert_eq!(align_of::<OpenPositionCommand>(), 64);
        assert_eq!(align_of::<ClosePositionCommand>(), 64);
    }

    // 新增命令测试
    #[test]
    fn test_cancel_order_command() {
        let order_id = OrderId::new("TEST-ORDER-123");
        let symbol = Symbol::new("BTCUSDT");
        let cmd = CancelOrderCommand::new(order_id.clone(), symbol);

        assert_eq!(cmd.order_id.as_str(), "TEST-ORDER-123");
        assert_eq!(cmd.symbol.as_str(), "BTCUSDT");
    }

    #[test]
    fn test_cancel_order_result() {
        let order_id = OrderId::new("TEST-ORDER-123");

        // 成功取消
        let result = CancelOrderResult::success(order_id.clone());
        assert!(result.cancelled);
        assert_eq!(result.status, OrderStatus::Cancelled);

        // 取消失败（已成交）
        let result = CancelOrderResult::failed(order_id, OrderStatus::Filled);
        assert!(!result.cancelled);
        assert_eq!(result.status, OrderStatus::Filled);
    }

    #[test]
    fn test_query_order_command() {
        let order_id = OrderId::new("TEST-ORDER-456");
        let symbol = Symbol::new("ETHUSDT");
        let cmd = QueryOrderCommand::new(order_id.clone(), symbol);

        assert_eq!(cmd.order_id.as_str(), "TEST-ORDER-456");
        assert_eq!(cmd.symbol.as_str(), "ETHUSDT");
    }

    #[test]
    fn test_query_position_command() {
        let symbol = Symbol::new("BTCUSDT");

        // 查询多头
        let cmd = QueryPositionCommand::long(symbol);
        assert_eq!(cmd.position_side, PositionSide::Long);
        assert_eq!(cmd.symbol.as_str(), "BTCUSDT");

        // 查询空头
        let cmd = QueryPositionCommand::short(symbol);
        assert_eq!(cmd.position_side, PositionSide::Short);

        // 查询单向持仓
        let cmd = QueryPositionCommand::both(symbol);
        assert_eq!(cmd.position_side, PositionSide::Both);
    }

    #[test]
    fn test_position_info() {
        let symbol = Symbol::new("BTCUSDT");

        // 空持仓
        let empty_pos = PositionInfo::empty(symbol, PositionSide::Long);
        assert!(!empty_pos.has_position());
        assert!(!empty_pos.is_long());
        assert_eq!(empty_pos.quantity.raw(), 0);

        // 有多头持仓
        let mut long_pos = PositionInfo::empty(symbol, PositionSide::Long);
        long_pos.quantity = Quantity::from_f64(1.5);
        assert!(long_pos.has_position());
        assert!(long_pos.is_long());
        assert!(!long_pos.is_short());

        // 有空头持仓
        let mut short_pos = PositionInfo::empty(symbol, PositionSide::Short);
        short_pos.quantity = Quantity::from_f64(2.0);
        assert!(short_pos.has_position());
        assert!(short_pos.is_short());
        assert!(!short_pos.is_long());
    }

    #[test]
    fn test_order_id_generation() {
        let order_id1 = OrderId::generate();
        let order_id2 = OrderId::generate();

        // 两个生成的ID应该不同
        assert_ne!(order_id1.as_str(), order_id2.as_str());

        // 应该都以"ORD-"开头
        assert!(order_id1.as_str().starts_with("ORD-"));
        assert!(order_id2.as_str().starts_with("ORD-"));
    }

    #[test]
    fn test_order_status_is_final() {
        assert!(OrderStatus::Filled.is_final());
        assert!(OrderStatus::Cancelled.is_final());
        assert!(OrderStatus::Rejected.is_final());

        assert!(!OrderStatus::Pending.is_final());
        assert!(!OrderStatus::Submitted.is_final());
        assert!(!OrderStatus::PartiallyFilled.is_final());
    }

    #[test]
    fn test_open_position_result_builders() {
        let order_id = OrderId::new("TEST-ORD-1");

        // Pending
        let result = OpenPositionResult::pending(order_id.clone());
        assert_eq!(result.status, OrderStatus::Pending);
        assert!(result.match_seq.is_none());
        assert!(result.trades.is_empty());

        // Accepted (进入订单簿)
        let result = OpenPositionResult::accepted(order_id.clone());
        assert_eq!(result.status, OrderStatus::Submitted);
        assert!(result.match_seq.is_none());
        assert!(result.trades.is_empty());

        // Filled - 创建成交明细
        let symbol = Symbol::new("BTCUSDT");
        let fee_asset = Symbol::new("USDT");
        let trades = vec![
            Trade::new(
                TradeId::new("TRD-1"),
                order_id.clone(),
                symbol,
                Side::Buy,
                Price::from_f64(50000.0),
                Quantity::from_f64(0.5),
                Price::from_f64(1.0),
                fee_asset,
                true,
            ),
            Trade::new(
                TradeId::new("TRD-2"),
                order_id.clone(),
                symbol,
                Side::Buy,
                Price::from_f64(50100.0),
                Quantity::from_f64(0.5),
                Price::from_f64(1.0),
                fee_asset,
                false,
            ),
        ];

        let result = OpenPositionResult::filled(order_id.clone(), trades.clone(), 12345);

        assert_eq!(result.status, OrderStatus::Filled);
        assert!(result.avg_price.is_some());
        assert_eq!(result.match_seq, Some(12345));
        assert_eq!(result.trades.len(), 2);

        // 验证均价计算: (50000 * 0.5 + 50100 * 0.5) / 1.0 = 50050
        let avg = result.avg_price.unwrap().to_f64();
        assert!((avg - 50050.0).abs() < 1.0);

        // 验证总量: 0.5 + 0.5 = 1.0
        assert_eq!(result.filled_quantity.to_f64(), 1.0);

        // Partially Filled
        let partial_trades = vec![trades[0].clone()];
        let result = OpenPositionResult::partially_filled(order_id, partial_trades, 12346);

        assert_eq!(result.status, OrderStatus::PartiallyFilled);
        assert!(result.avg_price.is_some());
        assert_eq!(result.trades.len(), 1);
        assert_eq!(result.filled_quantity.to_f64(), 0.5);
    }

    #[test]
    fn test_close_position_result_builders() {
        let order_id = OrderId::new("TEST-ORD-2");

        // Pending
        let result = ClosePositionResult::pending(order_id.clone());
        assert_eq!(result.status, OrderStatus::Pending);
        assert!(result.realized_pnl.is_none());
        assert!(result.trades.is_empty());

        // Accepted
        let result = ClosePositionResult::accepted(order_id.clone());
        assert_eq!(result.status, OrderStatus::Submitted);
        assert!(result.match_seq.is_none());
        assert!(result.trades.is_empty());

        // Filled - 创建成交明细
        let symbol = Symbol::new("BTCUSDT");
        let fee_asset = Symbol::new("USDT");
        let trades = vec![
            Trade::new(
                TradeId::new("TRD-3"),
                order_id.clone(),
                symbol,
                Side::Sell,
                Price::from_f64(55000.0),
                Quantity::from_f64(0.6),
                Price::from_f64(1.5),
                fee_asset,
                true,
            ),
            Trade::new(
                TradeId::new("TRD-4"),
                order_id.clone(),
                symbol,
                Side::Sell,
                Price::from_f64(55100.0),
                Quantity::from_f64(0.4),
                Price::from_f64(1.0),
                fee_asset,
                false,
            ),
        ];

        let result = ClosePositionResult::filled(
            order_id.clone(),
            trades.clone(),
            Price::from_f64(5000.0), // 盈利5000
            12347,                    // match_seq
        );

        assert_eq!(result.status, OrderStatus::Filled);
        assert!(result.realized_pnl.is_some());
        assert_eq!(result.realized_pnl.unwrap().to_f64(), 5000.0);
        assert_eq!(result.match_seq, Some(12347));
        assert_eq!(result.trades.len(), 2);

        // 验证均价计算: (55000 * 0.6 + 55100 * 0.4) / 1.0 = 55040
        let avg = result.avg_price.unwrap().to_f64();
        assert!((avg - 55040.0).abs() < 1.0);

        // 验证总量: 0.6 + 0.4 = 1.0
        assert_eq!(result.closed_quantity.to_f64(), 1.0);

        // Partially Filled
        let partial_trades = vec![trades[0].clone()];
        let result = ClosePositionResult::partially_filled(
            order_id,
            partial_trades,
            Price::from_f64(2500.0),
            12348,
        );

        assert_eq!(result.status, OrderStatus::PartiallyFilled);
        assert!(result.realized_pnl.is_some());
        assert_eq!(result.trades.len(), 1);
        assert_eq!(result.closed_quantity.to_f64(), 0.6);
    }

    #[test]
    fn test_prep_command_error_display() {
        let err = PrepCommandError::ValidationError("数量无效");
        assert_eq!(format!("{}", err), "验证错误: 数量无效");

        let err = PrepCommandError::InsufficientBalance;
        assert_eq!(format!("{}", err), "余额不足");

        let err = PrepCommandError::DuplicateOrderId("ORD-123".to_string());
        assert_eq!(format!("{}", err), "订单ID重复: ORD-123");
    }

    // 新增命令测试 - 修改订单
    #[test]
    fn test_modify_order_command_price() {
        let order_id = OrderId::new("TEST-ORD-100");
        let symbol = Symbol::new("BTCUSDT");

        let cmd = ModifyOrderCommand::new(order_id.clone(), symbol)
            .with_price(Price::from_f64(51000.0));

        assert_eq!(cmd.order_id.as_str(), "TEST-ORD-100");
        assert_eq!(cmd.symbol.as_str(), "BTCUSDT");
        assert!(cmd.new_price.is_some());
        assert!(cmd.new_quantity.is_none());
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_modify_order_command_quantity() {
        let order_id = OrderId::new("TEST-ORD-101");
        let symbol = Symbol::new("ETHUSDT");

        let cmd = ModifyOrderCommand::new(order_id, symbol)
            .with_quantity(Quantity::from_f64(2.5));

        assert!(cmd.new_price.is_none());
        assert!(cmd.new_quantity.is_some());
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_modify_order_command_both() {
        let order_id = OrderId::new("TEST-ORD-102");
        let symbol = Symbol::new("BTCUSDT");

        let cmd = ModifyOrderCommand::new(order_id, symbol)
            .with_price(Price::from_f64(52000.0))
            .with_quantity(Quantity::from_f64(1.5));

        assert!(cmd.new_price.is_some());
        assert!(cmd.new_quantity.is_some());
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn test_modify_order_validation_no_changes() {
        let order_id = OrderId::new("TEST-ORD-103");
        let symbol = Symbol::new("BTCUSDT");

        let cmd = ModifyOrderCommand::new(order_id, symbol);

        // 验证失败：必须至少修改一项
        assert!(cmd.validate().is_err());
        assert_eq!(cmd.validate().unwrap_err(), "必须指定新价格或新数量");
    }

    #[test]
    fn test_modify_order_validation_invalid_price() {
        let order_id = OrderId::new("TEST-ORD-104");
        let symbol = Symbol::new("BTCUSDT");

        let cmd = ModifyOrderCommand::new(order_id, symbol)
            .with_price(Price::from_raw(0));  // 无效价格

        assert!(cmd.validate().is_err());
        assert_eq!(cmd.validate().unwrap_err(), "新价格必须大于0");
    }

    #[test]
    fn test_modify_order_validation_invalid_quantity() {
        let order_id = OrderId::new("TEST-ORD-105");
        let symbol = Symbol::new("BTCUSDT");

        let cmd = ModifyOrderCommand::new(order_id, symbol)
            .with_quantity(Quantity::from_raw(-100));  // 负数数量

        assert!(cmd.validate().is_err());
        assert_eq!(cmd.validate().unwrap_err(), "新数量必须大于0");
    }

    #[test]
    fn test_modify_order_result_success() {
        let order_id = OrderId::new("TEST-ORD-106");
        let new_price = Price::from_f64(53000.0);
        let new_quantity = Quantity::from_f64(2.0);

        let result = ModifyOrderResult::success(
            order_id.clone(),
            Some(new_price),
            Some(new_quantity),
        );

        assert!(result.modified);
        assert_eq!(result.order_id.as_str(), "TEST-ORD-106");
        assert!(result.new_price.is_some());
        assert!(result.new_quantity.is_some());
    }

    #[test]
    fn test_modify_order_result_failed() {
        let order_id = OrderId::new("TEST-ORD-107");

        let result = ModifyOrderResult::failed(order_id.clone());

        assert!(!result.modified);
        assert_eq!(result.order_id.as_str(), "TEST-ORD-107");
        assert!(result.new_price.is_none());
        assert!(result.new_quantity.is_none());
    }

    // 批量取消订单测试
    #[test]
    fn test_cancel_all_orders_all() {
        let cmd = CancelAllOrdersCommand::all();

        assert!(cmd.symbol.is_none());
        assert!(cmd.position_side.is_none());
    }

    #[test]
    fn test_cancel_all_orders_by_symbol() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = CancelAllOrdersCommand::by_symbol(symbol);

        assert!(cmd.symbol.is_some());
        assert_eq!(cmd.symbol.unwrap().as_str(), "BTCUSDT");
        assert!(cmd.position_side.is_none());
    }

    #[test]
    fn test_cancel_all_orders_by_position_side() {
        let cmd = CancelAllOrdersCommand::by_position_side(PositionSide::Long);

        assert!(cmd.symbol.is_none());
        assert!(cmd.position_side.is_some());
        assert_eq!(cmd.position_side.unwrap(), PositionSide::Long);
    }

    #[test]
    fn test_cancel_all_orders_by_symbol_and_side() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = CancelAllOrdersCommand::by_symbol_and_side(symbol, PositionSide::Short);

        assert!(cmd.symbol.is_some());
        assert_eq!(cmd.symbol.unwrap().as_str(), "ETHUSDT");
        assert!(cmd.position_side.is_some());
        assert_eq!(cmd.position_side.unwrap(), PositionSide::Short);
    }

    #[test]
    fn test_cancel_all_orders_result_success() {
        let order_ids = vec![
            OrderId::new("ORD-1"),
            OrderId::new("ORD-2"),
            OrderId::new("ORD-3"),
        ];

        let result = CancelAllOrdersResult::new(order_ids.clone(), 1);

        assert_eq!(result.cancelled_count, 3);
        assert_eq!(result.failed_count, 1);
        assert_eq!(result.cancelled_order_ids.len(), 3);
        assert_eq!(result.cancelled_order_ids[0].as_str(), "ORD-1");
    }

    #[test]
    fn test_cancel_all_orders_result_empty() {
        let result = CancelAllOrdersResult::empty();

        assert_eq!(result.cancelled_count, 0);
        assert_eq!(result.failed_count, 0);
        assert!(result.cancelled_order_ids.is_empty());
    }

    // 查询订单簿测试
    #[test]
    fn test_query_order_book_command() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = QueryOrderBookCommand::new(symbol, 50);

        assert_eq!(cmd.symbol.as_str(), "BTCUSDT");
        assert_eq!(cmd.depth, 50);
    }

    #[test]
    fn test_query_order_book_command_default_depth() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = QueryOrderBookCommand::default_depth(symbol);

        assert_eq!(cmd.symbol.as_str(), "ETHUSDT");
        assert_eq!(cmd.depth, 20);  // 默认20档
    }

    #[test]
    fn test_price_level() {
        let level = PriceLevel::new(
            Price::from_f64(50000.0),
            Quantity::from_f64(10.5),
            5,
        );

        assert_eq!(level.price.to_f64(), 50000.0);
        assert_eq!(level.quantity.to_f64(), 10.5);
        assert_eq!(level.order_count, 5);
    }

    #[test]
    fn test_order_book_snapshot_creation() {
        let symbol = Symbol::new("BTCUSDT");

        let bids = vec![
            PriceLevel::new(Price::from_f64(49900.0), Quantity::from_f64(1.0), 1),
            PriceLevel::new(Price::from_f64(49800.0), Quantity::from_f64(2.0), 2),
        ];

        let asks = vec![
            PriceLevel::new(Price::from_f64(50100.0), Quantity::from_f64(1.5), 1),
            PriceLevel::new(Price::from_f64(50200.0), Quantity::from_f64(2.5), 2),
        ];

        let snapshot = OrderBookSnapshot::new(symbol, bids.clone(), asks.clone());

        assert_eq!(snapshot.symbol.as_str(), "BTCUSDT");
        assert_eq!(snapshot.bids.len(), 2);
        assert_eq!(snapshot.asks.len(), 2);
        assert!(snapshot.best_bid.is_some());
        assert!(snapshot.best_ask.is_some());
        assert_eq!(snapshot.best_bid.unwrap().to_f64(), 49900.0);
        assert_eq!(snapshot.best_ask.unwrap().to_f64(), 50100.0);
    }

    #[test]
    fn test_order_book_snapshot_empty() {
        let symbol = Symbol::new("BTCUSDT");
        let snapshot = OrderBookSnapshot::empty(symbol);

        assert_eq!(snapshot.symbol.as_str(), "BTCUSDT");
        assert!(snapshot.bids.is_empty());
        assert!(snapshot.asks.is_empty());
        assert!(snapshot.best_bid.is_none());
        assert!(snapshot.best_ask.is_none());
    }

    #[test]
    fn test_order_book_snapshot_spread() {
        let symbol = Symbol::new("BTCUSDT");

        let bids = vec![
            PriceLevel::new(Price::from_f64(50000.0), Quantity::from_f64(1.0), 1),
        ];

        let asks = vec![
            PriceLevel::new(Price::from_f64(50100.0), Quantity::from_f64(1.0), 1),
        ];

        let snapshot = OrderBookSnapshot::new(symbol, bids, asks);
        let spread = snapshot.spread().unwrap();

        // 价差应该是 50100 - 50000 = 100
        assert!((spread.to_f64() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_order_book_snapshot_mid_price() {
        let symbol = Symbol::new("BTCUSDT");

        let bids = vec![
            PriceLevel::new(Price::from_f64(50000.0), Quantity::from_f64(1.0), 1),
        ];

        let asks = vec![
            PriceLevel::new(Price::from_f64(50100.0), Quantity::from_f64(1.0), 1),
        ];

        let snapshot = OrderBookSnapshot::new(symbol, bids, asks);
        let mid_price = snapshot.mid_price().unwrap();

        // 中间价应该是 (50000 + 50100) / 2 = 50050
        assert!((mid_price.to_f64() - 50050.0).abs() < 0.01);
    }

    #[test]
    fn test_order_book_snapshot_empty_spread() {
        let symbol = Symbol::new("BTCUSDT");
        let snapshot = OrderBookSnapshot::empty(symbol);

        // 空订单簿没有价差
        assert!(snapshot.spread().is_none());
        assert!(snapshot.mid_price().is_none());
    }

    // 成交记录相关测试
    #[test]
    fn test_trade_id_generation() {
        let trade_id1 = TradeId::generate();
        std::thread::sleep(std::time::Duration::from_nanos(100));
        let trade_id2 = TradeId::generate();

        // 两个生成的ID应该不同
        assert_ne!(trade_id1.as_str(), trade_id2.as_str());

        // 应该都以"TRD-"开头
        assert!(trade_id1.as_str().starts_with("TRD-"));
        assert!(trade_id2.as_str().starts_with("TRD-"));

        // 测试手动创建
        let trade_id3 = TradeId::new("TRD-MANUAL");
        assert_eq!(trade_id3.as_str(), "TRD-MANUAL");
    }

    #[test]
    fn test_trade_creation() {
        let trade_id = TradeId::new("TRD-TEST-1");
        let order_id = OrderId::new("ORD-TEST-1");
        let symbol = Symbol::new("BTCUSDT");
        let fee_asset = Symbol::new("USDT");

        let trade = Trade::new(
            trade_id.clone(),
            order_id.clone(),
            symbol,
            Side::Buy,
            Price::from_f64(50000.0),
            Quantity::from_f64(1.5),
            Price::from_f64(2.5),
            fee_asset,
            true,
        );

        assert_eq!(trade.trade_id.as_str(), "TRD-TEST-1");
        assert_eq!(trade.order_id.as_str(), "ORD-TEST-1");
        assert_eq!(trade.symbol.as_str(), "BTCUSDT");
        assert_eq!(trade.side, Side::Buy);
        assert_eq!(trade.price.to_f64(), 50000.0);
        assert_eq!(trade.quantity.to_f64(), 1.5);
        assert_eq!(trade.fee.to_f64(), 2.5);
        assert_eq!(trade.fee_asset.as_str(), "USDT");
        assert!(trade.is_maker);
    }

    #[test]
    fn test_trade_notional_calculation() {
        let trade = Trade::new(
            TradeId::new("TRD-1"),
            OrderId::new("ORD-1"),
            Symbol::new("BTCUSDT"),
            Side::Buy,
            Price::from_f64(50000.0),
            Quantity::from_f64(2.0),
            Price::from_f64(1.0),
            Symbol::new("USDT"),
            true,
        );

        // 成交金额 = 50000 * 2.0 = 100000
        let notional = trade.notional();
        assert!((notional.to_f64() - 100000.0).abs() < 1.0);
    }

    #[test]
    fn test_query_trades_command_by_order_id() {
        let order_id = OrderId::new("ORD-123");
        let cmd = QueryTradesCommand::by_order_id(order_id.clone());

        assert!(cmd.order_id.is_some());
        assert_eq!(cmd.order_id.unwrap().as_str(), "ORD-123");
        assert!(cmd.symbol.is_none());
        assert!(cmd.start_time.is_none());
        assert!(cmd.end_time.is_none());
        assert_eq!(cmd.limit, 100);
    }

    #[test]
    fn test_query_trades_command_by_symbol() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = QueryTradesCommand::by_symbol(symbol);

        assert!(cmd.order_id.is_none());
        assert!(cmd.symbol.is_some());
        assert_eq!(cmd.symbol.unwrap().as_str(), "ETHUSDT");
        assert_eq!(cmd.limit, 100);
    }

    #[test]
    fn test_query_trades_command_by_time_range() {
        let cmd = QueryTradesCommand::by_time_range(1000000, 2000000);

        assert!(cmd.order_id.is_none());
        assert!(cmd.symbol.is_none());
        assert_eq!(cmd.start_time, Some(1000000));
        assert_eq!(cmd.end_time, Some(2000000));
        assert_eq!(cmd.limit, 100);
    }

    #[test]
    fn test_query_trades_command_builder() {
        let order_id = OrderId::new("ORD-456");
        let symbol = Symbol::new("BTCUSDT");

        let cmd = QueryTradesCommand::new()
            .with_order_id(order_id.clone())
            .with_symbol(symbol)
            .with_start_time(1000000)
            .with_end_time(2000000)
            .with_limit(50);

        assert!(cmd.order_id.is_some());
        assert_eq!(cmd.order_id.unwrap().as_str(), "ORD-456");
        assert!(cmd.symbol.is_some());
        assert_eq!(cmd.symbol.unwrap().as_str(), "BTCUSDT");
        assert_eq!(cmd.start_time, Some(1000000));
        assert_eq!(cmd.end_time, Some(2000000));
        assert_eq!(cmd.limit, 50);
    }

    #[test]
    fn test_trades_query_result_creation() {
        let symbol = Symbol::new("BTCUSDT");
        let fee_asset = Symbol::new("USDT");

        let trades = vec![
            Trade::new(
                TradeId::new("TRD-1"),
                OrderId::new("ORD-1"),
                symbol,
                Side::Buy,
                Price::from_f64(50000.0),
                Quantity::from_f64(1.0),
                Price::from_f64(1.0),
                fee_asset,
                true,
            ),
            Trade::new(
                TradeId::new("TRD-2"),
                OrderId::new("ORD-1"),
                symbol,
                Side::Buy,
                Price::from_f64(50100.0),
                Quantity::from_f64(1.0),
                Price::from_f64(1.0),
                fee_asset,
                false,
            ),
        ];

        let result = TradesQueryResult::new(trades.clone(), 100, true);

        assert_eq!(result.trades.len(), 2);
        assert_eq!(result.total_count, 100);
        assert!(result.has_more);
    }

    #[test]
    fn test_trades_query_result_empty() {
        let result = TradesQueryResult::empty();

        assert!(result.trades.is_empty());
        assert_eq!(result.total_count, 0);
        assert!(!result.has_more);
    }
}
