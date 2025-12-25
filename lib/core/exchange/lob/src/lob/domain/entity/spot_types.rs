use std::fmt;

use account::{AccountId, AssetId, Balance, TradingPair};

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

/// 订单方向（买入或卖出）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    Buy = b'B',  // 买入
    Sell = b'S'  // 卖出
}


/// 订单标识符
pub type OrderId = u64;

/// 价格（以分为单位，避免浮点运算）
pub type Price = i64;

/// 数量/规模
pub type Quantity = i64;

/// 订单簿条目（64字节缓存行对齐以提升性能）
#[repr(align(64))]
pub struct SpotOrder {
    // ===== 核心标识字段（24字节）=====
    pub order_id: OrderId,     // 订单ID (u64)
    pub trader_id: TraderId,   // 交易员ID ([u8; 8])
    pub account_id: AccountId, // 账户ID

    pub trading_pair: TradingPair, // 交易对 (u64)

    pub frozen_qty: Quantity,  // 冻结数据
    pub frozen_asset: AssetId, // 冻结资产

    // ===== 数量字段（8字节）=====
    pub total_qty: Quantity,            // 总数量
    pub filled_asset: AssetId,          // 购买资产
    pub price: Option<Price>,           // 订单价格 (0表示市价单)
    pub unfilled_qty: Quantity,         // 未成交数量
    pub executed_qty: Quantity,         // 已成交数量（计数器去重）
    pub average_price: Price,           // 平均成交价
    pub cumulative_quote_qty: Quantity, // 累计成交金额（Quote资产计价）
    pub commission_qty: Quantity,       // 手续费
    pub commission_asset: AssetId,      // 手续费资产

    // ===== 方向和状态（2字节）=====
    pub side: Side,          // 买卖方向 (BUY/SELL) (1字节)
    pub status: OrderStatus, // 订单状态 (1字节)

    // ===== 订单类型维度（4字节）⭐ 新增算法策略维度 =====
    pub execution_method: ExecutionMethod,     // 执行方式 (Limit/Market) (1字节)
    pub conditional_type: ConditionalType,     // 条件类型 (None/StopLoss/TakeProfit) (1字节)
    pub algorithm_strategy: AlgorithmStrategy, // 算法策略 (None/TWAP/VWAP/...) (1字节)
    pub maker_constraint: MakerConstraint,     // Maker约束 (None/PostOnly) (1字节)

    // ===== 有效期和防护（2字节）=====
    pub time_in_force: TimeInForce,                 // 有效期 (GTC/IOC/FOK/GTX) (1字节)
    pub self_trade_prevention: SelfTradePrevention, // 自交易防护 (1字节，固定ExpireTaker)

    // ===== P3 优先级：可选属性 =====
    pub stop_price: Option<Price>, // 止损/止盈触发价（仅conditional_type != None时有效）
    pub iceberg_qty: Option<Quantity>, // 冰山单显示数量

    // ===== 可选字段 =====
    // pub client_order_id: Option<String>,        // 客户订单ID
    // pub tag: Option<String>,                    // 订单标签

    // ===== 订单来源（Phase 3：1字节）=====
    pub source: OrderSource, // 订单来源 (API/WebUI/Algorithm/Conditional/System)

    // ===== 时间戳（8字节）=====
    pub timestamp: u64, // 创建时间戳 (ms)
    pub last_updated: u64
}

impl SpotOrder {
    pub fn frozen_asset_balance_id(&self) -> String { format!("{}:{}", self.account_id.0, self.frozen_asset.0) }
}

impl SpotOrder {
    pub fn is_all_filled(&self) -> bool { self.total_qty == self.executed_qty }

    pub fn make_trade(
        &mut self, matched_order: &mut &SpotOrder, quote_asset_balance: &mut Balance, base_asset_balance: &mut Balance,
        o_quote_asset_balance: &mut Balance, o_base_asset_balance: &mut Balance
    ) -> SpotTrade {
        let filled = self.unfilled_qty.min(matched_order.unfilled_qty);
        self.unfilled_qty -= filled;
        self.executed_qty += filled;

        match self.side {
            Side::Buy => {
                // todo 计算buy费
                // todo 计算sell费
                quote_asset_balance.frozen2pay(filled * self.price.unwrap(), now);
                base_asset_balance.add_balance(filled, now);
                o_quote_asset_balance.add_balance(filled * self.price.unwrap(), now);
                o_base_asset_balance.frozen2pay(filled, now);
            }
            Side::Sell => {
                base_asset_balance.frozen2pay(filled * self.price.unwrap(), now);
                quote_asset_balance.add_balance(filled, now);
                o_base_asset_balance.add_balance(filled * self.price.unwrap(), now);
                o_quote_asset_balance.frozen2pay(filled, now);
            }
        };


        // todo 生成 let trade= SpotTrade::new()
        // todo
        todo!()
    }

    pub fn frozen_margin(&mut self, balance: &mut Balance, now: u64) {
        // 根据买卖方向确定冻结资产
        match self.side {
            Side::Buy => {
                // 冻结，失败则reject
                self.frozen_qty = self.total_qty * self.price.unwrap();
                balance.frozen(self.frozen_qty, now);
            }
            Side::Sell => {
                self.frozen_qty = self.total_qty;
                balance.frozen(self.frozen_qty, now);
            }
        };

        // self.frozen_asset = self.frozen_asset;
        // self.frozen_qty=self.frozen_qty;

        // 冻结，失败则reject
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

/// 做市商约束 - 定义订单是否只做Maker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MakerConstraint {
    /// 无约束：可作为Taker或Maker
    None = 0,
    /// 仅做Maker：拒绝任何Taker成交（PostOnly）
    PostOnly = 1
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


impl SpotOrder {
    #[inline]
    pub fn pending(
        order_id: OrderId, trader: TraderId, frozen_qty: Quantity, frozen_asset: AssetId, account_id: AccountId,
        trading_pair: TradingPair, price: Option<Price>, quantity: Quantity, side: Side, timestamp: u64
    ) -> Self {
        Self {
            order_id,
            frozen_qty,
            frozen_asset,
            trader_id: trader,
            account_id,
            trading_pair,
            total_qty: quantity,
            unfilled_qty: 0,
            price,
            side,
            status: OrderStatus::Pending,                            // 初始状态：待冻结余额
            execution_method: ExecutionMethod::Limit,                // 默认限价单
            conditional_type: ConditionalType::None,                 // 默认无条件
            algorithm_strategy: AlgorithmStrategy::None,             // 默认无算法
            maker_constraint: MakerConstraint::None,                 // 默认无约束
            time_in_force: TimeInForce::GTC,                         // 默认 GTC
            self_trade_prevention: SelfTradePrevention::ExpireTaker, // 固定 ExpireTaker
            timestamp,                                               // 创建时间戳
            last_updated: timestamp,                                 // 初始更新时间与创建时间相同
            executed_qty: 0,                                         // 初始未成交
            average_price: 0,                                        // 初始平均价为 0
            cumulative_quote_qty: 0,                                 // 初始累计金额为 0
            commission_qty: 0,                                       // 初始手续费为 0
            stop_price: None,                                        // 初始无止损/止盈价
            iceberg_qty: None,                                       // 初始无冰山数量
            source: OrderSource::API,                                // Phase 3: 默认来源为 API
            next_idx: None                                           // 初始链表索引为空
        }
    }

    /// 创建新的订单条目（遗留接口，使用 pending 替代）
    #[inline]
    pub fn new(
        order_id: OrderId, trader: TraderId, frozen_qty: Quantity, frozen_asset: AssetId, symbol: TradingPair,
        price: Option<Price>, quantity: Quantity, side: Side, _order_type: OrderType, timestamp: u64
    ) -> Self {
        // 遗留接口：转发到 pending 方法
        // 注意：这个方法不会设置 account_id，需要调用者补充
        Self {
            order_id,
            trader_id: trader,
            frozen_qty,
            frozen_asset,
            account_id: Default::default(), // 使用默认值
            trading_pair: symbol,
            total_qty: quantity,
            unfilled_qty: 0,
            price,
            side,
            status: OrderStatus::Pending,
            execution_method: ExecutionMethod::Limit,
            conditional_type: ConditionalType::None,
            algorithm_strategy: AlgorithmStrategy::None,
            maker_constraint: MakerConstraint::None,
            time_in_force: TimeInForce::GTC,
            self_trade_prevention: SelfTradePrevention::ExpireTaker,
            timestamp,
            last_updated: timestamp,
            executed_qty: 0,
            average_price: 0,
            cumulative_quote_qty: 0,
            commission_qty: 0,
            stop_price: None,
            iceberg_qty: None,
            source: OrderSource::API,
            next_idx: None
        }
    }

    /// 检查订单是否仍然有效（有未成交数量）
    #[inline]
    pub fn is_active(&self) -> bool { self.unfilled_qty < self.total_qty }


    /// 检查订单是否已成交完毕
    #[inline]
    pub fn is_filled(&self) -> bool { self.unfilled_qty == 0 }

    /// 获取已成交百分比（0.0 - 1.0）
    #[inline]
    pub fn fill_ratio(&self) -> f32 {
        if self.total_qty == 0 { 0.0 } else { self.unfilled_qty as f32 / self.total_qty as f32 }
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


/// 交易执行记录
///
/// 记录一次撮合成交的完整信息，用于：
/// - 事件溯源（TradeCreated事件）
/// - 账户结算（确定资金划转方向）
/// - 交易历史查询
#[derive(Debug, Clone, Copy)]
pub struct SpotTrade {
    /// 交易唯一标识
    pub trade_id: u64,
    /// 成交价格
    pub price: Price,
    /// 成交数量
    pub quantity: Quantity,
    /// Taker交易员（主动方，新订单提交者）
    pub taker_trader: TraderId,
    /// Maker交易员（被动方，订单簿中的挂单方）
    pub maker_trader: TraderId,
    /// Taker订单ID
    pub taker_order_id: OrderId,
    /// Maker订单ID
    pub maker_order_id: OrderId,
    /// Taker方向（Buy=Taker买入, Sell=Taker卖出）
    pub taker_side: Side
}

impl SpotTrade {
    /// 创建新的交易记录
    #[inline]
    pub fn new(
        trade_id: u64, price: Price, quantity: Quantity, taker_trader: TraderId, maker_trader: TraderId,
        taker_order_id: OrderId, maker_order_id: OrderId, taker_side: Side
    ) -> Self {
        todo!()
    }


    /// 获取买方订单ID
    #[inline]
    pub fn buyer_order_id(&self) -> OrderId {
        match self.taker_side {
            Side::Buy => self.taker_order_id,
            Side::Sell => self.maker_order_id
        }
    }

    /// 获取卖方订单ID
    #[inline]
    pub fn seller_order_id(&self) -> OrderId {
        match self.taker_side {
            Side::Buy => self.maker_order_id,
            Side::Sell => self.taker_order_id
        }
    }
}
