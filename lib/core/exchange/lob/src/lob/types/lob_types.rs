/// 订单簿类型和数据结构
///
/// 本模块提供高性能订单簿基础类型，
/// 针对低时延交易系统进行优化。
use std::fmt;


/// 简化字段变更创建的宏
#[macro_export]
macro_rules! fields {
    // Create操作：fields!(create: "name" => value, ...)
    (create: $($name:expr => $value:expr),* $(,)?) => {
        vec![$($crate::lob::types::lob_types::FieldChange::created($name, $value)),*]
    };

    // Update操作：fields!(update: ("name", old, new), ...)
    (update: $(($name:expr, $old:expr, $new:expr)),* $(,)?) => {
        vec![$($crate::lob::types::lob_types::FieldChange::updated($name, $old, $new)),*]
    };

    // Delete操作：fields!(delete: "name" => value, ...)
    (delete: $($name:expr => $value:expr),* $(,)?) => {
        vec![$($crate::lob::types::lob_types::FieldChange::deleted($name, $value)),*]
    };
}

/// 简化事件创建的宏（适用所有实体）
#[macro_export]
macro_rules! event {
    // 单记录事件
    ($entity:expr, $op:expr, $event_id:expr, $tx_id:expr, $entity_id:expr => {
        $($field_spec:tt)*
    }) => {
        $crate::lob::types::lob_types::EntityEvent::single(
            $event_id,
            $tx_id,
            $entity,
            $op,
            $entity_id,
            $crate::fields!($($field_spec)*),
        )
    };

    // 批量记录事件
    (batch $entity:expr, $op:expr, $event_id:expr, $tx_id:expr => [
        $($entity_id:expr => { $($field_spec:tt)* }),* $(,)?
    ]) => {
        $crate::lob::types::lob_types::EntityEvent::batch(
            $event_id,
            $tx_id,
            $entity,
            $op,
            vec![$($crate::lob::types::lob_types::RecordChange::new($entity_id, $crate::fields!($($field_spec)*)),)*],
        )
    };
}

//快照概念，担心entity_event过长
pub struct Snapshot {}

/// 交易员标识符（8字节固定长度）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(align(8))]
pub struct TraderId([u8; 8]);

impl TraderId {
    /// 从字节数组创建交易员ID
    #[inline]
    pub fn new(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }

    /// 从字符串创建交易员ID（最多8字节）
    #[inline]
    pub fn from_str(s: &str) -> Self {
        let mut bytes = [0u8; 8];
        let len = s.len().min(8);
        bytes[..len].copy_from_slice(&s.as_bytes()[..len]);
        Self(bytes)
    }

    /// 获取底层字节数组的引用
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }
}

impl fmt::Display for TraderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = std::str::from_utf8(&self.0)
            .unwrap_or("INVALID")
            .trim_end_matches('\0');
        write!(f, "{}", s)
    }
}

/// 订单方向（买入或卖出）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    Buy = b'B',  // 买入
    Sell = b'S', // 卖出
}

impl Side {
    /// 获取相反方向
    #[inline]
    pub fn opposite(&self) -> Side {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Buy => write!(f, "BUY"),
            Side::Sell => write!(f, "SELL"),
        }
    }
}

/// 订单标识符
pub type OrderId = u64;

/// 价格（以分为单位，避免浮点运算）
pub type Price = u32;

/// 数量/规模
pub type Quantity = u32;

/// 事件操作类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventOperation {
    Create = 1, // 创建
    Update = 2, // 更新
    Delete = 3, // 删除
}

/// 字段值类型（支持订单簿常用数据类型）
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    U32(u32),
    U64(u64),
    OptionUsize(Option<usize>),
    TraderId(TraderId),
    OrderId(OrderId),
    Quantity(Quantity),
    Side(Side),
    // OrderEntry(OrderEntry),
}

/// 字段变更记录 (field_name, old_value, new_value)
#[derive(Debug, Clone)]
pub struct FieldChange {
    pub field_name: &'static str,
    pub old_value: Option<FieldValue>,
    pub new_value: Option<FieldValue>,
}

impl FieldChange {
    /// 创建字段（Create操作）
    #[inline]
    pub fn created(field_name: &'static str, new_value: FieldValue) -> Self {
        Self {
            field_name,
            old_value: None,
            new_value: Some(new_value),
        }
    }

    /// 更新字段（Update操作）
    #[inline]
    pub fn updated(field_name: &'static str, old_value: FieldValue, new_value: FieldValue) -> Self {
        Self {
            field_name,
            old_value: Some(old_value),
            new_value: Some(new_value),
        }
    }

    /// 删除字段（Delete操作）
    #[inline]
    pub fn deleted(field_name: &'static str, old_value: FieldValue) -> Self {
        Self {
            field_name,
            old_value: Some(old_value),
            new_value: None,
        }
    }
}

/// 单条记录的变更
#[derive(Debug, Clone)]
pub struct RecordChange {
    /// 记录ID（如order_id, trade_id等）
    pub entity_id: u64,
    /// 该记录的字段变更列表
    pub field_changes: Vec<FieldChange>,
}

impl RecordChange {
    #[inline]
    pub fn new(entity_id: u64, field_changes: Vec<FieldChange>) -> Self {
        Self {
            entity_id,
            field_changes,
        }
    }
}

/// 实体事件
/// 遵循事件溯源模式，支持批量记录多条记录的字段变更
#[derive(Debug, Clone)]
pub struct EntityEvent {
    /// 事件序列号（全局唯一）
    pub event_id: u64,
    /// 事务ID（同一事务中的多个事件共享此ID，确保原子性）
    pub transaction_id: u64,
    /// 实体名称（如 "Order", "Trade", "PricePoint"）
    pub entity_name: &'static str,
    /// 操作类型
    pub operation: EventOperation,
    /// 批量记录变更（支持多条记录）
    pub changes: Vec<RecordChange>,
}

impl EntityEvent {
    /// 创建新的实体事件
    #[inline]
    pub fn new(
        event_id: u64,
        transaction_id: u64,
        entity_name: &'static str,
        operation: EventOperation,
        changes: Vec<RecordChange>,
    ) -> Self {
        Self {
            event_id,
            transaction_id,
            entity_name,
            operation,
            changes,
        }
    }

    /// 创建单条记录的事件（便捷方法）
    #[inline]
    pub fn single(
        event_id: u64,
        transaction_id: u64,
        entity_name: &'static str,
        operation: EventOperation,
        entity_id: u64,
        field_changes: Vec<FieldChange>,
    ) -> Self {
        Self {
            event_id,
            transaction_id,
            entity_name,
            operation,
            changes: vec![RecordChange::new(entity_id, field_changes)],
        }
    }

    /// 批量创建记录的事件（便捷方法）
    #[inline]
    pub fn batch(
        event_id: u64,
        transaction_id: u64,
        entity_name: &'static str,
        operation: EventOperation,
        changes: Vec<RecordChange>,
    ) -> Self {
        Self::new(event_id, transaction_id, entity_name, operation, changes)
    }
}
/// 交易执行记录
#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub buyer: TraderId,    // 买方
    pub seller: TraderId,   // 卖方
    pub price: Price,       // 成交价格
    pub quantity: Quantity, // 成交数量
}

impl Trade {
    /// 创建新的交易记录
    #[inline]
    pub fn new(buyer: TraderId, seller: TraderId, price: Price, quantity: Quantity) -> Self {
        Self {
            buyer,
            seller,
            price,
            quantity,
        }
    }
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TRADE: {} <- {} @ {} x {}",
            self.buyer, self.seller, self.price, self.quantity
        )
    }
}

/// 订单簿条目（64字节缓存行对齐以提升性能）
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(align(64))]
pub struct OrderEntry {
    pub order_id: OrderId,           // 订单ID
    pub trader: TraderId,            // 交易员ID
    pub total_quantity: Quantity,    // 总数量
    pub unfilled_quantity: Quantity, // 未成交数量
    pub next_idx: Option<usize>,     // 链表中下一个订单的索引
}

impl OrderEntry {
    /// 创建新的订单条目
    #[inline]
    pub fn new(order_id: OrderId, trader: TraderId, quantity: Quantity) -> Self {
        Self {
            order_id,
            trader,
            total_quantity: quantity,
            unfilled_quantity: quantity,
            next_idx: None,
        }
    }

    /// 检查订单是否仍然有效（数量>0）
    #[inline]
    pub fn is_active(&self) -> bool {
        self.unfilled_quantity > 0
    }

    /// 取消订单（通过将数量置零，单次内存写入，速度快）
    #[inline]
    pub fn cancel(&mut self) {
        self.unfilled_quantity = 0;
    }
}

/// 订单簿中的价格点（链表头）
#[derive(Debug, Clone, Copy)]
pub struct PricePoint {
    pub first_order_idx: Option<usize>, // 该价格的第一个订单索引
    pub last_order_idx: Option<usize>,  // 该价格的最后一个订单索引
}

impl Default for PricePoint {
    fn default() -> Self {
        Self {
            first_order_idx: None,
            last_order_idx: None,
        }
    }
}

impl PricePoint {
    /// 检查该价格是否没有订单
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.first_order_idx.is_none()
    }

    /// 在链表尾部添加订单
    #[inline]
    pub fn push_back(&mut self, idx: usize) {
        match self.last_order_idx {
            None => {
                // 空链表
                self.first_order_idx = Some(idx);
                self.last_order_idx = Some(idx);
            }
            Some(_last_idx) => {
                // 追加到末尾（调用方需要更新条目的next_idx）
                self.last_order_idx = Some(idx);
            }
        }
    }
}
