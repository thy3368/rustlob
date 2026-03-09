/// 基于基础类型的 SpotOrder（SIMD 优化版本）
///
/// 设计目标：
/// 1. 零拷贝（0copy）：可以直接从二进制数据映射
/// 2. 零分配（0alloc）：不包含堆分配类型（String, Vec, Option）
/// 3. SIMD 友好：使用基础类型，便于向量化操作
/// 4. 缓存行对齐：128字节对齐（2个缓存行），避免 false sharing
///
/// # 内存布局
/// 总大小：128 字节（2个缓存行）
/// 字段按对齐要求排序：u64 → u32 → u8
///
/// # 使用场景
/// - 高性能订单簿
/// - 批量订单处理
/// - SIMD 批量撮合
/// - 零拷贝网络传输
#[repr(C, align(128))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpotOrderBase {
    // ===== 第一缓存行（64字节）- 核心标识和数量 =====
    /// 订单ID
    pub order_id: u64,
    /// 交易员ID
    pub trader_id: u64,
    /// 交易对
    pub trading_pair: u64,
    /// 总数量
    pub total_qty: u64,
    /// 订单价格（0表示市价单）
    pub price: u64,
    /// 已成交数量
    pub filled_qty: u64,
    /// 未成交数量
    pub unfilled_qty: u64,
    /// 创建时间戳（纳秒）
    pub timestamp: u64,

    // ===== 第二缓存行（64字节）- 计算字段和状态 =====
    /// 最后更新时间戳（纳秒）
    pub last_updated: u64,
    /// 冻结数量
    pub frozen_qty: u64,
    /// 平均成交价
    pub average_price: u64,
    /// 累计成交金额
    pub cumulative_quote_qty: u64,
    /// 手续费数量
    pub commission_qty: u64,
    /// 报价数量（市价单使用）
    pub quote_order_qty: u64,
    /// 止损/止盈触发价
    pub stop_price: u64,
    /// 过期时间（GTD订单）
    pub expire_time: u64,
}

impl SpotOrderBase {
    /// 创建新的订单
    #[inline]
    pub const fn new(
        order_id: u64,
        trader_id: u64,
        trading_pair: u64,
        total_qty: u64,
        price: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            order_id,
            trader_id,
            trading_pair,
            total_qty,
            price,
            filled_qty: 0,
            unfilled_qty: total_qty,
            timestamp,
            last_updated: timestamp,
            frozen_qty: 0,
            average_price: 0,
            cumulative_quote_qty: 0,
            commission_qty: 0,
            quote_order_qty: 0,
            stop_price: 0,
            expire_time: 0,
        }
    }

    /// 检查订单是否已完全成交
    #[inline]
    pub const fn is_filled(&self) -> bool {
        self.unfilled_qty == 0
    }

    /// 检查订单是否有未成交数量
    #[inline]
    pub const fn is_active(&self) -> bool {
        self.unfilled_qty > 0
    }

    /// 检查是否为市价单
    #[inline]
    pub const fn is_market_order(&self) -> bool {
        self.price == 0
    }

    /// 获取成交百分比（0-10000，表示0.00%-100.00%）
    #[inline]
    pub const fn fill_percentage(&self) -> u64 {
        if self.total_qty == 0 {
            0
        } else {
            (self.filled_qty * 10000) / self.total_qty
        }
    }

    /// 更新成交信息
    #[inline]
    pub fn update_fill(&mut self, filled_qty: u64, fill_price: u64, timestamp: u64) {
        self.filled_qty += filled_qty;
        self.unfilled_qty -= filled_qty;

        // 更新平均成交价
        let old_value = self.cumulative_quote_qty;
        let new_value = filled_qty * fill_price;
        self.cumulative_quote_qty += new_value;

        if self.filled_qty > 0 {
            self.average_price = self.cumulative_quote_qty / self.filled_qty;
        }

        self.last_updated = timestamp;
    }

    /// 获取结构体大小（编译时常量）
    pub const SIZE: usize = std::mem::size_of::<Self>();
}

// 静态断言：确保结构体大小为128字节
const _: () = assert!(std::mem::size_of::<SpotOrderBase>() == 128);
const _: () = assert!(std::mem::align_of::<SpotOrderBase>() == 128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_and_alignment() {
        assert_eq!(std::mem::size_of::<SpotOrderBase>(), 128);
        assert_eq!(std::mem::align_of::<SpotOrderBase>(), 128);
    }

    #[test]
    fn test_new_order() {
        let order = SpotOrderBase::new(1, 100, 1, 1000, 50000, 1000000);

        assert_eq!(order.order_id, 1);
        assert_eq!(order.trader_id, 100);
        assert_eq!(order.trading_pair, 1);
        assert_eq!(order.total_qty, 1000);
        assert_eq!(order.price, 50000);
        assert_eq!(order.filled_qty, 0);
        assert_eq!(order.unfilled_qty, 1000);
        assert!(order.is_active());
        assert!(!order.is_filled());
    }

    #[test]
    fn test_market_order() {
        let order = SpotOrderBase::new(1, 100, 1, 1000, 0, 1000000);
        assert!(order.is_market_order());
    }

    #[test]
    fn test_update_fill() {
        let mut order = SpotOrderBase::new(1, 100, 1, 1000, 50000, 1000000);

        // 第一次成交
        order.update_fill(300, 50000, 1000001);
        assert_eq!(order.filled_qty, 300);
        assert_eq!(order.unfilled_qty, 700);
        assert_eq!(order.average_price, 50000);
        assert!(order.is_active());

        // 第二次成交
        order.update_fill(700, 51000, 1000002);
        assert_eq!(order.filled_qty, 1000);
        assert_eq!(order.unfilled_qty, 0);
        assert!(order.is_filled());
        assert!(!order.is_active());
    }

    #[test]
    fn test_fill_percentage() {
        let mut order = SpotOrderBase::new(1, 100, 1, 1000, 50000, 1000000);

        assert_eq!(order.fill_percentage(), 0);

        order.update_fill(250, 50000, 1000001);
        assert_eq!(order.fill_percentage(), 2500); // 25.00%

        order.update_fill(750, 50000, 1000002);
        assert_eq!(order.fill_percentage(), 10000); // 100.00%
    }
}
