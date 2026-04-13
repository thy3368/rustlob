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

use base_types::base_types::TraderId;
pub use base_types::cqrs::cqrs_types::{CMetadata, Cmd, CmdResp};
use base_types::exchange::spot::spot_types::{OrderStatus, SpotTrade, TimeInForce};
use base_types::{AccountId, OrderId, OrderSide, Price, Quantity, TradingPair};
// ============================================================================
// 错误类型定义 (混合方案)
// ============================================================================

/// 通用命令错误（所有命令共享）
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

// 领域错误 面向的是直接用户，就是给你的用户传达什么信息
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
    /// 其他错误
    Other(String),
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
            Self::Other(msg) => {
                write!(f, "Other error: {}", msg)
            }
        }
    }
}

impl std::error::Error for CommonError {}

/// 现货命令错误
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

///设计思路：直接返回给用户的，要用anyhow,哪些应该反回给用户，哪些应该应该返回内部错误，怎么分类更合适？ 用户分为api用户(如客户端开发者）和交易用户(trader), 不同用户关注点不一样
pub enum SpotApiErrorAny {
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

impl std::fmt::Display for SpotApiErrorAny {
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

impl std::error::Error for SpotApiErrorAny {}

impl From<CommonError> for SpotApiErrorAny {
    fn from(e: CommonError) -> Self {
        Self::Common(e)
    }
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


