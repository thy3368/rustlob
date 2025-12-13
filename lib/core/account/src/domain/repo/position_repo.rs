//! 持仓仓储接口定义
//!
//! 遵循 Clean Architecture：Repository 只负责纯粹的 CRUD 操作
//! 业务逻辑在 Service 层实现

/// 持仓信息（泛型版本,由外部定义具体类型）
///
/// # 说明
/// 持仓信息的具体结构由使用方定义，这里只要求必须有以下特征：
/// - Clone: 支持克隆
/// - Send + Sync: 线程安全
pub trait Position: Clone + Send + Sync {
    /// 获取持仓的唯一标识（如交易对 Symbol）
    type Key: Clone + Eq + std::hash::Hash + Send + Sync;

    /// 获取持仓的键
    fn key(&self) -> Self::Key;
}

/// 持仓仓储接口（泛型版本）
///
/// # 类型参数
/// - `P`: 持仓类型，必须实现 Position trait
///
/// # 职责
/// - 提供持仓的持久化操作（纯 CRUD）
/// - 支持按键（如 Symbol）查询
pub trait PositionRepo<P: Position>: Send + Sync {
    /// 获取持仓
    ///
    /// # 参数
    /// - `key`: 持仓键（如交易对 Symbol）
    fn get(&self, key: P::Key) -> Option<&P>;

    /// 获取持仓（可变引用）
    fn get_mut(&mut self, key: P::Key) -> Option<&mut P>;

    /// 保存或更新持仓
    ///
    /// # 参数
    /// - `position`: 持仓信息
    fn save(&mut self, position: P);

    /// 删除持仓
    ///
    /// # 参数
    /// - `key`: 持仓键
    ///
    /// # 返回
    /// - `true`: 删除成功
    /// - `false`: 持仓不存在
    fn remove(&mut self, key: P::Key) -> bool;

    /// 获取所有持仓
    fn get_all(&self) -> Vec<P>;

    /// 检查持仓是否存在
    fn exists(&self, key: P::Key) -> bool {
        self.get(key).is_some()
    }
}
