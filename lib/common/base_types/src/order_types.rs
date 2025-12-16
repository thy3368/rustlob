//! 订单相关类型定义
//!
//! 包含订单ID、买卖方向等订单基础类型

/// 订单ID
pub type OrderId = u64;

/// 买卖方向
///
/// 定义交易的买卖方向，供 LOB、Account 等模块共享使用
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Side {
    Buy = 0,
    Sell = 1
}

impl Side {
    /// 获取相反方向
    #[inline]
    pub fn opposite(&self) -> Side {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy
        }
    }
}
