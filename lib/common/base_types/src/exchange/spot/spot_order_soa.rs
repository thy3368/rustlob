//! SIMD友好的订单SoA（Structure of Arrays）布局
//!
//! 设计目标：
//! - 使用SoA布局实现极致SIMD性能
//! - 支持批量订单操作，充分利用AVX2/AVX-512/NEON指令
//! - 缓存友好的内存访问模式

/// 订单SoA（Structure of Arrays）布局
///
/// 将多个订单的相同字段连续存储，便于SIMD批量处理
///
/// # 内存布局优势
///
/// 传统AoS（Array of Structures）：
/// ```text
/// [order_id1, trader_id1, price1, qty1, ...]
/// [order_id2, trader_id2, price2, qty2, ...]
/// ```
///
/// SoA（Structure of Arrays）：
/// ```text
/// order_ids:    [id1, id2, id3, id4, ...]  <- 连续内存，SIMD友好
/// trader_ids:   [tid1, tid2, tid3, tid4, ...]
/// prices:       [p1, p2, p3, p4, ...]      <- 可用AVX2一次处理4个u64
/// total_qtys:   [q1, q2, q3, q4, ...]
/// ```
///
/// # SIMD优化
///
/// - AVX2: 一次处理4个u64（256位）
/// - AVX-512: 一次处理8个u64（512位）
/// - ARM NEON: 一次处理2个u64（128位）
#[derive(Debug, Clone)]
pub struct SpotOrderSoa {
    // ===== 核心标识和数量 =====
    /// 订单ID数组
    pub order_ids: Vec<u64>,
    /// 交易员ID数组
    pub trader_ids: Vec<u64>,
    /// 交易对数组
    pub trading_pairs: Vec<u64>,
    /// 总数量数组
    pub total_qtys: Vec<u64>,
    /// 订单价格数组（0表示市价单）
    pub prices: Vec<u64>,
    /// 已成交数量数组
    pub filled_qtys: Vec<u64>,
    /// 未成交数量数组
    pub unfilled_qtys: Vec<u64>,
    /// 创建时间戳数组（纳秒）
    pub timestamps: Vec<u64>,

    // ===== 计算字段和状态 =====
    /// 最后更新时间戳数组（纳秒）
    pub last_updateds: Vec<u64>,
    /// 冻结数量数组
    pub frozen_qtys: Vec<u64>,
    /// 平均成交价数组
    pub average_prices: Vec<u64>,
    /// 累计成交金额数组
    pub cumulative_quote_qtys: Vec<u64>,
    /// 手续费数量数组
    pub commission_qtys: Vec<u64>,
    /// 报价数量数组（市价单使用）
    pub quote_order_qtys: Vec<u64>,
    /// 止损/止盈触发价数组
    pub stop_prices: Vec<u64>,
    /// 过期时间数组（GTD订单）
    pub expire_times: Vec<u64>,
}

impl SpotOrderSoa {
    /// 创建指定容量的SoA
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            order_ids: Vec::with_capacity(capacity),
            trader_ids: Vec::with_capacity(capacity),
            trading_pairs: Vec::with_capacity(capacity),
            total_qtys: Vec::with_capacity(capacity),
            prices: Vec::with_capacity(capacity),
            filled_qtys: Vec::with_capacity(capacity),
            unfilled_qtys: Vec::with_capacity(capacity),
            timestamps: Vec::with_capacity(capacity),
            last_updateds: Vec::with_capacity(capacity),
            frozen_qtys: Vec::with_capacity(capacity),
            average_prices: Vec::with_capacity(capacity),
            cumulative_quote_qtys: Vec::with_capacity(capacity),
            commission_qtys: Vec::with_capacity(capacity),
            quote_order_qtys: Vec::with_capacity(capacity),
            stop_prices: Vec::with_capacity(capacity),
            expire_times: Vec::with_capacity(capacity),
        }
    }

    /// 获取订单数量
    #[inline]
    pub fn len(&self) -> usize {
        self.order_ids.len()
    }

    /// 检查是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.order_ids.is_empty()
    }

    /// 添加订单
    #[inline]
    pub fn push(
        &mut self,
        order_id: u64,
        trader_id: u64,
        trading_pair: u64,
        total_qty: u64,
        price: u64,
        timestamp: u64,
    ) {
        self.order_ids.push(order_id);
        self.trader_ids.push(trader_id);
        self.trading_pairs.push(trading_pair);
        self.total_qtys.push(total_qty);
        self.prices.push(price);
        self.filled_qtys.push(0);
        self.unfilled_qtys.push(total_qty);
        self.timestamps.push(timestamp);
        self.last_updateds.push(timestamp);
        self.frozen_qtys.push(0);
        self.average_prices.push(0);
        self.cumulative_quote_qtys.push(0);
        self.commission_qtys.push(0);
        self.quote_order_qtys.push(0);
        self.stop_prices.push(0);
        self.expire_times.push(0);
    }

    /// 批量检查订单是否已完全成交
    #[inline]
    pub fn batch_is_filled(&self, indices: &[usize]) -> Vec<bool> {
        indices
            .iter()
            .map(|&idx| self.unfilled_qtys[idx] == 0)
            .collect()
    }

    /// 批量检查订单是否有未成交数量
    #[inline]
    pub fn batch_is_active(&self, indices: &[usize]) -> Vec<bool> {
        indices
            .iter()
            .map(|&idx| self.unfilled_qtys[idx] > 0)
            .collect()
    }

    /// 批量检查是否为市价单
    #[inline]
    pub fn batch_is_market_order(&self, indices: &[usize]) -> Vec<bool> {
        indices.iter().map(|&idx| self.prices[idx] == 0).collect()
    }

    /// 批量获取成交百分比（0-10000，表示0.00%-100.00%）
    #[inline]
    pub fn batch_fill_percentage(&self, indices: &[usize]) -> Vec<u64> {
        indices
            .iter()
            .map(|&idx| {
                if self.total_qtys[idx] == 0 {
                    0
                } else {
                    (self.filled_qtys[idx] * 10000) / self.total_qtys[idx]
                }
            })
            .collect()
    }

    /// 批量更新成交信息
    #[inline]
    pub fn batch_update_fill(
        &mut self,
        indices: &[usize],
        filled_qtys: &[u64],
        fill_prices: &[u64],
        timestamp: u64,
    ) {
        assert_eq!(indices.len(), filled_qtys.len());
        assert_eq!(indices.len(), fill_prices.len());

        for ((&idx, &filled_qty), &fill_price) in
            indices.iter().zip(filled_qtys.iter()).zip(fill_prices.iter())
        {
            self.filled_qtys[idx] += filled_qty;
            self.unfilled_qtys[idx] -= filled_qty;

            // 更新平均成交价
            let new_value = filled_qty * fill_price;
            self.cumulative_quote_qtys[idx] += new_value;

            if self.filled_qtys[idx] > 0 {
                self.average_prices[idx] =
                    self.cumulative_quote_qtys[idx] / self.filled_qtys[idx];
            }

            self.last_updateds[idx] = timestamp;
        }
    }

    /// 批量获取未成交数量
    #[inline]
    pub fn batch_get_unfilled(&self, indices: &[usize]) -> Vec<u64> {
        indices.iter().map(|&idx| self.unfilled_qtys[idx]).collect()
    }

    /// 批量获取价格
    #[inline]
    pub fn batch_get_prices(&self, indices: &[usize]) -> Vec<u64> {
        indices.iter().map(|&idx| self.prices[idx]).collect()
    }

    /// 批量设置冻结数量
    #[inline]
    pub fn batch_set_frozen(&mut self, indices: &[usize], frozen_qtys: &[u64]) {
        assert_eq!(indices.len(), frozen_qtys.len());

        for (&idx, &frozen_qty) in indices.iter().zip(frozen_qtys.iter()) {
            self.frozen_qtys[idx] = frozen_qty;
        }
    }
}

impl Default for SpotOrderSoa {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

// ============================================================================
// SIMD优化实现（x86-64 AVX2）
// ============================================================================

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub mod simd_x86 {
    use super::*;

    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    impl SpotOrderSoa {
        /// AVX2批量检查订单是否已完全成交（一次处理4个u64）
        ///
        /// # Safety
        /// 需要AVX2支持
        #[target_feature(enable = "avx2")]
        pub unsafe fn batch_is_filled_avx2(&self) -> Vec<bool> {
            let len = self.len();
            let mut results = Vec::with_capacity(len);

            let zero = _mm256_setzero_si256();
            let mut i = 0;

            // 每次处理4个u64（256位）
            while i + 4 <= len {
                let unfilled =
                    _mm256_loadu_si256(self.unfilled_qtys.as_ptr().add(i) as *const __m256i);
                let cmp = _mm256_cmpeq_epi64(unfilled, zero);

                let mask = _mm256_movemask_pd(_mm256_castsi256_pd(cmp));
                results.push((mask & 0b0001) != 0);
                results.push((mask & 0b0010) != 0);
                results.push((mask & 0b0100) != 0);
                results.push((mask & 0b1000) != 0);

                i += 4;
            }

            // 处理剩余元素
            while i < len {
                results.push(self.unfilled_qtys[i] == 0);
                i += 1;
            }

            results
        }

        /// AVX2批量计算成交百分比（一次处理4个u64）
        #[target_feature(enable = "avx2")]
        pub unsafe fn batch_fill_percentage_avx2(&self) -> Vec<u64> {
            let len = self.len();
            let mut results = Vec::with_capacity(len);

            // 注意：AVX2没有u64除法指令，这里使用标量处理
            for i in 0..len {
                if self.total_qtys[i] == 0 {
                    results.push(0);
                } else {
                    results.push((self.filled_qtys[i] * 10000) / self.total_qtys[i]);
                }
            }

            results
        }

        /// AVX2批量检查是否为市价单（一次处理4个u64）
        #[target_feature(enable = "avx2")]
        pub unsafe fn batch_is_market_order_avx2(&self) -> Vec<bool> {
            let len = self.len();
            let mut results = Vec::with_capacity(len);

            let zero = _mm256_setzero_si256();
            let mut i = 0;

            while i + 4 <= len {
                let prices = _mm256_loadu_si256(self.prices.as_ptr().add(i) as *const __m256i);
                let cmp = _mm256_cmpeq_epi64(prices, zero);

                let mask = _mm256_movemask_pd(_mm256_castsi256_pd(cmp));
                results.push((mask & 0b0001) != 0);
                results.push((mask & 0b0010) != 0);
                results.push((mask & 0b0100) != 0);
                results.push((mask & 0b1000) != 0);

                i += 4;
            }

            while i < len {
                results.push(self.prices[i] == 0);
                i += 1;
            }

            results
        }
    }
}

// ============================================================================
// SIMD优化实现（ARM64 NEON）
// ============================================================================

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub mod simd_arm {
    use super::*;

    #[cfg(target_arch = "aarch64")]
    use std::arch::aarch64::*;

    impl SpotOrderSoa {
        /// NEON批量检查订单是否已完全成交（一次处理2个u64）
        ///
        /// # Safety
        /// 需要NEON支持
        #[target_feature(enable = "neon")]
        pub unsafe fn batch_is_filled_neon(&self) -> Vec<bool> {
            let len = self.len();
            let mut results = Vec::with_capacity(len);

            let zero = vdupq_n_u64(0);
            let mut i = 0;

            // 每次处理2个u64（128位）
            while i + 2 <= len {
                let unfilled = vld1q_u64(self.unfilled_qtys.as_ptr().add(i));
                let cmp = vceqq_u64(unfilled, zero);

                let mut mask = [0u64; 2];
                vst1q_u64(mask.as_mut_ptr(), cmp);
                results.push(mask[0] != 0);
                results.push(mask[1] != 0);

                i += 2;
            }

            // 处理剩余元素
            while i < len {
                results.push(self.unfilled_qtys[i] == 0);
                i += 1;
            }

            results
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soa_creation() {
        let soa = SpotOrderSoa::with_capacity(10);
        assert_eq!(soa.len(), 0);
        assert!(soa.is_empty());
    }

    #[test]
    fn test_soa_push() {
        let mut soa = SpotOrderSoa::with_capacity(2);
        soa.push(1, 100, 1, 1000, 50000, 1000000);
        soa.push(2, 101, 1, 2000, 51000, 1000001);

        assert_eq!(soa.len(), 2);
        assert_eq!(soa.order_ids[0], 1);
        assert_eq!(soa.trader_ids[0], 100);
        assert_eq!(soa.total_qtys[0], 1000);
        assert_eq!(soa.prices[0], 50000);
        assert_eq!(soa.unfilled_qtys[0], 1000);
    }

    #[test]
    fn test_batch_is_filled() {
        let mut soa = SpotOrderSoa::with_capacity(3);
        soa.push(1, 100, 1, 1000, 50000, 1000000);
        soa.push(2, 101, 1, 2000, 51000, 1000001);

        // 第一个订单完全成交
        soa.unfilled_qtys[0] = 0;

        let results = soa.batch_is_filled(&[0, 1]);
        assert_eq!(results[0], true);
        assert_eq!(results[1], false);
    }

    #[test]
    fn test_batch_is_market_order() {
        let mut soa = SpotOrderSoa::with_capacity(2);
        soa.push(1, 100, 1, 1000, 0, 1000000); // 市价单
        soa.push(2, 101, 1, 2000, 51000, 1000001); // 限价单

        let results = soa.batch_is_market_order(&[0, 1]);
        assert_eq!(results[0], true);
        assert_eq!(results[1], false);
    }

    #[test]
    fn test_batch_fill_percentage() {
        let mut soa = SpotOrderSoa::with_capacity(2);
        soa.push(1, 100, 1, 1000, 50000, 1000000);
        soa.push(2, 101, 1, 2000, 51000, 1000001);

        // 第一个订单成交25%
        soa.filled_qtys[0] = 250;
        soa.unfilled_qtys[0] = 750;

        // 第二个订单成交50%
        soa.filled_qtys[1] = 1000;
        soa.unfilled_qtys[1] = 1000;

        let results = soa.batch_fill_percentage(&[0, 1]);
        assert_eq!(results[0], 2500); // 25.00%
        assert_eq!(results[1], 5000); // 50.00%
    }

    #[test]
    fn test_batch_update_fill() {
        let mut soa = SpotOrderSoa::with_capacity(2);
        soa.push(1, 100, 1, 1000, 50000, 1000000);
        soa.push(2, 101, 1, 2000, 51000, 1000001);

        soa.batch_update_fill(&[0, 1], &[300, 500], &[50000, 51000], 1000002);

        assert_eq!(soa.filled_qtys[0], 300);
        assert_eq!(soa.unfilled_qtys[0], 700);
        assert_eq!(soa.average_prices[0], 50000);

        assert_eq!(soa.filled_qtys[1], 500);
        assert_eq!(soa.unfilled_qtys[1], 1500);
        assert_eq!(soa.average_prices[1], 51000);
    }

    #[test]
    fn test_batch_get_unfilled() {
        let mut soa = SpotOrderSoa::with_capacity(2);
        soa.push(1, 100, 1, 1000, 50000, 1000000);
        soa.push(2, 101, 1, 2000, 51000, 1000001);

        soa.unfilled_qtys[0] = 700;
        soa.unfilled_qtys[1] = 1500;

        let results = soa.batch_get_unfilled(&[0, 1]);
        assert_eq!(results[0], 700);
        assert_eq!(results[1], 1500);
    }
}
