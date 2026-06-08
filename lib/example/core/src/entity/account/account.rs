use common_entity::{Entity, EntityError, EntityFieldChange};

const ACCOUNT_ENTITY_TYPE: u8 = 1;

/// 交易账户主档。
///
/// `Account` 只表达用户与账户生命周期，不承载某个币种的余额。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    /// 账户 ID。
    pub account_id: String,
    /// 用户 ID 或业务主体 ID。
    pub user_id: String,
    /// 账户是否允许交易和资金操作。
    pub active: bool,
    /// 当前账户实体版本。
    pub version: u64,
}

impl Account {
    /// 从已经校验过的业务事实或回放事件构造账户主档。
    pub fn new(account_id: String, user_id: String, active: bool, version: u64) -> Self {
        Self { account_id, user_id, active, version }
    }
}

impl Entity for Account {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.account_id.clone()
    }

    fn entity_type() -> u8 {
        ACCOUNT_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        self.version
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("user_id", "", self.user_id.clone()),
            EntityFieldChange::new("active", "", self.active.to_string()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        if self.user_id != other.user_id {
            changes.push(EntityFieldChange::new("user_id", &self.user_id, &other.user_id));
        }
        if self.active != other.active {
            changes.push(EntityFieldChange::new(
                "active",
                self.active.to_string(),
                other.active.to_string(),
            ));
        }
        changes
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "account_id" | "user_id" | "active" => 0,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_account_entity_id(&self.account_id))
    }
}

fn stable_account_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}
