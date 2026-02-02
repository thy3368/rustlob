// SIMD 优化的订单匹配引擎
use std::simd::{Simd, SimdPartialOrd, SimdFloat, SimdInt, Mask};
use std::simd::prelude::*;

#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
struct OrderBatch {
    // 每个批次8个订单
    order_ids: Simd<u64, 8>,
    prices: Simd<f64, 8>,
    quantities: Simd<f64, 8>,
    sides: Simd<u8, 8>,  // 0=买, 1=卖
    timestamps: Simd<u64, 8>,
}

impl OrderBatch {
    /// 批量匹配订单
    fn match_against_book(
        &self,
        best_bid: f64,
        best_ask: f64,
    ) -> (Simd<f64, 8>, Simd<u8, 8>) {
        // 广播最佳价格
        let best_bid_vec = Simd::splat(best_bid);
        let best_ask_vec = Simd::splat(best_ask);

        // 判断买订单是否能成交
        let bid_mask = self.sides.simd_eq(Simd::splat(0));
        let bid_executable = self.prices.simd_ge(best_ask_vec) & bid_mask;

        // 判断卖订单是否能成交
        let ask_mask = self.sides.simd_eq(Simd::splat(1));
        let ask_executable = self.prices.simd_le(best_bid_vec) & ask_mask;

        // 合并可执行标记
        let executable = bid_executable | ask_executable;

        // 计算成交价格
        let exec_prices = executable.select(
            // 可执行：使用对手方价格
            bid_executable.select(
                best_ask_vec,  // 买订单按卖一成交
                best_bid_vec,  // 卖订单按买一成交
            ),
            // 不可执行：价格为0
            Simd::splat(0.0),
        );

        (exec_prices, executable.to_int().cast())
    }

    /// 批量计算订单价值
    fn calculate_values(&self) -> Simd<f64, 8> {
        // 批量计算：价格 * 数量
        self.prices * self.quantities
    }

    /// 筛选符合条件的订单
    fn filter_by_price_range(
        &self,
        min_price: f64,
        max_price: f64,
    ) -> Mask<i8, 8> {
        let min_vec = Simd::splat(min_price);
        let max_vec = Simd::splat(max_price);

        // SIMD 并行范围检查
        (self.prices.simd_ge(min_vec) & self.prices.simd_le(max_vec))
            .to_int()
            .cast()
    }
}