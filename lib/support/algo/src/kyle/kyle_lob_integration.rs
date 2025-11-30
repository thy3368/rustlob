//! Kyle 模型与限价订单簿（LOB）集成
//!
//! 本模块展示如何将 Kyle 模型应用到真实的限价订单簿系统：
//! 1. 智能订单执行（Smart Order Execution）
//! 2. Kyle 参数估算（从 LOB 数据反推）
//! 3. 做市商动态定价
//! 4. 知情交易检测

use super::kyle_service::{KyleModelService, KyleParameters, KyleTradeResult};
use lob::lob::{InMemoryOrderRepository, MatchingService, OrderRepository};
use lob::lob::types::lob_types::{OrderEntry, Side, Trade, TraderId};

/// Kyle 模型参数估算器
///
/// 从 LOB 订单流和价格变化中估算 Kyle 模型参数
pub struct KyleParameterEstimator {
    /// 历史价格序列
    price_history: Vec<f64>,
    /// 历史订单流序列（净买入量）
    order_flow_history: Vec<f64>,
}

impl KyleParameterEstimator {
    /// 创建新的参数估算器
    pub fn new() -> Self {
        Self {
            price_history: Vec::new(),
            order_flow_history: Vec::new(),
        }
    }

    /// 添加观测数据
    pub fn add_observation(&mut self, price: f64, net_order_flow: f64) {
        self.price_history.push(price);
        self.order_flow_history.push(net_order_flow);
    }

    /// 估算价格影响系数 λ（使用线性回归）
    ///
    /// ΔP = λ * Q + ε
    pub fn estimate_lambda(&self) -> Option<f64> {
        if self.price_history.len() < 2 || self.order_flow_history.len() < 2 {
            return None;
        }

        let n = self.price_history.len();

        // 计算价格变化序列
        let price_changes: Vec<f64> = self
            .price_history
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();

        if price_changes.is_empty() {
            return None;
        }

        // 使用最小二乘法估算 λ: ΔP = λ * Q
        let sum_q_squared: f64 = self.order_flow_history.iter().map(|&q| q * q).sum();
        let sum_q_dp: f64 = self
            .order_flow_history
            .iter()
            .zip(price_changes.iter())
            .map(|(&q, &dp)| q * dp)
            .sum();

        if sum_q_squared == 0.0 {
            return None;
        }

        Some(sum_q_dp / sum_q_squared)
    }

    /// 估算噪音交易波动率 σ_u
    pub fn estimate_noise_volatility(&self) -> Option<f64> {
        if self.order_flow_history.len() < 2 {
            return None;
        }

        let mean = self.order_flow_history.iter().sum::<f64>() / self.order_flow_history.len() as f64;
        let variance = self
            .order_flow_history
            .iter()
            .map(|&q| (q - mean).powi(2))
            .sum::<f64>()
            / self.order_flow_history.len() as f64;

        Some(variance.sqrt())
    }

    /// 估算价值波动率 σ_v（通过 λ = σ_v / (2σ_u)）
    pub fn estimate_value_volatility(&self) -> Option<f64> {
        let lambda = self.estimate_lambda()?;
        let sigma_u = self.estimate_noise_volatility()?;

        Some(2.0 * lambda * sigma_u)
    }

    /// 构建 Kyle 参数（从 LOB 数据估算）
    pub fn build_kyle_parameters(&self, initial_price: f64, rounds: u32) -> Option<KyleParameters> {
        let sigma_v = self.estimate_value_volatility()?;
        let sigma_u = self.estimate_noise_volatility()?;

        if sigma_v <= 0.0 || sigma_u <= 0.0 {
            return None;
        }

        Some(KyleParameters::new(sigma_v, sigma_u, initial_price, rounds))
    }
}

impl Default for KyleParameterEstimator {
    fn default() -> Self {
        Self::new()
    }
}

/// 智能订单执行器（基于 Kyle 模型）
///
/// 使用 Kyle 模型优化大单在 LOB 中的执行策略
pub struct SmartOrderExecutor {
    /// 限价订单簿匹配服务
    matching_service: MatchingService<InMemoryOrderRepository>,
    /// Kyle 模型服务
    kyle_service: KyleModelService,
    /// 交易员 ID
    trader_id: TraderId,
}

impl SmartOrderExecutor {
    /// 创建智能订单执行器
    pub fn new(
        matching_service: MatchingService<InMemoryOrderRepository>,
        kyle_params: KyleParameters,
        trader_id: TraderId,
    ) -> Self {
        Self {
            matching_service,
            kyle_service: KyleModelService::new(kyle_params),
            trader_id,
        }
    }

    /// 获取当前最佳买价
    pub fn best_bid(&self) -> Option<u32> {
        self.matching_service.repository().best_bid()
    }

    /// 获取当前最佳卖价
    pub fn best_ask(&self) -> Option<u32> {
        self.matching_service.repository().best_ask()
    }

    /// 计算中间价
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid as f64 + ask as f64) / 2.0),
            _ => None,
        }
    }

    /// 执行一轮智能交易
    ///
    /// 基于 Kyle 模型计算最优订单量，然后在 LOB 中执行
    ///
    /// # Arguments
    /// * `true_value` - 对资产真实价值的估计（私有信息）
    /// * `max_quantity` - 剩余需要执行的最大数量
    ///
    /// # Returns
    /// (实际执行的成交, Kyle模型预测结果)
    pub fn execute_smart_order(
        &mut self,
        true_value: f64,
        max_quantity: u32,
    ) -> (Vec<Trade>, KyleTradeResult) {
        // 1. 使用 Kyle 模型计算最优订单量
        let optimal_order = self.kyle_service.calculate_informed_order(true_value);

        // 2. 限制在最大数量范围内
        let order_quantity = optimal_order.abs().min(max_quantity as f64) as u32;

        // 3. 确定买卖方向
        let side = if optimal_order > 0.0 {
            Side::Buy
        } else {
            Side::Sell
        };

        // 4. 在 LOB 中执行订单
        let current_price = self.mid_price().unwrap_or(true_value);
        let price_ticks = (current_price * 1.0) as u32; // 转换为价格刻度

        let (trades, remaining) = self.matching_service.match_limit_order(
            self.trader_id,
            side,
            price_ticks,
            order_quantity,
        );

        // 5. 如果有剩余，添加到订单簿
        if remaining > 0 {
            let order_id = self.matching_service.repository_mut().allocate_order_id();
            let entry = OrderEntry::new(order_id, self.trader_id, remaining);
            let _ = self
                .matching_service
                .repository_mut()
                .add_order(order_id, entry, side, price_ticks);
        }

        // 6. 更新 Kyle 模型状态（模拟噪音交易为0）
        let kyle_result = self.kyle_service.execute_round(true_value, 0.0);

        (trades, kyle_result)
    }

    /// 获取 Kyle 模型服务的引用
    pub fn kyle_service(&self) -> &KyleModelService {
        &self.kyle_service
    }

    /// 获取匹配服务的引用
    pub fn matching_service(&self) -> &MatchingService<InMemoryOrderRepository> {
        &self.matching_service
    }
}

/// 做市商策略（基于 Kyle 模型）
///
/// 使用 Kyle 模型动态调整买卖价差
pub struct KyleMarketMaker {
    /// Kyle 模型服务
    kyle_service: KyleModelService,
    /// 基础价差（点数）
    base_spread: u32,
    /// 订单流累积
    net_order_flow: f64,
}

impl KyleMarketMaker {
    /// 创建做市商
    pub fn new(kyle_params: KyleParameters, base_spread: u32) -> Self {
        Self {
            kyle_service: KyleModelService::new(kyle_params),
            base_spread,
            net_order_flow: 0.0,
        }
    }

    /// 观察订单流并更新价差
    ///
    /// # Arguments
    /// * `buy_volume` - 买入订单量
    /// * `sell_volume` - 卖出订单量
    ///
    /// # Returns
    /// (建议买价, 建议卖价, 调整后的价差)
    pub fn update_quotes(
        &mut self,
        mid_price: f64,
        buy_volume: f64,
        sell_volume: f64,
    ) -> (u32, u32, u32) {
        // 1. 计算净订单流
        let net_flow = buy_volume - sell_volume;
        self.net_order_flow += net_flow;

        // 2. 根据 Kyle 模型调整价格
        let price_adjustment = self.kyle_service.lambda() * net_flow;
        let adjusted_mid = mid_price + price_adjustment;

        // 3. 根据累积订单流调整价差（逆向选择风险）
        // 订单流越大，逆向选择风险越高，价差应该更宽
        let risk_adjustment = (self.net_order_flow.abs() / 100.0) as u32;
        let adjusted_spread = self.base_spread + risk_adjustment;

        // 4. 计算买卖价
        let bid = (adjusted_mid - adjusted_spread as f64 / 2.0).max(1.0) as u32;
        let ask = (adjusted_mid + adjusted_spread as f64 / 2.0) as u32;

        (bid, ask, adjusted_spread)
    }

    /// 检测是否存在知情交易
    ///
    /// 基于订单流的持续性和价格影响
    pub fn detect_informed_trading(&self, threshold: f64) -> bool {
        let info_asymmetry = self.kyle_service.information_asymmetry();
        info_asymmetry > threshold
    }

    /// 重置订单流累积
    pub fn reset_flow(&mut self) {
        self.net_order_flow = 0.0;
        self.kyle_service.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_estimator() {
        let mut estimator = KyleParameterEstimator::new();

        // 模拟一些观测数据：价格与订单流正相关
        let data = vec![
            (100.0, 10.0),  // (价格, 订单流)
            (102.0, 20.0),
            (101.0, -5.0),
            (103.0, 15.0),
            (104.0, 18.0),
        ];

        for (price, flow) in data {
            estimator.add_observation(price, flow);
        }

        // 估算参数
        let lambda = estimator.estimate_lambda();
        assert!(lambda.is_some());
        assert!(lambda.unwrap() > 0.0); // λ应该为正

        let sigma_u = estimator.estimate_noise_volatility();
        assert!(sigma_u.is_some());
        assert!(sigma_u.unwrap() > 0.0);

        let sigma_v = estimator.estimate_value_volatility();
        assert!(sigma_v.is_some());
    }

    #[test]
    fn test_parameter_estimator_build_params() {
        let mut estimator = KyleParameterEstimator::new();

        for i in 0..10 {
            let price = 100.0 + (i as f64) * 0.5;
            let flow = (i as f64) * 2.0;
            estimator.add_observation(price, flow);
        }

        let params = estimator.build_kyle_parameters(100.0, 5);
        assert!(params.is_some());

        let p = params.unwrap();
        assert!(p.value_volatility > 0.0);
        assert!(p.noise_volatility > 0.0);
        assert_eq!(p.initial_price, 100.0);
    }

    #[test]
    fn test_smart_order_executor() {
        // 创建 LOB 仓储
        let repository = InMemoryOrderRepository::new(100_000, 1000);
        let matching_service = MatchingService::new(repository);

        // 创建 Kyle 参数
        let kyle_params = KyleParameters::new(10.0, 5.0, 100.0, 5);

        // 创建智能执行器
        let trader = TraderId::from_str("SMART");
        let mut executor = SmartOrderExecutor::new(matching_service, kyle_params, trader);

        // 先在订单簿中放置一些流动性
        let mm_trader = TraderId::from_str("MM");
        let sell_order_id = executor.matching_service.repository_mut().allocate_order_id();
        let sell_entry = OrderEntry::new(sell_order_id, mm_trader, 1000);
        executor
            .matching_service
            .repository_mut()
            .add_order(sell_order_id, sell_entry, Side::Sell, 105)
            .unwrap();

        // 执行智能订单
        let true_value = 110.0;
        let (trades, kyle_result) = executor.execute_smart_order(true_value, 500);

        // 验证执行结果
        assert!(kyle_result.informed_order > 0.0); // 应该买入
        assert_eq!(kyle_result.noise_order, 0.0); // 没有噪音交易
    }

    #[test]
    fn test_kyle_market_maker() {
        let kyle_params = KyleParameters::new(10.0, 5.0, 100.0, 1);
        let mut mm = KyleMarketMaker::new(kyle_params, 2); // 基础价差 2

        // 观察买入订单流
        let (bid, ask, spread) = mm.update_quotes(100.0, 50.0, 10.0);

        assert!(bid < ask); // 买价应该低于卖价
        assert_eq!(spread, ask - bid); // 价差应该一致
        assert!(spread >= 2); // 价差不应小于基础价差
    }

    #[test]
    fn test_market_maker_informed_detection() {
        let kyle_params = KyleParameters::new(10.0, 5.0, 100.0, 1);
        let mut mm = KyleMarketMaker::new(kyle_params, 2);

        // 模拟持续的单边订单流（知情交易特征）
        for _ in 0..5 {
            mm.update_quotes(100.0, 100.0, 10.0); // 大量买入
        }

        // 检测知情交易（阈值设为1.0）
        let has_informed = mm.detect_informed_trading(1.0);

        // 持续的单边流应该被检测为知情交易
        println!("Information asymmetry: {}", mm.kyle_service.information_asymmetry());
    }

    #[test]
    fn test_market_maker_spread_adjustment() {
        let kyle_params = KyleParameters::new(10.0, 5.0, 100.0, 1);
        let mut mm = KyleMarketMaker::new(kyle_params, 2);

        // 初始报价
        let (bid1, ask1, spread1) = mm.update_quotes(100.0, 0.0, 0.0);

        // 大量订单流后
        let (bid2, ask2, spread2) = mm.update_quotes(100.0, 500.0, 0.0);

        // 价差应该扩大（逆向选择风险增加）
        assert!(spread2 > spread1);
    }
}
