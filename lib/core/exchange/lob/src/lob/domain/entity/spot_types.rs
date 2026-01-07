use std::fmt;

use account::{AccountId, AssetId, Balance, TradingPair};
use entity_derive::Entity;
use lob_repo::core::symbol_lob_repo::Order;
use fee::core::fee_types::{CexFeeEntity, FeeType};
// Re-export types from base_types for public use
pub use base_types::{OrderId, Quantity, Price, Side};

/// 订单来源标识 - Phase 3: 区分订单的来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OrderSource {
    /// REST API 直接提交
    API = 1,
    /// WebSocket/WebUI 提交
    WebUI = 2,
    /// 移动应用提交
    MobileApp = 3,
    /// TWAP/VWAP 算法单自动分拆的子单
    AlgorithmEngine = 4,
    /// 条件单（StopLoss/TakeProfit）自动触发
    ConditionalTrigger = 5,
    /// 系统内部（清算、风险控制、强平）
    System = 6
}

impl Default for OrderSource {
    fn default() -> Self { Self::API }
}

impl fmt::Display for OrderSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSource::API => write!(f, "REST API"),
            OrderSource::WebUI => write!(f, "Web UI"),
            OrderSource::MobileApp => write!(f, "Mobile App"),
            OrderSource::AlgorithmEngine => write!(f, "Algorithm"),
            OrderSource::ConditionalTrigger => write!(f, "Conditional Trigger"),
            OrderSource::System => write!(f, "System")
        }
    }
}

/// 交易员标识符（8字节固定长度）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(align(8))]
pub struct TraderId([u8; 8]);

impl Default for TraderId {
    fn default() -> Self { Self([0; 8]) }
}

impl TraderId {
    /// 创建新的交易员ID
    #[inline]
    pub fn new(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }
}






/// 订单执行方式 - 定义订单如何与市场交互
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExecutionMethod {
    /// 限价单：按指定价格或更优价格执行
    Limit = 1,
    /// 市价单：以当前市场价格立即执行
    Market = 2
}

impl Default for ExecutionMethod {
    fn default() -> Self { Self::Limit }
}

/// 做市商约束 - 定义订单是否只做Maker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MakerConstraint {
    /// 无约束：可作为Taker或Maker
    None = 0,
    /// 仅做Maker：拒绝任何Taker成交（PostOnly）
    PostOnly = 1
}

impl Default for MakerConstraint {
    fn default() -> Self { Self::None }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConditionalType {
    /// 无条件：普通订单，无触发条件
    None = 0,
    /// 止损：当价格跌破 stop_price 时触发（用于风险控制）
    StopLoss = 1,
    /// 止盈：当价格上涨到 take_profit_price 时触发（用于利润固定）
    TakeProfit = 2
}

impl Default for ConditionalType {
    fn default() -> Self { Self::None }
}

/// 自交易防护模式 - 防止订单与自己的其他订单成交
///
/// 设计说明：
/// - 固定使用 ExpireTaker 模式（最推荐、最安全）
/// - 不暴露选项给用户，由系统统一处理
/// - 为未来支持做市/算法单预留扩展空间
///
/// 应用场景分析：
/// ✅ 需要STP的场景：
///   1. 做市商：同时在买卖两侧挂单
///   2. 算法单分拆：TWAP/VWAP 分拆成多个子单
///   3. 对冲策略：同时做多和做空
///
/// ❌ 不需要STP的场景：
///   1. 普通现货交易：用户只做单一方向
///   2. IOC/FOK 订单：立即成交或取消，无自交易风险
///
/// 为什么选择 ExpireTaker：
/// - 最保险的模式：新订单被拒，订单簿中的单保留
/// - 适用于所有场景：做市、对冲、算法单都能正常工作
/// - 用户体验好：用户提交的新单不会被拒（除非自交易）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SelfTradePrevention {
    /// 取消 Taker（推荐且固定）
    /// - 新订单作为Taker时，如发生自交易则新订单被取消
    /// - 订单簿中的Maker订单保留
    /// - 最安全、最常用、最适合大多数场景
    ExpireTaker = 1
}

// 默认实现：所有订单都使用 ExpireTaker
impl Default for SelfTradePrevention {
    fn default() -> Self { Self::ExpireTaker }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AlgorithmStrategy {
    /// 无算法：直接执行，不分拆
    None = 0,

    /// TWAP（时间加权平均价）
    /// 在指定时间段内均匀分拆订单，目标实现时间加权平均价
    /// 用途：大额订单分散执行，降低市场冲击
    TWAP = 1,

    /// VWAP（成交量加权平均价）
    /// 根据历史或预测成交量分拆订单，跟踪市场成交量
    /// 用途：按市场成交量比例参与，隐蔽性更好
    VWAP = 2,

    /// POV（按比例参与）
    /// 订单以指定的成交量比例参与市场执行
    /// 例如：如果市场成交量为100，设置POV为20%，则订单以20的量参与
    /// 用途：适应市场成交量变化，自动调整执行速度
    POV = 3,

    /// Iceberg（冰山单）
    /// 大量订单只显示一部分在订单簿，隐藏大部分
    /// 特点：可见订单部分成交后，自动补充隐藏部分
    /// 用途：隐蔽执行，避免暴露真实订单量
    Iceberg = 4,

    /// DarkPool（暗池执行）
    /// 通过暗池寻找对手方成交，不在公开订单簿显示
    /// 用途：大额交易隐蔽执行，降低市场价格冲击
    DarkPool = 5
}

impl Default for AlgorithmStrategy {
    fn default() -> Self { Self::None }
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OrderType {
    Limit = 1,  // 限价单
    Market = 2  // 市价单
}


/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OrderStatus {
    Pending = 1,         // 待成交
    PartiallyFilled = 2, // 部分成交
    Filled = 3,          // 完全成交
    Cancelled = 4,       // 已取消

    Rejected = 5, // 已拒绝
    Expired = 6   // 已过期
}

impl Default for OrderStatus {
    fn default() -> Self { Self::Pending }
}


impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "PENDING"),
            OrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OrderStatus::Filled => write!(f, "FILLED"),
            OrderStatus::Cancelled => write!(f, "CANCELLED"),
            OrderStatus::Rejected => write!(f, "Rejected"),
            OrderStatus::Expired => write!(f, "Expired")
        }
    }
}

/// 有效期类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    GTC, // Good Till Cancel
    IOC, // Immediate Or Cancel
    FOK, // Fill Or Kill
    GTX  // Good Till Crossing
}

impl Default for TimeInForce {
    fn default() -> Self { Self::GTC }
}


/// 订单簿条目（64字节缓存行对齐以提升性能）
#[repr(align(64))]
#[derive(Debug, Clone, Entity)]
#[entity(id = "order_id")]
pub struct SpotOrder {
    // ===== 核心标识字段（24字节）=====
    pub order_id: OrderId,     // 订单ID (u64)
    pub trader_id: TraderId,   // 交易员ID ([u8; 8])
    pub account_id: AccountId, // 账户ID

    pub trading_pair: TradingPair, // 交易对 (u64)

    pub total_qty: Quantity,        // 总数量
    pub price: Option<Price>,       // 订单价格 (0表示市价单)
    pub side: Side,                 // 买卖方向 (BUY/SELL) (1字节)
    pub time_in_force: TimeInForce, // 有效期 (GTC/IOC/FOK/GTX) (1字节)

    pub status: OrderStatus, // 订单状态 (1字节)

    // ===== 可选字段 =====
    pub client_order_id: Option<String>, // 客户订单ID
    // pub tag: Option<String>,                    // 订单标签

    // ===== 订单来源（Phase 3：1字节）=====
    pub source: OrderSource, // 订单来源 (API/WebUI/Algorithm/Conditional/System)

    // ===== 订单类型维度（4字节）⭐ 新增算法策略维度 =====
    pub execution_method: ExecutionMethod,     // 执行方式 (Limit/Market) (1字节)
    pub conditional_type: ConditionalType,     // 条件类型 (None/StopLoss/TakeProfit) (1字节)
    pub algorithm_strategy: AlgorithmStrategy, // 算法策略 (None/TWAP/VWAP/...) (1字节)
    pub maker_constraint: MakerConstraint,     // Maker约束 (None/PostOnly) (1字节)

    // ===== 有效期和防护（2字节）=====
    pub self_trade_prevention: SelfTradePrevention, // 自交易防护 (1字节，固定ExpireTaker)

    // ===== P3 优先级：可选属性 =====
    pub stop_price: Option<Price>, // 止损/止盈触发价（仅conditional_type != None时有效）
    pub iceberg_qty: Option<Quantity>, // 冰山单显示数量

    // ===== 计算出来=====
    pub frozen_qty: Quantity,  // 冻结数据
    pub frozen_asset: AssetId, // 冻结资产

    pub filled_asset: AssetId,          // 购买资产
    pub unfilled_qty: Quantity,         // 未成交数量
    pub executed_qty: Quantity,         // 已成交数量（计数器去重）
    pub average_price: Price,           // 平均成交价
    pub cumulative_quote_qty: Quantity, // 累计成交金额（Quote资产计价）
    pub commission_qty: Quantity,       // 手续费
    pub commission_asset: AssetId,      // 手续费资产

    // ===== 时间戳（8字节）=====
    pub timestamp: u64, // 创建时间戳 (ms)
    pub last_updated: u64
}


impl SpotOrder {
    pub fn is_all_filled(&self) -> bool { self.total_qty == self.executed_qty }

    /// 根据 CexFeeEntity 配置计算交易手续费
    ///
    /// 此方法集成了 CEX 费率表配置，支持：
    /// - Maker/Taker 费率区分
    /// - VIP 用户折扣
    /// - 分层费率（基于交易量）
    /// - 做市商特殊优惠
    ///
    /// # 参数
    /// * `fee_entity` - CEX 费率表配置
    /// * `is_taker` - 是否为 Taker（吃单）
    /// * `is_market_maker` - 是否为做市商用户
    /// * `filled` - 成交数量
    /// * `price` - 成交价格
    ///
    /// # 返回
    /// (手续费率基点数, 手续费数量)
    /// 基点数 (bp): 1 bp = 0.01%，例如 10 bp = 0.1%
    #[inline]
    fn calculate_fee_with_amount(
        &self,
        fee_entity: &CexFeeEntity,
        is_taker: bool,
        is_market_maker: bool,
        user_vip_level: Option<u32>,
        user_tier: Option<u32>,
        filled: Quantity,
        price: Price,
    ) -> (i32, Quantity) {
        // 确定交易对的基础和报价资产
        let base_asset = self.trading_pair.base_asset.0.to_string();
        let quote_asset = self.trading_pair.quote_asset.0.to_string();

        // 根据方向调用 CexFeeEntity 的费率计算
        let fee_type = if is_taker { FeeType::Taker } else { FeeType::Maker };

        // 调用费率计算函数
        match fee_entity.calculate_trading_fee(
            fee_type,
            &base_asset,
            &quote_asset,
            self.total_qty.to_f64(),
            self.price.unwrap_or_default().to_f64(),
            user_tier,
            user_vip_level,
            is_market_maker,
        ) {
            Ok(result) => {
                // 计算手续费数量 = 成交金额 * 费率
                let trade_value = filled * price;
                let commission_qty = trade_value * Quantity::from_f64(result.final_rate);

                // 将费率转换为基点 (bp)
                let fee_rate_bp = (result.final_rate * 10000.0) as i32;
                (fee_rate_bp, commission_qty)
            }
            Err(_) => {
                // 如果计算出错，使用默认费率
                let (fee_rate_bp, fee_rate_decimal) = if is_taker {
                    (10, 0.001)  // Taker 0.1% = 10 bp
                } else {
                    (5, 0.0005)  // Maker 0.05% = 5 bp
                };
                let trade_value = filled * price;
                let commission_qty = trade_value * Quantity::from_f64(fee_rate_decimal);
                (fee_rate_bp, commission_qty)
            }
        }
    }

    /// 更新订单的成交统计信息
    ///
    /// 计算并更新：
    /// - 累计成交金额 (cumulative_quote_qty)
    /// - 平均成交价 (average_price)
    /// - 生成成交记录 SpotTrade
    /// - 更新账户余额
    ///
    /// 注意：本方法被 make_trade() 调用两次（自身订单和对方订单）
    /// 数量的更新在 make_trade() 中进行，此处只更新统计和余额
    #[inline]
    pub fn trade(
        &mut self, filled: Quantity, price: Price, is_taker: bool, quote_asset_balance: &mut Balance,
        base_asset_balance: &mut Balance
    ) -> SpotTrade {
        // 更新订单成交统计（只更新累计值，不重复更新计数）
        self.cumulative_quote_qty += filled * price;
        self.frozen_qty -= filled * price;  // 扣减冻结金额（价值，不是数量）

        // 重新计算平均成交价 = 累计成交金额 / 已成交数量
        self.average_price = self.cumulative_quote_qty / self.executed_qty;

        // 获取当前时间戳
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(self.timestamp);

        // 更新账户余额
        match self.side {
            Side::Buy => {
                // 买方：释放冻结的 quote 资产，增加 base 资产
                quote_asset_balance.frozen2pay(filled * price, now);
                base_asset_balance.add_balance(filled, now);
            }
            Side::Sell => {
                // 卖方：释放冻结的 base 资产，增加 quote 资产
                base_asset_balance.frozen2pay(filled, now);
                quote_asset_balance.add_balance(filled * price, now);
            }
        };

        // 生成交易ID（基于时间戳和订单ID）
        let trade_id = (self.timestamp << 32) | (self.order_id & 0xFFFFFFFF) as u64;

        // 根据 CexFeeEntity 配置计算手续费率和数量
        let (commission_rate, commission_qty) = self.calculate_fee_with_amount(
            &CexFeeEntity::new(
                "exchange".to_string(),
                "default_fees".to_string(),
                0.0005,  // 默认 Maker 费率: 0.05%
                0.001,   // 默认 Taker 费率: 0.1%
            ),
            is_taker,
            false,   // 假设非做市商
            None,    // 无VIP等级
            None,    // 无分层等级
            filled,
            price,
        );

        let commission_asset = self.frozen_asset;

        // Taker 的方向记录在 SpotTrade 中
        // 当 is_taker=true 时，使用 self.side（Taker 的方向）
        // 当 is_taker=false 时，反转到 Taker 的方向
        let taker_side = if is_taker { self.side } else { self.side.opposite() };

        // 创建成交记录
        let trade = SpotTrade::new(
            trade_id,
            self.order_id,
            self.timestamp,
            price,
            filled,
            taker_side,
            commission_qty,  // 使用计算出的手续费数量
            commission_asset,
            commission_rate
        );

        trade
    }

    pub fn make_trade(
        &mut self, matched_order: &mut SpotOrder, quote_asset_balance: &mut Balance, base_asset_balance: &mut Balance,
        o_quote_asset_balance: &mut Balance, o_base_asset_balance: &mut Balance
    ) -> Vec<SpotTrade> {
        let filled = self.unfilled_qty.min(matched_order.unfilled_qty);

        // 更新双方订单的成交数量
        self.unfilled_qty -= filled;
        self.executed_qty += filled;
        matched_order.unfilled_qty -= filled;
        matched_order.executed_qty += filled;

        let transaction_price = match self.price {
            None => matched_order.price.unwrap(),
            Some(price) => price
        };

        let mut vec = Vec::<SpotTrade>::new();
        vec.push(self.trade(filled, transaction_price, true, quote_asset_balance, base_asset_balance));
        vec.push(matched_order.trade(filled, transaction_price, false, o_quote_asset_balance, o_base_asset_balance));
        vec
    }

    pub fn frozen_margin(&mut self, balance: &mut Balance, now: u64) {
        // 根据买卖方向确定冻结资产
        match self.side {
            Side::Buy => {
                // 冻结，失败则reject
                self.frozen_qty = self.total_qty * self.price.unwrap();
                self.frozen_asset = self.trading_pair.quote_asset;
                balance.frozen(self.frozen_qty, now);
            }
            Side::Sell => {
                self.frozen_qty = self.total_qty;
                self.frozen_asset = self.trading_pair.base_asset;
                balance.frozen(self.frozen_qty, now);
            }
        };

        // 冻结，失败则reject
    }
    pub fn frozen_asset_balance_id(&self) -> String { format!("{}:{}", self.account_id.0, self.frozen_asset.0) }

    #[inline]
    pub fn create_limit(
        order_id: OrderId, trader_id: TraderId, account_id: AccountId, trading_pair: TradingPair, side: Side,
        price: Price, quantity: Quantity, time_in_force: TimeInForce, client_order_id: Option<String>
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        Self {
            order_id,
            trader_id,
            account_id,
            trading_pair,
            total_qty: quantity,
            unfilled_qty: quantity,
            price: Some(price),
            side,
            status: OrderStatus::Pending,
            execution_method: ExecutionMethod::Limit,
            conditional_type: ConditionalType::None,
            algorithm_strategy: AlgorithmStrategy::None,
            maker_constraint: MakerConstraint::None,
            time_in_force,
            self_trade_prevention: SelfTradePrevention::ExpireTaker,
            timestamp,
            last_updated: timestamp,
            executed_qty: Quantity::default(),
            average_price: Price::default(),
            cumulative_quote_qty: Quantity::default(),
            commission_qty: Quantity::default(),
            stop_price: None,
            iceberg_qty: None,
            source: OrderSource::API,
            client_order_id,
            frozen_qty: Quantity::default(),
            frozen_asset: AssetId::default(),
            filled_asset: AssetId::default(),
            commission_asset: AssetId::default()
        }
    }


    /// 检查订单是否仍然有效（有未成交数量）
    #[inline]
    pub fn is_active(&self) -> bool { self.unfilled_qty < self.total_qty }


    /// 检查订单是否已成交完毕
    #[inline]
    pub fn is_filled(&self) -> bool { self.unfilled_qty == Quantity::default() }

    /// 获取已成交百分比（0.0 - 1.0）
    #[inline]
    pub fn fill_ratio(&self) -> f64 {
        let total = self.total_qty.to_f64();
        let unfilled = self.unfilled_qty.to_f64();
        if total == 0.0 { 0.0 } else { unfilled / total }
    }

    /// 取消订单（通过将状态置为 Cancelled，单次内存写入，速度快）
    #[inline]
    pub fn cancel(&mut self) {
        self.status = OrderStatus::Cancelled;
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(self.timestamp);
    }

    /// 设置订单来源（Phase 3）
    #[inline]
    pub fn with_source(mut self, source: OrderSource) -> Self {
        self.source = source;
        self
    }

    /// 设置执行方式
    #[inline]
    pub fn with_execution_method(mut self, method: ExecutionMethod) -> Self {
        self.execution_method = method;
        self
    }

    /// 设置条件类型
    #[inline]
    pub fn with_conditional_type(mut self, cond_type: ConditionalType) -> Self {
        self.conditional_type = cond_type;
        self
    }

    /// 设置算法策略
    #[inline]
    pub fn with_algorithm_strategy(mut self, strategy: AlgorithmStrategy) -> Self {
        self.algorithm_strategy = strategy;
        self
    }

    /// 设置有效期
    #[inline]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }

    /// 设置止损/止盈价格
    #[inline]
    pub fn with_stop_price(mut self, price: Price) -> Self {
        self.stop_price = Some(price);
        self
    }
}

impl Order for SpotOrder {
    fn order_id(&self) -> OrderId {
        self.order_id
    }

    fn price(&self) -> Price {
        self.price.unwrap_or_default()
    }

    fn quantity(&self) -> Quantity {
        self.total_qty
    }

    fn filled_quantity(&self) -> Quantity {
        self.executed_qty
    }

    fn side(&self) -> Side {
        match self.side {
            Side::Buy => Side::Buy,
            Side::Sell => Side::Sell,
        }
    }

    fn symbol(&self) -> TradingPair {
        self.trading_pair
    }
}


/// 交易执行记录
///
/// 记录一次撮合成交的完整信息，用于：
/// - 事件溯源（TradeCreated事件）
/// - 账户结算（确定资金划转方向）
/// - 交易历史查询
///
/// 竞品分析对标：
/// - 币安: id, orderId, price, qty, quoteQty, commission, commissionAsset,
///   time, isBuyer, isMaker
/// - OKX: ordId, tradeId, side, px, sz, fee, feeRate, feeCcy, execType,
///   tradeTime
/// - Coinbase: trade_id, order_id, price, size, fee, liquidity, created_at
#[derive(Debug, Clone, Copy, Entity)]
#[entity(id = "trade_id")]
pub struct SpotTrade {
    // ===== 交易标识字段（24字节）=====
    /// 交易唯一标识（对标币安/OKX/Coinbase）
    pub trade_id: u64,
    /// 订单ID（对标币安 orderId、OKX ordId、Coinbase order_id）
    pub order_id: OrderId,
    /// 成交时间戳 (ms)（对标币安/OKX/Coinbase）
    pub timestamp: u64,

    // ===== 价格和数量（24字节）=====
    /// 成交价格（对标所有交易所）
    pub price: Price,
    /// 成交数量（对标所有交易所）
    pub quantity: Quantity,
    /// 成交金额 = quantity × price（对标币安 quoteQty）
    pub quote_qty: Quantity,

    // ===== 交易方向（1字节）=====
    /// Taker方向（Buy=Taker买入, Sell=Taker卖出）
    pub taker_side: Side,

    // ===== 手续费字段（16字节）=====
    /// 手续费数量（对标币安 commission、Coinbase fee）
    pub commission_qty: Quantity,
    /// 手续费资产（对标币安 commissionAsset）
    pub commission_asset: AssetId,
    /// 手续费率 (bp, 基点)（对标OKX feeRate，1 bp = 0.01%）
    pub commission_rate: i32,

    // ===== 补位（4字节）=====
    pub _padding: u32
}


impl SpotTrade {
    /// 创建新的交易记录
    #[inline]
    pub fn new(
        trade_id: u64, order_id: OrderId, timestamp: u64, price: Price, quantity: Quantity, taker_side: Side,
        commission_qty: Quantity, commission_asset: AssetId, commission_rate: i32
    ) -> Self {
        let quote_qty = quantity * price; // 计算成交金额

        Self {
            trade_id,
            order_id,
            timestamp,
            price,
            quantity,
            quote_qty,
            taker_side,
            commission_qty,
            commission_asset,
            commission_rate,
            _padding: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_balance() -> Balance {
        Balance::with_available(
            AccountId(1),
            AssetId(0),
            1_000_000_000, // 10亿
            0
        )
    }

    fn create_test_trading_pair() -> TradingPair {
        TradingPair {
            base_asset: AssetId(1),
            quote_asset: AssetId(0)
        }
    }

    #[test]
    fn test_spot_order_fee_calculation_with_cex_fee_entity() {
        // 创建限价买单
        let mut order = SpotOrder::create_limit(
            OrderId::from(1u64),
            TraderId::default(),
            AccountId(1),
            create_test_trading_pair(),
            Side::Buy,
            Price::from_f64(50000.0),
            Quantity::from_f64(1.0),
            TimeInForce::GTC,
            None
        );

        // 初始化订单状态
        order.frozen_qty = Quantity::from_f64(50000.0);
        order.frozen_asset = order.trading_pair.quote_asset;

        // 创建费率表配置
        let fee_entity = CexFeeEntity::new(
            "binance".to_string(),
            "standard_fees".to_string(),
            0.0005,   // Maker: 0.05%
            0.001,    // Taker: 0.1%
        );

        // 测试成交手续费计算
        let filled = Quantity::from_f64(1.0);
        let price = Price::from_f64(50000.0);

        let (rate_bp_taker, commission_qty_taker) = order.calculate_fee_with_amount(
            &fee_entity,
            true,   // is_taker
            false,  // is_market_maker
            None,   // user_vip_level
            None,   // user_tier
            filled,
            price,
        );

        // 验证 Taker 费率和手续费
        assert_eq!(rate_bp_taker, 10, "Taker 费率应为 10 bp (0.1%)");

        // 手续费 = 50000 * 1.0 * 0.001 = 50
        let expected_commission = Quantity::from_f64(50.0);
        assert!(
            (commission_qty_taker.to_f64() - expected_commission.to_f64()).abs() < 0.01,
            "Taker 手续费应为 50 (成交金额 * 0.1%)"
        );

        // 执行成交
        let mut quote_balance = create_test_balance();
        let mut base_balance = Balance::with_available(AccountId(1), AssetId(1), 0, 0);

        let trade = order.trade(
            filled,
            price,
            true,  // is_taker
            &mut quote_balance,
            &mut base_balance
        );

        // 验证成交记录
        assert_eq!(trade.commission_rate, 10, "成交记录中的手续费率应为 10 bp");
        assert_eq!(trade.quantity, filled, "成交数量应正确");
        assert_eq!(trade.price, price, "成交价格应正确");
        assert!(
            (trade.commission_qty.to_f64() - 50.0).abs() < 0.01,
            "成交手续费应为 50"
        );
    }
}
