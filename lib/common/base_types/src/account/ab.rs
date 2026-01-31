use immutable_derive::immutable;

use crate::{
    account::account::{AccountStatus, AccountType},
    AccountId, Timestamp, UserId,
};

/// 账户表实体 - 不可变值对象
///
/// # 设计原则
/// - 缓存行对齐(64字节)以优化并发访问性能
/// - 使用 #[immutable] 宏自动生成 const getter 方法
/// - 符合 Clean Architecture 的实体层设计
/// - 所有字段私有，通过 getter 访问保证不可变性
#[repr(align(64))]
#[immutable]
pub struct AccountTable {
    /// 账户ID
    id: AccountId,
    /// 所属用户ID
    user_id: UserId,
    /// 账户类型
    account_type: AccountType,
    /// 账户状态
    status: AccountStatus,
    /// 创建时间
    created_at: Timestamp,
    /// 更新时间
    updated_at: Timestamp,
}

impl AccountTable {
    // `pub const fn new` 由 #[immutable] 宏自动生成

    // ❌ 此方法违反不可变性约束
    // immutable_derive 宏应该检测到此错误并在编译时报错
    pub fn set_id(&mut self) {
        self.id = AccountId(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_getters() {
        let now: Timestamp = 1234567890;
        let account = AccountTable {
            id: AccountId(1),
            user_id: UserId(100),
            account_type: AccountType::Spot,
            status: AccountStatus::Active,
            created_at: now,
            updated_at: now,
        };

        // 测试自动生成的 getter 方法
        assert_eq!(account.id(), &AccountId(1));
        assert_eq!(account.user_id(), &UserId(100));
        assert_eq!(account.account_type(), &AccountType::Spot);
        assert_eq!(account.status(), &AccountStatus::Active);
        assert_eq!(account.created_at(), &now);
        assert_eq!(account.updated_at(), &now);
    }

    #[test]
    fn test_new_constructor() {
        let now: Timestamp = 1234567890;
        let account = AccountTable::new(
            AccountId(1),
            UserId(100),
            AccountType::Spot,
            AccountStatus::Active,
            now,
            now,
        );

        assert_eq!(account.id(), &AccountId(1));
        assert_eq!(account.user_id(), &UserId(100));
        assert_eq!(account.created_at(), &now);
    }

    #[test]
    fn test_immutability() {
        let now: Timestamp = 1234567890;
        let account = AccountTable::new(
            AccountId(1),
            UserId(100),
            AccountType::Spot,
            AccountStatus::Active,
            now,
            now,
        );

        // 通过 getter 可以访问字段
        let _id = account.id();
        let _status = account.status();

        // ❌ 下面的代码应该无法编译（字段是私有的）
        // account.status = AccountStatus::Suspended;  // 编译错误
        // account.id = AccountId(2);                  // 编译错误
    }
}
