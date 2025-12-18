use diff::{Entity, ChangeLogEntry};

/// 订单仓储接口
///
/// 定义订单数据的存储和检索操作
/// 仅暴露业务层需要的操作，内部实现细节（如链表遍历、价格点管理）由具体实现封装
///
/// # 泛型参数
/// - `O`: 实现了 Entity trait 的订单类型
///
/// # 核心功能
/// - 快照支持：通过 RepoSnapshot trait 实现
/// - 事件回放：通过 EventReplay trait 实现
/// - 批量操作：支持高效的批量操作
pub trait DBRepo<O: Entity> {
    /// 从事件日志中回放单个事件
    ///
    /// 此方法根据变更日志条目的类型（Created/Updated/Deleted）
    /// 对仓储进行相应的修改操作
    ///
    /// # 参数
    /// - `event`: 变更日志条目，包含：
    ///   - `entity_id`: 实体唯一标识符
    ///   - `change_type`: 变更类型（Created/Updated/Deleted）
    ///   - `timestamp`: 变更发生的时间戳（纳秒）
    ///   - `sequence`: 变更的序列号
    ///
    /// # 返回
    /// - `Ok(())`: 事件回放成功
    /// - `Err(RepoError)`: 回放失败
    ///   - `OrderAlreadyExists`: 创建事件中订单已存在
    ///   - `OrderNotFound`: 更新/删除的订单不存在
    ///   - `PriceOutOfRange`: 订单价格超出范围
    ///   - `CapacityExceeded`: 仓储容量已满
    ///   - `DeserializationFailed`: 无法反序列化订单对象
    ///   - `SymbolMismatch`: 订单交易对不匹配
    ///
    /// # 行为说明
    /// - **Created 事件**: 创建新订单
    ///   - 检查订单是否已存在（幂等）
    ///   - 从 Created 事件的字段信息重构订单对象
    ///   - 将订单添加到仓储
    ///
    /// - **Updated 事件**: 更新现有订单
    ///   - 查找订单ID对应的订单对象
    ///   - 应用字段变更（通过 Entity::replay 方法）
    ///   - 更新内部索引和缓存（如价格级别、最佳价格等）
    ///
    /// - **Deleted 事件**: 删除订单
    ///   - 查找订单ID
    ///   - 删除订单并更新索引
    ///   - 必要时更新缓存的最佳价格信息
    ///
    /// # 性能特性
    /// - 时间复杂度：O(1) for add/delete, O(k) for update（k=字段数）
    /// - 支持单订单重放，也可用于批量回放
    /// - 建议在批量回放时配合 replay_events 使用以获得最佳性能
    ///
    /// # 示例
    /// ```ignore
    /// use diff::ChangeLogEntry;
    ///
    /// let event = ChangeLogEntry::new(
    ///     "order_123",
    ///     "Order",
    ///     diff::ChangeType::Created { fields },
    ///     timestamp,
    ///     sequence
    /// );
    ///
    /// lob.replay_event(&event)?;
    /// ```
    fn replay_event(&mut self, event: &ChangeLogEntry) -> Result<(), RepoError>;

    /// 批量回放多个事件
    ///
    /// 此方法按顺序应用多个事件，适用于从事件日志恢复仓储状态
    ///
    /// # 参数
    /// - `events`: 事件列表，应按时间或序列号排序
    ///
    /// # 返回
    /// - `Ok(())`: 所有事件回放成功
    /// - `Err(RepoError)`: 任何事件回放失败时返回错误
    ///   第一个失败的事件会导致整个操作失败
    ///
    /// # 行为说明
    /// - 按顺序逐个回放事件
    /// - 如果某个事件失败，不再处理后续事件
    /// - 不自动回滚已经应用的事件
    ///
    /// # 性能特性
    /// - 默认实现：逐个调用 replay_event
    /// - 某些实现可能会进行批量优化（如排序后批处理）
    /// - 推荐事件数 < 1000 使用此方法
    ///
    /// # 示例
    /// ```ignore
    /// let events = vec![
    ///     ChangeLogEntry::new(...),  // Create order 1
    ///     ChangeLogEntry::new(...),  // Update order 1
    ///     ChangeLogEntry::new(...),  // Create order 2
    /// ];
    ///
    /// lob.replay_events(&events)?;
    /// ```
    fn replay_events(&mut self, events: &[ChangeLogEntry]) -> Result<(), RepoError> {
        for event in events {
            self.replay_event(event)?;
        }
        Ok(())
    }

    /// 从指定序列号开始回放事件
    ///
    /// 此方法用于快照+增量日志的混合恢复场景
    /// 在快照恢复后，仅回放快照之后的事件
    ///
    /// # 参数
    /// - `events`: 事件列表
    /// - `from_sequence`: 起始序列号（包含此序列号）
    ///
    /// # 返回
    /// - `Ok(())`: 回放成功
    /// - `Err(RepoError)`: 回放失败
    ///
    /// # 行为说明
    /// - 只处理 `sequence >= from_sequence` 的事件
    /// - 其他事件被跳过
    /// - 事件应按序列号排序
    ///
    /// # 性能特性
    /// - 时间复杂度：O(n)，其中 n 为满足条件的事件数
    /// - 相比重新应用所有事件，节约大量重复操作
    ///
    /// # 使用场景
    /// ```ignore
    /// // 恢复流程：
    /// // 1. 加载快照
    /// let snapshot = load_snapshot()?;
    /// let mut lob = LocalLob::new(...);
    /// lob.restore_from_snapshot(&snapshot)?;  // sequence = 5000
    ///
    /// // 2. 加载变更日志
    /// let events = load_changelog()?;  // 包含所有历史事件
    ///
    /// // 3. 仅应用快照之后的事件
    /// lob.replay_from_sequence(&events, snapshot.sequence + 1)?;
    /// // 只会应用 sequence >= 5001 的事件
    /// ```
    fn replay_from_sequence(
        &mut self,
        events: &[ChangeLogEntry],
        from_sequence: u64,
    ) -> Result<(), RepoError> {
        for event in events {
            if event.sequence >= from_sequence {
                self.replay_event(event)?;
            }
        }
        Ok(())
    }
}

/// 仓储错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoError {
    /// 容量已满
    CapacityExceeded,
    /// 订单已存在
    OrderAlreadyExists,
    /// 订单未找到
    OrderNotFound,
    /// 价格超出范围
    PriceOutOfRange,
    /// 不支持快照功能
    SnapshotNotSupported,
    /// 反序列化失败
    DeserializationFailed(String),
    /// 交易对不匹配
    SymbolMismatch {
        expected: String,
        actual: String,
    },
    /// 序列化失败
    SerializationFailed(String),
}

impl std::fmt::Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::CapacityExceeded => write!(f, "订单容量已满"),
            RepoError::OrderAlreadyExists => write!(f, "订单已存在"),
            RepoError::OrderNotFound => write!(f, "订单未找到"),
            RepoError::PriceOutOfRange => write!(f, "价格超出范围"),
            RepoError::SnapshotNotSupported => write!(f, "不支持快照功能"),
            RepoError::DeserializationFailed(msg) => write!(f, "反序列化失败: {}", msg),
            RepoError::SymbolMismatch { expected, actual } => {
                write!(f, "交易对不匹配: 期望 {}, 实际 {}", expected, actual)
            }
            RepoError::SerializationFailed(msg) => write!(f, "序列化失败: {}", msg),
        }
    }
}

impl std::error::Error for RepoError {}

