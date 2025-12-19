use diff::{ChangeLogEntry, ChangeType, Entity, FromCreatedEvent};
use mysql::prelude::*;

use crate::core::db_repo::{DBRepo, RepoError};

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
pub struct MySqlDbRepo<E: Entity> {
    connection: Option<mysql::PooledConn>,
    _entity: std::marker::PhantomData<E>
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
            connection: Some(conn),
            _entity: std::marker::PhantomData
        })
    }

    /// 创建一个无连接的实例（用于测试）
    pub fn new_mock() -> Self {
        MySqlDbRepo {
            connection: None,
            _entity: std::marker::PhantomData
        }
    }
}

impl<E: Entity> Default for MySqlDbRepo<E> {
    fn default() -> Self { Self::new_mock() }
}

impl<E: Entity + FromCreatedEvent> DBRepo for MySqlDbRepo<E> {
    type E = E;

    fn replay_event(&mut self, event: &ChangeLogEntry) -> Result<(), RepoError> {
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
        if self.connection.is_none() {
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
    fn insert_entity(&mut self, event: &ChangeLogEntry) -> Result<(), RepoError> {
        // For mock instance, return immediately
        if self.connection.is_none() {
            return Ok(());
        }

        // 根据 event 生成 INSERT SQL
        let sql = self.generate_insert_sql(event)?;

        // 执行 SQL 语句
        self.execute_sql(&sql)?;

        Ok(())
    }

    /// 根据字段信息生成 INSERT SQL
    fn generate_insert_sql(&self, event: &ChangeLogEntry) -> Result<String, RepoError> {
        let table_name = event.entity_type.clone();

        // 构建列名和值
        let mut column_names = vec![];

        let mut values = vec![];

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
        let sql = format!("INSERT INTO {} ({}) VALUES ({})", table_name, column_names.join(", "), values.join(", "));

        Ok(sql)
    }

    /// 执行 SQL 语句
    fn execute_sql(&mut self, sql: &str) -> Result<(), RepoError> {
        if let Some(ref mut conn) = self.connection {
            // TODO: 实现真实的 MySQL 执行逻辑
            // 示例（使用 mysql crate）：
            // conn.query_drop(sql)
            //     .map_err(|e| RepoError::DeserializationFailed(format!("SQL
            // execution failed: {}", e)))?;
        }
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
        if self.connection.is_none() {
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
        &mut self, entity_id: &str, entity_type: &str, _entity: &E, event: &ChangeLogEntry,
        changed_fields: &[diff::FieldChange]
    ) -> Result<(), RepoError> {
        // For mock instance, return immediately
        if self.connection.is_none() {
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
    fn delete_entity(&mut self, entity_id: &str, entity_type: &str) -> Result<(), RepoError> {
        // For mock instance, return immediately
        if self.connection.is_none() {
            return Ok(());
        }

        // 生成 DELETE SQL
        let sql = format!("DELETE FROM entities WHERE entity_id = '{}' AND entity_type = '{}'", entity_id, entity_type);

        // 执行 SQL 语句
        self.execute_sql(&sql)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use base_types::{Price, Quantity, Side, Symbol};

    use super::*;

    // 简单的测试 Entity 实现

    #[derive(Debug, Clone, PartialEq, entity_derive::Entity)]
    struct TestEntity {
        id: u64,
        // #[replay(skip)]
        symbol: Symbol,
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

        // repo.replay_event(&event).unwrap();
        let sql = repo.generate_insert_sql(&event).expect("Failed to generate SQL");

        // 验证 SQL 包含必要的字段
        assert!(sql.contains("INSERT INTO entities"));
        assert!(sql.contains("entity_id"));
        assert!(sql.contains("entity_type"));
        assert!(sql.contains("timestamp"));
        assert!(sql.contains("sequence"));
        assert!(sql.contains("symbol"));
        assert!(sql.contains("price"));
        assert!(sql.contains("order_123"));
        assert!(sql.contains("TestEntity"));
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

        let sql =
            repo.generate_update_sql(&event).expect("Failed to generate SQL");

        // 验证 UPDATE 语句
        assert!(sql.contains("UPDATE entities"));
        assert!(sql.contains("SET"));
        assert!(sql.contains("timestamp"));
        assert!(sql.contains("sequence"));
        assert!(sql.contains("price"));
        assert!(sql.contains("WHERE"));
        assert!(sql.contains("entity_id"));
        assert!(sql.contains("entity_type"));
    }

    #[test]
    fn test_mock_repo_creation() {
        let repo: MySqlDbRepo<TestEntity> = MySqlDbRepo::new_mock();
        // 验证 mock repo 创建成功
        let exists = repo.entity_exists("test", "test").unwrap();
        assert!(!exists, "Mock repo 中实体不应该存在");
    }
}
