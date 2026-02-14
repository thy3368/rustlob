use std::fmt;
use diff::ChangeLogEntry;
use entity_derive::Entity;

use crate::account::balance::Balance;
use crate::base_types::TraderId;
use crate::fee::fee_types::{CexFeeEntity, FeeType};
use crate::lob::lob::LobOrder;
use crate::{
    AccountId, AssetId, InstrumentType, OrderId, OrderSide, Price, Quantity, Timestamp, TradingPair,
};

/// 订单来源标识 - Phase 3: 区分订单的来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    System = 6,
}

impl Default for OrderSource {
    fn default() -> Self {
        Self::API
    }
}

impl fmt::Display for OrderSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSource::API => write!(f, "REST API"),
            OrderSource::WebUI => write!(f, "Web UI"),
            OrderSource::MobileApp => write!(f, "Mobile App"),
            OrderSource::AlgorithmEngine => write!(f, "Algorithm"),
            OrderSource::ConditionalTrigger => write!(f, "Conditional Trigger"),
            OrderSource::System => write!(f, "System"),
        }
    }
}

/// 订单执行方式 - 定义订单如何与市场交互
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExecutionMethod {
    /// 限价单：按指定价格或更优价格执行
    Limit = 1,
    /// 市价单：以当前市场价格立即执行
    Market = 2,
}

impl Default for ExecutionMethod {
    fn default() -> Self {
        Self::Limit
    }
}

/// 做市商约束 - 定义订单是否只做Maker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MakerConstraint {
    /// 无约束：可作为Taker或Maker
    None = 0,
    /// 仅做Maker：拒绝任何Taker成交（PostOnly）
    PostOnly = 1,
}

impl Default for MakerConstraint {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConditionalType {
    /// 无条件：普通订单，无触发条件
    None = 0,
    /// 止损：当价格跌破 stop_price 时触发（用于风险控制）
    StopLoss = 1,
    /// 止盈：当价格上涨到 take_profit_price 时触发（用于利润固定）
    TakeProfit = 2,
}

impl Default for ConditionalType {
    fn default() -> Self {
        Self::None
    }
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelfTradePrevention {
    /// 取消 Taker（推荐且固定）
    /// - 新订单作为Taker时，如发生自交易则新订单被取消
    /// - 订单簿中的Maker订单保留
    /// - 最安全、最常用、最适合大多数场景
    ExpireTaker = 1,
}

// 默认实现：所有订单都使用 ExpireTaker
impl Default for SelfTradePrevention {
    fn default() -> Self {
        Self::ExpireTaker
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    DarkPool = 5,
}

impl Default for AlgorithmStrategy {
    fn default() -> Self {
        Self::None
    }
}

/// 订单类型 - 定义订单的执行方式和触发条件
///
/// 订单类型是对订单行为的高层分类，结合 `ExecutionMethod`、`ConditionalType` 和 `TimeInForce` 字段
/// 可以完整描述订单的所有特性。
///
/// ## 订单类型矩阵
///
/// | 订单类型 | ExecutionMethod | ConditionalType | TimeInForce | 触发条件 | 应用场景 |
/// |---------|-----------------|-----------------|-------------|---------|---------|
/// | Limit | Limit | None | GTC/IOC/FOK/GTX/GTD | 价格达到限价 | 精确价格交易 |
/// | Market | Market | None | IOC | 立即执行 | 快速成交 |
/// | StopLoss | Market | StopLoss | IOC | 价格跌破止损价 | 风险控制 |
/// | StopLossLimit | Limit | StopLoss | GTC/IOC/FOK/GTX/GTD | 价格跌破止损价 | 精确止损 |
/// | TakeProfit | Market | TakeProfit | IOC | 价格上涨到止盈价 | 利润固定 |
/// | TakeProfitLimit | Limit | TakeProfit | GTC/IOC/FOK/GTX/GTD | 价格上涨到止盈价 | 精确止盈 |
/// | LimitMaker | Limit | None | GTX | 仅做Maker | 做市商策略 |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrderType {
    /// **限价单 (Limit Order)**
    ///
    /// 按指定价格或更优价格执行的订单。订单进入订单簿，等待价格匹配时成交。
    ///
    /// **特性**:
    /// - 执行方式: `ExecutionMethod::Limit`
    /// - 条件类型: `ConditionalType::None`
    /// - 支持的TimeInForce: GTC, IOC, FOK, GTX, GTD
    /// - 价格: 必须指定
    /// - 数量: 必须指定
    ///
    /// **应用场景**:
    /// - 精确价格交易：以特定价格买入/卖出
    /// - 做市商策略：在买卖两侧同时挂单
    /// - 套利交易：等待特定价格差异
    ///
    /// **示例**:
    /// - 以50000 USDT买入1 BTC，等待卖家成交
    /// - 以51000 USDT卖出1 BTC，等待买家成交
    ///
    /// **手续费**: Maker手续费（通常0.05%）
    Limit,

    /// **市价单 (Market Order)**
    ///
    /// 以当前市场价格立即执行的订单。按对手盘最优价格逐级成交，不进入订单簿。
    ///
    /// **特性**:
    /// - 执行方式: `ExecutionMethod::Market`
    /// - 条件类型: `ConditionalType::None`
    /// - 支持的TimeInForce: IOC（固定）
    /// - 价格: 无（为None）
    /// - 数量或报价数量: 必须指定其一
    ///
    /// **应用场景**:
    /// - 快速成交：需要立即执行
    /// - 大额交易：快速吃掉对手盘流动性
    /// - 追单：追随市场价格变化
    ///
    /// **示例**:
    /// - 市价买入1 BTC，按当前最低卖价成交
    /// - 市价卖出1 BTC，按当前最高买价成交
    ///
    /// **手续费**: Taker手续费（通常0.1%）
    /// **风险**: 可能因流动性不足而滑点
    Market,

    /// **止损单 (Stop Loss Order)**
    ///
    /// 当价格跌破指定的止损价时，自动以市价执行的订单。用于风险控制和止损。
    ///
    /// **特性**:
    /// - 执行方式: `ExecutionMethod::Market`
    /// - 条件类型: `ConditionalType::StopLoss`
    /// - 支持的TimeInForce: IOC（固定）
    /// - 触发价格: `stop_price`（必须指定）
    /// - 执行价格: 无（市价执行）
    /// - 数量: 必须指定
    ///
    /// **触发逻辑**:
    /// - 买单: 当价格上涨到stop_price时触发
    /// - 卖单: 当价格跌破stop_price时触发
    ///
    /// **应用场景**:
    /// - 风险控制：限制亏损
    /// - 止损保护：自动平仓
    /// - 反向对冲：对冲头寸风险
    ///
    /// **示例**:
    /// - 持有1 BTC，设置止损价49000 USDT，价格跌破时自动卖出
    /// - 空头头寸，设置止损价51000 USDT，价格上涨时自动平仓
    ///
    /// **手续费**: Taker手续费（通常0.1%）
    /// **风险**: 市价执行可能滑点，极端行情可能无法成交
    StopLoss,

    /// **止损限价单 (Stop Loss Limit Order)**
    ///
    /// 当价格跌破止损价时，以指定的限价执行的订单。结合止损和限价的优势。
    ///
    /// **特性**:
    /// - 执行方式: `ExecutionMethod::Limit`
    /// - 条件类型: `ConditionalType::StopLoss`
    /// - 支持的TimeInForce: GTC, IOC, FOK, GTX, GTD
    /// - 触发价格: `stop_price`（必须指定）
    /// - 执行价格: `price`（必须指定）
    /// - 数量: 必须指定
    ///
    /// **触发逻辑**:
    /// - 价格跌破stop_price时，订单激活
    /// - 激活后按限价规则执行（进入订单簿等待匹配）
    ///
    /// **应用场景**:
    /// - 精确止损：控制止损价格
    /// - 风险控制：避免市价滑点
    /// - 套利交易：在特定价格范围内止损
    ///
    /// **示例**:
    /// - 持有1 BTC，设置止损价49000，限价48500，价格跌破49000时以≤48500价格卖出
    ///
    /// **手续费**: Maker手续费（通常0.05%）
    /// **优势**: 精确控制止损价格，避免市价滑点
    /// **风险**: 激活后可能无法以限价成交
    StopLossLimit,

    /// **止盈单 (Take Profit Order)**
    ///
    /// 当价格上涨到指定的止盈价时，自动以市价执行的订单。用于利润固定。
    ///
    /// **特性**:
    /// - 执行方式: `ExecutionMethod::Market`
    /// - 条件类型: `ConditionalType::TakeProfit`
    /// - 支持的TimeInForce: IOC（固定）
    /// - 触发价格: `stop_price`（必须指定）
    /// - 执行价格: 无（市价执行）
    /// - 数量: 必须指定
    ///
    /// **触发逻辑**:
    /// - 买单: 当价格跌破stop_price时触发
    /// - 卖单: 当价格上涨到stop_price时触发
    ///
    /// **应用场景**:
    /// - 利润固定：自动平仓获利
    /// - 目标价格成交：达到目标价自动执行
    /// - 趋势交易：在关键价位自动止盈
    ///
    /// **示例**:
    /// - 以50000 USDT买入1 BTC，设置止盈价52000，价格上涨到52000时自动卖出
    ///
    /// **手续费**: Taker手续费（通常0.1%）
    /// **风险**: 市价执行可能滑点，可能无法以预期价格成交
    TakeProfit,

    /// **止盈限价单 (Take Profit Limit Order)**
    ///
    /// 当价格上涨到止盈价时，以指定的限价执行的订单。结合止盈和限价的优势。
    ///
    /// **特性**:
    /// - 执行方式: `ExecutionMethod::Limit`
    /// - 条件类型: `ConditionalType::TakeProfit`
    /// - 支持的TimeInForce: GTC, IOC, FOK, GTX, GTD
    /// - 触发价格: `stop_price`（必须指定）
    /// - 执行价格: `price`（必须指定）
    /// - 数量: 必须指定
    ///
    /// **触发逻辑**:
    /// - 价格上涨到stop_price时，订单激活
    /// - 激活后按限价规则执行（进入订单簿等待匹配）
    ///
    /// **应用场景**:
    /// - 精确止盈：控制止盈价格
    /// - 风险控制：避免市价滑点
    /// - 分层止盈：多个止盈单在不同价位
    ///
    /// **示例**:
    /// - 以50000 USDT买入1 BTC，设置止盈价52000，限价52500，价格上涨到52000时以≥52500价格卖出
    ///
    /// **手续费**: Maker手续费（通常0.05%）
    /// **优势**: 精确控制止盈价格，避免市价滑点
    /// **风险**: 激活后可能无法以限价成交
    TakeProfitLimit,

    /// **限价只挂单 (Limit Maker / Post-Only Order)**
    ///
    /// 只能作为Maker（挂单方）的限价单。如果会立即成交则拒绝，确保享受Maker手续费优惠。
    ///
    /// **特性**:
    /// - 执行方式: `ExecutionMethod::Limit`
    /// - 条件类型: `ConditionalType::None`
    /// - 支持的TimeInForce: GTX（固定，PostOnly）
    /// - Maker约束: `MakerConstraint::PostOnly`
    /// - 价格: 必须指定
    /// - 数量: 必须指定
    ///
    /// **执行逻辑**:
    /// - 检查限价是否会立即匹配对手盘
    /// - 如果会立即成交，订单被拒绝（Rejected）
    /// - 如果不会立即成交，订单进入订单簿
    ///
    /// **应用场景**:
    /// - 做市商策略：确保享受Maker手续费
    /// - 流动性提供：在买卖两侧挂单
    /// - 高频交易：优化手续费成本
    ///
    /// **示例**:
    /// - 当前卖一价50000，提交49999买单 → 进入订单簿（Maker）
    /// - 当前卖一价50000，提交50000买单 → 订单被拒绝（会立即成交）
    ///
    /// **手续费**: Maker手续费（通常0.05%）
    /// **优势**: 保证Maker身份，手续费最低
    /// **风险**: 订单可能被拒绝，需要重新提交
    LimitMaker,
}

impl Default for OrderType {
    fn default() -> Self {
        Self::Limit
    }
}

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OrderStatus {
    Pending = 1,         // 待成交
    PartiallyFilled = 2, // 部分成交
    Filled = 3,          // 完全成交
    Cancelled = 4,       // 已取消

    Rejected = 5, // 已拒绝
    Expired = 6,  // 已过期
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "PENDING"),
            OrderStatus::PartiallyFilled => write!(f, "PARTIALLY_FILLED"),
            OrderStatus::Filled => write!(f, "FILLED"),
            OrderStatus::Cancelled => write!(f, "CANCELLED"),
            OrderStatus::Rejected => write!(f, "Rejected"),
            OrderStatus::Expired => write!(f, "Expired"),
        }
    }
}

/// 有效期类型 - 定义订单在订单簿中的存续时间和执行策略
///
/// ## 限价单 (Limit Order) 与市价单 (Market Order) 的使用对比
///
/// | TimeInForce | 限价单使用 | 市价单使用 |
/// |-------------|-----------|-----------|
/// | **GTC**     | ✅ 常用    | ❌ 不支持  |
/// | **IOC**     | ✅ 支持    | ✅ 等价于市价单 |
/// | **FOK**     | ✅ 支持    | ✅ 支持    |
/// | **GTX**     | ✅ 常用    | ❌ 不支持  |
/// | **GTD**     | ✅ 支持    | ❌ 不支持  |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimeInForce {
    /// GTC (Good Till Cancel) - 持续有效直到取消
    /// 订单会一直保留在订单簿中，直到完全成交或用户手动取消
    /// 适用场景：普通挂单交易，无时间限制
    ///
    /// **限价单 (Limit Order):**
    /// ✅ **最常用组合** - 按指定价格挂单，等待成交
    /// - 执行逻辑：订单进入订单簿，价格合适时匹配成交
    /// - 示例：以 50000 USDT 价格挂买单 1 BTC，等待卖家成交
    /// - 优点：精确控制价格，可享受 Maker 手续费优惠
    ///
    /// **市价单 (Market Order):**
    /// ❌ **不支持** - 市价单要求立即成交，与 GTC "持续有效" 语义冲突
    /// - 原因：市价单按当前市场价立即执行，不会进入订单簿等待
    GTC,

    /// IOC (Immediate Or Cancel) - 立即成交否则取消
    /// 订单立即执行能成交的部分，未成交部分自动取消，不会进入订单簿
    /// 适用场景：快速吃单，不想留下挂单
    ///
    /// **限价单 (Limit Order):**
    /// ✅ **支持** - 以限价或更优价格立即吃单，未成交部分取消
    /// - 执行逻辑：扫描订单簿对手盘，价格满足条件立即成交
    /// - 示例：以 50000 USDT 限价买入 BTC，立即吃掉 ≤50000 的卖单，剩余取消
    /// - 优点：控制最差价格（不超过限价），同时快速执行
    /// - 应用：大额订单分拆执行，避免单次冲击市场
    ///
    /// **市价单 (Market Order):**
    /// ✅ **等价于市价单默认行为** - 立即按市场价成交
    /// - 执行逻辑：扫描订单簿对手盘，按最优价格逐级成交
    /// - 示例：以市价买入 1 BTC，从最低卖价开始吃单
    /// - 注意：市价单 + IOC 本质上就是标准市价单
    IOC,

    /// FOK (Fill Or Kill) - 全部成交否则取消
    /// 订单必须立即完全成交，否则整个订单取消，不允许部分成交
    /// 适用场景：大额交易，要求一次性完整执行
    ///
    /// **限价单 (Limit Order):**
    /// ✅ **支持** - 必须以限价或更优价格一次性全部成交
    /// - 执行逻辑：检查订单簿深度，如果对手盘流动性足够则全部成交，否则取消
    /// - 示例：以 50000 USDT 买入 10 BTC，如果 ≤50000 的卖单总量 <10 BTC 则取消
    /// - 优点：避免部分成交导致的仓位不完整
    /// - 应用：对冲交易、套利交易（要求精确数量匹配）
    ///
    /// **市价单 (Market Order):**
    /// ✅ **支持** - 必须以市价一次性全部成交
    /// - 执行逻辑：检查订单簿深度，如果对手盘流动性足够则全部成交，否则取消
    /// - 示例：以市价买入 10 BTC，如果订单簿卖单总量 <10 BTC 则取消
    /// - 风险：流动性不足时订单被拒，需要重新提交
    /// - 应用：大额交易风险控制，防止滑点过大
    FOK,

    /// GTX (Good Till Crossing) - 只做Maker直到成交
    /// 订单只能作为Maker（挂单方），如果会立即成交则取消
    /// 等同于 PostOnly，确保订单不会作为Taker支付更高手续费
    /// 适用场景：做市商策略，追求Maker手续费优惠
    ///
    /// **限价单 (Limit Order):**
    /// ✅ **常用于做市商** - 确保订单进入订单簿，避免立即成交
    /// - 执行逻辑：检查限价是否会立即匹配，若会立即成交则拒绝订单
    /// - 示例：当前卖一价 50000，提交 50000 买单会立即成交 → 订单被拒绝
    /// - 示例：当前卖一价 50000，提交 49999 买单不会立即成交 → 订单进入订单簿
    /// - 优点：保证享受 Maker 手续费优惠（通常为 0.05% vs Taker 0.1%）
    /// - 应用：高频做市商、流动性提供商
    ///
    /// **市价单 (Market Order):**
    /// ❌ **不支持** - 市价单本质是立即成交（Taker），与 GTX "只做Maker"
    /// 语义冲突
    /// - 原因：市价单目的就是立即吃单，不可能作为 Maker
    GTX,

    /// GTD (Good Till Date) - 持续有效直到指定日期/时间
    /// 订单在指定时间前有效，到期后自动取消（Expired）
    /// 需要配合 SpotOrder.expire_time 字段使用
    /// 适用场景：日内交易、避免隔夜持仓、定时策略
    ///
    /// **限价单 (Limit Order):**
    /// ✅ **支持** - 设置订单过期时间，到期自动取消
    /// - 执行逻辑：订单进入订单簿，价格合适时成交，到期时间前未成交则自动取消
    /// - 示例：提交限价单在今天 23:59:59 前有效，之后自动取消
    /// - 示例：提交限价单在未来 1 小时内有效（当前时间 + 3600秒）
    /// - 优点：自动风险控制，无需手动取消过期订单
    /// - 应用场景：
    ///   - 日内交易：避免订单隔夜（设置收盘时间前过期）
    ///   - 短线策略：限定订单生命周期（如5分钟、1小时）
    ///   - 套利窗口：仅在特定时间窗口内参与（如开盘前竞价）
    ///
    /// **市价单 (Market Order):**
    /// ❌ **不支持** - 市价单立即执行，设置过期时间无意义
    /// - 原因：市价单提交后立即匹配成交或取消，不会等待
    ///
    /// **实现注意事项：**
    /// - `expire_time` 字段必须设置（Unix时间戳，毫秒）
    /// - 撮合引擎需要定期扫描过期订单并自动取消
    /// - 过期订单状态设置为 `OrderStatus::Expired`
    /// - 过期时释放冻结资金，发布 `OrderExpired` 事件
    GTD,
}

impl Default for TimeInForce {
    fn default() -> Self {
        Self::GTC
    }
}

impl LobOrder for SpotOrder {
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

    fn side(&self) -> OrderSide {
        match self.side {
            OrderSide::Buy => OrderSide::Buy,
            OrderSide::Sell => OrderSide::Sell,
        }
    }

    fn symbol(&self) -> TradingPair {
        self.trading_pair
    }
}

/// 订单簿条目（64字节缓存行对齐以提升性能）
#[repr(align(64))]
#[derive(Debug, Clone, Entity)]
#[entity(id = "order_id")]
pub struct SpotOrder {
    // ===== 核心标识字段（24字节）=====
    pub order_id: OrderId,   // 订单ID (u64)
    pub trader_id: TraderId, // 交易员ID ([u8; 8])

    pub trading_pair: TradingPair, // 交易对 (u64)

    pub total_qty: Quantity,               // 总数量
    pub price: Option<Price>,              // 订单价格 (0表示市价单)
    pub quote_order_qty: Option<Quantity>, // 报价数量（市价单使用，最多花费金额）
    pub side: OrderSide,                   // 买卖方向 (BUY/SELL) (1字节)
    pub time_in_force: TimeInForce,        // 有效期 (GTC/IOC/FOK/GTX/GTD) (1字节)

    pub status: OrderStatus, // 订单状态 (1字节)

    // ===== 可选字段 =====
    pub client_order_id: Option<String>, // 客户订单ID

    // ===== 订单来源（Phase 3：1字节）=====
    pub source: OrderSource, // 订单来源 (API/WebUI/Algorithm/Conditional/System)

    // ===== 订单类型维度（4字节）⭐ 新增算法策略维度 =====
    pub order_type: OrderType, // 订单类型 (Limit/Market/StopLoss/...)
    pub execution_method: ExecutionMethod, // 执行方式 (Limit/Market) (1字节)
    pub conditional_type: ConditionalType, // 条件类型 (None/StopLoss/TakeProfit) (1字节)
    pub algorithm_strategy: AlgorithmStrategy, // 算法策略 (None/TWAP/VWAP/...) (1字节)

    // ===== 有效期和防护（2字节）=====
    pub self_trade_prevention: SelfTradePrevention, // 自交易防护 (1字节，固定ExpireTaker)

    // ===== P3 优先级：可选属性 =====
    pub stop_price: Option<Price>, // 止损/止盈触发价（仅conditional_type != None时有效）
    pub iceberg_qty: Option<Quantity>, // 冰山单显示数量
    pub expire_time: Option<Timestamp>, // GTD过期时间（Unix时间戳，毫秒）仅time_in_force=GTD时有效

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
    pub timestamp: Timestamp, // 创建时间戳 (ms)
    pub last_updated: Timestamp,
}

impl SpotOrder {
    pub fn frozen_asset_id(&self) -> AssetId {
        // 根据买卖方向冻结相应的资产余额：买则冻结计算资产，卖则冻结基础资产
        let frozen_asset_id = match self.side() {
            OrderSide::Buy => self.trading_pair.quote_asset(),
            OrderSide::Sell => self.trading_pair.base_asset(),
        };

        frozen_asset_id
    }
}

impl SpotOrder {
    pub fn is_all_filled(&self) -> bool {
        self.total_qty == self.executed_qty
    }

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
        let base_asset = self.trading_pair.base_asset().as_str().to_string();
        let quote_asset = self.trading_pair.quote_asset().as_str().to_string();

        // 根据方向调用 CexFeeEntity 的费率计算
        let fee_type = if is_taker { FeeType::Taker } else { FeeType::Maker };

        // 调用费率计算函数（使用新的 API）
        match fee_entity.calculate_product_trading_fee(
            InstrumentType::Spot, // 现货交易
            fee_type,
            &base_asset,
            &quote_asset,
            filled.to_f64(),
            price.to_f64(),
            user_tier.map(|t| t as f64), // 30天交易量（用户分层）
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
                    (10, 0.001) // Taker 0.1% = 10 bp
                } else {
                    (5, 0.0005) // Maker 0.05% = 5 bp
                };
                let trade_value = filled * price;
                let commission_qty = trade_value * Quantity::from_f64(fee_rate_decimal);
                (fee_rate_bp, commission_qty)
            }
        }
    }

    pub fn make_trade(
        &mut self,
        matched_order: &mut SpotOrder,
        quote_asset_balance: &mut Balance,
        base_asset_balance: &mut Balance,
        o_quote_asset_balance: &mut Balance,
        o_base_asset_balance: &mut Balance,
    ) -> SpotTrade {
        let filled = self.unfilled_qty.min(matched_order.unfilled_qty);

        // 更新双方订单的成交数量
        self.unfilled_qty -= filled;
        self.executed_qty += filled;
        matched_order.unfilled_qty -= filled;
        matched_order.executed_qty += filled;

        let transaction_price = match self.price {
            None => matched_order.price.unwrap(),
            Some(price) => price,
        };

        // 计算 Taker 的手续费
        let (taker_commission_rate, taker_commission_qty) = self.calculate_fee_with_amount(
            &CexFeeEntity::new(),
            true,  // is_taker
            false, // is_market_maker
            None,  // user_vip_level
            None,  // user_tier
            filled,
            transaction_price,
        );

        // 计算 Maker 的手续费
        let (maker_commission_rate, maker_commission_qty) = matched_order.calculate_fee_with_amount(
            &CexFeeEntity::new(),
            false, // is_maker
            false, // is_market_maker
            None,  // user_vip_level
            None,  // user_tier
            filled,
            transaction_price,
        );

        // 更新 Taker 的余额
        match self.side {
            OrderSide::Buy => {
                quote_asset_balance.frozen2pay(filled * transaction_price, Timestamp::now_as_nanos());
                base_asset_balance.add_balance(filled, Timestamp::now_as_nanos());
            }
            OrderSide::Sell => {
                base_asset_balance.frozen2pay(filled, Timestamp::now_as_nanos());
                quote_asset_balance.add_balance(filled * transaction_price, Timestamp::now_as_nanos());
            }
        };

        // 更新 Maker 的余额
        match matched_order.side {
            OrderSide::Buy => {
                o_quote_asset_balance.frozen2pay(filled * transaction_price, Timestamp::now_as_nanos());
                o_base_asset_balance.add_balance(filled, Timestamp::now_as_nanos());
            }
            OrderSide::Sell => {
                o_base_asset_balance.frozen2pay(filled, Timestamp::now_as_nanos());
                o_quote_asset_balance.add_balance(filled * transaction_price, Timestamp::now_as_nanos());
            }
        };

        // 生成交易ID
        let trade_id = (self.timestamp.0 << 32) | (self.order_id & 0xFFFFFFFF) as u64;

        // 创建一条 trade 记录（包含买卖双方信息）
        SpotTrade::new(
            trade_id,
            self.order_id,
            matched_order.order_id,
            Timestamp::now_as_nanos(),
            transaction_price,
            filled,
            self.side,
            taker_commission_qty,
            maker_commission_qty,
            self.frozen_asset,
            taker_commission_rate,
            maker_commission_rate,
        )
    }

    pub fn frozen_margin(&mut self, balance: &mut Balance, now: Timestamp) {
        // 根据买卖方向确定冻结资产
        match self.side {
            OrderSide::Buy => {
                // 冻结，失败则reject
                self.frozen_qty = self.total_qty * self.price.unwrap();
                self.frozen_asset = self.trading_pair.quote_asset();
                balance.frozen(self.frozen_qty, now);
            }
            OrderSide::Sell => {
                self.frozen_qty = self.total_qty;
                self.frozen_asset = self.trading_pair.base_asset();
                balance.frozen(self.frozen_qty, now);
            }
        };

        // 冻结，失败则reject
    }

    #[inline]
    pub fn create_order(
        order_id: OrderId,
        trader_id: TraderId,
        trading_pair: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
        time_in_force: TimeInForce,
        client_order_id: Option<String>,
        quote_order_qty: Option<Quantity>,
    ) -> Self {
        let timestamp = Timestamp::now_as_nanos();

        Self {
            order_id,
            trader_id,
            trading_pair,
            total_qty: quantity,
            unfilled_qty: quantity,
            price: Some(price),
            quote_order_qty,
            side,
            status: OrderStatus::Pending,
            order_type: OrderType::Limit,
            execution_method: ExecutionMethod::Limit,
            conditional_type: ConditionalType::None,
            algorithm_strategy: AlgorithmStrategy::None,
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
            expire_time: None, // GTD 过期时间，默认为 None
            source: OrderSource::API,
            client_order_id,
            frozen_qty: Quantity::default(),
            frozen_asset: AssetId::default(),
            filled_asset: AssetId::default(),
            commission_asset: AssetId::default(),
        }
    }

    /// 检查订单是否仍然有效（有未成交数量）
    #[inline]
    pub fn is_active(&self) -> bool {
        self.unfilled_qty < self.total_qty
    }

    /// 检查订单是否已成交完毕
    #[inline]
    pub fn is_filled(&self) -> bool {
        self.unfilled_qty == Quantity::default()
    }

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
        self.last_updated = Timestamp::now_as_nanos();
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
///
/// 一笔成交只生成一条记录，包含买卖双方的订单ID
#[derive(Debug, Clone, Copy, Entity)]
#[entity(id = "trade_id")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpotTrade {
    // ===== 交易标识字段（32字节）=====
    /// 交易唯一标识
    pub trade_id: u64,
    /// Taker 订单ID（新提交的订单）
    pub taker_order_id: OrderId,
    /// Maker 订单ID（订单簿中的订单）
    pub maker_order_id: OrderId,
    /// 成交时间戳 (ms)
    pub timestamp: Timestamp,

    // ===== 价格和数量（24字节）=====
    /// 成交价格
    pub price: Price,
    /// 成交数量
    pub quantity: Quantity,
    /// 成交金额 = quantity × price
    pub quote_qty: Quantity,

    // ===== 交易方向（1字节）=====
    /// Taker方向（Buy=Taker买入, Sell=Taker卖出）
    pub taker_side: OrderSide,

    // ===== 手续费字段（32字节）=====
    /// Taker 手续费数量
    pub taker_commission_qty: Quantity,
    /// Maker 手续费数量
    pub maker_commission_qty: Quantity,
    /// 手续费资产
    pub commission_asset: AssetId,
    /// Taker 手续费率 (bp, 基点)
    pub taker_commission_rate: i32,
    /// Maker 手续费率 (bp, 基点)
    pub maker_commission_rate: i32,
}

impl SpotTrade {
    
    
    
    
    //todo 计算balance change logs
    pub fn cal_balance(&self, taker_base_balance:  &mut Balance, taker_quoto_balance:  &mut Balance, marker_base_balance:  &mut Balance, maker_quoto_balance:  &mut Balance) -> Vec<ChangeLogEntry> {
        todo!()
    }
}

impl SpotTrade {
    /// 创建新的交易记录（一笔成交只生成一条记录）
    #[inline]
    pub fn new(
        trade_id: u64,
        taker_order_id: OrderId,
        maker_order_id: OrderId,
        timestamp: Timestamp,
        price: Price,
        quantity: Quantity,
        taker_side: OrderSide,
        taker_commission_qty: Quantity,
        maker_commission_qty: Quantity,
        commission_asset: AssetId,
        taker_commission_rate: i32,
        maker_commission_rate: i32,
    ) -> Self {
        let quote_qty = quantity * price;

        Self {
            trade_id,
            taker_order_id,
            maker_order_id,
            timestamp,
            price,
            quantity,
            quote_qty,
            taker_side,
            taker_commission_qty,
            maker_commission_qty,
            commission_asset,
            taker_commission_rate,
            maker_commission_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_balance() -> Balance {
        Balance::with_available(
            AccountId(1),
            AssetId::Usdt,
            1_000_000_000, // 10亿
            Timestamp::now_as_nanos(),
        )
    }

    fn create_test_trading_pair() -> TradingPair {
        TradingPair::BtcUsdt
    }

}
