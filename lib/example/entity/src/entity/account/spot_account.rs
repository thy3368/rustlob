use common_entity::{
    AggregateRole, Entity, EntityFieldChange, FieldDiff, FourColorArchetype, ObjectType, function,
};
use serde::{Deserialize, Serialize};

const ACCOUNT_ENTITY_TYPE: u8 = 1;

/// 账户主档生命周期状态。
///
/// `Frozen` 明确表示账户被风控冻结；`Closed` 是预留终态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountStatus {
    /// 账户可正常交易和资金操作。
    Active,
    /// 账户被风控冻结，禁止交易和资金操作。
    Frozen,
    /// 账户已关闭。
    Closed,
}

impl AccountStatus {
    /// 返回稳定的小写状态标签，用于 replay 字段序列化。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Frozen => "frozen",
            Self::Closed => "closed",
        }
    }
}

/// 交易账户主档。
///
/// `Account` 属于 `Party/Place/Thing`，表达用户与账户生命周期，不承载某个币种的余额。
#[ObjectType(four_color = "party_place_thing", aggregate_role = "aggregate_root")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotAccount {
    /// 账户 ID。
    pub account_id: u64,
    /// 用户 ID 或业务主体 ID。
    pub party_id: u64,
    /// 账户生命周期状态。
    pub status: AccountStatus,
    // /// 订单创建时间，单位为 Unix 纳秒。
    // pub created_at: u64,
    // /// 订单最后更新时间，单位为 Unix 纳秒。
    // pub updated_at: u64,
    /// 当前账户实体版本。
    pub version: u64,
}

impl SpotAccount {
    /// 从已经校验过的业务事实或回放事件装配账户主档。
    ///
    /// 该构造器只装配事实，不承担业务校验；风控冻结/解冻应通过业务方法完成。
    pub fn new(account_id: u64, party_id: u64, status: AccountStatus, version: u64) -> Self {
        Self { account_id, party_id, status, version }
    }

    /// 返回账户当前是否处于可交易、可资金操作的激活状态。
    #[function(kind = "business_query")]
    pub fn is_active(&self) -> bool {
        self.status == AccountStatus::Active
    }

    /// 返回账户当前是否处于风控冻结状态。
    #[function(kind = "business_query")]
    pub fn is_frozen(&self) -> bool {
        self.status == AccountStatus::Frozen
    }

    /// 返回账户当前是否已经关闭。
    #[function(kind = "business_query")]
    pub fn is_closed(&self) -> bool {
        self.status == AccountStatus::Closed
    }

    /// 返回账户当前是否允许交易和资金操作。
    #[function(kind = "business_query")]
    pub fn allows_trading_and_funding(&self) -> bool {
        self.is_active()
    }

    /// 对激活账户执行风控冻结。
    ///
    /// 仅 `Active` 账户允许转为 `Frozen`；成功后返回版本 `+1` 的新状态。
    #[function(kind = "behavior")]
    pub fn freeze_for_risk(&self) -> Option<Self> {
        if !self.is_active() {
            return None;
        }

        Some(Self::new(
            self.account_id,
            self.party_id,
            AccountStatus::Frozen,
            self.version.checked_add(1).unwrap_or(self.version),
        ))
    }

    /// 将已冻结账户从风控冻结中解冻。
    ///
    /// 仅 `Frozen` 账户允许恢复为 `Active`；成功后返回版本 `+1` 的新状态。
    #[function(kind = "behavior")]
    pub fn unfreeze_from_risk(&self) -> Option<Self> {
        if !self.is_frozen() {
            return None;
        }

        Some(Self::new(
            self.account_id,
            self.party_id,
            AccountStatus::Active,
            self.version.checked_add(1).unwrap_or(self.version),
        ))
    }
}

impl FieldDiff for SpotAccount {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("account_id", "", self.account_id.to_string()),
            EntityFieldChange::new("party_id", "", self.party_id.to_string()),
            EntityFieldChange::new("status", "", self.status.as_str()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::with_capacity(0);
        if self.party_id != other.party_id {
            changes.push(EntityFieldChange::new(
                "party_id",
                self.party_id.to_string(),
                other.party_id.to_string(),
            ));
        }
        if self.status != other.status {
            changes.push(EntityFieldChange::new(
                "status",
                self.status.as_str(),
                other.status.as_str(),
            ));
        }
        changes
    }
}

impl common_entity::Shard for SpotAccount {
    fn partition(&self) -> u8 {
        (self.party_id % 256) as u8
    }
}

impl Entity for SpotAccount {
    fn entity_id(&self) -> u64 {
        self.account_id
    }

    fn is_derived() -> bool {
        false
    }

    fn entity_type() -> u8 {
        ACCOUNT_ENTITY_TYPE
    }

    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::PartyPlaceThing
    }

    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::AggregateRoot
    }

    fn lsn(&self) -> u64 {
        self.version
    }
    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "account_id" | "party_id" => 1,
            "status" => 0,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{AggregateRole, FourColorArchetype};

    use super::*;

    fn sample_account(status: AccountStatus, version: u64) -> SpotAccount {
        SpotAccount::new(1001, 513, status, version)
    }

    #[test]
    fn constructor_preserves_status_and_queries() {
        let active = sample_account(AccountStatus::Active, 3);
        let frozen = sample_account(AccountStatus::Frozen, 4);
        let closed = sample_account(AccountStatus::Closed, 5);

        assert_eq!(active.status, AccountStatus::Active);
        assert!(active.is_active());
        assert!(!active.is_frozen());
        assert!(!active.is_closed());
        assert!(active.allows_trading_and_funding());

        assert_eq!(frozen.status, AccountStatus::Frozen);
        assert!(!frozen.is_active());
        assert!(frozen.is_frozen());
        assert!(!frozen.is_closed());
        assert!(!frozen.allows_trading_and_funding());

        assert_eq!(closed.status, AccountStatus::Closed);
        assert!(!closed.is_active());
        assert!(!closed.is_frozen());
        assert!(closed.is_closed());
        assert!(!closed.allows_trading_and_funding());

        assert_eq!(SpotAccount::four_color_archetype(), FourColorArchetype::PartyPlaceThing);
        assert_eq!(SpotAccount::aggregate_role(), AggregateRole::AggregateRoot);
    }

    #[test]
    fn freeze_for_risk_only_transitions_active_account() {
        let active = sample_account(AccountStatus::Active, 7);
        let frozen = sample_account(AccountStatus::Frozen, 9);
        let closed = sample_account(AccountStatus::Closed, 11);

        let next = active.freeze_for_risk().expect("active account should freeze");
        assert_eq!(next.status, AccountStatus::Frozen);
        assert_eq!(next.version, 8);
        assert_eq!(next.account_id, active.account_id);
        assert_eq!(next.party_id, active.party_id);

        assert_eq!(frozen.freeze_for_risk(), None);
        assert_eq!(closed.freeze_for_risk(), None);
    }

    #[test]
    fn unfreeze_from_risk_only_transitions_frozen_account() {
        let active = sample_account(AccountStatus::Active, 7);
        let frozen = sample_account(AccountStatus::Frozen, 9);
        let closed = sample_account(AccountStatus::Closed, 11);

        let next = frozen.unfreeze_from_risk().expect("frozen account should unfreeze");
        assert_eq!(next.status, AccountStatus::Active);
        assert_eq!(next.version, 10);
        assert_eq!(next.account_id, frozen.account_id);
        assert_eq!(next.party_id, frozen.party_id);

        assert_eq!(active.unfreeze_from_risk(), None);
        assert_eq!(closed.unfreeze_from_risk(), None);
    }

    #[test]
    fn entity_replay_uses_status_field() {
        let active = sample_account(AccountStatus::Active, 1);
        let frozen = sample_account(AccountStatus::Frozen, 2);
        let closed = sample_account(AccountStatus::Closed, 3);

        assert_eq!(
            active.created_field_changes(),
            vec![
                EntityFieldChange::new("account_id", "", "1001"),
                EntityFieldChange::new("party_id", "", "513"),
                EntityFieldChange::new("status", "", "active"),
            ]
        );
        assert_eq!(
            frozen.created_field_changes(),
            vec![
                EntityFieldChange::new("account_id", "", "1001"),
                EntityFieldChange::new("party_id", "", "513"),
                EntityFieldChange::new("status", "", "frozen"),
            ]
        );
        assert_eq!(
            closed.created_field_changes(),
            vec![
                EntityFieldChange::new("account_id", "", "1001"),
                EntityFieldChange::new("party_id", "", "513"),
                EntityFieldChange::new("status", "", "closed"),
            ]
        );

        assert_eq!(
            active.diff(&frozen),
            vec![EntityFieldChange::new("status", "active", "frozen")]
        );
        assert_eq!(SpotAccount::replay_field_type("status"), 0);
        assert_eq!(SpotAccount::replay_field_type("account_id"), 1);
        assert_eq!(SpotAccount::replay_field_type("party_id"), 1);
        assert_eq!(
            active.diff(&SpotAccount::new(1001, 514, AccountStatus::Active, 2)),
            vec![EntityFieldChange::new("party_id", "513", "514")]
        );
    }

    #[test]
    fn entity_id_and_shard_follow_numeric_ids() {
        use common_entity::Shard;

        let account = sample_account(AccountStatus::Active, 1);
        assert_eq!(account.account_id, 1001);
        assert_eq!(account.party_id, 513);
        assert_eq!(account.entity_id(), 1001);
        assert_eq!(account.topic(), 0);
        assert_eq!(account.partition(), 1);
        assert_eq!(SpotAccount::new(9999, 255, AccountStatus::Active, 1).partition(), 255);
    }
}
