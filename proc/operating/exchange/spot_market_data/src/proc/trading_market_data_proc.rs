//! 行情数据查询处理器 (Market Data Query Processor)
//!
//! 提供 Level 1/2/3 行情数据查询功能，遵循 CQRS 模式（Query 查询模型，Command 命令模型）
//!
//! # 架构设计
//!
//! 本模块采用 Clean Architecture 分层设计：
//! - **Domain Layer**: 定义查询命令、结果和错误类型
//! - **Repository Layer**: 定义数据仓储接口
//! - **Service Layer**: 定义查询处理器接口
//!
//! # 主要功能
//!
//! - 📊 **快照查询**: 查询 L1/L2/L3 市场快照数据
//! - 📈 **增量数据**: 实时订单簿变更、成交和价格变动
//! - 🔍 **批量查询**: 支持多交易对批量查询
//! - ⚡ **高性能**: 零拷贝设计，支持低延迟场景
//!
//! # 使用示例
//!
//! ```rust,no_run
//! use lob::lob::*;
//!
//! // 1. 查询 L1 数据
//! let query = QueryLevel1::new(symbol_id, sequence);
//! let result = processor.handle_query_level1(query)?;
//!
//! // 2. 查询 L2 深度数据
//! let query = QueryLevel2::depth_10(symbol_id, sequence);
//! let result = processor.handle_query_level2(query);
//!
//! // 3. 查询增量数据
//! let query = QueryIncrementalData::new(symbol_id, from_seq, to_seq);
//! let result = processor.handle_query_incremental_data(query)?;
//! ```

use lob::lob::*;
use spot_market_data::domain::entity::level_types::{Level1, Level2, Level3, Level3Order, PriceLevel};
use std::fmt;

// ============================================================================
// 类型别名
// ============================================================================

/// 交易对ID类型别名
///
/// 使用 u32 类型表示交易对ID，支持最多 4,294,967,295 个交易对
pub type SymbolId = u32;

/// 序列号类型别名
///
/// 用于标识事件的顺序，确保数据一致性
pub type SequenceNumber = u64;

// ============================================================================
// 数据提供者 Trait 定义
// ============================================================================

/// Level 1 数据快照提供者
pub trait Level1SnapshotRepo {
    /// 获取 Level 1 快照
    fn query_level1(&self, symbol_id: SymbolId, sequence: u64) -> Option<Level1>;
}

/// Level 2 数据快照提供者
pub trait Level2SnapshotRepo: Level1SnapshotRepo {
    /// 获取 Level 2 快照
    fn query_level2(&self, symbol_id: SymbolId, sequence: u64, depth: usize) -> Level2<10>;
}

/// Level 3 数据快照提供者
pub trait Level3SnapshotRepo: Level2SnapshotRepo {
    /// 获取 Level 3 快照
    fn query_level3(&self, symbol_id: SymbolId, sequence: u64) -> Level3;
}

// ============================================================================
// 错误类型
// ============================================================================

/// 行情查询错误
///
/// 定义所有可能的查询错误类型，支持详细的错误信息和上下文
#[derive(Debug, Clone, PartialEq)]
pub enum MarketDataQueryError {
    /// 交易对不存在
    ///
    /// 当查询的交易对ID在系统中不存在时返回此错误
    SymbolNotFound {
        /// 交易对ID
        symbol_id: SymbolId,
    },

    /// 订单不存在
    ///
    /// 当查询的订单ID在订单簿中不存在时返回此错误
    OrderNotFound {
        /// 订单ID
        order_id: OrderId,
    },

    /// 订单簿为空
    ///
    /// 当交易对的订单簿中没有任何订单时返回此错误
    EmptyOrderBook {
        /// 交易对ID
        symbol_id: SymbolId,
    },

    /// 流动性不足
    ///
    /// 当请求的数量超过可用流动性时返回此错误
    InsufficientLiquidity {
        /// 交易对ID
        symbol_id: SymbolId,
        /// 买卖方向
        side: Side,
        /// 请求数量
        requested: Quantity,
        /// 可用数量
        available: Quantity,
    },

    /// 参数无效
    ///
    /// 当查询参数不符合要求时返回此错误
    InvalidParameter {
        /// 参数字段名
        field: &'static str,
        /// 错误原因
        reason: &'static str,
    },

    /// 序列号范围无效
    ///
    /// 当序列号范围不合法时返回此错误（如 from >= to）
    InvalidSequenceRange {
        /// 起始序列号
        from: SequenceNumber,
        /// 结束序列号
        to: SequenceNumber,
    },

    /// 深度参数无效
    ///
    /// 当请求的深度超出限制时返回此错误
    InvalidDepth {
        /// 请求的深度
        requested: usize,
        /// 最大允许深度
        max_allowed: usize,
    },

    /// 内部错误
    ///
    /// 系统内部错误，包含详细错误信息
    Internal {
        /// 错误消息
        message: String,
    },
}

impl fmt::Display for MarketDataQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SymbolNotFound { symbol_id } => {
                write!(f, "交易对不存在: symbol_id={}", symbol_id)
            }
            Self::OrderNotFound { order_id } => {
                write!(f, "订单不存在: order_id={}", order_id)
            }
            Self::EmptyOrderBook { symbol_id } => {
                write!(f, "订单簿为空: symbol_id={}", symbol_id)
            }
            Self::InsufficientLiquidity {
                symbol_id,
                side,
                requested,
                available,
            } => {
                write!(
                    f,
                    "流动性不足: symbol_id={}, side={:?}, requested={}, available={}",
                    symbol_id, side, requested, available
                )
            }
            Self::InvalidParameter { field, reason } => {
                write!(f, "参数无效: field='{}', reason='{}'", field, reason)
            }
            Self::InvalidSequenceRange { from, to } => {
                write!(
                    f,
                    "序列号范围无效: from={}, to={} (from must be < to)",
                    from, to
                )
            }
            Self::InvalidDepth {
                requested,
                max_allowed,
            } => {
                write!(
                    f,
                    "深度参数无效: requested={}, max_allowed={}",
                    requested, max_allowed
                )
            }
            Self::Internal { message } => {
                write!(f, "内部错误: {}", message)
            }
        }
    }
}

impl std::error::Error for MarketDataQueryError {}

impl MarketDataQueryError {
    /// 创建交易对不存在错误
    #[inline]
    pub fn symbol_not_found(symbol_id: SymbolId) -> Self {
        Self::SymbolNotFound { symbol_id }
    }

    /// 创建订单不存在错误
    #[inline]
    pub fn order_not_found(order_id: OrderId) -> Self {
        Self::OrderNotFound { order_id }
    }

    /// 创建订单簿为空错误
    #[inline]
    pub fn empty_order_book(symbol_id: SymbolId) -> Self {
        Self::EmptyOrderBook { symbol_id }
    }

    /// 创建流动性不足错误
    #[inline]
    pub fn insufficient_liquidity(
        symbol_id: SymbolId,
        side: Side,
        requested: Quantity,
        available: Quantity,
    ) -> Self {
        Self::InsufficientLiquidity {
            symbol_id,
            side,
            requested,
            available,
        }
    }

    /// 创建参数无效错误
    #[inline]
    pub fn invalid_parameter(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidParameter { field, reason }
    }

    /// 创建序列号范围无效错误
    #[inline]
    pub fn invalid_sequence_range(from: SequenceNumber, to: SequenceNumber) -> Self {
        Self::InvalidSequenceRange { from, to }
    }

    /// 创建深度参数无效错误
    #[inline]
    pub fn invalid_depth(requested: usize, max_allowed: usize) -> Self {
        Self::InvalidDepth {
            requested,
            max_allowed,
        }
    }

    /// 创建内部错误
    #[inline]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// 判断是否为客户端错误（4xx类错误）
    #[inline]
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::SymbolNotFound { .. }
                | Self::OrderNotFound { .. }
                | Self::InvalidParameter { .. }
                | Self::InvalidSequenceRange { .. }
                | Self::InvalidDepth { .. }
        )
    }

    /// 判断是否为服务端错误（5xx类错误）
    #[inline]
    pub fn is_server_error(&self) -> bool {
        matches!(self, Self::Internal { .. })
    }
}

// ============================================================================
// Level 1 查询命令
// ============================================================================

/// Level 1 查询命令：获取单个交易对的最优买卖价
///
/// # 示例
///
/// ```rust,no_run
/// # use lob::lob::*;
/// let query = QueryLevel1::new(1, 12345);
/// // 或使用 Builder 模式
/// let query = QueryLevel1::builder()
///     .symbol_id(1)
///     .sequence(12345)
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryLevel1 {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 序列号
    pub sequence: SequenceNumber,
}

impl QueryLevel1 {
    /// 创建 Level 1 查询命令
    ///
    /// # 参数
    ///
    /// * `symbol_id` - 交易对ID
    /// * `sequence` - 序列号
    #[inline]
    pub fn new(symbol_id: SymbolId, sequence: SequenceNumber) -> Self {
        Self {
            symbol_id,
            sequence,
        }
    }

    /// 创建 Builder
    #[inline]
    pub fn builder() -> QueryLevel1Builder {
        QueryLevel1Builder::default()
    }

    /// 获取交易对ID
    #[inline]
    pub fn symbol_id(&self) -> SymbolId {
        self.symbol_id
    }

    /// 获取序列号
    #[inline]
    pub fn sequence(&self) -> SequenceNumber {
        self.sequence
    }
}

/// Level 1 查询命令 Builder
#[derive(Debug, Default)]
pub struct QueryLevel1Builder {
    symbol_id: Option<SymbolId>,
    sequence: Option<SequenceNumber>,
}

impl QueryLevel1Builder {
    /// 设置交易对ID
    pub fn symbol_id(mut self, symbol_id: SymbolId) -> Self {
        self.symbol_id = Some(symbol_id);
        self
    }

    /// 设置序列号
    pub fn sequence(mut self, sequence: SequenceNumber) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// 构建查询命令
    ///
    /// # Panics
    ///
    /// 如果必需字段未设置，将会 panic
    pub fn build(self) -> QueryLevel1 {
        QueryLevel1 {
            symbol_id: self.symbol_id.expect("symbol_id is required"),
            sequence: self.sequence.expect("sequence is required"),
        }
    }

    /// 尝试构建查询命令
    ///
    /// # 错误
    ///
    /// 如果必需字段未设置，返回错误
    pub fn try_build(self) -> Result<QueryLevel1, MarketDataQueryError> {
        Ok(QueryLevel1 {
            symbol_id: self
                .symbol_id
                .ok_or_else(|| MarketDataQueryError::invalid_parameter("symbol_id", "required"))?,
            sequence: self
                .sequence
                .ok_or_else(|| MarketDataQueryError::invalid_parameter("sequence", "required"))?,
        })
    }
}

/// Level 1 批量查询命令：获取多个交易对的行情
///
/// # 示例
///
/// ```rust,no_run
/// # use lob::lob::*;
/// let query = QueryLevel1Batch::new(vec![1, 2, 3], 12345);
/// // 或使用 Builder 模式
/// let query = QueryLevel1Batch::builder()
///     .add_symbol(1)
///     .add_symbol(2)
///     .add_symbol(3)
///     .sequence(12345)
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryLevel1Batch {
    /// 交易对ID列表
    pub symbol_ids: Vec<SymbolId>,
    /// 批次序列号
    pub sequence: SequenceNumber,
}

impl QueryLevel1Batch {
    /// 创建批量查询命令
    ///
    /// # 参数
    ///
    /// * `symbol_ids` - 交易对ID列表
    /// * `sequence` - 序列号
    pub fn new(symbol_ids: Vec<SymbolId>, sequence: SequenceNumber) -> Self {
        Self {
            symbol_ids,
            sequence,
        }
    }

    /// 创建 Builder
    #[inline]
    pub fn builder() -> QueryLevel1BatchBuilder {
        QueryLevel1BatchBuilder::default()
    }

    /// 获取交易对数量
    #[inline]
    pub fn len(&self) -> usize {
        self.symbol_ids.len()
    }

    /// 判断是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.symbol_ids.is_empty()
    }

    /// 获取序列号
    #[inline]
    pub fn sequence(&self) -> SequenceNumber {
        self.sequence
    }
}

/// Level 1 批量查询命令 Builder
#[derive(Debug, Default)]
pub struct QueryLevel1BatchBuilder {
    symbol_ids: Vec<SymbolId>,
    sequence: Option<SequenceNumber>,
}

impl QueryLevel1BatchBuilder {
    /// 添加交易对ID
    pub fn add_symbol(mut self, symbol_id: SymbolId) -> Self {
        self.symbol_ids.push(symbol_id);
        self
    }

    /// 添加多个交易对ID
    pub fn add_symbols(mut self, symbol_ids: impl IntoIterator<Item = SymbolId>) -> Self {
        self.symbol_ids.extend(symbol_ids);
        self
    }

    /// 设置序列号
    pub fn sequence(mut self, sequence: SequenceNumber) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// 构建查询命令
    ///
    /// # Panics
    ///
    /// 如果必需字段未设置，将会 panic
    pub fn build(self) -> QueryLevel1Batch {
        QueryLevel1Batch {
            symbol_ids: self.symbol_ids,
            sequence: self.sequence.expect("sequence is required"),
        }
    }

    /// 尝试构建查询命令
    ///
    /// # 错误
    ///
    /// 如果必需字段未设置或交易对列表为空，返回错误
    pub fn try_build(self) -> Result<QueryLevel1Batch, MarketDataQueryError> {
        if self.symbol_ids.is_empty() {
            return Err(MarketDataQueryError::invalid_parameter(
                "symbol_ids",
                "cannot be empty",
            ));
        }

        Ok(QueryLevel1Batch {
            symbol_ids: self.symbol_ids,
            sequence: self
                .sequence
                .ok_or_else(|| MarketDataQueryError::invalid_parameter("sequence", "required"))?,
        })
    }
}



// ============================================================================
// Level 1 查询结果
// ============================================================================

/// Level 1 查询结果
#[derive(Debug, Clone, Copy)]
pub struct Level1QueryResult {
    /// Level 1 快照数据
    pub snapshot: Level1,
}

/// Level 1 批量查询结果
#[derive(Debug, Clone)]
pub struct Level1BatchQueryResult {
    /// 成功的快照列表
    pub snapshots: Vec<(SymbolId, Level1)>,
    /// 失败的交易对ID列表
    pub failed_symbols: Vec<SymbolId>,
}



// ============================================================================
// Level 2 查询命令
// ============================================================================

/// Level 2 查询命令：获取指定深度的盘口快照
///
/// # 示例
///
/// ```rust,no_run
/// # use lob::lob::*;
/// // 使用预定义深度
/// let query = QueryLevel2::depth_10(1, 12345);
/// let query = QueryLevel2::depth_20(1, 12345);
///
/// // 自定义深度
/// let query = QueryLevel2::new(1, 12345, 50);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryLevel2 {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 序列号
    pub sequence: SequenceNumber,
    /// 深度档位（例如 10档、20档）
    pub depth: usize,
}

impl QueryLevel2 {
    /// 最大允许深度
    pub const MAX_DEPTH: usize = 100;

    /// 创建 Level 2 查询命令
    ///
    /// # 参数
    ///
    /// * `symbol_id` - 交易对ID
    /// * `sequence` - 序列号
    /// * `depth` - 深度档位
    ///
    /// # Panics
    ///
    /// 如果深度超过最大允许值，将会 panic
    pub fn new(symbol_id: SymbolId, sequence: SequenceNumber, depth: usize) -> Self {
        assert!(
            depth > 0 && depth <= Self::MAX_DEPTH,
            "depth must be between 1 and {}",
            Self::MAX_DEPTH
        );
        Self {
            symbol_id,
            sequence,
            depth,
        }
    }

    /// 尝试创建 Level 2 查询命令
    ///
    /// # 错误
    ///
    /// 如果深度超出范围，返回错误
    pub fn try_new(
        symbol_id: SymbolId,
        sequence: SequenceNumber,
        depth: usize,
    ) -> Result<Self, MarketDataQueryError> {
        if depth == 0 {
            return Err(MarketDataQueryError::invalid_parameter(
                "depth",
                "must be greater than 0",
            ));
        }
        if depth > Self::MAX_DEPTH {
            return Err(MarketDataQueryError::invalid_depth(depth, Self::MAX_DEPTH));
        }
        Ok(Self {
            symbol_id,
            sequence,
            depth,
        })
    }

    /// 创建10档深度查询
    #[inline]
    pub fn depth_10(symbol_id: SymbolId, sequence: SequenceNumber) -> Self {
        Self::new(symbol_id, sequence, 10)
    }

    /// 创建20档深度查询
    #[inline]
    pub fn depth_20(symbol_id: SymbolId, sequence: SequenceNumber) -> Self {
        Self::new(symbol_id, sequence, 20)
    }

    /// 创建50档深度查询
    #[inline]
    pub fn depth_50(symbol_id: SymbolId, sequence: SequenceNumber) -> Self {
        Self::new(symbol_id, sequence, 50)
    }

    /// 获取交易对ID
    #[inline]
    pub fn symbol_id(&self) -> SymbolId {
        self.symbol_id
    }

    /// 获取序列号
    #[inline]
    pub fn sequence(&self) -> SequenceNumber {
        self.sequence
    }

    /// 获取深度
    #[inline]
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// 验证深度是否有效
    #[inline]
    pub fn validate(&self) -> Result<(), MarketDataQueryError> {
        if self.depth == 0 {
            return Err(MarketDataQueryError::invalid_parameter(
                "depth",
                "must be greater than 0",
            ));
        }
        if self.depth > Self::MAX_DEPTH {
            return Err(MarketDataQueryError::invalid_depth(
                self.depth,
                Self::MAX_DEPTH,
            ));
        }
        Ok(())
    }
}




// ============================================================================
// Level 2 查询结果
// ============================================================================

/// Level 2 查询结果
#[derive(Debug, Clone)]
pub struct Level2QueryResult {
    /// Level 2 快照数据
    pub snapshot: Level2<10>,
}



// ============================================================================
// Level 3 查询命令
// ============================================================================

/// Level 3 查询命令：获取完整订单簿快照
#[derive(Debug, Clone, Copy)]
pub struct QueryLevel3 {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 序列号
    pub sequence: u64,
}

impl QueryLevel3 {
    pub fn new(symbol_id: SymbolId, sequence: u64) -> Self {
        Self {
            symbol_id,
            sequence,
        }
    }
}






// ============================================================================
// Level 3 查询结果
// ============================================================================

/// Level 3 查询结果
#[derive(Debug, Clone)]
pub struct Level3QueryResult {
    /// Level 3 快照数据
    pub snapshot: Level3,
}






// ============================================================================
// 增量数据事件定义
// ============================================================================

/// 订单簿变更类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderBookChangeType {
    /// 新增订单
    Add,
    /// 修改订单（数量变化）
    Modify,
    /// 删除订单
    Delete,
}

/// 订单簿增量变更事件
#[derive(Debug, Clone, Copy)]
pub struct OrderBookDelta {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 事件时间戳（纳秒）
    pub timestamp: u64,
    /// 序列号
    pub sequence: u64,
    /// 变更类型
    pub change_type: OrderBookChangeType,
    /// 订单ID
    pub order_id: OrderId,
    /// 买卖方向
    pub side: Side,
    /// 价格
    pub price: Price,
    /// 数量（新数量或变化量）
    pub quantity: Quantity,
    /// 交易者ID（可选）
    pub trader_id: Option<TraderId>,
}

/// 成交事件
#[derive(Debug, Clone, Copy)]
pub struct TradeEvent {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 成交时间戳（纳秒）
    pub timestamp: u64,
    /// 序列号
    pub sequence: u64,
    /// 成交ID
    pub trade_id: u64,
    /// 买方订单ID
    pub buyer_order_id: OrderId,
    /// 卖方订单ID
    pub seller_order_id: OrderId,
    /// 成交价格
    pub price: Price,
    /// 成交数量
    pub quantity: Quantity,
    /// 主动方（买方或卖方）
    pub aggressor_side: Side,
}

/// 最优买卖价变更事件
#[derive(Debug, Clone, Copy)]
pub struct BboChangeEvent {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 事件时间戳（纳秒）
    pub timestamp: u64,
    /// 序列号
    pub sequence: u64,
    /// 最优买价
    pub best_bid: Option<Price>,
    /// 最优买价数量
    pub best_bid_quantity: Quantity,
    /// 最优卖价
    pub best_ask: Option<Price>,
    /// 最优卖价数量
    pub best_ask_quantity: Quantity,
}

/// 市场数据增量事件（统一枚举）
#[derive(Debug, Clone, Copy)]
pub enum MarketDataDelta {
    /// 订单簿变更
    OrderBookChange(OrderBookDelta),
    /// 成交事件
    Trade(TradeEvent),
    /// 最优买卖价变更
    BboChange(BboChangeEvent),
}

// ============================================================================
// 增量数据查询命令
// ============================================================================

/// 查询增量数据命令
///
/// 用于查询指定序列号范围内的增量数据（订单簿变更、成交、价格变动等）
///
/// # 示例
///
/// ```rust,no_run
/// # use lob::lob::*;
/// // 查询序列号 1000 到 2000 之间的增量数据
/// let query = QueryIncrementalData::new(1, 1000, 2000);
///
/// // 使用 Builder 模式
/// let query = QueryIncrementalData::builder()
///     .symbol_id(1)
///     .from_sequence(1000)
///     .to_sequence(2000)
///     .build();
///
/// // 查询最近 100 个事件
/// let query = QueryIncrementalData::last_n(1, current_seq, 100);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryIncrementalData {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 起始序列号（不包含）
    pub from_sequence: SequenceNumber,
    /// 结束序列号（包含）
    pub to_sequence: SequenceNumber,
}

impl QueryIncrementalData {
    /// 最大查询范围（防止一次查询过多数据）
    pub const MAX_RANGE: u64 = 10000;

    /// 创建增量数据查询命令
    ///
    /// # 参数
    ///
    /// * `symbol_id` - 交易对ID
    /// * `from_sequence` - 起始序列号（不包含）
    /// * `to_sequence` - 结束序列号（包含）
    ///
    /// # Panics
    ///
    /// 如果序列号范围无效，将会 panic
    pub fn new(
        symbol_id: SymbolId,
        from_sequence: SequenceNumber,
        to_sequence: SequenceNumber,
    ) -> Self {
        assert!(
            from_sequence < to_sequence,
            "from_sequence must be less than to_sequence"
        );
        Self {
            symbol_id,
            from_sequence,
            to_sequence,
        }
    }

    /// 尝试创建增量数据查询命令
    ///
    /// # 错误
    ///
    /// 如果序列号范围无效，返回错误
    pub fn try_new(
        symbol_id: SymbolId,
        from_sequence: SequenceNumber,
        to_sequence: SequenceNumber,
    ) -> Result<Self, MarketDataQueryError> {
        if from_sequence >= to_sequence {
            return Err(MarketDataQueryError::invalid_sequence_range(
                from_sequence,
                to_sequence,
            ));
        }

        let range = to_sequence - from_sequence;
        if range > Self::MAX_RANGE {
            return Err(MarketDataQueryError::invalid_parameter(
                "sequence_range",
                "range too large",
            ));
        }

        Ok(Self {
            symbol_id,
            from_sequence,
            to_sequence,
        })
    }

    /// 创建查询最近 N 个事件的命令
    ///
    /// # 参数
    ///
    /// * `symbol_id` - 交易对ID
    /// * `latest_sequence` - 最新序列号
    /// * `count` - 要查询的事件数量
    pub fn last_n(symbol_id: SymbolId, latest_sequence: SequenceNumber, count: u64) -> Self {
        let from_sequence = latest_sequence.saturating_sub(count);
        Self {
            symbol_id,
            from_sequence,
            to_sequence: latest_sequence,
        }
    }

    /// 创建 Builder
    #[inline]
    pub fn builder() -> QueryIncrementalDataBuilder {
        QueryIncrementalDataBuilder::default()
    }

    /// 获取交易对ID
    #[inline]
    pub fn symbol_id(&self) -> SymbolId {
        self.symbol_id
    }

    /// 获取起始序列号
    #[inline]
    pub fn from_sequence(&self) -> SequenceNumber {
        self.from_sequence
    }

    /// 获取结束序列号
    #[inline]
    pub fn to_sequence(&self) -> SequenceNumber {
        self.to_sequence
    }

    /// 获取查询范围大小
    #[inline]
    pub fn range(&self) -> u64 {
        self.to_sequence - self.from_sequence
    }

    /// 验证查询参数
    pub fn validate(&self) -> Result<(), MarketDataQueryError> {
        if self.from_sequence >= self.to_sequence {
            return Err(MarketDataQueryError::invalid_sequence_range(
                self.from_sequence,
                self.to_sequence,
            ));
        }

        let range = self.range();
        if range > Self::MAX_RANGE {
            return Err(MarketDataQueryError::invalid_parameter(
                "sequence_range",
                "range too large",
            ));
        }

        Ok(())
    }

    /// 分页查询：获取下一页
    ///
    /// 返回一个新的查询命令，用于查询下一页数据
    pub fn next_page(&self, page_size: u64) -> Self {
        Self {
            symbol_id: self.symbol_id,
            from_sequence: self.to_sequence,
            to_sequence: self.to_sequence + page_size,
        }
    }
}

/// 增量数据查询命令 Builder
#[derive(Debug, Default)]
pub struct QueryIncrementalDataBuilder {
    symbol_id: Option<SymbolId>,
    from_sequence: Option<SequenceNumber>,
    to_sequence: Option<SequenceNumber>,
}

impl QueryIncrementalDataBuilder {
    /// 设置交易对ID
    pub fn symbol_id(mut self, symbol_id: SymbolId) -> Self {
        self.symbol_id = Some(symbol_id);
        self
    }

    /// 设置起始序列号
    pub fn from_sequence(mut self, from_sequence: SequenceNumber) -> Self {
        self.from_sequence = Some(from_sequence);
        self
    }

    /// 设置结束序列号
    pub fn to_sequence(mut self, to_sequence: SequenceNumber) -> Self {
        self.to_sequence = Some(to_sequence);
        self
    }

    /// 设置查询范围
    pub fn range(mut self, from: SequenceNumber, to: SequenceNumber) -> Self {
        self.from_sequence = Some(from);
        self.to_sequence = Some(to);
        self
    }

    /// 构建查询命令
    ///
    /// # Panics
    ///
    /// 如果必需字段未设置，将会 panic
    pub fn build(self) -> QueryIncrementalData {
        QueryIncrementalData {
            symbol_id: self.symbol_id.expect("symbol_id is required"),
            from_sequence: self.from_sequence.expect("from_sequence is required"),
            to_sequence: self.to_sequence.expect("to_sequence is required"),
        }
    }

    /// 尝试构建查询命令
    ///
    /// # 错误
    ///
    /// 如果必需字段未设置或参数无效，返回错误
    pub fn try_build(self) -> Result<QueryIncrementalData, MarketDataQueryError> {
        let symbol_id = self
            .symbol_id
            .ok_or_else(|| MarketDataQueryError::invalid_parameter("symbol_id", "required"))?;
        let from_sequence = self
            .from_sequence
            .ok_or_else(|| MarketDataQueryError::invalid_parameter("from_sequence", "required"))?;
        let to_sequence = self
            .to_sequence
            .ok_or_else(|| MarketDataQueryError::invalid_parameter("to_sequence", "required"))?;

        QueryIncrementalData::try_new(symbol_id, from_sequence, to_sequence)
    }
}

/// 增量数据查询结果
///
/// 包含查询到的增量事件列表和分页信息
#[derive(Debug, Clone)]
pub struct IncrementalDataResult {
    /// 交易对ID
    pub symbol_id: SymbolId,
    /// 增量事件列表
    pub deltas: Vec<MarketDataDelta>,
    /// 起始序列号
    pub from_sequence: SequenceNumber,
    /// 结束序列号
    pub to_sequence: SequenceNumber,
    /// 是否有更多数据
    pub has_more: bool,
}

impl IncrementalDataResult {
    /// 创建增量数据查询结果
    pub fn new(
        symbol_id: SymbolId,
        deltas: Vec<MarketDataDelta>,
        from_sequence: SequenceNumber,
        to_sequence: SequenceNumber,
        has_more: bool,
    ) -> Self {
        Self {
            symbol_id,
            deltas,
            from_sequence,
            to_sequence,
            has_more,
        }
    }

    /// 获取事件数量
    #[inline]
    pub fn len(&self) -> usize {
        self.deltas.len()
    }

    /// 判断是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.deltas.is_empty()
    }

    /// 获取查询范围
    #[inline]
    pub fn range(&self) -> u64 {
        self.to_sequence - self.from_sequence
    }

    /// 过滤订单簿变更事件
    pub fn filter_orderbook_changes(&self) -> Vec<&OrderBookDelta> {
        self.deltas
            .iter()
            .filter_map(|delta| match delta {
                MarketDataDelta::OrderBookChange(change) => Some(change),
                _ => None,
            })
            .collect()
    }

    /// 过滤成交事件
    pub fn filter_trades(&self) -> Vec<&TradeEvent> {
        self.deltas
            .iter()
            .filter_map(|delta| match delta {
                MarketDataDelta::Trade(trade) => Some(trade),
                _ => None,
            })
            .collect()
    }

    /// 过滤最优买卖价变更事件
    pub fn filter_bbo_changes(&self) -> Vec<&BboChangeEvent> {
        self.deltas
            .iter()
            .filter_map(|delta| match delta {
                MarketDataDelta::BboChange(bbo) => Some(bbo),
                _ => None,
            })
            .collect()
    }

    /// 统计各类事件数量
    pub fn count_by_type(&self) -> EventTypeCount {
        let mut count = EventTypeCount::default();
        for delta in &self.deltas {
            match delta {
                MarketDataDelta::OrderBookChange(_) => count.orderbook_changes += 1,
                MarketDataDelta::Trade(_) => count.trades += 1,
                MarketDataDelta::BboChange(_) => count.bbo_changes += 1,
            }
        }
        count
    }

    /// 获取第一个事件的序列号
    pub fn first_sequence(&self) -> Option<SequenceNumber> {
        self.deltas.first().map(|delta| match delta {
            MarketDataDelta::OrderBookChange(d) => d.sequence,
            MarketDataDelta::Trade(t) => t.sequence,
            MarketDataDelta::BboChange(b) => b.sequence,
        })
    }

    /// 获取最后一个事件的序列号
    pub fn last_sequence(&self) -> Option<SequenceNumber> {
        self.deltas.last().map(|delta| match delta {
            MarketDataDelta::OrderBookChange(d) => d.sequence,
            MarketDataDelta::Trade(t) => t.sequence,
            MarketDataDelta::BboChange(b) => b.sequence,
        })
    }
}

/// 事件类型统计
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EventTypeCount {
    /// 订单簿变更数量
    pub orderbook_changes: usize,
    /// 成交数量
    pub trades: usize,
    /// 最优买卖价变更数量
    pub bbo_changes: usize,
}

impl EventTypeCount {
    /// 获取总事件数
    #[inline]
    pub fn total(&self) -> usize {
        self.orderbook_changes + self.trades + self.bbo_changes
    }
}

// ============================================================================
// 增量数据仓储接口
// ============================================================================

/// 增量数据仓储接口
pub trait IncrementalDataRepo {
    /// 查询增量数据
    fn query_incremental_data(
        &self,
        symbol_id: SymbolId,
        from_sequence: u64,
        to_sequence: u64,
    ) -> Result<Vec<MarketDataDelta>, MarketDataQueryError>;

    /// 获取最新序列号
    fn get_latest_sequence(&self, symbol_id: SymbolId) -> Option<u64>;
}

// ============================================================================
// 查询处理器接口
// ============================================================================

pub trait MarketDataQueryProc {
    /// 处理 Level 1 查询
    fn handle_query_level1(
        &self,
        query: QueryLevel1,
    ) -> Result<Level1QueryResult, MarketDataQueryError>;

    /// 处理 Level 1 批量查询
    fn handle_query_level1_batch(&self, query: QueryLevel1Batch) -> Level1BatchQueryResult;

    /// 处理 Level 2 查询
    fn handle_query_level2(&self, query: QueryLevel2) -> Level2QueryResult;

    /// 处理 Level 3 查询
    fn handle_query_level3(&self, query: QueryLevel3) -> Level3QueryResult;

    /// 处理增量数据查询
    fn handle_query_incremental_data(
        &self,
        query: QueryIncrementalData,
    ) -> Result<IncrementalDataResult, MarketDataQueryError>;
}


// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_level1_creation() {
        let query = QueryLevel1::new(1, 12345);
        assert_eq!(query.symbol_id, 1);
        assert_eq!(query.sequence, 12345);
    }

    #[test]
    fn test_query_incremental_data_creation() {
        let query = QueryIncrementalData::new(1, 100, 200);
        assert_eq!(query.symbol_id, 1);
        assert_eq!(query.from_sequence, 100);
        assert_eq!(query.to_sequence, 200);
    }

    #[test]
    fn test_orderbook_change_type() {
        assert_eq!(OrderBookChangeType::Add, OrderBookChangeType::Add);
        assert_ne!(OrderBookChangeType::Add, OrderBookChangeType::Modify);
        assert_ne!(OrderBookChangeType::Modify, OrderBookChangeType::Delete);
    }

    #[test]
    fn test_orderbook_delta_creation() {
        let delta = OrderBookDelta {
            symbol_id: 1,
            timestamp: 1234567890,
            sequence: 100,
            change_type: OrderBookChangeType::Add,
            order_id: 12345,
            side: Side::Buy,
            price: 50000,
            quantity: 100,
            trader_id: Some(TraderId::new([1, 2, 3, 4, 5, 6, 7, 8])),
        };

        assert_eq!(delta.symbol_id, 1);
        assert_eq!(delta.change_type, OrderBookChangeType::Add);
        assert_eq!(delta.order_id, 12345);
    }

    #[test]
    fn test_trade_event_creation() {
        let trade = TradeEvent {
            symbol_id: 1,
            timestamp: 1234567890,
            sequence: 100,
            trade_id: 5000,
            buyer_order_id: 12345,
            seller_order_id: 67890,
            price: 50000,
            quantity: 10,
            aggressor_side: Side::Buy,
        };

        assert_eq!(trade.trade_id, 5000);
        assert_eq!(trade.buyer_order_id, 12345);
        assert_eq!(trade.seller_order_id, 67890);
    }

    #[test]
    fn test_bbo_change_event_creation() {
        let bbo = BboChangeEvent {
            symbol_id: 1,
            timestamp: 1234567890,
            sequence: 100,
            best_bid: Some(49999),
            best_bid_quantity: 100,
            best_ask: Some(50001),
            best_ask_quantity: 200,
        };

        assert_eq!(bbo.symbol_id, 1);
        assert_eq!(bbo.best_bid, Some(49999));
        assert_eq!(bbo.best_ask, Some(50001));
    }

    #[test]
    fn test_market_data_delta_variants() {
        let orderbook_delta = OrderBookDelta {
            symbol_id: 1,
            timestamp: 1234567890,
            sequence: 100,
            change_type: OrderBookChangeType::Add,
            order_id: 12345,
            side: Side::Buy,
            price: 50000,
            quantity: 100,
            trader_id: None,
        };

        let delta = MarketDataDelta::OrderBookChange(orderbook_delta);
        match delta {
            MarketDataDelta::OrderBookChange(d) => {
                assert_eq!(d.order_id, 12345);
            }
            _ => panic!("Expected OrderBookChange variant"),
        }
    }
}
