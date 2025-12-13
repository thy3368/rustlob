use crate::UserId;

#[derive(Debug, Clone)]
pub struct PositionInfo {
    /// 所属用户ID
    pub user_id: UserId,
    /// 持仓ID
    pub position_id: PositionId,
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