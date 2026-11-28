use crate::{PartyId, Timestamp};

/// 交易账户
#[derive(Debug, Clone)]
#[repr(align(64))]
pub struct User {
    /// 所属用户ID
    pub party_id: PartyId,

    // /// 账户状态
    // pub status: AccountStatus,
    /// 创建时间
    pub created_at: Timestamp,
    /// 更新时间
    pub updated_at: Timestamp,
}
