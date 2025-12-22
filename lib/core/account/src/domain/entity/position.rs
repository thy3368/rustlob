//! 持仓实体定义
//!
//! 包含持仓信息、持仓方向等核心类型

// Re-export from base_types
pub use base_types::{PositionId, PositionInfo, PositionSide, Price, Quantity, TradingPair};

// ============================================================================
// 实现 Position trait（用于 PositionRepo）
// ============================================================================

impl crate::domain::repo::Position for PositionInfo {
    type Key = TradingPair;

    fn key(&self) -> Self::Key {
        self.trading_pair
    }
}

