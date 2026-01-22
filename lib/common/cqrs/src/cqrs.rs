//! CQRS (Command Query Responsibility Segregation) 统一抽象框架
//!
//! ## 设计理念
//!
//! 所有应用服务遵守统一的 Command/Query/Result 模式：
//! - **Command<T>**: 命令包装器，包含业务命令 + 元数据
//! - **CommandResult<T>**: 命令结果包装器，包含结果数据 + 元数据
//! - **Query<T>**: 查询包装器，包含查询参数 + 元数据
//! - **QueryResult<T>**: 查询结果包装器，包含查询数据 + 元数据
//!
//! ## 核心优势
//!
//! 1. **统一接口**: 所有服务使用相同的模式
//! 2. **可序列化**: 支持持久化、网络传输、事件溯源
//! 3. **可追溯**: 内置元数据（ID、时间戳、追踪信息）
//! 4. **类型安全**: 编译时检查
//! 5. **易扩展**: 添加中间件（日志、监控、权限）
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! // 1. 定义业务命令
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct PlaceOrder {
//!     pub trader_id: TraderId,
//!     pub side: Side,
//!     pub price: Price,
//!     pub quantity: Quantity,
//! }
//!
//! // 2. 定义命令结果
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct PlaceOrderResult {
//!     pub order_id: OrderId,
//!     pub status: OrderStatus,
//! }
//!
//! // 3. 使用命令
//! let cmd = Command::new(PlaceOrder {
//!     trader_id: "TRADER1".into(),
//!     side: Side::Buy,
//!     price: 10000,
//!     quantity: 100,
//! });
//!
//! // 4. 处理命令
//! let result: CommandResult<PlaceOrderResult> = handler.execute(cmd).await?;
//! println!("Order placed: {:?}", result.data.order_id);
//! ```

use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH}
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


// ==================== 命令（Command）====================

/// 命令包装器
///
/// 封装业务命令及其元数据，支持：
/// - 命令追踪（ID、时间戳）
/// - 审计日志
/// - 命令重放
/// - 事件溯源
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cmd<T> {
    /// 业务命令负载
    pub payload: T,
    /// 命令元数据
    pub metadata: CmdMetadata
}

impl<T> Cmd<T> {
    /// 创建新命令（自动生成元数据）
    pub fn new(payload: T) -> Self {
        Self {
            payload,
            metadata: CmdMetadata::new()
        }
    }

    /// 创建带自定义元数据的命令
    pub fn with_metadata(payload: T, metadata: CmdMetadata) -> Self {
        Self {
            payload,
            metadata
        }
    }

    /// 映射命令负载
    pub fn map<U, F>(self, f: F) -> Cmd<U>
    where
        F: FnOnce(T) -> U
    {
        Cmd {
            payload: f(self.payload),
            metadata: self.metadata
        }
    }
}

/// 命令元数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CmdMetadata {
    /// 命令唯一ID（用于幂等性和追踪）
    pub command_id: String,
    /// 命令创建时间戳（Unix 毫秒）
    pub timestamp: u64,
    /// 关联ID（用于分布式追踪）
    pub correlation_id: Option<String>,
    /// 因果ID（用于事件溯源）
    pub causation_id: Option<String>,
    /// 用户/系统标识
    pub actor: Option<String>,
    /// 自定义属性
    pub attributes: Vec<(String, String)>
}

impl CmdMetadata {
    /// 创建新的命令元数据
    pub fn new() -> Self {
        Self {
            command_id: Self::generate_id(),
            timestamp: Self::current_timestamp(),
            correlation_id: None,
            causation_id: None,
            actor: None,
            attributes: Vec::new()
        }
    }

    /// 生成唯一ID（简单实现，生产环境应使用 UUID）
    fn generate_id() -> String { format!("cmd_{}_{:x}", Self::current_timestamp(), rand::random::<u32>()) }

    /// 获取当前时间戳（Unix 毫秒）
    fn current_timestamp() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 }

    /// 设置关联ID
    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// 设置执行者
    pub fn with_actor(mut self, actor: String) -> Self {
        self.actor = Some(actor);
        self
    }

    /// 添加自定义属性
    pub fn add_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.push((key, value));
        self
    }
}

impl Default for CmdMetadata {
    fn default() -> Self { Self::new() }
}

/// 命令结果包装器
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CmdResult<T> {
    /// 结果数据
    pub data: T,
    /// 结果元数据
    pub metadata: ResultMetadata
}

impl<T> CmdResult<T> {
    /// 创建成功结果
    pub fn success(data: T) -> Self {
        Self {
            data,
            metadata: ResultMetadata::success()
        }
    }

    /// 创建带自定义元数据的结果
    pub fn with_metadata(data: T, metadata: ResultMetadata) -> Self {
        Self {
            data,
            metadata
        }
    }

    /// 映射结果数据
    pub fn map<U, F>(self, f: F) -> CmdResult<U>
    where
        F: FnOnce(T) -> U
    {
        CmdResult {
            data: f(self.data),
            metadata: self.metadata
        }
    }

    /// 判断是否成功
    pub fn is_success(&self) -> bool { self.metadata.success }
}

// ==================== 查询（Query）====================

/// 查询包装器
///
/// 封装业务查询及其元数据，支持：
/// - 查询追踪
/// - 缓存策略
/// - 性能监控
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Query<T> {
    /// 查询负载
    pub payload: T,
    /// 查询元数据
    pub metadata: QueryMetadata
}

impl<T> Query<T> {
    /// 创建新查询
    pub fn new(payload: T) -> Self {
        Self {
            payload,
            metadata: QueryMetadata::new()
        }
    }

    /// 创建带自定义元数据的查询
    pub fn with_metadata(payload: T, metadata: QueryMetadata) -> Self {
        Self {
            payload,
            metadata
        }
    }

    /// 映射查询负载
    pub fn map<U, F>(self, f: F) -> Query<U>
    where
        F: FnOnce(T) -> U
    {
        Query {
            payload: f(self.payload),
            metadata: self.metadata
        }
    }
}

/// 查询元数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMetadata {
    /// 查询唯一ID
    pub query_id: String,
    /// 查询时间戳
    pub timestamp: u64,
    /// 关联ID
    pub correlation_id: Option<String>,
    /// 查询者标识
    pub actor: Option<String>,
    /// 是否启用缓存
    pub cache_enabled: bool,
    /// 缓存TTL（秒）
    pub cache_ttl: Option<u64>,
    /// 自定义属性
    pub attributes: Vec<(String, String)>
}

impl QueryMetadata {
    /// 创建新的查询元数据
    pub fn new() -> Self {
        Self {
            query_id: Self::generate_id(),
            timestamp: Self::current_timestamp(),
            correlation_id: None,
            actor: None,
            cache_enabled: false,
            cache_ttl: None,
            attributes: Vec::new()
        }
    }

    /// 生成唯一ID
    fn generate_id() -> String { format!("qry_{}_{:x}", Self::current_timestamp(), rand::random::<u32>()) }

    /// 获取当前时间戳
    fn current_timestamp() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 }

    /// 启用缓存
    pub fn with_cache(mut self, ttl_seconds: u64) -> Self {
        self.cache_enabled = true;
        self.cache_ttl = Some(ttl_seconds);
        self
    }

    /// 设置执行者
    pub fn with_actor(mut self, actor: String) -> Self {
        self.actor = Some(actor);
        self
    }
}

impl Default for QueryMetadata {
    fn default() -> Self { Self::new() }
}

/// 查询结果包装器
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryResult<T> {
    /// 查询数据
    pub data: T,
    /// 结果元数据
    pub metadata: ResultMetadata
}

impl<T> QueryResult<T> {
    /// 创建成功结果
    pub fn success(data: T) -> Self {
        Self {
            data,
            metadata: ResultMetadata::success()
        }
    }

    /// 创建带自定义元数据的结果
    pub fn with_metadata(data: T, metadata: ResultMetadata) -> Self {
        Self {
            data,
            metadata
        }
    }

    /// 映射结果数据
    pub fn map<U, F>(self, f: F) -> QueryResult<U>
    where
        F: FnOnce(T) -> U
    {
        QueryResult {
            data: f(self.data),
            metadata: self.metadata
        }
    }

    /// 判断是否成功
    pub fn is_success(&self) -> bool { self.metadata.success }

    /// 判断是否来自缓存
    pub fn is_from_cache(&self) -> bool { self.metadata.from_cache }
}

// ==================== 结果元数据（共享）====================

/// 结果元数据
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResultMetadata {
    /// 执行是否成功
    pub success: bool,
    /// 执行时间戳
    pub timestamp: u64,
    /// 执行耗时（微秒）
    pub duration_micros: Option<u64>,
    /// 是否来自缓存
    pub from_cache: bool,
    /// 警告信息
    pub warnings: Vec<String>,
    /// 自定义属性
    pub attributes: Vec<(String, String)>
}

impl ResultMetadata {
    /// 创建成功的元数据
    pub fn success() -> Self {
        Self {
            success: true,
            timestamp: Self::current_timestamp(),
            duration_micros: None,
            from_cache: false,
            warnings: Vec::new(),
            attributes: Vec::new()
        }
    }

    /// 创建失败的元数据
    pub fn failure() -> Self {
        Self {
            success: false,
            timestamp: Self::current_timestamp(),
            duration_micros: None,
            from_cache: false,
            warnings: Vec::new(),
            attributes: Vec::new()
        }
    }

    /// 获取当前时间戳
    fn current_timestamp() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 }

    /// 设置执行耗时
    pub fn with_duration(mut self, duration_micros: u64) -> Self {
        self.duration_micros = Some(duration_micros);
        self
    }

    /// 标记为缓存结果
    pub fn from_cache(mut self) -> Self {
        self.from_cache = true;
        self
    }

    /// 添加警告
    pub fn add_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }
}

// ==================== 错误处理====================

/// CQRS 错误类型
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CqrsError {
    /// 命令执行错误
    CommandError { code: String, message: String, details: Option<String> },
    /// 查询执行错误
    QueryError { code: String, message: String, details: Option<String> },
    /// 验证错误
    ValidationError { field: String, message: String },
    /// 未找到
    NotFound { entity: String, id: String },
    /// 冲突（如幂等性检查失败）
    Conflict { message: String },
    /// 未授权
    Unauthorized { message: String },
    /// 内部错误
    InternalError { message: String }
}

impl fmt::Display for CqrsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CqrsError::CommandError {
                code,
                message,
                ..
            } => {
                write!(f, "Command error [{}]: {}", code, message)
            }
            CqrsError::QueryError {
                code,
                message,
                ..
            } => {
                write!(f, "Query error [{}]: {}", code, message)
            }
            CqrsError::ValidationError {
                field,
                message
            } => {
                write!(f, "Validation error on {}: {}", field, message)
            }
            CqrsError::NotFound {
                entity,
                id
            } => {
                write!(f, "{} not found: {}", entity, id)
            }
            CqrsError::Conflict {
                message
            } => {
                write!(f, "Conflict: {}", message)
            }
            CqrsError::Unauthorized {
                message
            } => {
                write!(f, "Unauthorized: {}", message)
            }
            CqrsError::InternalError {
                message
            } => {
                write!(f, "Internal error: {}", message)
            }
        }
    }
}

impl std::error::Error for CqrsError {}

// ==================== Handler Trait ====================

/// 命令处理器 trait
///
/// 所有命令处理器必须实现此 trait
#[async_trait::async_trait]
pub trait CommandHandler<T>: Send + Sync {
    /// 命令结果类型
    type Result;

    /// 执行命令
    async fn handle(&self, command: Cmd<T>) -> Result<CmdResult<Self::Result>, CqrsError>;
}

/// 查询处理器 trait
///
/// 所有查询处理器必须实现此 trait
#[async_trait::async_trait]
pub trait QueryHandler<T>: Send + Sync {
    /// 查询结果类型
    type Result;

    /// 执行查询
    async fn handle(&self, query: Query<T>) -> Result<QueryResult<Self::Result>, CqrsError>;
}

// ==================== 便利宏 ====================

/// 定义命令的便利宏
///
/// ```rust,ignore
/// define_command! {
///     PlaceOrder {
///         trader_id: TraderId,
///         side: Side,
///         price: Price,
///         quantity: Quantity,
///     } -> PlaceOrderResult {
///         order_id: OrderId,
///         status: OrderStatus,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_command {
    (
        $cmd_name:ident {
            $($field:ident: $ty:ty),* $(,)?
        } -> $result_name:ident {
            $($result_field:ident: $result_ty:ty),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $cmd_name {
            $(pub $field: $ty),*
        }

        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $result_name {
            $(pub $result_field: $result_ty),*
        }
    };
}

/// 定义查询的便利宏
#[macro_export]
macro_rules! define_query {
    (
        $query_name:ident {
            $($field:ident: $ty:ty),* $(,)?
        } -> $result_name:ident {
            $($result_field:ident: $result_ty:ty),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $query_name {
            $(pub $field: $ty),*
        }

        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $result_name {
            $(pub $result_field: $result_ty),*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Cmd::new(42);
        assert_eq!(cmd.payload, 42);
        assert!(cmd.metadata.command_id.starts_with("cmd_"));
    }

    #[test]
    fn test_query_creation() {
        let query = Query::new("test");
        assert_eq!(query.payload, "test");
        assert!(query.metadata.query_id.starts_with("qry_"));
    }

    #[test]
    fn test_command_result() {
        let result = CmdResult::success("ok");
        assert!(result.is_success());
        assert_eq!(result.data, "ok");
    }

    #[test]
    fn test_query_result() {
        let result = QueryResult::success(vec![1, 2, 3]);
        assert!(result.is_success());
        assert_eq!(result.data.len(), 3);
    }
}
