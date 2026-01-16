// ! 强平流程相关类型定义
//! 定义强平流程涉及的核心类型

use base_types::PrepPosition;
use crate::proc::trading_prep_order_proc::*;

// ============================================================================
// 强平类型
// ============================================================================

/// 强平类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidationType {
    /// 市场强平
    Market,
    /// 风险保障基金接管
    InsuranceFund,
    /// 自动减仓
    ADL
}

/// 强平结果
#[derive(Debug, Clone)]
pub struct LiquidationResult {
    pub position_id: PositionId,
    pub liquidation_type: LiquidationType,
    pub liquidation_price: Price,
    pub liquidated_quantity: Quantity,
    pub margin_loss: Price,
    pub insurance_fund_loss: Price,
    pub order_status: OrderStatus
}

/// 保险基金接管结果
#[derive(Debug, Clone)]
pub struct InsuranceFundTakeover {
    pub total_loss: Price
}

/// ADL执行结果
#[derive(Debug, Clone)]
pub struct ADLResult {
    pub affected_positions: Vec<PositionId>
}

/// 保险基金容量
#[derive(Debug, Clone)]
pub struct InsuranceFundCapacity {
    pub available_balance: Price
}

impl InsuranceFundCapacity {
    pub fn can_takeover(&self, position: &PrepPosition) -> bool {
        // 简化判断：保险基金余额 > 持仓保证金
        self.available_balance > position.margin
    }
}

/// 强平订单
#[derive(Debug, Clone)]
pub struct LiquidationOrder {
    pub position_id: PositionId,
    pub symbol: TradingPair,
    pub side: Side,
    pub quantity: Quantity,
    pub order_type: OrderType,
    pub is_liquidation: bool,
    pub priority: OrderPriority
}

/// 订单优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum OrderPriority {
    /// 普通订单
    Normal = 1,
    /// 紧急订单（强平订单）
    Urgent = 2
}

// ============================================================================
// 保险基金接口
// ============================================================================

/// 保险基金接口
#[async_trait::async_trait]
pub trait InsuranceFund: Send + Sync {
    async fn check_capacity(&self) -> Result<InsuranceFundCapacity, PrepCmdError>;
    async fn takeover(&self, position: &PrepPosition) -> Result<InsuranceFundTakeover, PrepCmdError>;
}

/// ADL引擎接口
#[async_trait::async_trait]
pub trait ADLEngine: Send + Sync {
    async fn find_counterparties(&self, symbol: TradingPair, side: Side) -> Result<Vec<PrepPosition>, PrepCmdError>;

    async fn execute_adl(
        &self, liquidated_position: &PrepPosition, counterparties: Vec<PrepPosition>
    ) -> Result<ADLResult, PrepCmdError>;
}

// ============================================================================
// 错误类型扩展
// ============================================================================

impl PrepCmdError {
    /// 市场流动性不足错误
    pub fn market_liquidity_insufficient() -> Self {
        Self::RiskControlRejected("市场流动性不足，无法完成市场强平".to_string())
    }

    /// 保险基金余额不足错误
    pub fn insurance_fund_insufficient() -> Self { Self::RiskControlRejected("保险基金余额不足".to_string()) }

    /// 无ADL对手方错误
    pub fn no_counterparties_for_adl() -> Self { Self::RiskControlRejected("无可用的ADL对手方".to_string()) }
}
