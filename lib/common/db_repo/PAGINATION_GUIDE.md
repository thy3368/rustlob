# DBQueryRepo 分页功能指南

## 概述

`DBQueryRepo` trait 已优化并添加了完整的分页支持，遵循 Clean Architecture 和低延迟设计原则。

## 核心数据结构

### PageRequest - 分页请求

```rust
pub struct PageRequest {
    pub page: u64,          // 页号（0-based，第一页为 0）
    pub page_size: u64,     // 每页记录数
}
```

**关键方法**:
- `new(page, page_size)` - 创建分页请求
- `offset()` - 获取 OFFSET 值（用于 SQL）
- `limit()` - 获取 LIMIT 值（用于 SQL）
- `next_page()` - 获取下一页请求
- `prev_page()` - 获取上一页请求（返回 Option）

**性能考虑**:
- `page_size` 建议范围：10-1000
- `page_size` 过小 < 10：增加数据库访问次数
- `page_size` 过大 > 10000：增加单次查询延迟
- 避免大偏移量查询（如第 1000 页+），建议使用游标分页

### PageResult - 分页结果

```rust
pub struct PageResult<T> {
    pub content: Vec<T>,           // 当前页数据
    pub total_elements: u64,       // 符合条件的总记录数
    pub page: u64,                 // 当前页号
    pub page_size: u64,            // 每页记录数
}
```

**关键方法**:
- `total_pages()` - 获取总分页数
- `page_elements()` - 当前页元素数
- `has_next()` - 是否有下一页
- `has_previous()` - 是否有上一页
- `is_first_page()` - 是否为第一页
- `is_last_page()` - 是否为最后一页
- `map<U, F>()` - 转换结果中的数据类型

## DBQueryRepo 方法一览

### 1. find_by_sequence()
按序列号查询单个实体

```rust
fn find_by_sequence(&self, sequence: u64) -> Result<Option<Self::E>, RepoError>;
```

**用途**: 事件重放场景、验证特定序列号的实体状态
**时间复杂度**: O(1) with index, O(n) without
**性能优化**: 使用序列号索引

### 2. find_one_by_condition()
按条件查询单个实体

```rust
fn find_one_by_condition(&self, condition: Self::E) -> Result<Option<Self::E>, RepoError>;
```

**用途**: 唯一性查询（如按ID查询）
**性能优化**: 找到第一条就可提前终止

### 3. find_all_by_condition()
按条件查询所有匹配实体

```rust
fn find_all_by_condition(&self, condition: Self::E) -> Result<Vec<Self::E>, RepoError>;
```

**用途**: 批量操作、完整列表
**性能考虑**: O(n) 复杂度，避免在热路径使用

### 4. find_all_by_condition_paginated() ⭐
按条件分页查询（推荐）

```rust
fn find_all_by_condition_paginated(
    &self,
    condition: Self::E,
    page_req: PageRequest,
) -> Result<PageResult<Self::E>, RepoError>;
```

**用途**: UI列表、API分页响应
**性能特性**:
- 时间复杂度：O(m + log n)，m 为当前页实体数
- 通过 LIMIT/OFFSET 在数据库层实现
- 总记录数可能需要额外 COUNT 查询

**设计建议**:
1. 为查询条件和排序字段建立复合索引
2. 对于深分页，使用游标分页替代 OFFSET 分页
3. 缓存总数信息
4. 设置 page_size 限制（如最大 1000）

**示例**:
```rust
let condition = OrderQuery { symbol: "BTCUSDT" };
let page_req = PageRequest::new(0, 20);  // 第一页，每页 20 条

let result = repo.find_all_by_condition_paginated(condition, page_req)?;

println!("总共 {} 条记录，第 {} 页，共 {} 页",
    result.total_elements,
    result.page + 1,
    result.total_pages());

for order in result.content {
    println!("订单: {:?}", order);
}

if result.has_next() {
    let next_result = repo.find_all_by_condition_paginated(
        condition,
        page_req.next_page()
    )?;
}
```

### 5. find_range_by_sequence_paginated()
按序列号范围分页查询

```rust
fn find_range_by_sequence_paginated(
    &self,
    from_sequence: u64,
    to_sequence: u64,
    page_req: PageRequest,
) -> Result<PageResult<Self::E>, RepoError>;
```

**用途**: 事件日志查询、快照恢复
**示例**:
```rust
let page_req = PageRequest::new(0, 100);
let result = repo.find_range_by_sequence_paginated(1000, 2000, page_req)?;
println!("序列号 1000-2000 范围内有 {} 条记录", result.total_elements);
```

### 6. find_by_id()
按实体ID查询单个实体

```rust
fn find_by_id(&self, entity_id: &str) -> Result<Option<Self::E>, RepoError>;
```

**用途**: 主键查询
**性能特性**: O(1) with unique index
**推荐**: 在热路径中使用此方法

### 7. find_by_cursor() 🚀
基于游标的分页查询（深分页优化）

```rust
fn find_by_cursor(
    &self,
    condition: Self::E,
    cursor: Option<String>,
    limit: u64,
    forward: bool,
) -> Result<(Vec<Self::E>, Option<String>), RepoError>;
```

**用途**: 深分页场景（> 1000 页）
**时间复杂度**: O(limit + log n)，与偏移量无关
**优势**: 避免大偏移量导致的性能问题

**游标分页 vs OFFSET 分页**:

| 特性 | OFFSET 分页 | 游标分页 |
|------|-----------|--------|
| 时间复杂度 | O(offset + limit + log n) | O(limit + log n) |
| 深分页性能 | ❌ 差 | ✅ 优秀 |
| 跳页能力 | ✅ 支持 | ❌ 不支持 |
| 数据变化敏感 | ✅ 敏感 | ❌ 不敏感 |
| 适用场景 | 小范围分页 | 深分页、流式加载 |

**示例**:
```rust
let condition = OrderQuery { symbol: "BTCUSDT" };

// 第一页
let (items, next_cursor) = repo.find_by_cursor(condition.clone(), None, 20, true)?;

// 第二页
let (next_items, next_cursor) = repo.find_by_cursor(
    condition,
    next_cursor,  // 使用前一次返回的游标
    20,
    true
)?;

// 向后翻页
let (prev_items, prev_cursor) = repo.find_by_cursor(
    condition,
    next_cursor,
    20,
    false  // forward = false
)?;
```

### 8. exists()
轻量级存在性检查

```rust
fn exists(&self, entity_id: &str) -> Result<bool, RepoError>;
```

**用途**: 前置验证、存在性检查
**性能优化**: 避免完整实体的反序列化开销
**推荐**: 在热路径中使用此方法而非 find_by_id

### 9. count()
获取实体总数

```rust
fn count(&self) -> Result<u64, RepoError>;
```

**用途**: 监控、统计
**性能优化**: 维护计数器避免每次扫描

## 性能优化建议

### 索引策略

**必需索引**:
```sql
-- 主键索引（唯一）
CREATE UNIQUE INDEX idx_entity_id ON entities(entity_id);

-- 序列号索引
CREATE INDEX idx_sequence ON entities(sequence);

-- 范围查询索引
CREATE INDEX idx_sequence_range ON entities(from_sequence, to_sequence);

-- 分页查询复合索引
CREATE INDEX idx_condition_order ON entities(condition_field, sort_field);
```

### 查询优化

1. **避免 N+1 问题**
   - 使用 JOIN 而非多次查询
   - 批量加载相关数据

2. **缓存策略**
   - 缓存总记录数（数据稳定时）
   - 缓存热查询结果
   - 缓存热分页（第 1-5 页）

3. **分页大小选择**
   ```rust
   // 推荐根据数据量和UI选择
   let page_size = match data_type {
       Large => 100,      // 大数据集
       Medium => 20,      // 中等数据集
       Small => 10,       // 小数据集
   };
   ```

4. **深分页处理**
   ```rust
   // 对于第 1000+ 页，使用游标分页
   if page_req.page > 100 {
       // 使用 find_by_cursor
   } else {
       // 使用 find_all_by_condition_paginated
   }
   ```

## Clean Architecture 设计

所有查询方法遵循以下原则：

1. **依赖倒置**: 返回领域对象 `E`，而非数据库模型
2. **接口隐藏**: 调用方无需知道数据存储细节
3. **单一职责**: 分离单条和多条查询逻辑
4. **可测试性**: 支持 mock 实现

## 低延迟特性

- ✅ O(1) 查询复杂度（使用索引）
- ✅ 避免全表扫描（分页、游标）
- ✅ 零分配操作（使用栈分配）
- ✅ CPU缓存友好（减少内存跳跃）

## 实现示例

### 参考实现框架

```rust
impl<E: Entity> DBQueryRepo for MyRepository<E> {
    type E = E;

    fn find_by_sequence(&self, sequence: u64) -> Result<Option<Self::E>, RepoError> {
        // 实现：使用序列号索引快速查询
        // SELECT * FROM entities WHERE sequence = ? LIMIT 1
        Ok(None)
    }

    fn find_one_by_condition(&self, condition: Self::E) -> Result<Option<Self::E>, RepoError> {
        // 实现：查询单条匹配结果
        Ok(None)
    }

    fn find_all_by_condition(&self, condition: Self::E) -> Result<Vec<Self::E>, RepoError> {
        // 实现：查询所有匹配结果
        Ok(Vec::new())
    }

    fn find_all_by_condition_paginated(
        &self,
        condition: Self::E,
        page_req: PageRequest,
    ) -> Result<PageResult<Self::E>, RepoError> {
        // 实现步骤：
        // 1. 构建 WHERE 条件
        // 2. 执行 COUNT 获取总数
        // 3. 执行分页查询：LIMIT ? OFFSET ?
        // 4. 反序列化结果
        // 5. 返回 PageResult

        let offset = page_req.offset();
        let limit = page_req.limit();

        // SQL: SELECT * FROM entities WHERE ... LIMIT ? OFFSET ?
        let content = vec![];
        let total_elements = 0;

        Ok(PageResult::new(content, total_elements, page_req.page, page_req.page_size))
    }

    fn find_range_by_sequence_paginated(
        &self,
        from_sequence: u64,
        to_sequence: u64,
        page_req: PageRequest,
    ) -> Result<PageResult<Self::E>, RepoError> {
        // 实现：范围分页查询
        // SQL: SELECT * FROM entities
        //      WHERE sequence >= ? AND sequence <= ?
        //      LIMIT ? OFFSET ?
        Ok(PageResult::new(Vec::new(), 0, page_req.page, page_req.page_size))
    }

    fn find_by_cursor(
        &self,
        condition: Self::E,
        cursor: Option<String>,
        limit: u64,
        forward: bool,
    ) -> Result<(Vec<Self::E>, Option<String>), RepoError> {
        // 实现：游标分页
        // 1. 解析游标值
        // 2. 构建条件：WHERE id > cursor (forward) 或 WHERE id < cursor (backward)
        // 3. 查询 limit+1 条记录（用于判断是否有下一页）
        // 4. 返回前 limit 条，最后一条记录作为下一个游标
        Ok((Vec::new(), None))
    }
}
```

## 总结

优化后的 `DBQueryRepo` 提供了：

- ✅ 6 种查询方法（按需选择）
- ✅ 完整的分页支持（OFFSET 和游标）
- ✅ Clean Architecture 遵循
- ✅ 低延迟设计
- ✅ 详细的文档和最佳实践
- ✅ 灵活的实现策略

根据不同场景选择合适的方法，即可获得最佳的性能和易用性。
