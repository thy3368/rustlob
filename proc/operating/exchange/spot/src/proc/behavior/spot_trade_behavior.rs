//! 订单命令处理器
//!
//! 采用分层命令模式设计：
//! - SpotCommand: 核心现货订单（由 MatchingService 直接处理）
//! - AlgoCommand: 算法交易订单（由 AlgoService 处理，内部调用 SpotCommand）
//! - ConditionalCommand: 条件订单（由 ConditionalService 处理）
//!
//! 幂等性设计：
//! - 所有命令通过 Command<C> 包装，携带 nonce 实现幂等

// ============================================================================
// 幂等性包装 (Idempotent Command)
// ============================================================================

use base_types::exchange::spot::spot_types::{OrderStatus, SpotTrade, TimeInForce, TraderId};
use base_types::{AccountId, OrderId, Price, Quantity, Side, TradingPair, UserId};

/// Nonce 类型 - 客户端生成的唯一标识
pub type Nonce = u64;

/// 幂等命令包装
///
/// 所有命令通过此结构包装，实现幂等性检查
#[derive(Debug, Clone)]
pub struct Cmd<C> {
    /// 角色
    pub user_id: UserId,
    /// 客户端生成的唯一标识（同一 nonce 只处理一次）
    pub nonce: Nonce,
    /// 命令时间戳（Unix毫秒，用于过期检查）
    pub timestamp_ms: u64,
    /// 实际命令内容
    pub payload: C,
}

impl<C> Cmd<C> {
    /// 创建新命令
    pub fn new(user_id: UserId, nonce: Nonce, payload: C) -> Self {
        Self {
            user_id,
            nonce,
            timestamp_ms: 0, // 由调用方设置
            payload,
        }
    }

    /// 创建带时间戳的命令
    pub fn with_timestamp(user_id: UserId, nonce: Nonce, timestamp_ms: u64, payload: C) -> Self {
        Self { user_id, nonce, timestamp_ms, payload }
    }
}

// ============================================================================
// 错误类型定义 (混合方案)
// ============================================================================

/// 通用命令错误（所有命令共享）
#[derive(Debug, Clone, PartialEq)]
pub enum CommonError {
    /// 账户余额不足
    InsufficientBalance { required: u64, available: u64 },
    /// 订单不存在
    OrderNotFound { order_id: OrderId },
    /// 非法订单状态转换
    InvalidStatusTransition { from: OrderStatus, to: OrderStatus },
    /// 非法参数
    InvalidParameter { field: &'static str, reason: &'static str },
    /// 账户被冻结
    AccountFrozen { account_id: u64 },
    /// 交易对不存在
    TradingPairNotFound { symbol: String },
    /// 系统内部错误
    Internal { message: String },
}

impl CommonError {
    pub fn Runtime(p0: String) -> CommonError {
        todo!()
    }
}

impl CommonError {
    pub fn NotImplemented(p0: String) -> CommonError {
        todo!()
    }
}

impl CommonError {
    pub fn Serialization(p0: String) -> CommonError {
        todo!()
    }
}

impl CommonError {
    pub fn Network(p0: String) -> CommonError {
        todo!()
    }
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientBalance { required, available } => {
                write!(f, "Insufficient balance: required {}, available {}", required, available)
            }
            Self::OrderNotFound { order_id } => {
                write!(f, "Order not found: {}", order_id)
            }
            Self::InvalidStatusTransition { from, to } => {
                write!(f, "Invalid status transition: {:?} -> {:?}", from, to)
            }
            Self::InvalidParameter { field, reason } => {
                write!(f, "Invalid parameter '{}': {}", field, reason)
            }
            Self::AccountFrozen { account_id } => {
                write!(f, "Account frozen: {}", account_id)
            }
            Self::TradingPairNotFound { symbol } => {
                write!(f, "Trading pair not found: {}", symbol)
            }
            Self::Internal { message } => {
                write!(f, "Internal error: {}", message)
            }
        }
    }
}

impl std::error::Error for CommonError {}

/// 现货命令错误
#[derive(Debug, Clone, PartialEq)]
pub enum SpotCmdError {
    /// 通用错误
    Common(CommonError),
    /// FOK订单无法全部成交被拒绝
    FillOrKillRejected { order_id: OrderId, filled: Quantity, requested: Quantity },
    /// 非法的TimeInForce
    InvalidTimeInForce { reason: &'static str },
    /// 价格超出限制
    PriceOutOfRange { price: Price, min: Price, max: Price },
    /// 数量超出限制
    QuantityOutOfRange { quantity: Quantity, min: Quantity, max: Quantity },
}

impl std::fmt::Display for SpotCmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Common(e) => write!(f, "{}", e),
            Self::FillOrKillRejected { order_id, filled, requested } => {
                write!(f, "FOK order {} rejected: filled {}/{}", order_id, filled, requested)
            }
            Self::InvalidTimeInForce { reason } => {
                write!(f, "Invalid TimeInForce: {}", reason)
            }
            Self::PriceOutOfRange { price, min, max } => {
                write!(f, "Price {} out of range [{}, {}]", price, min, max)
            }
            Self::QuantityOutOfRange { quantity, min, max } => {
                write!(f, "Quantity {} out of range [{}, {}]", quantity, min, max)
            }
        }
    }
}

impl std::error::Error for SpotCmdError {}

impl From<CommonError> for SpotCmdError {
    fn from(e: CommonError) -> Self {
        Self::Common(e)
    }
}

/// 算法交易命令错误
#[derive(Debug, Clone, PartialEq)]
pub enum AlgoCmdError {
    /// 通用错误
    Common(CommonError),
    /// 非法的VWAP目标价格
    InvalidVwapTarget { target: Price, market_price: Price },
    /// 参与率过高
    ParticipationRateTooHigh { rate: u32, max: u32 },
    /// TWAP持续时间非法
    InvalidTwapDuration { duration_secs: u64, min: u64, max: u64 },
    /// 时间间隔非法
    InvalidInterval { interval_secs: u64, min: u64 },
}

impl std::fmt::Display for AlgoCmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Common(e) => write!(f, "{}", e),
            Self::InvalidVwapTarget { target, market_price } => {
                write!(f, "Invalid VWAP target: {} (market: {})", target, market_price)
            }
            Self::ParticipationRateTooHigh { rate, max } => {
                write!(f, "Participation rate {} exceeds max {}", rate, max)
            }
            Self::InvalidTwapDuration { duration_secs, min, max } => {
                write!(f, "TWAP duration {}s out of range [{}, {}]", duration_secs, min, max)
            }
            Self::InvalidInterval { interval_secs, min } => {
                write!(f, "Interval {}s below minimum {}", interval_secs, min)
            }
        }
    }
}

impl std::error::Error for AlgoCmdError {}

impl From<CommonError> for AlgoCmdError {
    fn from(e: CommonError) -> Self {
        Self::Common(e)
    }
}

/// 条件订单命令错误
#[derive(Debug, Clone, PartialEq)]
pub enum CondCmdError {
    /// 通用错误
    Common(CommonError),
    /// 止损价格非法
    InvalidStopPrice { stop_price: Price, current_price: Price },
    /// 追踪距离非法
    InvalidTrailAmount { trail_amount: Price },
    /// 冰山单显示数量非法
    InvalidDisplayQuantity { display: Quantity, total: Quantity },
    /// 钉住偏移量非法
    InvalidPegOffset { offset: i32 },
    /// OCO订单配置冲突
    ConflictingOcoOrders { reason: &'static str },
}

impl std::fmt::Display for CondCmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Common(e) => write!(f, "{}", e),
            Self::InvalidStopPrice { stop_price, current_price } => {
                write!(f, "Invalid stop price: {} (current: {})", stop_price, current_price)
            }
            Self::InvalidTrailAmount { trail_amount } => {
                write!(f, "Invalid trail amount: {}", trail_amount)
            }
            Self::InvalidDisplayQuantity { display, total } => {
                write!(f, "Display quantity {} exceeds total {}", display, total)
            }
            Self::InvalidPegOffset { offset } => {
                write!(f, "Invalid peg offset: {}", offset)
            }
            Self::ConflictingOcoOrders { reason } => {
                write!(f, "Conflicting OCO orders: {}", reason)
            }
        }
    }
}

impl std::error::Error for CondCmdError {}

impl From<CommonError> for CondCmdError {
    fn from(e: CommonError) -> Self {
        Self::Common(e)
    }
}

/// 做市商命令错误
#[derive(Debug, Clone, PartialEq)]
pub enum MarketMakerCmdError {
    /// 通用错误
    Common(CommonError),
    /// 买卖价差非法
    InvalidSpread { bid: Price, ask: Price, min_spread: Price },
    /// 非法的拍卖类型
    InvalidAuctionType { reason: &'static str },
    /// 做市商义务未履行
    ObligationNotMet { reason: &'static str },
}

impl std::fmt::Display for MarketMakerCmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Common(e) => write!(f, "{}", e),
            Self::InvalidSpread { bid, ask, min_spread } => {
                write!(f, "Invalid spread: bid {} ask {} (min: {})", bid, ask, min_spread)
            }
            Self::InvalidAuctionType { reason } => {
                write!(f, "Invalid auction type: {}", reason)
            }
            Self::ObligationNotMet { reason } => {
                write!(f, "Market maker obligation not met: {}", reason)
            }
        }
    }
}

impl std::error::Error for MarketMakerCmdError {}

impl From<CommonError> for MarketMakerCmdError {
    fn from(e: CommonError) -> Self {
        Self::Common(e)
    }
}

/// 命令执行元数据
///
/// 包含幂等性和追踪信息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CmdMetadata {
    /// 命令唯一标识（客户端生成）
    pub nonce: Nonce,
    /// 是否为重复命令（幂等命中）
    pub is_duplicate: bool,
    /// 命令接收时间戳
    pub received_at: u64,
}

impl CmdMetadata {
    /// 创建新命令元数据
    #[inline]
    pub fn new(nonce: Nonce) -> Self {
        Self {
            nonce,
            is_duplicate: false,
            received_at: 0, // 由处理器设置
        }
    }

    /// 标记为重复命令
    #[inline]
    pub fn mark_duplicate(mut self) -> Self {
        self.is_duplicate = true;
        self
    }
}

/// 带元数据的命令响应
///
/// 包含执行结果和幂等性/追踪信息
/// 使用 Result<CommandResponse<T>, CommandError> 的方式返回
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CmdResp<T> {
    /// 命令元数据
    pub metadata: CmdMetadata,
    /// 成功结果
    pub result: T,
}

impl<T> CmdResp<T> {
    /// 创建新响应
    #[inline]
    pub fn new(nonce: Nonce, result: T) -> Self {
        Self { metadata: CmdMetadata::new(nonce), result }
    }

    /// 创建重复命令的响应
    #[inline]
    pub fn duplicate(nonce: Nonce, result: T) -> Self {
        Self { metadata: CmdMetadata::new(nonce).mark_duplicate(), result }
    }

    /// 映射结果值
    #[inline]
    pub fn map<U, F>(self, f: F) -> CmdResp<U>
    where
        F: FnOnce(T) -> U,
    {
        CmdResp { metadata: self.metadata, result: f(self.result) }
    }
}

/// 类型别名 - 各类幂等命令
pub type IdemSpotCmd = Cmd<SpotCmdAny>;
pub type IdemAlgoCmd = Cmd<AlgoCmdAny>;
pub type IdemCondCmd = Cmd<ConditionalCmdAny>;
pub type IdemMarketMakerCmd = Cmd<MarketMakerCmdAny>;

/// 类型别名 - 各类命令结果
///
/// 使用标准 Result 包装，支持 ? 操作符和所有 Result 方法
/// 每个命令类型使用自己的错误类型，提供类型安全
pub type IdemSpotResult = Result<CmdResp<SpotCmdRes>, SpotCmdError>;
pub type IdemAlgoResult = Result<CmdResp<AlgoCmdResult>, AlgoCmdError>;
pub type IdemCondResult = Result<CmdResp<CondCmdResult>, CondCmdError>;
pub type IdemMarketMakerResult = Result<CmdResp<MarketMakerCmdResult>, MarketMakerCmdError>;

// ============================================================================
// 核心现货命令 (SpotCommand)
// ============================================================================

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CMetadata {
    /// 命令唯一ID（用于幂等性和追踪）
    #[cfg_attr(feature = "serde", serde(default = "default_command_id"))]
    pub command_id: String,
    /// 命令创建时间戳（Unix 毫秒）
    #[cfg_attr(feature = "serde", serde(default = "default_timestamp"))]
    pub timestamp: u64,
    /// 关联ID（用于分布式追踪）
    #[cfg_attr(feature = "serde", serde(default))]
    pub correlation_id: Option<String>,
    /// 因果ID（用于事件溯源）
    #[cfg_attr(feature = "serde", serde(default))]
    pub causation_id: Option<String>,
    /// 用户/系统标识
    #[cfg_attr(feature = "serde", serde(default))]
    pub actor: Option<String>,
    /// 自定义属性
    #[cfg_attr(feature = "serde", serde(default))]
    pub attributes: Vec<(String, String)>,
}

#[cfg(feature = "serde")]
fn default_command_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(feature = "serde")]
fn default_timestamp() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LimitOrder {
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: CMetadata,
    pub trader: TraderId,
    pub account_id: AccountId,
    pub trading_pair: TradingPair,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
    pub time_in_force: TimeInForce,
    pub client_order_id: Option<String>, // 客户端订单ID（可选）
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarketOrder {
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: CMetadata,

    pub trader: TraderId,
    pub account_id: AccountId,
    pub trading_pair: TradingPair,
    pub side: Side,
    pub quantity: Quantity,
    pub price_limit: Option<Price>, // 价格保护：买单最高价/卖单最低价
    pub time_in_force: Option<TimeInForce>, /* None=IOC(默认), Some(FOK)=全部成交或全部取消,
                                     * Some(IOC)=立即成交或取消 */
    pub client_order_id: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelOrder {
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: CMetadata,
    pub order_id: OrderId,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct CancelAllOrders {
    pub metadata: CMetadata,

    pub trader: TraderId,
    pub trading_pair: Option<TradingPair>, // 可选：只取消指定交易对
    pub side: Option<Side>,
}

/// 现货订单命令
///
/// 核心订单类型，由 MatchingService 直接处理
#[derive(Debug, Clone)]
pub enum SpotCmdAny {
    /// 限价单
    LimitOrder(LimitOrder),
    /// 市价单
    MarketOrder(MarketOrder),
    /// 取消订单
    CancelOrder(CancelOrder),
    /// 批量取消订单
    CancelAllOrders(CancelAllOrders),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LimitOrderRes {
    order_id: OrderId,
    status: OrderStatus,
    filled_quantity: Quantity,
    remaining_quantity: Quantity,
    trades: Vec<SpotTrade>,
}

impl LimitOrderRes {
    pub fn order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn filled_quantity(&self) -> Quantity {
        self.filled_quantity
    }

    pub fn remaining_quantity(&self) -> Quantity {
        self.remaining_quantity
    }

    pub fn trades(&self) -> &[SpotTrade] {
        &self.trades
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarketOrderRes {
    status: OrderStatus,
    filled_quantity: Quantity,
    trades: Vec<SpotTrade>,
}

impl MarketOrderRes {
    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn filled_quantity(&self) -> Quantity {
        self.filled_quantity
    }

    pub fn trades(&self) -> &[SpotTrade] {
        &self.trades
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelOrderRes {
    order_id: OrderId,
    status: OrderStatus,
}

impl CancelOrderRes {
    pub fn order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CancelAllOrdersRes {
    cancelled_count: usize,
    order_ids: Vec<OrderId>,
}

impl CancelAllOrdersRes {
    pub fn cancelled_count(&self) -> usize {
        self.cancelled_count
    }

    pub fn order_ids(&self) -> &[OrderId] {
        &self.order_ids
    }
}

/// 现货命令执行结果
///
/// 只包含成功情况，错误通过 CommandError 返回
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpotCmdRes {
    /// 限价单结果
    LimitOrder(LimitOrderRes),

    /// 市价单结果
    MarketOrder(MarketOrderRes),

    /// 取消订单结果
    CancelOrder(CancelOrderRes),

    /// 批量取消订单结果
    CancelAllOrders(CancelAllOrdersRes),
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
pub enum AlgoCmdAny {
    /// TWAP订单 (Time-Weighted Average Price)
    /// 按时间均匀分配订单
    Twap { trader: TraderId, side: Side, total_quantity: Quantity, duration_secs: u64, interval_secs: u64 },

    /// VWAP订单 (Volume-Weighted Average Price)
    /// 按市场成交量分配订单
    Vwap { trader: TraderId, side: Side, total_quantity: Quantity, target_vwap: Option<Price> },

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
    ImplementationShortfall { trader: TraderId, side: Side, total_quantity: Quantity, urgency: UrgencyLevel },
}

/// 算法命令执行结果
#[derive(Debug, Clone)]
pub enum AlgoCmdResult {
    /// TWAP结果
    Twap { parent_order_id: OrderId, child_orders: Vec<OrderId>, total_traded: Quantity, avg_price: Option<Price> },

    /// VWAP结果
    Vwap { parent_order_id: OrderId, child_orders: Vec<OrderId>, total_traded: Quantity, achieved_vwap: Option<Price> },

    /// POV结果
    Pov { parent_order_id: OrderId, child_orders: Vec<OrderId>, total_traded: Quantity, actual_participation_rate: u32 },

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
pub trait AlgoTradeProc: Send + Sync {
    fn handle(&mut self, cmd: IdemAlgoCmd) -> IdemAlgoResult;
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
pub enum ConditionalCmdAny {
    // ========== 止损订单 ==========
    /// 止损市价单
    /// 当市价达到触发价时，转为市价单
    StopMarket { trader: TraderId, side: Side, stop_price: Price, quantity: Quantity },

    /// 止损限价单
    /// 当市价达到触发价时，转为限价单
    StopLimit { trader: TraderId, side: Side, stop_price: Price, limit_price: Price, quantity: Quantity },

    /// 追踪止损单
    /// 止损价随市价变化而移动
    TrailingStop { trader: TraderId, side: Side, trail_amount: Price, quantity: Quantity },

    /// 追踪止损百分比单
    TrailingStopPercent {
        trader: TraderId,
        side: Side,
        trail_percent: u32, // basis points
        quantity: Quantity,
    },

    // ========== 组合订单 ==========
    // 注意：FOK、IOC、GTD 已移至 TimeInForce，应通过 SpotCommand::LimitOrder 使用
    /// OCO订单 (One-Cancels-Other)
    /// 一个成交则取消另一个
    Oco { trader: TraderId, order1: Box<ConditionalCmdAny>, order2: Box<ConditionalCmdAny> },

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
    /// 冰山单 - 部分隐藏订单
    /// 只显示 display_quantity，成交后自动从隐藏部分补充
    Iceberg { trader: TraderId, side: Side, price: Price, total_quantity: Quantity, display_quantity: Quantity },

    /// 隐藏订单
    /// 完全不显示在订单簿中
    Hidden { trader: TraderId, side: Side, price: Price, quantity: Quantity },

    /// 钉住订单
    /// 价格自动跟随市场最优价
    Pegged { trader: TraderId, side: Side, offset: i32, quantity: Quantity, peg_type: PegType },

    /// 最小成交量订单
    MinimumQuantity { trader: TraderId, side: Side, price: Price, quantity: Quantity, min_quantity: Quantity },
}

/// 条件命令执行结果
#[derive(Debug, Clone)]
pub enum CondCmdResult {
    // ========== 止损订单结果 ==========
    StopMarket {
        order_id: OrderId,
        triggered: bool,
        trades: Vec<SpotTrade>,
    },

    StopLimit {
        order_id: OrderId,
        triggered: bool,
        trades: Vec<SpotTrade>,
    },

    TrailingStop {
        order_id: OrderId,
        current_stop_price: Price,
        triggered: bool,
        trades: Vec<SpotTrade>,
    },

    TrailingStopPercent {
        order_id: OrderId,
        current_stop_price: Price,
        triggered: bool,
        trades: Vec<SpotTrade>,
    },

    // ========== 组合订单结果 ==========
    // 注意：FOK、IOC、GTD 结果通过 SpotCommandResult::LimitOrder 返回
    Oco {
        executed_order: Box<CondCmdResult>,
        cancelled_order_id: Option<OrderId>,
    },

    Bracket {
        entry_order_id: OrderId,
        take_profit_order_id: OrderId,
        stop_loss_order_id: OrderId,
        entry_trades: Vec<SpotTrade>,
        exit_trades: Vec<SpotTrade>,
    },

    // ========== 高级订单结果 ==========
    /// 冰山单结果
    Iceberg {
        order_id: OrderId,
        trades: Vec<SpotTrade>,
        remaining_total: Quantity,
        current_display: Quantity,
    },

    Hidden {
        order_id: OrderId,
        trades: Vec<SpotTrade>,
    },

    Pegged {
        order_id: OrderId,
        current_price: Price,
        trades: Vec<SpotTrade>,
    },

    MinimumQuantity {
        order_id: OrderId,
        trades: Vec<SpotTrade>,
        all_fills_meet_minimum: bool,
    },

    /// 未实现
    NotImplemented,
}

/// 条件订单处理器
pub trait ConditionalTradeProc: Send + Sync {
    fn handle(&mut self, cmd: IdemCondCmd) -> IdemCondResult;
}

// ============================================================================
// 做市商命令 (MarketMakerCommand) - 可选扩展
// ============================================================================

/// 做市商命令
#[derive(Debug, Clone)]
pub enum MarketMakerCmdAny {
    /// 双向报价
    TwoWayQuote { trader: TraderId, bid_price: Price, bid_quantity: Quantity, ask_price: Price, ask_quantity: Quantity },

    /// 拍卖订单
    AuctionOrder { trader: TraderId, side: Side, price: Price, quantity: Quantity, auction_type: AuctionType },
}

/// 做市商命令结果
#[derive(Debug, Clone)]
pub enum MarketMakerCmdResult {
    TwoWayQuote { bid_order_id: OrderId, ask_order_id: OrderId, bid_trades: Vec<SpotTrade>, ask_trades: Vec<SpotTrade> },

    AuctionOrder { order_id: OrderId, auction_price: Option<Price>, trades: Vec<SpotTrade> },

    NotImplemented,
}

/// 做市商处理器
pub trait MarketMakerProc: Send + Sync {
    fn handle(&mut self, cmd: IdemMarketMakerCmd) -> IdemMarketMakerResult;
}

// ============================================================================
// 查询命令 (QueryCommand) - CQRS 读侧
// ============================================================================

/// 查询错误
#[derive(Debug, Clone, PartialEq)]
pub enum QueryError {
    /// 订单不存在
    OrderNotFound { order_id: OrderId },
    /// 权限不足
    PermissionDenied { reason: &'static str },
    /// 数据库错误
    DatabaseError { message: String },
    /// 非法参数
    InvalidParameter { field: &'static str, reason: &'static str },
    /// 系统内部错误
    Internal { message: String },
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OrderNotFound { order_id } => {
                write!(f, "Order not found: {}", order_id)
            }
            Self::PermissionDenied { reason } => {
                write!(f, "Permission denied: {}", reason)
            }
            Self::DatabaseError { message } => {
                write!(f, "Database error: {}", message)
            }
            Self::InvalidParameter { field, reason } => {
                write!(f, "Invalid parameter '{}': {}", field, reason)
            }
            Self::Internal { message } => {
                write!(f, "Internal error: {}", message)
            }
        }
    }
}

impl std::error::Error for QueryError {}

/// 订单查询命令
#[derive(Debug, Clone)]
pub enum OrderQueryCmd {
    /// 查询当前活跃订单
    QueryOpenOrders { trader: TraderId, trading_pair: Option<TradingPair>, side: Option<Side>, page: Option<u32> },

    /// 查询订单详情
    QueryOrderDetail { order_id: OrderId },

    /// 查询历史订单
    QueryOrderHistory {
        trader: TraderId,
        trading_pair: Option<TradingPair>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        page: Option<u32>,
    },

    /// 查询成交历史
    QueryTradeHistory {
        trader: TraderId,
        trading_pair: Option<TradingPair>,
        order_id: Option<OrderId>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    },
}

/// 订单视图（只读DTO）
#[derive(Debug, Clone)]
pub struct OrderView {
    pub order_id: OrderId,
    pub trader: TraderId,
    pub side: Side,
    pub price: Option<Price>,
    pub quantity: Quantity,
    pub filled_quantity: Quantity,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub created_at: u64,
}

/// 订单详情视图
#[derive(Debug, Clone)]
pub struct OrderDetailView {
    pub order_id: OrderId,
    pub trader: TraderId,
    pub side: Side,
    pub price: Option<Price>,
    pub quantity: Quantity,
    pub filled_quantity: Quantity,
    pub remaining_quantity: Quantity,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub created_at: u64,
    pub updated_at: u64,
    pub trades: Vec<TradeView>,
}

/// 成交视图
#[derive(Debug, Clone)]
pub struct TradeView {
    pub trade_id: u64,
    pub order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
    pub side: Side,
    pub timestamp: u64,
    pub is_maker: bool,
}

/// 查询结果
#[derive(Debug, Clone)]
pub enum OrderQueryResult {
    /// 活跃订单列表
    OpenOrders { orders: Vec<OrderView>, total: usize, page: u32 },

    /// 订单详情
    OrderDetail { order: Option<OrderDetailView> },

    /// 历史订单
    OrderHistory { orders: Vec<OrderView>, total: usize, page: u32 },

    /// 成交历史
    TradeHistory { trades: Vec<TradeView>, total: usize },
}

/// 订单查询处理器接口
///
/// 负责处理所有只读查询操作（CQRS 读侧）
pub trait OrderQueryProc: Send + Sync {
    fn handle(&self, query: OrderQueryCmd) -> Result<OrderQueryResult, QueryError>;
}

/// 现货订单处理器
///
/// 核心订单处理接口，返回 Result<CommandResponse, SpotCommandError>
/// 支持 ? 操作符进行错误传播
pub trait SpotTradeBehavior: Send + Sync {
    fn handle(&mut self, cmd: SpotCmdAny) -> IdemSpotResult;
}
