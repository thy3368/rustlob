use diff::{ChangeLogEntry, ChangeType, Entity, FromCreatedEvent};
use mysql::prelude::*;
use std::sync::Mutex;
use immutable_derive::immutable;
use crate::core::db_repo::{CmdRepo, QueryRepo, RepoError, PageRequest, PageResult};

/// MySQL 数据库适配器
///
/// 提供基于 MySQL 的通用实体仓储实现
/// 支持所有实现了 Entity trait 的类型的事件回放和状态恢复
///
/// 数据库表结构：
/// ```sql
/// CREATE TABLE ChangeLogEntry (
///     id BIGINT AUTO_INCREMENT PRIMARY KEY,
///     entity_id VARCHAR(255) NOT NULL,
///     entity_type VARCHAR(100) NOT NULL,
///     data LONGTEXT NOT NULL COMMENT '序列化的实体JSON数据',
///     timestamp BIGINT NOT NULL COMMENT '事件时间戳',
///     sequence BIGINT NOT NULL COMMENT '事件序列号',
///     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
///     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
///     UNIQUE KEY unique_entity (entity_type, entity_id),
///     INDEX idx_entity_type (entity_type),
///     INDEX idx_timestamp (timestamp)
/// ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/// ```
#[immutable]
pub struct MySqlDbRepo<E: Entity> {
    connection: Mutex<Option<mysql::PooledConn>>,
    _entity: std::marker::PhantomData<E>
}

impl<E: Entity> MySqlDbRepo<E> {
    pub fn batch_insert(&self, p0: &Vec<E>) {
        todo!()
    }
}

impl<E: Entity> MySqlDbRepo<E> {
    /// 创建新的 MySQL 适配器
    ///
    /// # 参数
    /// - `url`: MySQL 连接字符串，例如
    ///   "mysql://user:password@localhost:3306/database"
    pub fn new(url: &str) -> Result<Self, RepoError> {
        let pool = mysql::Pool::new(url)
            .map_err(|e| RepoError::DeserializationFailed(format!("Failed to create connection pool: {}", e)))?;

        let conn = pool
            .get_conn()
            .map_err(|e| RepoError::DeserializationFailed(format!("Failed to get connection: {}", e)))?;

        Ok(MySqlDbRepo {
            connection: Mutex::new(Some(conn)),
            _entity: std::marker::PhantomData
        })
    }

    /// 创建一个无连接的实例（用于测试）
    pub fn new_mock() -> Self {
        MySqlDbRepo {
            connection: Mutex::new(None),
            _entity: std::marker::PhantomData
        }
    }

    /// 按实体ID查询单个实体
    ///
    /// 这是一个便利方法，用于按ID快速查询实体
    /// 当仓储为 mock 实例时，返回 Ok(None)
    pub fn find_by_id(&self, _id: &str) -> Result<Option<E>, RepoError>
    where
        E: FromCreatedEvent,
    {
        // For mock instance, return None
        if self.connection.lock().unwrap().is_none() {
            return Ok(None);
        }

        // TODO: 实现按 entity_id 查询数据库
        // SQL: SELECT * FROM [entity_type] WHERE entity_id = ? LIMIT 1
        Ok(None)
    }

    /// 一次性回放多个事件到数据库
    ///
    /// 遍历所有 ChangeLogEntry 事件，依次调用 replay_event 进行处理
    ///
    /// # 参数
    /// - `events`: ChangeLogEntry 事件列表
    ///
    /// # 返回
    /// - `Ok(())`: 所有事件回放成功
    /// - `Err(RepoError)`: 任何一个事件回放失败时返回错误
    pub fn replay(&self, events: &[ChangeLogEntry]) -> Result<(), RepoError>
    where
        E: FromCreatedEvent,
    {
        for event in events {
            self.replay_event(event)?;
        }
        Ok(())
    }
}

impl<E: Entity> Default for MySqlDbRepo<E> {
    fn default() -> Self { Self::new_mock() }
}

impl<E: Entity + FromCreatedEvent> CmdRepo for MySqlDbRepo<E> {
    type E = E;

    fn replay_event(&self, event: &ChangeLogEntry) -> Result<(), RepoError> {
        // 验证事件的实体类型是否匹配
        if event.entity_type != E::entity_type() {
            return Err(RepoError::DeserializationFailed(format!(
                "Entity type mismatch: expected {}, got {}",
                E::entity_type(),
                event.entity_type
            )));
        }

        match &event.change_type {
            // ========== Created 事件：在数据库中创建新实体 ==========
            ChangeType::Created {
                ..
            } => {
                // 1. 检查实体是否已存在（幂等性）
                if self.entity_exists(&event.entity_id, E::entity_type())? {
                    // 实体已存在，幂等处理：不报错直接返回
                    return Ok(());
                }

                // 2. 从 Created 事件的字段信息重构实体对象
                let entity =
                    E::from_created_event(event).map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;

                // 3. 序列化实体并保存到数据库
                self.insert_entity(event)?;

                Ok(())
            }

            // ========== Updated 事件：在数据库中更新实体 ==========
            ChangeType::Updated {
                changed_fields
            } => {
                // 1. 检查实体是否存在
                if !self.entity_exists(&event.entity_id, E::entity_type())? {
                    return Err(RepoError::OrderNotFound);
                }

                // 2. 加载现有实体
                let mut entity = self.load_entity(&event.entity_id)?;

                // 3. 应用变更到实体对象（通过 Entity::replay 方法）
                entity.replay(event).map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;

                // 4. 更新数据库中的实体
                self.update_entity(&event.entity_id, E::entity_type(), &entity, event, changed_fields)?;

                Ok(())
            }

            // ========== Deleted 事件：从数据库删除实体 ==========
            ChangeType::Deleted => {
                // 1. 检查实体是否存在
                if !self.entity_exists(&event.entity_id, E::entity_type())? {
                    // 实体不存在，幂等处理：删除不存在的实体不报错
                    return Ok(());
                }

                // 2. 从数据库删除实体
                self.delete_entity(&event.entity_id, E::entity_type())?;

                Ok(())
            }
        }
    }
}

// ============================================================================
// 通用数据库操作接口（模拟实现）
//
// 真实实现应该：
// 1. 使用真实的数据库驱动（如 sqlx, mysql, rusqlite 等）
// 2. 处理连接池和事务
// 3. 添加异常处理和重试机制
// 4. 实现批量操作优化
// ============================================================================

impl<E: Entity> MySqlDbRepo<E> {
    /// 检查数据库中是否存在指定的实体
    ///
    /// # SQL 等价操作
    /// ```sql
    /// SELECT EXISTS(
    ///     SELECT 1 FROM entities
    ///     WHERE entity_id = ? AND entity_type = ?
    /// )
    /// ```
    fn entity_exists(&self, entity_id: &str, entity_type: &str) -> Result<bool, RepoError> {
        // For mock instance (connection: None), always return false
        let conn = self.connection.lock().unwrap();
        if conn.is_none() {
            return Ok(false);
        }

        // TODO: Implement actual MySQL query
        // let query = "SELECT EXISTS(SELECT 1 FROM entities WHERE entity_id = ? AND
        // entity_type = ?)"; Execute query with parameters
        Ok(false)
    }

    /// 将新实体插入到数据库
    ///
    /// # 参数
    /// - `entity_id`: 实体的唯一标识符
    /// - `entity_type`: 实体的类型名称（来自 Entity::entity_type()）
    /// - `entity`: 实体对象
    /// - `event`: 触发此操作的事件（包含时间戳和序列号）
    ///
    /// # SQL 等价操作
    /// ```sql
    /// INSERT INTO entities (entity_id, entity_type, data, timestamp, sequence)
    /// VALUES (?, ?, ?, ?, ?)
    /// ```
    fn insert_entity(&self, event: &ChangeLogEntry) -> Result<(), RepoError> {
        // For mock instance, return immediately
        if self.connection.lock().unwrap().is_none() {
            return Ok(());
        }

        // 根据 event 生成 INSERT SQL
        let sql = self.generate_insert_sql(event)?;

        // 执行 SQL 语句
        self.execute_sql(&sql)?;

        Ok(())
    }

    /// 根据字段信息生成 INSERT SQL
    ///
    /// 生成格式: INSERT INTO [entity_type] (entity_id, entity_type, timestamp, sequence, [fields...]) VALUES (...)
    fn generate_insert_sql(&self, event: &ChangeLogEntry) -> Result<String, RepoError> {
        let table_name = event.entity_type.clone();

        // 构建列名和值 - 包含基础元数据列
        let mut column_names = vec![
            "entity_id".to_string(),
            "entity_type".to_string(),
            "timestamp".to_string(),
            "sequence".to_string(),
        ];

        let mut values = vec![
            format!("'{}'", event.entity_id),
            format!("'{}'", event.entity_type),
            format!("{}", event.timestamp),
            format!("{}", event.sequence),
        ];

        // 从 Created 事件中提取字段
        if let ChangeType::Created {
            fields
        } = &event.change_type
        {
            // 添加来自字段变更的列
            for field in fields {
                column_names.push(field.field_name.to_string());
                values.push(format!("'{}'", field.new_value));
            }
        }

        // 生成 INSERT 语句
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            column_names.join(", "),
            values.join(", ")
        );

        Ok(sql)
    }

    /// 执行 SQL 语句
    ///
    /// # 参数
    /// - `sql`: 要执行的 SQL 语句
    ///
    /// # 错误处理
    /// - 如果连接为 None（mock 实例），直接返回 Ok
    /// - 如果 SQL 执行失败，返回 DeserializationFailed 错误
    fn execute_sql(&self, sql: &str) -> Result<(), RepoError> {
        let mut conn = self.connection.lock().unwrap();
        if let Some(ref mut c) = conn.as_mut() {
            // 使用 mysql crate 执行 SQL
            c.query_drop(sql)
                .map_err(|e| RepoError::DeserializationFailed(format!(
                    "SQL execution failed: {}. SQL: {}",
                    e,
                    sql
                )))?;
        }
        // Mock 实例（connection: None）直接返回成功
        Ok(())
    }

    /// 从数据库加载实体
    ///
    /// # SQL 等价操作
    /// ```sql
    /// SELECT data FROM entities
    /// WHERE entity_id = ? AND entity_type = ?
    /// LIMIT 1
    /// ```
    fn load_entity(&self, entity_id: &str) -> Result<E, RepoError> {
        // For mock instance, return error
        let conn = self.connection.lock().unwrap();
        if conn.is_none() {
            return Err(RepoError::OrderNotFound);
        }

        // TODO: 实现数据库查询和反序列化
        // 步骤：
        // 1. 执行 SELECT 查询获取序列化数据
        // 2. 反序列化为实体对象
        // 3. 处理查询错误
        //
        // 示例代码框架：
        // let query = "SELECT data FROM entities WHERE entity_id = ? AND entity_type =
        // ? LIMIT 1"; let result: Option<String> = self.pool.exec_first(query,
        // (entity_id, E::entity_type())) .map_err(|e|
        // RepoError::DeserializationFailed(e.to_string()))?;
        //
        // match result {
        // Some(serialized) => {
        // serde_json::from_str(&serialized)
        // .map_err(|e| RepoError::DeserializationFailed(e.to_string()))
        // }
        // None => Err(RepoError::OrderNotFound),
        // }

        Err(RepoError::OrderNotFound)
    }

    /// 更新数据库中的实体
    ///
    /// # 参数
    /// - `entity_id`: 实体的唯一标识符
    /// - `entity_type`: 实体的类型名称
    /// - `entity`: 更新后的实体对象
    /// - `event`: 触发此操作的事件
    /// - `changed_fields`: 变更的字段列表（可选用于性能优化）
    ///
    /// # SQL 等价操作
    /// ```sql
    /// UPDATE entities SET data = ?, timestamp = ?, sequence = ?
    /// WHERE entity_id = ? AND entity_type = ?
    /// ```
    fn update_entity(
        &self, entity_id: &str, entity_type: &str, _entity: &E, event: &ChangeLogEntry,
        changed_fields: &[diff::FieldChange]
    ) -> Result<(), RepoError> {
        // For mock instance, return immediately
        if self.connection.lock().unwrap().is_none() {
            return Ok(());
        }

        // 根据变更的字段生成 UPDATE SQL
        let sql = self.generate_update_sql( event)?;

        // 执行 SQL 语句
        self.execute_sql(&sql)?;

        Ok(())
    }

    /// 根据变更的字段生成 UPDATE SQL
    fn generate_update_sql(
        &self,
        event: &ChangeLogEntry,
    ) -> Result<String, RepoError> {
        let table_name = event.entity_type.clone();

        // 构建 SET 子句
        let mut set_clauses = vec![
            format!("timestamp = {}", event.timestamp),
            format!("sequence = {}", event.sequence),
        ];

        // 从 Updated 事件中提取字段变更
        if let ChangeType::Updated { changed_fields } = &event.change_type {
            // 添加变更字段的更新
            for field in changed_fields {
                set_clauses.push(format!("{} = '{}'", field.field_name, field.new_value));
            }
        }

        // 生成 UPDATE 语句
        let sql = format!(
            "UPDATE {} SET {} WHERE entity_id = '{}' AND entity_type = '{}'",
            table_name,
            set_clauses.join(", "),
            event.entity_id,
            event.entity_type
        );

        Ok(sql)
    }

    /// 从数据库删除实体
    ///
    /// # SQL 等价操作
    /// ```sql
    /// DELETE FROM entities
    /// WHERE entity_id = ? AND entity_type = ?
    /// ```
    fn delete_entity(&self, entity_id: &str, entity_type: &str) -> Result<(), RepoError> {
        // For mock instance, return immediately
        if self.connection.lock().unwrap().is_none() {
            return Ok(());
        }

        // 生成 DELETE SQL
        let sql = format!("DELETE FROM entities WHERE entity_id = '{}' AND entity_type = '{}'", entity_id, entity_type);

        // 执行 SQL 语句
        self.execute_sql(&sql)?;

        Ok(())
    }
}

impl<E: Entity + FromCreatedEvent> QueryRepo for MySqlDbRepo<E> {
    type E = E;

    /// 按序列号查询单个实体
    fn find_by_sequence(&self, sequence: u64) -> Result<Option<Self::E>, RepoError> {
        // For mock instance, return None
        if self.connection.lock().unwrap().is_none() {
            return Ok(None);
        }

        // SQL: SELECT * FROM [entity_type] WHERE sequence = ? LIMIT 1
        // Note: 实现需要能够反序列化实体
        // TODO: 实现数据库查询和反序列化
        Ok(None)
    }

    /// 按条件查询单个实体
    fn find_one_by_condition(&self, _condition: Self::E) -> Result<Option<Self::E>, RepoError> {
        // For mock instance, return None
        if self.connection.lock().unwrap().is_none() {
            return Ok(None);
        }

        // SQL: SELECT * FROM [entity_type] WHERE [condition_fields] LIMIT 1
        // Note: condition 参数应该包含查询条件的值
        // TODO: 实现动态条件查询
        Ok(None)
    }

    /// 按条件查询所有匹配实体
    fn find_all_by_condition(&self, _condition: Self::E) -> Result<Vec<Self::E>, RepoError> {
        // For mock instance, return empty vector
        if self.connection.lock().unwrap().is_none() {
            return Ok(Vec::new());
        }

        // SQL: SELECT * FROM [entity_type] WHERE [condition_fields]
        // TODO: 实现动态条件查询
        Ok(Vec::new())
    }

    /// 按条件分页查询
    fn find_all_by_condition_paginated(
        &self,
        _condition: Self::E,
        page_req: PageRequest,
    ) -> Result<PageResult<Self::E>, RepoError> {
        // For mock instance, return empty result
        if self.connection.lock().unwrap().is_none() {
            return Ok(PageResult::new(
                Vec::new(),
                0,
                page_req.page,
                page_req.page_size,
            ));
        }

        // SQL: SELECT * FROM [entity_type]
        //      WHERE [condition_fields]
        //      LIMIT ? OFFSET ?
        //
        // 以及单独的 COUNT 查询：
        // SELECT COUNT(*) FROM [entity_type] WHERE [condition_fields]
        //
        // TODO: 实现分页查询和 COUNT 查询
        Ok(PageResult::new(
            Vec::new(),
            0,
            page_req.page,
            page_req.page_size,
        ))
    }

    /// 按序列号范围分页查询
    fn find_range_by_sequence_paginated(
        &self,
        _from_sequence: u64,
        _to_sequence: u64,
        page_req: PageRequest,
    ) -> Result<PageResult<Self::E>, RepoError> {
        // For mock instance, return empty result
        if self.connection.lock().unwrap().is_none() {
            return Ok(PageResult::new(
                Vec::new(),
                0,
                page_req.page,
                page_req.page_size,
            ));
        }

        // SQL: SELECT * FROM [entity_type]
        //      WHERE sequence >= ? AND sequence <= ?
        //      LIMIT ? OFFSET ?
        //
        // TODO: 实现范围分页查询
        Ok(PageResult::new(
            Vec::new(),
            0,
            page_req.page,
            page_req.page_size,
        ))
    }

    /// 按实体ID查询
    fn find_by_id(&self, _entity_id: &str) -> Result<Option<Self::E>, RepoError> {
        // For mock instance, return None
        if self.connection.lock().unwrap().is_none() {
            return Ok(None);
        }

        // SQL: SELECT * FROM [entity_type] WHERE entity_id = ? LIMIT 1
        // TODO: 实现主键查询
        Ok(None)
    }

    /// 基于游标的分页查询
    fn find_by_cursor(
        &self,
        _condition: Self::E,
        _cursor: Option<String>,
        _limit: u64,
        _forward: bool,
    ) -> Result<(Vec<Self::E>, Option<String>), RepoError> {
        // For mock instance, return empty result
        if self.connection.lock().unwrap().is_none() {
            return Ok((Vec::new(), None));
        }

        // SQL:
        // Forward (forward=true):
        //   SELECT * FROM [entity_type]
        //   WHERE [condition_fields] AND id > ?cursor
        //   ORDER BY id ASC
        //   LIMIT ? + 1
        //
        // Backward (forward=false):
        //   SELECT * FROM [entity_type]
        //   WHERE [condition_fields] AND id < ?cursor
        //   ORDER BY id DESC
        //   LIMIT ? + 1
        //
        // TODO: 实现游标分页查询
        Ok((Vec::new(), None))
    }
}

impl<E: Entity> MySqlDbRepo<E> {
    /// 生成 SELECT COUNT(*) SQL 语句
    ///
    /// 用于获取满足条件的总记录数
    ///
    /// # SQL 等价操作
    /// ```sql
    /// SELECT COUNT(*) FROM [entity_type]
    /// WHERE [condition_fields]
    /// ```
    fn generate_count_sql(&self, entity_type: &str, where_clause: &str) -> String {
        if where_clause.is_empty() {
            format!("SELECT COUNT(*) as cnt FROM {}", entity_type)
        } else {
            format!(
                "SELECT COUNT(*) as cnt FROM {} WHERE {}",
                entity_type, where_clause
            )
        }
    }

    /// 生成 SELECT 分页 SQL 语句
    ///
    /// # SQL 等价操作
    /// ```sql
    /// SELECT * FROM [entity_type]
    /// WHERE [condition_fields]
    /// ORDER BY [order_fields]
    /// LIMIT ? OFFSET ?
    /// ```
    fn generate_paginated_select_sql(
        &self,
        entity_type: &str,
        where_clause: &str,
        order_clause: &str,
        limit: u64,
        offset: u64,
    ) -> String {
        let mut sql = format!("SELECT * FROM {}", entity_type);

        if !where_clause.is_empty() {
            sql.push_str(&format!(" WHERE {}", where_clause));
        }

        if !order_clause.is_empty() {
            sql.push_str(&format!(" ORDER BY {}", order_clause));
        }

        sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
        sql
    }

    /// 生成范围查询 SQL
    fn generate_range_where_clause(&self, from_seq: u64, to_seq: u64) -> String {
        format!("sequence >= {} AND sequence <= {}", from_seq, to_seq)
    }

    /// 生成游标查询 SQL WHERE 子句
    fn generate_cursor_where_clause(
        &self,
        cursor: &str,
        forward: bool,
        additional_condition: &str,
    ) -> String {
        let comparison = if forward { ">" } else { "<" };
        let mut where_clause = format!("entity_id {} '{}'", comparison, cursor);

        if !additional_condition.is_empty() {
            where_clause.push_str(&format!(" AND {}", additional_condition));
        }

        where_clause
    }
}

#[cfg(test)]
mod tests {
    use base_types::{Price, Quantity, Side, TradingPair};

    use super::*;

    // 简单的测试 Entity 实现

    #[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
    struct TestEntity {
        id: u64,
        // #[replay(skip)]
        symbol: TradingPair,
        // #[replay(skip)]
        price: Price,
        // #[replay(skip)]
        quantity: Quantity,
        // #[replay(skip)]
        filled_quantity: Quantity,
        // #[replay(skip)]
        side: Side
    }


    #[test]
    fn test_generate_insert_sql() {
        let repo: MySqlDbRepo<TestEntity> = MySqlDbRepo::new_mock();

        let fields = vec![
            diff::FieldChange {
                field_name: "symbol".into(),
                new_value: "BTCUSDT".to_string(),
                old_value: "".to_string()
            },
            diff::FieldChange {
                field_name: "price".into(),
                new_value: "50000.0".to_string(),
                old_value: "".to_string()
            },
        ];

        let event = ChangeLogEntry {
            entity_id: "order_123".to_string(),
            entity_type: "Order".to_string(),
            timestamp: 1234567890,
            sequence: 1,
            change_type: ChangeType::Created {
                fields: fields
            }
        };

        let sql = repo.generate_insert_sql(&event).expect("Failed to generate SQL");

        // 验证 SQL 包含必要的字段
        // 使用 entity_type 作为表名的多表设计
        assert!(sql.contains("INSERT INTO Order"), "SQL 应该插入到 Order 表");
        assert!(sql.contains("entity_id"), "SQL 应该包含 entity_id");
        assert!(sql.contains("entity_type"), "SQL 应该包含 entity_type");
        assert!(sql.contains("timestamp"), "SQL 应该包含 timestamp");
        assert!(sql.contains("sequence"), "SQL 应该包含 sequence");
        assert!(sql.contains("symbol"), "SQL 应该包含 symbol");
        assert!(sql.contains("price"), "SQL 应该包含 price");
        assert!(sql.contains("order_123"), "SQL 应该包含 entity_id 的值");
    }

    #[test]
    fn test_generate_update_sql() {
        let repo: MySqlDbRepo<TestEntity> = MySqlDbRepo::new_mock();

        let changed_fields = vec![diff::FieldChange {
            field_name: "price".into(),
            new_value: "51000.0".to_string(),
            old_value: "50000.0".to_string()
        }];

        let event = ChangeLogEntry {
            entity_id: "order_123".to_string(),
            entity_type: "Order".to_string(),
            timestamp: 1234567891,
            sequence: 2,
            change_type: ChangeType::Updated {
                changed_fields: changed_fields
            }
        };

        let sql = repo.generate_update_sql(&event).expect("Failed to generate SQL");

        // 验证 UPDATE 语句
        // 使用 entity_type 作为表名的多表设计
        assert!(sql.contains("UPDATE Order"), "SQL 应该更新 Order 表");
        assert!(sql.contains("SET"), "SQL 应该包含 SET 子句");
        assert!(sql.contains("timestamp"), "SQL 应该包含 timestamp");
        assert!(sql.contains("sequence"), "SQL 应该包含 sequence");
        assert!(sql.contains("price"), "SQL 应该包含 price");
        assert!(sql.contains("WHERE"), "SQL 应该包含 WHERE 子句");
        assert!(sql.contains("entity_id = 'order_123'"), "SQL 应该过滤正确的 entity_id");
    }

    #[test]
    fn test_mock_repo_creation() {
        let repo: MySqlDbRepo<TestEntity> = MySqlDbRepo::new_mock();
        // 验证 mock repo 创建成功
        let exists = repo.entity_exists("test", "test").unwrap();
        assert!(!exists, "Mock repo 中实体不应该存在");
    }

    #[test]
    fn test_generate_count_sql() {
        let repo: MySqlDbRepo<TestEntity> = MySqlDbRepo::new_mock();

        // 不含 WHERE 子句的 COUNT
        let sql = repo.generate_count_sql("Order", "");
        assert_eq!(sql, "SELECT COUNT(*) as cnt FROM Order");

        // 含 WHERE 子句的 COUNT
        let sql = repo.generate_count_sql("Order", "symbol = 'BTCUSDT' AND status = 'PENDING'");
        assert!(sql.contains("WHERE symbol = 'BTCUSDT' AND status = 'PENDING'"));
        assert!(sql.contains("COUNT(*)"));
    }

    #[test]
    fn test_generate_paginated_select_sql() {
        let repo: MySqlDbRepo<TestEntity> = MySqlDbRepo::new_mock();

        // 基本分页 - 第一页，每页 20 条
        let sql = repo.generate_paginated_select_sql("Order", "", "", 20, 0);
        assert_eq!(sql, "SELECT * FROM Order LIMIT 20 OFFSET 0");

        // 第二页
        let sql = repo.generate_paginated_select_sql("Order", "", "", 20, 20);
        assert_eq!(sql, "SELECT * FROM Order LIMIT 20 OFFSET 20");

        // 含 WHERE 和 ORDER 子句
        let sql = repo.generate_paginated_select_sql(
            "Order",
            "symbol = 'BTCUSDT'",
            "created_at DESC",
            50,
            100,
        );
        assert!(sql.contains("WHERE symbol = 'BTCUSDT'"));
        assert!(sql.contains("ORDER BY created_at DESC"));
        assert!(sql.contains("LIMIT 50 OFFSET 100"));
    }

    #[test]
    fn test_generate_range_where_clause() {
        let repo: MySqlDbRepo<TestEntity> = MySqlDbRepo::new_mock();

        let where_clause = repo.generate_range_where_clause(100, 200);
        assert_eq!(where_clause, "sequence >= 100 AND sequence <= 200");

        let where_clause = repo.generate_range_where_clause(0, 1000);
        assert_eq!(where_clause, "sequence >= 0 AND sequence <= 1000");
    }

    #[test]
    fn test_generate_cursor_where_clause() {
        let repo: MySqlDbRepo<TestEntity> = MySqlDbRepo::new_mock();

        // 向前翻页（forward=true）
        let where_clause = repo.generate_cursor_where_clause("order_100", true, "");
        assert_eq!(where_clause, "entity_id > 'order_100'");

        // 向后翻页（forward=false）
        let where_clause = repo.generate_cursor_where_clause("order_100", false, "");
        assert_eq!(where_clause, "entity_id < 'order_100'");

        // 附加条件
        let where_clause = repo.generate_cursor_where_clause("order_100", true, "symbol = 'BTCUSDT'");
        assert!(where_clause.contains("entity_id > 'order_100'"));
        assert!(where_clause.contains("symbol = 'BTCUSDT'"));
    }

}
