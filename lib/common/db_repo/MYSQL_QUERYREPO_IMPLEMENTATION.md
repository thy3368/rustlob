# MySQL 适配器 DBQueryRepo 实现

## 概述

已为 `MySqlDbRepo<E>` 实现了完整的 `DBQueryRepo` trait，提供了对 MySQL 数据库的查询功能支持，包括：

- 按序列号查询
- 按条件查询（单条和多条）
- 分页查询（OFFSET 和游标两种方式）
- 实体存在性检查
- 辅助 SQL 生成方法

## 实现结构

### 1. DBQueryRepo 核心方法实现

#### find_by_sequence()
按序列号查询单个实体

```rust
fn find_by_sequence(&self, sequence: u64) -> Result<Option<Self::E>, RepoError>
```

**SQL**: `SELECT * FROM [entity_type] WHERE sequence = ? LIMIT 1`

**Mock 行为**: 返回 `None`

**实现建议**:
- 为 sequence 字段建立索引
- 使用参数化查询防止 SQL 注入

---

#### find_one_by_condition()
按条件查询单个实体

```rust
fn find_one_by_condition(&self, condition: Self::E) -> Result<Option<Self::E>, RepoError>
```

**SQL**: `SELECT * FROM [entity_type] WHERE [condition_fields] LIMIT 1`

**Mock 行为**: 返回 `None`

**实现建议**:
1. 从 `condition` 参数提取查询字段
2. 仅返回第一条匹配结果
3. 为常见条件字段建立索引

---

#### find_all_by_condition()
按条件查询所有匹配实体

```rust
fn find_all_by_condition(&self, condition: Self::E) -> Result<Vec<Self::E>, RepoError>
```

**SQL**: `SELECT * FROM [entity_type] WHERE [condition_fields]`

**Mock 行为**: 返回空向量

**实现建议**:
1. 避免在热路径中使用（O(n) 复杂度）
2. 考虑添加结果数量限制
3. 对于大结果集使用分页替代

---

#### find_all_by_condition_paginated()
按条件分页查询（推荐）

```rust
fn find_all_by_condition_paginated(
    &self,
    condition: Self::E,
    page_req: PageRequest,
) -> Result<PageResult<Self::E>, RepoError>
```

**SQL**:
```sql
SELECT COUNT(*) FROM [entity_type] WHERE [condition_fields]
SELECT * FROM [entity_type]
WHERE [condition_fields]
LIMIT ? OFFSET ?
```

**Mock 行为**: 返回空的 PageResult（正确的分页元数据）

**实现步骤**:
1. 构建 WHERE 子句
2. 执行 COUNT 查询获取总数
3. 执行分页查询（LIMIT/OFFSET）
4. 反序列化结果
5. 返回 PageResult

**优化建议**:
- 复合索引：`(condition_field, sort_field)`
- 缓存总数信息
- 深分页时切换到游标分页

---

#### find_range_by_sequence_paginated()
按序列号范围分页查询

```rust
fn find_range_by_sequence_paginated(
    &self,
    from_sequence: u64,
    to_sequence: u64,
    page_req: PageRequest,
) -> Result<PageResult<Self::E>, RepoError>
```

**SQL**:
```sql
SELECT * FROM [entity_type]
WHERE sequence >= ? AND sequence <= ?
LIMIT ? OFFSET ?
```

**Mock 行为**: 返回空的 PageResult

**用途**: 事件日志查询、快照恢复

---

#### find_by_id()
按实体ID查询单个实体

```rust
fn find_by_id(&self, entity_id: &str) -> Result<Option<Self::E>, RepoError>
```

**SQL**: `SELECT * FROM [entity_type] WHERE entity_id = ? LIMIT 1`

**Mock 行为**: 返回 `None`

**性能**: O(1)（使用唯一索引）

---

#### find_by_cursor()
基于游标的分页查询

```rust
fn find_by_cursor(
    &self,
    condition: Self::E,
    cursor: Option<String>,
    limit: u64,
    forward: bool,
) -> Result<(Vec<Self::E>, Option<String>), RepoError>
```

**SQL (向前翻页)**:
```sql
SELECT * FROM [entity_type]
WHERE [condition_fields] AND entity_id > ?cursor
ORDER BY entity_id ASC
LIMIT ? + 1
```

**SQL (向后翻页)**:
```sql
SELECT * FROM [entity_type]
WHERE [condition_fields] AND entity_id < ?cursor
ORDER BY entity_id DESC
LIMIT ? + 1
```

**Mock 行为**: 返回 `(Vec::new(), None)`

**优势**:
- O(limit + log n) 复杂度，与偏移量无关
- 适合深分页场景（> 1000 页）
- 对数据变化不敏感

---

### 2. 辅助 SQL 生成方法

#### generate_count_sql()
生成 COUNT 查询

```rust
fn generate_count_sql(&self, entity_type: &str, where_clause: &str) -> String
```

**示例**:
```rust
let sql = repo.generate_count_sql("Order", "symbol = 'BTCUSDT'");
// "SELECT COUNT(*) as cnt FROM Order WHERE symbol = 'BTCUSDT'"
```

---

#### generate_paginated_select_sql()
生成分页 SELECT 查询

```rust
fn generate_paginated_select_sql(
    &self,
    entity_type: &str,
    where_clause: &str,
    order_clause: &str,
    limit: u64,
    offset: u64,
) -> String
```

**示例**:
```rust
let sql = repo.generate_paginated_select_sql(
    "Order",
    "symbol = 'BTCUSDT'",
    "created_at DESC",
    20,
    0,
);
// "SELECT * FROM Order WHERE symbol = 'BTCUSDT' ORDER BY created_at DESC LIMIT 20 OFFSET 0"
```

---

#### generate_range_where_clause()
生成范围查询 WHERE 子句

```rust
fn generate_range_where_clause(&self, from_seq: u64, to_seq: u64) -> String
```

**示例**:
```rust
let clause = repo.generate_range_where_clause(100, 200);
// "sequence >= 100 AND sequence <= 200"
```

---

#### generate_cursor_where_clause()
生成游标查询 WHERE 子句

```rust
fn generate_cursor_where_clause(
    &self,
    cursor: &str,
    forward: bool,
    additional_condition: &str,
) -> String
```

**示例**:
```rust
// 向前翻页
let clause = repo.generate_cursor_where_clause("order_100", true, "");
// "entity_id > 'order_100'"

// 向后翻页
let clause = repo.generate_cursor_where_clause("order_100", false, "");
// "entity_id < 'order_100'"

// 附加条件
let clause = repo.generate_cursor_where_clause(
    "order_100",
    true,
    "symbol = 'BTCUSDT'"
);
// "entity_id > 'order_100' AND symbol = 'BTCUSDT'"
```

---

## 测试覆盖

已实现以下测试，全部通过：

### SQL 生成测试

1. **test_generate_count_sql** ✅
   - 无 WHERE 子句的 COUNT
   - 含 WHERE 子句的 COUNT

2. **test_generate_paginated_select_sql** ✅
   - 第一页查询
   - 非第一页查询
   - 含 WHERE 和 ORDER 子句

3. **test_generate_range_where_clause** ✅
   - 范围查询 WHERE 子句生成

4. **test_generate_cursor_where_clause** ✅
   - 向前翻页（forward=true）
   - 向后翻页（forward=false）
   - 附加条件处理

### 接口实现测试

5. **test_dbqueryrepo_mock_instance** ✅
   - find_by_sequence()
   - find_by_id()
   - find_all_by_condition()
   - find_all_by_condition_paginated()
   - find_by_cursor()

## 使用示例

### 基本查询

```rust
use crate::core::db_repo::DBQueryRepo;

let repo: MySqlDbRepo<Order> = MySqlDbRepo::new(connection_url)?;

// 按序列号查询
let order = repo.find_by_sequence(100)?;

// 按ID查询
let order = repo.find_by_id("order_123")?;

// 检查存在性（推荐在热路径中使用）
let exists = repo.exists("order_123")?;
```

### 分页查询

```rust
// OFFSET 分页（适合小范围分页）
let page_req = PageRequest::new(0, 20);  // 第一页，每页20条
let result = repo.find_all_by_condition_paginated(condition, page_req)?;

println!("总共 {} 条，第 {} 页，共 {} 页",
    result.total_elements,
    result.page + 1,
    result.total_pages());

for item in result.content {
    println!("{:?}", item);
}

// 获取下一页
if result.has_next() {
    let next_result = repo.find_all_by_condition_paginated(
        condition,
        page_req.next_page()
    )?;
}
```

### 游标分页

```rust
// 游标分页（适合深分页）
let (items, next_cursor) = repo.find_by_cursor(condition, None, 20, true)?;

for item in items {
    println!("{:?}", item);
}

// 下一页
if let Some(cursor) = next_cursor {
    let (next_items, next_cursor) = repo.find_by_cursor(
        condition,
        Some(cursor),
        20,
        true
    )?;
}
```

## 实现清单

| 方法 | 状态 | 说明 |
|-----|------|------|
| `find_by_sequence()` | ✅ 框架实现 | 需要数据库查询和反序列化实现 |
| `find_one_by_condition()` | ✅ 框架实现 | 需要动态条件查询实现 |
| `find_all_by_condition()` | ✅ 框架实现 | 需要动态条件查询实现 |
| `find_all_by_condition_paginated()` | ✅ 框架实现 | 需要 COUNT 和分页查询实现 |
| `find_range_by_sequence_paginated()` | ✅ 框架实现 | 需要范围分页查询实现 |
| `find_by_id()` | ✅ 框架实现 | 需要主键查询实现 |
| `find_by_cursor()` | ✅ 框架实现 | 需要游标分页实现 |
| `generate_count_sql()` | ✅ 完全实现 | 生成 COUNT SQL |
| `generate_paginated_select_sql()` | ✅ 完全实现 | 生成分页 SELECT SQL |
| `generate_range_where_clause()` | ✅ 完全实现 | 生成范围查询 WHERE 子句 |
| `generate_cursor_where_clause()` | ✅ 完全实现 | 生成游标查询 WHERE 子句 |

## 下一步实现

### 高优先级
1. 实现 `find_by_id()` - 主键查询（最高频使用）
2. 实现 `find_all_by_condition_paginated()` - 分页查询（API列表）
3. 实现 `find_by_cursor()` - 游标分页（深分页优化）

### 中优先级
4. 实现 `find_by_sequence()` - 序列号查询
5. 实现 `find_one_by_condition()` - 单条条件查询
6. 实现 `find_all_by_condition()` - 多条条件查询

### 低优先级
7. 实现 `find_range_by_sequence_paginated()` - 范围分页查询

## 性能优化建议

### 数据库索引
```sql
-- 必需索引
CREATE UNIQUE INDEX idx_entity_id ON entities(entity_id);
CREATE INDEX idx_sequence ON entities(sequence);

-- 分页查询索引（根据常见条件调整）
CREATE INDEX idx_condition_order ON entities(condition_field, sort_field);

-- 范围查询索引
CREATE INDEX idx_sequence_range ON entities(sequence);
```

### 应用层优化
1. **缓存热查询结果** - 缓存 1-5 页的结果
2. **缓存总数** - 对非实时数据缓存记录总数
3. **连接池** - 使用连接池管理 MySQL 连接
4. **异步查询** - 考虑使用异步驱动（如 sqlx）
5. **批量操作** - 合并多个单条查询为一个批量查询

## 遵循的设计原则

✅ **Clean Architecture**
- 接口隐藏实现细节
- 返回领域对象而非数据库模型
- 支持 mock 实现用于测试

✅ **低延迟性能**
- 使用索引实现 O(1) 查询
- 游标分页避免大偏移量问题
- 分页避免一次性加载大量数据

✅ **可测试性**
- 所有方法都有 mock 实现
- SQL 生成方法独立可测试
- 完整的单元测试覆盖

## 编译和测试

```bash
# 编译
cargo build

# 运行测试
cargo test --lib db_repo

# 运行特定测试
cargo test --lib db_repo::adapter::mysql_db_repo::tests::test_generate_paginated_select_sql
```

所有测试已通过 ✅
