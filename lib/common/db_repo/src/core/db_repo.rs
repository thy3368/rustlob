use std::fmt;

use diff::{ChangeLogEntry, Entity};

/// 分页参数
///
/// 用于指定查询的分页信息
/// 遵循 0-based 索引约定，便于数据库实现
///
/// # 性能特性
/// - 分页查询应使用 LIMIT/OFFSET 在数据库层实现
/// - 避免在应用层加载全部数据后再分页
/// - 推荐配合数据库分页索引使用
///
/// # 示例
/// ```ignore
/// let page_req = PageRequest::new(0, 20);  // 第一页，每页20条
/// let results = repo.find_all_by_condition_paginated(condition, page_req)?;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    /// 分页号（0-based，第一页为 0）
    pub page: u64,
    /// 每页记录数
    pub page_size: u64,
}

impl PageRequest {
    /// 创建新的分页请求
    ///
    /// # 参数
    /// - `page`: 页号（0-based）
    /// - `page_size`: 每页记录数
    ///
    /// # 性能考虑
    /// - `page_size` 过小（< 10）会增加数据库访问次数
    /// - `page_size` 过大（> 10000）会增加单次查询延迟
    /// - 建议范围：10-1000
    ///
    /// # Panics
    /// - 如果 `page_size` 为 0，会 panic
    pub fn new(page: u64, page_size: u64) -> Self {
        assert!(page_size > 0, "page_size must be greater than 0");
        Self { page, page_size }
    }

    /// 获取OFFSET值（数据库LIMIT/OFFSET中的OFFSET）
    ///
    /// # 示例
    /// ```ignore
    /// let page_req = PageRequest::new(2, 20);
    /// assert_eq!(page_req.offset(), 40);  // 跳过前 40 条记录
    /// ```
    #[inline]
    pub fn offset(&self) -> u64 {
        self.page * self.page_size
    }

    /// 获取LIMIT值（数据库LIMIT/OFFSET中的LIMIT）
    #[inline]
    pub fn limit(&self) -> u64 {
        self.page_size
    }

    /// 获取下一页的分页请求
    #[inline]
    pub fn next_page(&self) -> Self {
        Self { page: self.page + 1, page_size: self.page_size }
    }

    /// 获取上一页的分页请求
    #[inline]
    pub fn prev_page(&self) -> Option<Self> {
        if self.page > 0 {
            Some(Self { page: self.page - 1, page_size: self.page_size })
        } else {
            None
        }
    }
}

impl fmt::Display for PageRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "page={}, page_size={}", self.page, self.page_size)
    }
}

/// 分页结果
///
/// 包含分页查询的结果和元数据
///
/// # 字段说明
/// - `content`: 当前页的数据
/// - `total_elements`: 符合条件的总记录数
/// - `total_pages`: 总分页数
/// - `page`: 当前页号
/// - `page_size`: 每页记录数
/// - `has_next`: 是否还有下一页
/// - `has_previous`: 是否有上一页
///
/// # 性能特性
/// - 获取 `total_elements` 可能需要额外的 COUNT 查询
/// - 推荐将总数缓存一段时间（如非实时数据）
/// - 可选实现：某些场景下可以不获取总数，仅判断是否有下一页
///
/// # 示例
/// ```ignore
/// let results = repo.find_all_by_condition_paginated(condition, page_req)?;
/// println!("第 {} 页，共 {} 页", results.page + 1, results.total_pages);
/// ```
#[derive(Debug, Clone)]
pub struct PageResult<T> {
    /// 当前页的数据
    pub content: Vec<T>,
    /// 符合条件的总记录数
    pub total_elements: u64,
    /// 当前页号（0-based）
    pub page: u64,
    /// 每页记录数
    pub page_size: u64,
}

impl<T> PageResult<T> {
    /// 创建新的分页结果
    pub fn new(content: Vec<T>, total_elements: u64, page: u64, page_size: u64) -> Self {
        Self { content, total_elements, page, page_size }
    }

    /// 获取总分页数
    #[inline]
    pub fn total_pages(&self) -> u64 {
        (self.total_elements + self.page_size - 1) / self.page_size
    }

    /// 当前页的元素数
    #[inline]
    pub fn page_elements(&self) -> u64 {
        self.content.len() as u64
    }

    /// 是否有下一页
    #[inline]
    pub fn has_next(&self) -> bool {
        (self.page + 1) < self.total_pages()
    }

    /// 是否有上一页
    #[inline]
    pub fn has_previous(&self) -> bool {
        self.page > 0
    }

    /// 是否为最后一页
    #[inline]
    pub fn is_last_page(&self) -> bool {
        !self.has_next()
    }

    /// 是否为第一页
    #[inline]
    pub fn is_first_page(&self) -> bool {
        self.page == 0
    }

    /// 转换分页结果的数据类型
    pub fn map<U, F>(self, f: F) -> PageResult<U>
    where
        F: FnMut(T) -> U,
    {
        PageResult {
            content: self.content.into_iter().map(f).collect(),
            total_elements: self.total_elements,
            page: self.page,
            page_size: self.page_size,
        }
    }
}

impl<T: fmt::Display> fmt::Display for PageResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PageResult {{ page: {}, page_size: {}, total: {}, elements: {} }}",
            self.page,
            self.page_size,
            self.total_elements,
            self.page_elements()
        )
    }
}

/// 订单仓储接口
///
/// 定义订单数据的存储和检索操作
/// 仅暴露业务层需要的操作，内部实现细节（如链表遍历、价格点管理）由具体实现封装
///
/// # 关联类型
/// - `E`: 实现了 Entity trait 的订单类型
///
/// # 核心功能
/// - 快照支持：通过 RepoSnapshot trait 实现
/// - 事件回放：通过 EventReplay trait 实现
/// - 批量操作：支持高效的批量操作
pub trait CmdRepo: Send + Sync {
    /// 仓储中存储的实体类型
    type E: Entity;
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
    /// lob_repo.replay_event(&event)?;
    /// ```
    fn replay_event(&self, event: &ChangeLogEntry) -> Result<(), RepoError>;

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
    /// lob_repo.replay_events(&events)?;
    /// ```
    fn replay_events(&self, events: &[ChangeLogEntry]) -> Result<(), RepoError> {
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
    /// let mut lob_repo = LocalLob::new(...);
    /// lob_repo.restore_from_snapshot(&snapshot)?;  // sequence = 5000
    ///
    /// // 2. 加载变更日志
    /// let events = load_changelog()?;  // 包含所有历史事件
    ///
    /// // 3. 仅应用快照之后的事件
    /// lob_repo.replay_from_sequence(&events, snapshot.sequence + 1)?;
    /// // 只会应用 sequence >= 5001 的事件
    /// ```
    fn replay_from_sequence(
        &self,
        events: &[ChangeLogEntry],
        from_sequence: u64,
    ) -> Result<(), RepoError> {
        for event in events {
            if event.sequence() >= &from_sequence {
                self.replay_event(event)?;
            }
        }
        Ok(())
    }
}

/// 仓储查询接口
///
/// 定义对实体的查询操作，支持按不同条件检索数据
/// 遵循 Clean Architecture 原则，依赖倒置设计，允许灵活的实现策略
///
/// # 关联类型
/// - `E`: 实现了 Entity trait 的实体类型
///
/// # 核心功能
/// - 按序列号查询：查询特定序列号的实体
/// - 条件查询单条：通过自定义条件查询单个实体
/// - 条件查询全部：通过自定义条件查询所有匹配实体
///
/// # 性能特性
/// - 查询时间复杂度：O(1) for indexed lookups, O(n) for scans
/// - 支持内存查询和数据库查询的多种实现
/// - 推荐使用索引加速高频查询
///
/// # 设计原则
/// - 遵循领域驱动设计（DDD）：查询方法返回领域对象
/// - 接口隐藏实现细节：调用方无需关心数据存储位置
/// - 可测试性优先：支持 mock 实现用于单元测试
/// - 单一职责：分离返回单条和多条数据的方法
pub trait QueryRepo: Send + Sync {
    /// 仓储中存储的实体类型
    type E: Entity;

    /// 按序列号查询单个实体
    ///
    /// 用于查询特定序列号的实体，常用于事件重放场景
    /// 在快照+增量日志恢复时，用于验证特定序列号的实体状态
    ///
    /// # 参数
    /// - `sequence`: 实体的序列号
    ///
    /// # 返回
    /// - `Ok(Some(entity))`: 找到指定序列号的实体
    /// - `Ok(None)`: 不存在该序列号的实体
    /// - `Err(RepoError)`: 查询失败
    ///   - `OrderNotFound`: 实体不存在
    ///   - `DeserializationFailed`: 反序列化失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(1) 如果使用索引，O(n) 如果线性扫描
    /// - 推荐使用 B-tree 或 Hash 索引加速查询
    /// - 高频查询场景建议结合缓存使用
    ///
    /// # 示例
    /// ```ignore
    /// let repo: Arc<dyn DBQueryRepo<E=Order>> = ...;
    ///
    /// match repo.find_by_sequence(100)? {
    ///     Some(order) => println!("找到订单: {:?}", order),
    ///     None => println!("序列号 100 的订单不存在"),
    /// }
    /// ```
    fn find_by_sequence(&self, sequence: u64) -> Result<Option<Self::E>, RepoError>;

    /// 按条件查询单个实体
    ///
    /// 通过自定义条件查询**单个**匹配的实体
    /// 如果存在多个匹配结果，返回其中之一（具体实现定义优先级）
    ///
    /// # 参数
    /// - `condition`: 查询条件对象
    ///   - 具体实现应定义如何从此对象提取查询条件
    ///   - 推荐使用专门的查询对象而不是实体本身
    ///
    /// # 返回
    /// - `Ok(Some(entity))`: 找到匹配条件的实体（多个结果时返回第一个）
    /// - `Ok(None)`: 不存在匹配条件的实体
    /// - `Err(RepoError)`: 查询失败
    ///   - `OrderNotFound`: 实体不存在
    ///   - `DeserializationFailed`: 反序列化失败
    ///
    /// # 性能特性
    /// - 时间复杂度：取决于具体实现和索引策略
    /// - 应为常见查询建立索引
    /// - 不推荐在热路径中使用复杂条件查询
    /// - 相比 find_all_by_condition，避免加载所有匹配结果
    ///
    /// # 设计建议
    /// 实现此方法时，应考虑：
    /// 1. 使用专门的查询对象而非实体对象作为条件
    /// 2. 建立必要的索引加速查询
    /// 3. 缓存热查询结果
    /// 4. 当确定只需要单条结果时，提前终止查询
    ///
    /// # 示例
    /// ```ignore
    /// // 推荐：使用专门的查询对象
    /// struct OrderQuery {
    ///     order_id: OrderId,
    /// }
    ///
    /// let query = OrderQuery { order_id: "123".parse()? };
    /// let order = repo.find_one_by_condition(query)?;
    /// ```
    fn find_one_by_condition(&self, condition: Self::E) -> Result<Option<Self::E>, RepoError>;

    /// 按条件查询所有匹配的实体
    ///
    /// 通过自定义条件查询**所有**匹配的实体
    /// 用于批量操作、报表等场景
    ///
    /// # 参数
    /// - `condition`: 查询条件对象
    ///   - 具体实现应定义如何从此对象提取查询条件
    ///   - 推荐使用专门的查询对象而不是实体本身
    ///
    /// # 返回
    /// - `Ok(entities)`: 所有匹配条件的实体列表（可能为空）
    /// - `Err(RepoError)`: 查询失败
    ///   - `DeserializationFailed`: 反序列化失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(n)，其中 n 为匹配的实体数
    /// - 对于大结果集，应考虑分页处理
    /// - 推荐在后台任务或非热路径中使用
    /// - 相比 find_one_by_condition，需要加载所有匹配结果
    ///
    /// # 设计建议
    /// 1. 对于超大结果集，应提供分页支持
    /// 2. 考虑在数据库层面使用流式查询避免全量加载
    /// 3. 缓存热条件的查询结果（需要注意缓存失效）
    /// 4. 限制返回结果数量，防止内存溢出
    ///
    /// # 示例
    /// ```ignore
    /// struct SymbolQuery {
    ///     symbol: Symbol,
    /// }
    ///
    /// let query = SymbolQuery { symbol: "BTCUSDT".parse()? };
    /// let orders = repo.find_all_by_condition(query)?;
    /// println!("找到 {} 个订单", orders.len());
    /// ```
    fn find_all_by_condition(&self, condition: Self::E) -> Result<Vec<Self::E>, RepoError>;

    /// 按实体ID查询单个实体
    ///
    /// 常见的查询场景，按实体的主键（ID）查询
    /// 此方法为 find_one_by_condition 的语义补充
    ///
    /// # 参数
    /// - `entity_id`: 实体的唯一标识符
    ///
    /// # 返回
    /// - `Ok(Some(entity))`: 找到指定ID的实体
    /// - `Ok(None)`: 指定ID的实体不存在
    /// - `Err(RepoError)`: 查询失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(1)（应使用主键索引）
    /// - 推荐为实体ID建立唯一索引
    ///
    /// # 示例
    /// ```ignore
    /// let repo: Arc<dyn DBQueryRepo<E=Order>> = ...;
    ///
    /// match repo.find_by_id("order_123")? {
    ///     Some(order) => println!("订单状态: {:?}", order),
    ///     None => println!("订单不存在"),
    /// }
    /// ```
    fn find_by_id(&self, entity_id: &str) -> Result<Option<Self::E>, RepoError> {
        // 默认实现：返回未实现
        // 具体实现应提供高性能的ID查询
        Err(RepoError::OrderNotFound)
    }

    /// 按范围查询多个实体
    ///
    /// 查询指定序列号范围内的多个实体
    /// 用于快照恢复、批量操作等场景
    ///
    /// # 参数
    /// - `from_sequence`: 起始序列号（包含）
    /// - `to_sequence`: 结束序列号（包含）
    ///
    /// # 返回
    /// - `Ok(entities)`: 序列号在指定范围内的实体列表
    /// - `Err(RepoError)`: 查询失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(m + log n)，其中 n 为总实体数，m 为结果数
    /// - 推荐使用范围索引加速查询
    /// - 对于大范围查询，考虑分页处理
    ///
    /// # 设计建议
    /// 1. 对于超大范围查询，应提供分页支持
    /// 2. 考虑在数据库层面使用流式查询避免全量加载
    /// 3. 缓存热范围的查询结果
    ///
    /// # 示例
    /// ```ignore
    /// let repo: Arc<dyn DBQueryRepo<E=Order>> = ...;
    ///
    /// let entities = repo.find_range_by_sequence(100, 200)?;
    /// println!("找到 {} 个实体", entities.len());
    /// ```
    fn find_range_by_sequence(
        &self,
        from_sequence: u64,
        to_sequence: u64,
    ) -> Result<Vec<Self::E>, RepoError> {
        // 默认实现：返回空向量
        // 具体实现应支持范围查询
        Ok(Vec::new())
    }

    /// 获取仓储中的实体总数
    ///
    /// 用于监控和统计目的
    ///
    /// # 返回
    /// - `Ok(count)`: 实体总数
    /// - `Err(RepoError)`: 查询失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(1) 如果维护计数器，O(n) 如果每次扫描
    /// - 推荐维护计数器避免每次扫描
    ///
    /// # 示例
    /// ```ignore
    /// let count = repo.count()?;
    /// println!("仓储中有 {} 个实体", count);
    /// ```
    fn count(&self) -> Result<u64, RepoError> {
        Ok(0)
    }

    /// 检查仓储是否包含指定ID的实体
    ///
    /// 轻量级查询，仅检查存在性而不加载完整实体
    /// 适用于前置验证和存在性检查
    ///
    /// # 参数
    /// - `entity_id`: 实体的唯一标识符
    ///
    /// # 返回
    /// - `Ok(true)`: 实体存在
    /// - `Ok(false)`: 实体不存在
    /// - `Err(RepoError)`: 查询失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(1)（应使用主键索引）
    /// - 相比 find_by_id，避免完整实体的反序列化开销
    /// - 推荐在热路径中使用此方法而非 find_by_id
    ///
    /// # 示例
    /// ```ignore
    /// if repo.exists("order_123")? {
    ///     println!("订单存在");
    /// } else {
    ///     println!("订单不存在，准备创建");
    /// }
    /// ```
    fn exists(&self, entity_id: &str) -> Result<bool, RepoError> {
        match self.find_by_id(entity_id) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(_) => Ok(false),
        }
    }

    /// 按条件分页查询单个实体
    ///
    /// 结合条件查询和分页功能
    /// 用于UI列表、API分页响应等场景
    ///
    /// # 参数
    /// - `condition`: 查询条件对象
    /// - `page_req`: 分页参数
    ///
    /// # 返回
    /// - `Ok(PageResult)`: 分页结果，包含数据和元数据
    /// - `Err(RepoError)`: 查询失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(m + log n)，其中 n 为总实体数，m 为当前页实体数
    /// - 通过 LIMIT/OFFSET 在数据库层实现分页
    /// - 总记录数可能需要额外 COUNT 查询，推荐缓存
    /// - 避免大偏移量查询（如第 1000 页），考虑基于游标的分页
    ///
    /// # 设计建议
    /// 1. 为查询条件和排序字段建立复合索引
    /// 2. 对于深分页，考虑使用游标分页替代 OFFSET 分页
    /// 3. 缓存总数信息，避免每次都执行 COUNT 查询
    /// 4. 设置合理的 page_size 限制（如最大 1000）
    ///
    /// # 示例
    /// ```ignore
    /// struct SymbolQuery {
    ///     symbol: Symbol,
    /// }
    ///
    /// let condition = SymbolQuery { symbol: "BTCUSDT".parse()? };
    /// let page_req = PageRequest::new(0, 20);  // 第一页，每页 20 条
    ///
    /// let result = repo.find_all_by_condition_paginated(condition, page_req)?;
    /// println!("总共 {} 条记录，第 {} 页", result.total_elements, result.page + 1);
    /// for order in result.content {
    ///     println!("订单: {:?}", order);
    /// }
    /// ```
    fn find_all_by_condition_paginated(
        &self,
        condition: Self::E,
        page_req: PageRequest,
    ) -> Result<PageResult<Self::E>, RepoError>;

    /// 按序列号范围分页查询实体
    ///
    /// 结合范围查询和分页功能
    /// 用于事件日志查询、快照恢复等场景
    ///
    /// # 参数
    /// - `from_sequence`: 起始序列号（包含）
    /// - `to_sequence`: 结束序列号（包含）
    /// - `page_req`: 分页参数
    ///
    /// # 返回
    /// - `Ok(PageResult)`: 分页结果
    /// - `Err(RepoError)`: 查询失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(m + log n)
    /// - 推荐使用序列号索引加速范围查询
    /// - 对于大范围查询，使用分页避免一次性加载过多数据
    ///
    /// # 示例
    /// ```ignore
    /// let page_req = PageRequest::new(0, 100);
    /// let result = repo.find_range_by_sequence_paginated(1000, 2000, page_req)?;
    /// println!("序列号 1000-2000 范围内有 {} 条记录", result.total_elements);
    /// ```
    fn find_range_by_sequence_paginated(
        &self,
        from_sequence: u64,
        to_sequence: u64,
        page_req: PageRequest,
    ) -> Result<PageResult<Self::E>, RepoError> {
        // 默认实现：返回空结果
        Ok(PageResult::new(Vec::new(), 0, page_req.page, page_req.page_size))
    }

    /// 基于游标的分页查询（可选优化）
    ///
    /// 对于需要深分页的场景，游标分页优于 OFFSET 分页
    /// 避免大偏移量导致的性能问题
    ///
    /// # 参数
    /// - `condition`: 查询条件
    /// - `cursor`: 游标值（通常是上一页最后一条记录的ID）
    /// - `limit`: 要获取的记录数
    /// - `forward`: true 表示向前翻页（获取之后的记录），false 表示向后翻页
    ///
    /// # 返回
    /// - `Ok((items, next_cursor))`: 返回数据和下一个游标
    /// - `Err(RepoError)`: 查询失败
    ///
    /// # 性能特性
    /// - 时间复杂度：O(limit + log n)，与偏移量无关
    /// - 推荐用于深分页场景（> 1000 页）
    /// - 游标通常是实体ID或序列号
    ///
    /// # 设计建议
    /// 1. 游标应为不可变的唯一标识符
    /// 2. 返回的游标用于下一次查询
    /// 3. 不支持跳页，只能顺序浏览
    ///
    /// # 示例
    /// ```ignore
    /// let condition = SymbolQuery { symbol: "BTCUSDT".parse()? };
    /// let (items, next_cursor) = repo.find_by_cursor(
    ///     condition,
    ///     None,  // 第一页无游标
    ///     20,
    ///     true
    /// )?;
    ///
    /// // 下一页
    /// let (next_items, next_next_cursor) = repo.find_by_cursor(
    ///     condition,
    ///     next_cursor,  // 使用前一次返回的游标
    ///     20,
    ///     true
    /// )?;
    /// ```
    fn find_by_cursor(
        &self,
        condition: Self::E,
        cursor: Option<String>,
        limit: u64,
        forward: bool,
    ) -> Result<(Vec<Self::E>, Option<String>), RepoError> {
        // 默认实现：返回空结果
        Ok((Vec::new(), None))
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
    SymbolMismatch { expected: String, actual: String },
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
