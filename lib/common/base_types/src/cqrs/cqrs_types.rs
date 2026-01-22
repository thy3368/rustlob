use crate::UserId;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CMetadata {
    /// 命令唯一ID（用于幂等性和追踪）
    #[cfg_attr(feature = "serde", serde(default))]
    pub command_id: String,
    /// 命令创建时间戳（Unix 毫秒）
    #[cfg_attr(feature = "serde", serde(default))]
    pub timestamp: u64,
    /// 关联ID（用于分布式追踪）
    #[cfg_attr(feature = "serde", serde(default))]
    pub correlation_id: Option<String>,
    /// 因果ID（用于事件溯源）
    #[cfg_attr(feature = "serde", serde(default))]
    pub causation_id: Option<String>,
    /// 用户/系统标识
    #[cfg_attr(feature = "serde", serde(default))]
    pub actor: Option<String>,
    /// 自定义属性
    #[cfg_attr(feature = "serde", serde(default))]
    pub attributes: Vec<(String, String)>,
}

/// 带元数据的命令响应
///
/// 包含执行结果和幂等性/追踪信息
/// 使用 Result<CommandResponse<T>, CommandError> 的方式返回
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CmdResp<T> {
    /// 命令元数据
    pub metadata: ResMetadata,
    /// 成功结果
    pub result: T,
}

impl<T> CmdResp<T> {
    /// 创建新响应
    #[inline]
    pub fn new(nonce: Nonce, result: T) -> Self {
        Self { metadata: ResMetadata::new(nonce), result }
    }

    /// 创建重复命令的响应
    #[inline]
    pub fn duplicate(nonce: Nonce, result: T) -> Self {
        Self { metadata: ResMetadata::new(nonce).mark_duplicate(), result }
    }

    /// 映射结果值
    #[inline]
    pub fn map<U, F>(self, f: F) -> CmdResp<U>
    where
        F: FnOnce(T) -> U,
    {
        CmdResp { metadata: self.metadata, result: f(self.result) }
    }
}

/// 命令执行元数据
///
/// 包含幂等性和追踪信息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResMetadata {
    /// 命令唯一标识（客户端生成）
    pub nonce: Nonce,
    /// 是否为重复命令（幂等命中）
    pub is_duplicate: bool,
    /// 命令接收时间戳
    pub received_at: u64,
}

impl ResMetadata {
    /// 创建新命令元数据
    #[inline]
    pub fn new(nonce: Nonce) -> Self {
        Self {
            nonce,
            is_duplicate: false,
            received_at: 0, // 由处理器设置
        }
    }

    /// 标记为重复命令
    #[inline]
    pub fn mark_duplicate(mut self) -> Self {
        self.is_duplicate = true;
        self
    }
}

/// Nonce 类型 - 客户端生成的唯一标识
pub type Nonce = u64;

/// 幂等命令包装
///
/// 所有命令通过此结构包装，实现幂等性检查
#[derive(Debug, Clone)]
pub struct Cmd<C> {
    /// 角色
    pub user_id: UserId,
    /// 客户端生成的唯一标识（同一 nonce 只处理一次）
    pub nonce: Nonce,
    /// 命令时间戳（Unix毫秒，用于过期检查）
    pub timestamp_ms: u64,
    /// 实际命令内容
    pub payload: C,
}

impl<C> Cmd<C> {
    /// 创建新命令
    pub fn new(user_id: UserId, nonce: Nonce, payload: C) -> Self {
        Self {
            user_id,
            nonce,
            timestamp_ms: 0, // 由调用方设置
            payload,
        }
    }

    /// 创建带时间戳的命令
    pub fn with_timestamp(user_id: UserId, nonce: Nonce, timestamp_ms: u64, payload: C) -> Self {
        Self { user_id, nonce, timestamp_ms, payload }
    }
}
