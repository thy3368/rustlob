// // === 事件溯源 ===
//
// /// 重放事件列表，将事件应用到仓储状态
// ///
// /// # 参数
// /// - `events`: 事件列表（按event_id顺序）
// ///
// /// # 返回
// /// - `Ok(())`: 成功应用所有事件
// /// - `Err(RepositoryError)`: 应用事件失败
// fn replay(&mut self, events: Vec<EntityEvent>) -> Result<(), RepoError>;

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
    SymbolMismatch { expected: String, actual: String },
    /// 序列化失败
    SerializationFailed(String),
}

/// 仓储快照能力 Trait
///
/// 所有支持快照的仓储都可以实现此 trait
/// 快照功能用于保存和恢复仓储的完整状态
///
/// # 使用场景
/// - 事件溯源架构
/// - 持久化存储
/// - 灾难恢复
/// - 分布式系统同步
/// - 时间旅行调试
///
/// # 示例
/// ```ignore
/// use lob_repo::core::symbol_lob_repo::{RepoSnapshot, LobSnapshot};
///
/// // 实现快照能力
/// impl RepoSnapshot for MyLob {
///     type Snapshot = LobSnapshot;
///
///     fn create_snapshot(&self, timestamp: u64, sequence: u64) -> Result<Self::Snapshot, RepoError> {
///         // 序列化仓储状态
///         let data = self.serialize_state()?;
///         Ok(LobSnapshot::new(
///             self.symbol,
///             timestamp,
///             sequence,
///             data,
///             self.best_bid(),
///             self.best_ask(),
///             self.last_price(),
///         ))
///     }
///
///     fn restore_from_snapshot(&mut self, snapshot: &Self::Snapshot) -> Result<(), RepoError> {
///         // 反序列化并恢复状态
///         self.deserialize_state(&snapshot.data)?;
///         Ok(())
///     }
/// }
/// ```
pub trait RepoSnapshot {
    /// 快照类型
    ///
    /// 不同的仓储可以使用不同的快照类型
    /// 例如：LobSnapshot, EntitySnapshot 等
    type Snapshot: Clone;

    /// 创建仓储快照
    ///
    /// # 参数
    /// - `timestamp`: 快照时间戳（纳秒）
    /// - `sequence`: 快照序列号（用于排序和版本控制）
    ///
    /// # 返回
    /// - `Ok(Snapshot)`: 成功创建快照
    /// - `Err(RepoError)`: 快照创建失败
    ///
    /// # 说明
    /// 快照应包含仓储在指定时间点的完整状态，
    /// 包括所有必要的数据以便完全恢复仓储状态
    fn create_snapshot(&self, timestamp: u64, sequence: u64) -> Result<Self::Snapshot, RepoError>;

    /// 从快照恢复仓储状态
    ///
    /// # 参数
    /// - `snapshot`: 快照数据
    ///
    /// # 返回
    /// - `Ok(())`: 成功恢复状态
    /// - `Err(RepoError)`: 恢复失败
    ///
    /// # 说明
    /// 此方法会清空当前状态并从快照中完全恢复
    /// 实现时应注意验证快照的有效性
    fn restore_from_snapshot(&mut self, snapshot: &Self::Snapshot) -> Result<(), RepoError>;
}

/// 事件回放能力 Trait
///
/// 所有支持事件溯源的仓储都可以实现此 trait
/// 用于从事件流重建仓储状态
///
/// # 使用场景
/// - 事件溯源（Event Sourcing）架构
/// - CQRS（Command Query Responsibility Segregation）模式
/// - 审计日志重放
/// - 状态重建
/// - 时间旅行功能
///
/// # 示例
/// ```ignore
/// use lob_repo::core::symbol_lob_repo::EventReplay;
/// use diff::ChangeLogEntry;
///
/// impl EventReplay for MyLob {
///     type Event = ChangeLogEntry;
///
///     fn replay_event(&mut self, event: &Self::Event) -> Result<(), RepoError> {
///         match &event.change_type {
///             ChangeType::Created => {
///                 // 处理创建事件
///             }
///             ChangeType::Updated { changed_fields } => {
///                 // 处理更新事件
///             }
///             ChangeType::Deleted => {
///                 // 处理删除事件
///             }
///         }
///         Ok(())
///     }
///
///     fn replay_events(&mut self, events: &[Self::Event]) -> Result<(), RepoError> {
///         for event in events {
///             self.replay_event(event)?;
///         }
///         Ok(())
///     }
/// }
/// ```
pub trait EventReplay {
    /// 事件类型
    ///
    /// 不同的仓储可以使用不同的事件类型
    /// 例如：ChangeLogEntry, OrderEvent, TradeEvent 等
    type Event;

    /// 回放单个事件
    ///
    /// # 参数
    /// - `event`: 事件数据
    ///
    /// # 返回
    /// - `Ok(())`: 成功应用事件
    /// - `Err(RepoError)`: 应用事件失败
    ///
    /// # 说明
    /// 将事件应用到仓储状态，更新相应的数据
    fn replay_event(&mut self, event: &Self::Event) -> Result<(), RepoError>;

    /// 批量回放事件列表
    ///
    /// # 参数
    /// - `events`: 事件列表（应按时间或序列号排序）
    ///
    /// # 返回
    /// - `Ok(())`: 成功应用所有事件
    /// - `Err(RepoError)`: 应用事件失败
    ///
    /// # 说明
    /// 默认实现按顺序逐个回放事件
    /// 具体实现可以覆盖此方法进行批量优化
    fn replay_events(&mut self, events: &[Self::Event]) -> Result<(), RepoError> {
        for event in events {
            self.replay_event(event)?;
        }
        Ok(())
    }

    /// 从指定序列号开始回放事件
    ///
    /// # 参数
    /// - `events`: 事件列表
    /// - `from_sequence`: 起始序列号（包含）
    ///
    /// # 返回
    /// - `Ok(())`: 成功应用事件
    /// - `Err(RepoError)`: 应用事件失败
    ///
    /// # 说明
    /// 只回放序列号 >= from_sequence 的事件
    /// 用于从快照后的增量事件开始回放
    fn replay_from_sequence(
        &mut self,
        events: &[Self::Event],
        from_sequence: u64,
    ) -> Result<(), RepoError>
    where
        Self::Event: HasSequence,
    {
        for event in events {
            if event.sequence() >= from_sequence {
                self.replay_event(event)?;
            }
        }
        Ok(())
    }
}

/// 事件序列号访问 Trait
///
/// 用于获取事件的序列号，支持按序列号过滤
pub trait HasSequence {
    /// 获取事件的序列号
    fn sequence(&self) -> u64;
}
