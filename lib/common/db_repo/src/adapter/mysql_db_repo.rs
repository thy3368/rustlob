use crate::core::db_repo::{DBRepo, RepoError};
use diff::{Entity, ChangeLogEntry, ChangeType, FromCreatedEvent};
use mysql::prelude::*;

/// MySQL 数据库适配器
///
/// 提供基于 MySQL 的通用实体仓储实现
/// 支持所有实现了 Entity trait 的类型的事件回放和状态恢复
///
/// 数据库表结构：
/// ```sql
/// CREATE TABLE entities (
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
pub struct MySqlDbRepo {
    connection: Option<mysql::PooledConn>,
}

impl MySqlDbRepo {
    /// 创建新的 MySQL 适配器
    ///
    /// # 参数
    /// - `url`: MySQL 连接字符串，例如 "mysql://user:password@localhost:3306/database"
    pub fn new(url: &str) -> Result<Self, RepoError> {
        let pool = mysql::Pool::new(url)
            .map_err(|e| RepoError::DeserializationFailed(format!("Failed to create connection pool: {}", e)))?;

        let conn = pool.get_conn()
            .map_err(|e| RepoError::DeserializationFailed(format!("Failed to get connection: {}", e)))?;

        Ok(MySqlDbRepo {
            connection: Some(conn),
        })
    }

    /// 创建一个无连接的实例（用于测试）
    pub fn new_mock() -> Self {
        MySqlDbRepo {
            connection: None,
        }
    }
}

impl Default for MySqlDbRepo {
    fn default() -> Self {
        Self::new_mock()
    }
}

impl<O: Entity + FromCreatedEvent> DBRepo<O> for MySqlDbRepo {
    fn replay_event(&mut self, event: &ChangeLogEntry) -> Result<(), RepoError> {
        // 验证事件的实体类型是否匹配
        if event.entity_type != O::entity_type() {
            return Err(RepoError::DeserializationFailed(
                format!(
                    "Entity type mismatch: expected {}, got {}",
                    O::entity_type(),
                    event.entity_type
                )
            ));
        }

        match &event.change_type {
            // ========== Created 事件：在数据库中创建新实体 ==========
            ChangeType::Created { .. } => {
                // 1. 检查实体是否已存在（幂等性）
                if self.entity_exists(&event.entity_id, O::entity_type())? {
                    // 实体已存在，幂等处理：不报错直接返回
                    return Ok(());
                }

                // 2. 从 Created 事件的字段信息重构实体对象
                let entity = O::from_created_event(event)
                    .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;

                // 3. 序列化实体并保存到数据库
                self.insert_entity(&event.entity_id, O::entity_type(), &entity, event)?;

                Ok(())
            }

            // ========== Updated 事件：在数据库中更新实体 ==========
            ChangeType::Updated { changed_fields } => {
                // 1. 检查实体是否存在
                if !self.entity_exists(&event.entity_id, O::entity_type())? {
                    return Err(RepoError::OrderNotFound);
                }

                // 2. 加载现有实体
                let mut entity = self.load_entity::<O>(&event.entity_id)?;

                // 3. 应用变更到实体对象（通过 Entity::replay 方法）
                entity
                    .replay(event)
                    .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;

                // 4. 更新数据库中的实体
                self.update_entity(&event.entity_id, O::entity_type(), &entity, event, changed_fields)?;

                Ok(())
            }

            // ========== Deleted 事件：从数据库删除实体 ==========
            ChangeType::Deleted => {
                // 1. 检查实体是否存在
                if !self.entity_exists(&event.entity_id, O::entity_type())? {
                    // 实体不存在，幂等处理：删除不存在的实体不报错
                    return Ok(());
                }

                // 2. 从数据库删除实体
                self.delete_entity(&event.entity_id, O::entity_type())?;

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

impl MySqlDbRepo {
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
        // let query = "SELECT EXISTS(SELECT 1 FROM entities WHERE entity_id = ? AND entity_type = ?)";
        // Execute query with parameters
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
    fn insert_entity<O: Entity>(
        &mut self,
        entity_id: &str,
        entity_type: &str,
        entity: &O,
        event: &ChangeLogEntry,
    ) -> Result<(), RepoError> {
        // For mock instance, return immediately
        if self.connection.is_none() {
            return Ok(());
        }


        //todo
        entity.table_schema();

        //todo 根据table_schema和ChangeLogEntry 生成sql

        // Serialize entity to JSON
        let data = serde_json::to_value(entity)
            .map_err(|e| RepoError::SerializationFailed(e.to_string()))?
            .to_string()
            .into_bytes();

        // Extract fields from the Created event to understand the schema
        let _fields = match &event.change_type {
            ChangeType::Created { fields } => fields,
            _ => &vec![],
        };

        // TODO: Implement actual MySQL INSERT
        // The table schema is defined in the fields from the Created event
        // Insert into entities table: entity_id, entity_type, data, timestamp, sequence
        // let query = "INSERT INTO entities (entity_id, entity_type, data, timestamp, sequence)
        //              VALUES (?, ?, ?, ?, ?)";
        // Execute with parameters: (entity_id, entity_type, data, event.timestamp, event.sequence)

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
    fn load_entity<O: Entity>(&self, entity_id: &str) -> Result<O, RepoError> {
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
        /*
        let query = "SELECT data FROM entities WHERE entity_id = ? AND entity_type = ? LIMIT 1";
        let result: Option<String> = self.pool.exec_first(query, (entity_id, O::entity_type()))
            .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;

        match result {
            Some(serialized) => {
                serde_json::from_str(&serialized)
                    .map_err(|e| RepoError::DeserializationFailed(e.to_string()))
            }
            None => Err(RepoError::OrderNotFound),
        }
        */

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
    fn update_entity<O: Entity>(
        &mut self,
        entity_id: &str,
        entity_type: &str,
        entity: &O,
        event: &ChangeLogEntry,
        _changed_fields: &[diff::FieldChange],
    ) -> Result<(), RepoError> {
        // For mock instance, return immediately
        if self.connection.is_none() {
            return Ok(());
        }

        // Serialize entity to JSON
        let _data = serde_json::to_value(entity)
            .map_err(|e| RepoError::SerializationFailed(e.to_string()))?
            .to_string();

        // TODO: 实现数据库更新
        // 步骤：
        // 1. 序列化更新后的实体对象
        // 2. 执行 UPDATE 语句，更新序列化数据、时间戳和序列号
        // 3. 可选：仅更新变更的字段以减少 I/O（性能优化）
        // 4. 处理数据库错误
        //
        // 示例代码框架：
        /*
        let query = "UPDATE entities SET data = ?, timestamp = ?, sequence = ?
                     WHERE entity_id = ? AND entity_type = ?";
        self.connection.exec_drop(query, (&data, event.timestamp, event.sequence, entity_id, entity_type))
            .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;

        // 性能优化：仅更新变更的字段
        // if !changed_fields.is_empty() {
        //     // 构建更新语句，仅更新变更的字段
        //     let updates = changed_fields.iter()
        //         .map(|fc| format!("{} = ?", fc.field_name))
        //         .collect::<Vec<_>>()
        //         .join(", ");
        //     let optimized_query = format!("UPDATE entities SET {} WHERE entity_id = ? AND entity_type = ?", updates);
        //     // 执行优化的查询...
        // }
        */

        Ok(())
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

        // TODO: 实现数据库删除
        // 步骤：
        // 1. 执行 DELETE 语句
        // 2. 处理数据库错误
        // 3. 可选：记录删除日志或启用软删除
        //
        // 示例代码框架：
        /*
        let query = "DELETE FROM entities WHERE entity_id = ? AND entity_type = ?";
        self.connection.exec_drop(query, (entity_id, entity_type))
            .map_err(|e| RepoError::DeserializationFailed(e.to_string()))?;

        // 软删除（推荐用于审计）：
        // let query = "UPDATE entities SET deleted_at = ? WHERE entity_id = ? AND entity_type = ?";
        // self.connection.exec_drop(query, (std::time::SystemTime::now(), entity_id, entity_type))
        */

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mysql_adapter_creation() {
        let adapter = MySqlDbRepo::new();
        let adapter2 = MySqlDbRepo::default();
        // 验证适配器创建成功
        assert_eq!(std::mem::size_of_val(&adapter), std::mem::size_of_val(&adapter2));
    }

    #[test]
    fn test_entity_exists_initially_false() {
        let repo = MySqlDbRepo::new();

        // 验证新建的存储中实体不存在
        let exists = repo.entity_exists("test_id", "TestType").unwrap();
        assert!(!exists, "新建的存储中实体应该不存在");
    }

    #[test]
    fn test_insert_entity_duplicate_check() {
        // 这是一个集成测试，演示 insert_entity 的行为
        // 实际的完整实现需要在真实的 Entity 实现中进行

        let mut repo = MySqlDbRepo::new();

        // 创建一个最小化的 test struct
        struct DummyEntity;
        impl Entity for DummyEntity {
            fn entity_type() -> &'static str {
                "Dummy"
            }

            fn entity_id(&self) -> String {
                "test".to_string()
            }

            fn replay(&mut self, _event: &ChangeLogEntry) -> Result<(), diff::EntityError> {
                Ok(())
            }

            fn diff(&self, _other: &Self) -> Vec<diff::FieldChange> {
                vec![]
            }

            type Id = String;
        }

        let entity = DummyEntity;
        let event = ChangeLogEntry {
            entity_id: "test_id".to_string(),
            entity_type: "TestType".to_string(),
            timestamp: 1234567890,
            sequence: 1,
            change_type: ChangeType::Created { fields: vec![] },
        };

        // 第一次插入应该成功
        let result = repo.insert_entity("test_id", "TestType", &entity, &event);
        assert!(result.is_ok(), "第一次插入应该成功");

        // 验证实体已存在
        let exists = repo.entity_exists("test_id", "TestType").unwrap();
        assert!(exists, "插入后实体应该存在");

        // 第二次插入应该失败（重复）
        let result2 = repo.insert_entity("test_id", "TestType", &entity, &event);
        assert!(
            matches!(result2, Err(RepoError::OrderAlreadyExists)),
            "重复插入应该返回 OrderAlreadyExists 错误"
        );
    }

    #[test]
    fn test_insert_multiple_entities() {
        let mut repo = MySqlDbRepo::new();

        struct DummyEntity;
        impl Entity for DummyEntity {
            fn entity_type() -> &'static str {
                "Dummy"
            }

            fn entity_id(&self) -> String {
                "test".to_string()
            }

            fn replay(&mut self, _event: &ChangeLogEntry) -> Result<(), diff::EntityError> {
                Ok(())
            }

            fn diff(&self, _other: &Self) -> Vec<diff::FieldChange> {
                vec![]
            }

            type Id = String;
        }

        let entity = DummyEntity;
        let event = ChangeLogEntry {
            entity_id: "test_id".to_string(),
            entity_type: "TestType".to_string(),
            timestamp: 1234567890,
            sequence: 1,
            change_type: ChangeType::Created { fields: vec![] },
        };

        // 插入多个不同 ID 的实体
        for i in 0..3 {
            let id = format!("entity_{}", i);
            let result = repo.insert_entity(&id, "TestType", &entity, &event);
            assert!(result.is_ok(), "插入 {} 应该成功", id);

            let exists = repo.entity_exists(&id, "TestType").unwrap();
            assert!(exists, "实体 {} 应该存在", id);
        }

        // 验证所有实体都存在
        for i in 0..3 {
            let id = format!("entity_{}", i);
            let exists = repo.entity_exists(&id, "TestType").unwrap();
            assert!(exists, "实体 {} 应该仍然存在", id);
        }

        // 验证不存在的实体
        let exists = repo.entity_exists("entity_999", "TestType").unwrap();
        assert!(!exists, "不存在的实体应该返回 false");
    }
}
