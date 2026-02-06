//! Kyle 模型实现 (1985)
//!
//! Kyle 模型是金融微观结构理论中的经典模型，描述了在存在信息不对称的市场中：
//! - **知情交易者（Informed Trader）**: 拥有私有信息，最优化执行交易
//! - **做市商（Market Maker）**: 通过观察订单流学习并调整价格
//! - **噪音交易者（Noise Traders）**: 随机交易，提供流动性掩护
//!
//! # Kyle 模型核心方程
//!
//! 1. **价格影响系数（Price Impact）**: λ = σ_v / (2σ_u)
//!    - σ_v: 资产真实价值的标准差
//!    - σ_u: 噪音交易量的标准差
//!
//! 2. **做市商定价**: P_t = P_0 + λ * Q_t
//!    - P_t: t时刻价格
//!    - P_0: 初始价格（先验期望）
//!    - Q_t: 累积净订单流
//!
//! 3. **知情交易者最优策略**: X = β * (V - P_0)
//!    - β: 交易强度系数（由均衡求解）
//!    - V: 资产真实价值（私有信息）
//!
//! # Clean Architecture 分层
//!
//! - **Entity**: KyleParameters, KyleState (领域实体)
//! - **Service**: KyleModelService (领域服务)
//! - **Value Objects**: Price, Quantity (值对象)

use std::fmt;

/// Kyle 模型参数（不可变配置）
///
/// 缓存行对齐确保高性能访问
#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
pub struct KyleParameters {
    /// 资产真实价值标准差（σ_v）
    pub value_volatility: f64,
    /// 噪音交易标准差（σ_u）
    pub noise_volatility: f64,
    /// 初始价格/先验期望（P_0）
    pub initial_price: f64,
    /// 总交易轮数（T）
    pub total_rounds: u32,
    /// 风险厌恶系数（扩展模型使用）
    pub risk_aversion: f64,
}

impl KyleParameters {
    /// 创建新的 Kyle 模型参数
    ///
    /// # Arguments
    /// * `value_volatility` - 资产价值不确定性（σ_v > 0）
    /// * `noise_volatility` - 噪音交易波动性（σ_u > 0）
    /// * `initial_price` - 市场初始价格
    /// * `total_rounds` - 交易总轮数
    ///
    /// # Panics
    /// 当波动率参数为非正数时 panic
    #[inline]
    pub fn new(
        value_volatility: f64,
        noise_volatility: f64,
        initial_price: f64,
        total_rounds: u32,
    ) -> Self {
        assert!(value_volatility > 0.0, "Value volatility must be positive");
        assert!(noise_volatility > 0.0, "Noise volatility must be positive");
        assert!(total_rounds > 0, "Total rounds must be positive");

        Self {
            value_volatility,
            noise_volatility,
            initial_price,
            total_rounds,
            risk_aversion: 0.0, // 默认风险中性
        }
    }

    /// 设置风险厌恶系数（用于扩展模型）
    #[inline]
    pub fn with_risk_aversion(mut self, risk_aversion: f64) -> Self {
        self.risk_aversion = risk_aversion;
        self
    }

    /// 计算价格影响系数 λ（Kyle's lambda）
    ///
    /// 公式: λ = σ_v / (2σ_u)
    ///
    /// 经济含义：衡量订单流对价格的影响程度
    #[inline]
    pub fn price_impact(&self) -> f64 {
        self.value_volatility / (2.0 * self.noise_volatility)
    }

    /// 计算知情交易者的交易强度 β
    ///
    /// 单期模型: β = σ_u / σ_v
    /// 多期模型: β = σ_u / (σ_v * sqrt(T))
    #[inline]
    pub fn trading_intensity(&self) -> f64 {
        if self.total_rounds == 1 {
            // 单期 Kyle 模型
            self.noise_volatility / self.value_volatility
        } else {
            // 多期 Kyle 模型
            self.noise_volatility / (self.value_volatility * (self.total_rounds as f64).sqrt())
        }
    }

    /// 计算知情交易者的期望利润
    ///
    /// 公式: E[π] = (1/2) * σ_v * σ_u
    #[inline]
    pub fn expected_profit(&self) -> f64 {
        0.5 * self.value_volatility * self.noise_volatility
    }

    /// 计算市场深度（Market Depth）
    ///
    /// 定义为价格影响的倒数: 1/λ
    #[inline]
    pub fn market_depth(&self) -> f64 {
        1.0 / self.price_impact()
    }
}

/// Kyle 模型状态（可变市场状态）
///
/// 跟踪市场演化过程中的动态信息
#[derive(Debug, Clone)]
pub struct KyleState {
    /// 当前价格（P_t）
    pub current_price: f64,
    /// 累积净订单流（Q_t）
    pub cumulative_order_flow: f64,
    /// 当前交易轮次（t）
    pub current_round: u32,
    /// 知情交易者累积持仓
    pub informed_position: f64,
    /// 做市商累积盈亏
    pub market_maker_pnl: f64,
    /// 历史价格路径
    pub price_history: Vec<f64>,
    /// 历史订单流
    pub order_flow_history: Vec<f64>,
}

impl Default for KyleState {
    fn default() -> Self {
        Self {
            current_price: 0.0,
            cumulative_order_flow: 0.0,
            current_round: 0,
            informed_position: 0.0,
            market_maker_pnl: 0.0,
            price_history: Vec::new(),
            order_flow_history: Vec::new(),
        }
    }
}

impl KyleState {
    /// 创建初始状态
    #[inline]
    pub fn new(initial_price: f64) -> Self {
        Self {
            current_price: initial_price,
            price_history: vec![initial_price],
            ..Default::default()
        }
    }

    /// 重置到初始状态
    #[inline]
    pub fn reset(&mut self, initial_price: f64) {
        self.current_price = initial_price;
        self.cumulative_order_flow = 0.0;
        self.current_round = 0;
        self.informed_position = 0.0;
        self.market_maker_pnl = 0.0;
        self.price_history.clear();
        self.order_flow_history.clear();
        self.price_history.push(initial_price);
    }

    /// 获取价格变化量
    #[inline]
    pub fn price_change(&self) -> f64 {
        if self.price_history.is_empty() { 0.0 } else { self.current_price - self.price_history[0] }
    }

    /// 计算价格波动率（历史标准差）
    #[inline]
    pub fn price_volatility(&self) -> f64 {
        if self.price_history.len() < 2 {
            return 0.0;
        }

        let mean = self.price_history.iter().sum::<f64>() / self.price_history.len() as f64;
        let variance = self.price_history.iter().map(|&p| (p - mean).powi(2)).sum::<f64>()
            / self.price_history.len() as f64;

        variance.sqrt()
    }
}

/// Kyle 模型交易结果
#[derive(Debug, Clone, Copy)]
pub struct KyleTradeResult {
    /// 知情交易者订单（X）
    pub informed_order: f64,
    /// 噪音交易者订单（U）
    pub noise_order: f64,
    /// 总订单流（Q = X + U）
    pub total_order_flow: f64,
    /// 执行价格（P）
    pub execution_price: f64,
    /// 价格影响（ΔP = λ * Q）
    pub price_impact: f64,
    /// 知情交易者本轮利润
    pub informed_profit: f64,
}

impl fmt::Display for KyleTradeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KyleTrade[Informed={:.2}, Noise={:.2}, Total={:.2}, Price={:.4}, Impact={:.4}, Profit={:.2}]",
            self.informed_order,
            self.noise_order,
            self.total_order_flow,
            self.execution_price,
            self.price_impact,
            self.informed_profit
        )
    }
}

/// Kyle 模型服务（领域服务）
///
/// 提供 Kyle 模型的核心交易逻辑和价格发现机制
pub struct KyleModelService {
    /// 模型参数（不可变）
    params: KyleParameters,
    /// 市场状态（可变）
    state: KyleState,
    /// 价格影响系数（缓存计算结果）
    lambda: f64,
    /// 交易强度（缓存计算结果）
    beta: f64,
}

impl KyleModelService {
    /// 创建新的 Kyle 模型服务
    #[inline]
    pub fn new(params: KyleParameters) -> Self {
        let lambda = params.price_impact();
        let beta = params.trading_intensity();
        let state = KyleState::new(params.initial_price);

        Self { params, state, lambda, beta }
    }

    /// 获取模型参数的不可变引用
    #[inline]
    pub fn parameters(&self) -> &KyleParameters {
        &self.params
    }

    /// 获取当前状态的不可变引用
    #[inline]
    pub fn state(&self) -> &KyleState {
        &self.state
    }

    /// 获取价格影响系数
    #[inline]
    pub fn lambda(&self) -> f64 {
        self.lambda
    }

    /// 获取交易强度
    #[inline]
    pub fn beta(&self) -> f64 {
        self.beta
    }

    /// 重置模型到初始状态
    #[inline]
    pub fn reset(&mut self) {
        self.state.reset(self.params.initial_price);
    }

    /// 计算知情交易者的最优订单
    ///
    /// 公式: X = β * (V - P_0)
    ///
    /// # Arguments
    /// * `true_value` - 资产真实价值（知情交易者的私有信息）
    ///
    /// # Returns
    /// 最优订单量（正数表示买入，负数表示卖出）
    #[inline]
    pub fn calculate_informed_order(&self, true_value: f64) -> f64 {
        self.beta * (true_value - self.state.current_price)
    }

    /// 做市商根据订单流更新价格
    ///
    /// 价格更新规则: P_{t+1} = P_t + λ * Q_t
    ///
    /// # Arguments
    /// * `order_flow` - 净订单流（Q = 知情订单 + 噪音订单）
    ///
    /// # Returns
    /// 新的市场价格
    #[inline]
    pub fn update_price(&mut self, order_flow: f64) -> f64 {
        let price_change = self.lambda * order_flow;
        self.state.current_price += price_change;
        self.state.cumulative_order_flow += order_flow;
        self.state.current_round += 1;

        // 记录历史
        self.state.price_history.push(self.state.current_price);
        self.state.order_flow_history.push(order_flow);

        self.state.current_price
    }

    /// 执行一轮完整的 Kyle 模型交易
    ///
    /// # Arguments
    /// * `true_value` - 资产真实价值
    /// * `noise_order` - 噪音交易订单量
    ///
    /// # Returns
    /// 交易结果详情
    #[inline]
    pub fn execute_round(&mut self, true_value: f64, noise_order: f64) -> KyleTradeResult {
        // 1. 知情交易者计算最优订单
        let informed_order = self.calculate_informed_order(true_value);

        // 2. 总订单流
        let total_order_flow = informed_order + noise_order;

        // 3. 做市商观察订单流，更新价格
        let old_price = self.state.current_price;
        let new_price = self.update_price(total_order_flow);
        let price_impact = new_price - old_price;

        // 4. 更新知情交易者持仓
        self.state.informed_position += informed_order;

        // 5. 计算知情交易者利润（使用执行价格）
        // 利润 = 持仓 * (真实价值 - 执行价格)
        let execution_price = (old_price + new_price) / 2.0; // 平均价格
        let informed_profit = informed_order * (true_value - execution_price);

        // 6. 更新做市商盈亏（做市商总是对手方）
        self.state.market_maker_pnl -= informed_profit;

        KyleTradeResult {
            informed_order,
            noise_order,
            total_order_flow,
            execution_price: new_price,
            price_impact,
            informed_profit,
        }
    }

    /// 模拟多轮交易（完整的 Kyle 模型均衡路径）
    ///
    /// # Arguments
    /// * `true_value` - 资产真实价值（保持不变）
    /// * `noise_orders` - 每轮的噪音交易序列
    ///
    /// # Returns
    /// 每轮的交易结果向量
    pub fn simulate(&mut self, true_value: f64, noise_orders: &[f64]) -> Vec<KyleTradeResult> {
        self.reset();

        let rounds = noise_orders.len().min(self.params.total_rounds as usize);
        let mut results = Vec::with_capacity(rounds);

        for &noise in &noise_orders[..rounds] {
            let result = self.execute_round(true_value, noise);
            results.push(result);
        }

        results
    }

    /// 估计资产真实价值（做市商的学习过程）
    ///
    /// 基于贝叶斯更新: E[V | Q_1, ..., Q_t] = P_0 + λ * Σ Q_i
    #[inline]
    pub fn estimate_true_value(&self) -> f64 {
        self.params.initial_price + self.lambda * self.state.cumulative_order_flow
    }

    /// 计算当前的信息不对称程度
    ///
    /// 衡量指标：累积订单流的信息含量
    #[inline]
    pub fn information_asymmetry(&self) -> f64 {
        if self.state.order_flow_history.is_empty() {
            return 0.0;
        }

        // 信息不对称 = 订单流的标准差 / 噪音标准差
        let mean_flow = self.state.cumulative_order_flow / self.state.current_round as f64;
        let variance =
            self.state.order_flow_history.iter().map(|&q| (q - mean_flow).powi(2)).sum::<f64>()
                / self.state.order_flow_history.len() as f64;

        variance.sqrt() / self.params.noise_volatility
    }

    /// 计算市场效率（价格发现速度）
    ///
    /// 定义为：当前价格与初始价格的偏离程度
    #[inline]
    pub fn market_efficiency(&self) -> f64 {
        (self.state.current_price - self.params.initial_price).abs() / self.params.value_volatility
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyle_parameters() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 1);

        assert_eq!(params.value_volatility, 10.0);
        assert_eq!(params.noise_volatility, 5.0);
        assert_eq!(params.initial_price, 100.0);

        // λ = σ_v / (2σ_u) = 10 / (2*5) = 1.0
        assert_eq!(params.price_impact(), 1.0);

        // β = σ_u / σ_v = 5 / 10 = 0.5 (单期)
        assert_eq!(params.trading_intensity(), 0.5);

        // 期望利润 = 0.5 * 10 * 5 = 25
        assert_eq!(params.expected_profit(), 25.0);

        // 市场深度 = 1/λ = 1.0
        assert_eq!(params.market_depth(), 1.0);
    }

    #[test]
    fn test_kyle_parameters_multi_round() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 4);

        // β = σ_u / (σ_v * sqrt(4)) = 5 / (10 * 2) = 0.25
        assert_eq!(params.trading_intensity(), 0.25);
    }

    #[test]
    fn test_kyle_state() {
        let mut state = KyleState::new(100.0);

        assert_eq!(state.current_price, 100.0);
        assert_eq!(state.current_round, 0);
        assert_eq!(state.price_change(), 0.0);

        state.current_price = 105.0;
        assert_eq!(state.price_change(), 5.0);

        state.reset(100.0);
        assert_eq!(state.current_price, 100.0);
        assert_eq!(state.cumulative_order_flow, 0.0);
    }

    #[test]
    fn test_kyle_service_informed_order() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 1);
        let service = KyleModelService::new(params);

        // 真实价值高于当前价格，应该买入
        let true_value = 110.0;
        let order = service.calculate_informed_order(true_value);

        // X = β * (V - P) = 0.5 * (110 - 100) = 5.0
        assert_eq!(order, 5.0);

        // 真实价值低于当前价格，应该卖出
        let true_value = 90.0;
        let order = service.calculate_informed_order(true_value);

        // X = 0.5 * (90 - 100) = -5.0
        assert_eq!(order, -5.0);
    }

    #[test]
    fn test_kyle_service_price_update() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 1);
        let mut service = KyleModelService::new(params);

        // 买入订单流推高价格
        let new_price = service.update_price(5.0);

        // P_new = 100 + 1.0 * 5.0 = 105.0
        assert_eq!(new_price, 105.0);
        assert_eq!(service.state.current_round, 1);
        assert_eq!(service.state.cumulative_order_flow, 5.0);

        // 卖出订单流压低价格
        let new_price = service.update_price(-3.0);

        // P_new = 105 + 1.0 * (-3.0) = 102.0
        assert_eq!(new_price, 102.0);
        assert_eq!(service.state.cumulative_order_flow, 2.0);
    }

    #[test]
    fn test_kyle_service_execute_round() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 1);
        let mut service = KyleModelService::new(params);

        let true_value = 110.0;
        let noise_order = 2.0;

        let result = service.execute_round(true_value, noise_order);

        // 知情订单 = 0.5 * (110 - 100) = 5.0
        assert_eq!(result.informed_order, 5.0);
        assert_eq!(result.noise_order, 2.0);

        // 总订单流 = 5 + 2 = 7
        assert_eq!(result.total_order_flow, 7.0);

        // 价格影响 = 1.0 * 7 = 7.0
        assert_eq!(result.price_impact, 7.0);

        // 新价格 = 100 + 7 = 107
        assert_eq!(result.execution_price, 107.0);

        // 知情交易者持仓应该增加
        assert_eq!(service.state.informed_position, 5.0);
    }

    #[test]
    fn test_kyle_service_simulate() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 3);
        let mut service = KyleModelService::new(params);

        let true_value = 110.0;
        let noise_orders = vec![1.0, -0.5, 2.0];

        let results = service.simulate(true_value, &noise_orders);

        assert_eq!(results.len(), 3);

        // 验证价格逐步收敛到真实价值
        assert!(service.state.current_price > params.initial_price);

        // 验证做市商亏损（知情交易者利润为正）
        assert!(service.state.market_maker_pnl < 0.0);
    }

    #[test]
    fn test_kyle_service_true_value_estimation() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 1);
        let mut service = KyleModelService::new(params);

        // 执行一些交易
        service.execute_round(110.0, 2.0);
        service.execute_round(110.0, -1.0);

        // 做市商估计的真实价值应该接近实际值
        let estimated = service.estimate_true_value();

        // 估计值 = 100 + λ * Σ Q
        assert!(estimated > params.initial_price);
    }

    #[test]
    fn test_price_volatility() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 5);
        let mut service = KyleModelService::new(params);

        let noise_orders = vec![1.0, -1.0, 2.0, -2.0, 1.5];
        service.simulate(110.0, &noise_orders);

        let volatility = service.state.price_volatility();
        assert!(volatility > 0.0);
    }

    #[test]
    #[should_panic(expected = "Value volatility must be positive")]
    fn test_invalid_parameters() {
        KyleParameters::new(0.0, 5.0, 100.0, 1);
    }

    #[test]
    fn test_market_efficiency() {
        let params = KyleParameters::new(10.0, 5.0, 100.0, 1);
        let mut service = KyleModelService::new(params);

        // 初始效率为0
        assert_eq!(service.market_efficiency(), 0.0);

        // 执行交易后效率提升
        service.execute_round(110.0, 2.0);
        assert!(service.market_efficiency() > 0.0);
    }
}
