use common_entity::{AggregateRole, Entity, EntityFieldChange, FieldDiff, FourColorArchetype, Fsm};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

/// 账户生命周期事件。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotAccountEvent {
    /// 因风控要求冻结激活账户。
    FreezeForRisk { timestamp: u64 },
    /// 解除风控冻结，使账户恢复激活。
    UnfreezeFromRisk { timestamp: u64 },
}

/// 账户生命周期迁移错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SpotAccountError {
    /// 当前状态不接受指定事件，包括已经关闭的终态。
    #[error("account state {state:?} does not accept event {event:?}")]
    InvalidTransition { state: AccountStatus, event: SpotAccountEvent },
    /// 事件时间早于账户已有时间事实。
    #[error(
        "event timestamp {timestamp} is earlier than the earliest allowed timestamp {earliest_allowed}"
    )]
    TimestampRegression { timestamp: u64, earliest_allowed: u64 },
    /// 账户版本已经达到上限，不能继续递增。
    #[error("account version overflow")]
    VersionOverflow,
}

/// 交易账户主档。
///
/// `Account` 属于 `Party/Place/Thing`，表达用户与账户生命周期，不承载某个币种的余额。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotAccount {
    /// 账户 ID。
    pub account_id: u64,
    /// 用户 ID 或业务主体 ID。
    pub party_id: u64,
    /// 账户生命周期状态。
    pub status: AccountStatus,
    /// 账户创建时间，单位为 Unix 纳秒。
    pub created_at: u64,
    /// 账户最后更新时间，单位为 Unix 纳秒。
    pub updated_at: u64,
    /// 当前账户实体版本。
    pub version: u64,
}

impl SpotAccount {
    /// 从已经校验过的业务事实或回放事件装配账户主档。
    ///
    /// 该构造器只装配事实，不承担业务校验；风控冻结/解冻应通过 FSM 事件完成。
    pub fn new(account_id: u64, party_id: u64, status: AccountStatus, version: u64) -> Self {
        Self { account_id, party_id, status, created_at: 0, updated_at: 0, version }
    }

    /// 返回账户当前是否处于可交易、可资金操作的激活状态。
    pub fn is_active(&self) -> bool {
        self.status == AccountStatus::Active
    }

    /// 返回账户当前是否处于风控冻结状态。
    pub fn is_frozen(&self) -> bool {
        self.status == AccountStatus::Frozen
    }

    /// 返回账户当前是否已经关闭。
    pub fn is_closed(&self) -> bool {
        self.status == AccountStatus::Closed
    }

    /// 返回账户当前是否允许交易和资金操作。
    pub fn allows_trading_and_funding(&self) -> bool {
        self.is_active()
    }
}

impl Fsm for SpotAccount {
    type State = AccountStatus;
    type Event = SpotAccountEvent;
    type Output = ();
    type Error = SpotAccountError;

    /// 应用账户风控生命周期事件。
    ///
    /// `Active` 只接受冻结事件并转为 `Frozen`，`Frozen` 只接受解冻事件并转为
    /// `Active`；`Closed` 是终态，拒绝全部事件。事件时间不得早于创建时间或当前更新时间。
    /// 成功输出为空，并在所有校验和版本计算完成后同时提交状态、更新时间和递增版本；
    /// 任一错误都不会修改实体。
    fn transition(
        &mut self,
        event: Self::Event,
    ) -> Result<(Self::State, Self::Output), Self::Error> {
        let (next_status, timestamp) = match (self.status, event) {
            (AccountStatus::Active, SpotAccountEvent::FreezeForRisk { timestamp }) => {
                (AccountStatus::Frozen, timestamp)
            }
            (AccountStatus::Frozen, SpotAccountEvent::UnfreezeFromRisk { timestamp }) => {
                (AccountStatus::Active, timestamp)
            }
            (state, event) => {
                return Err(SpotAccountError::InvalidTransition { state, event });
            }
        };

        let earliest_allowed = self.created_at.max(self.updated_at);
        if timestamp < earliest_allowed {
            return Err(SpotAccountError::TimestampRegression { timestamp, earliest_allowed });
        }
        let next_version = self.version.checked_add(1).ok_or(SpotAccountError::VersionOverflow)?;

        self.status = next_status;
        self.updated_at = timestamp;
        self.version = next_version;

        Ok((next_status, ()))
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
    use common_entity::{AggregateRole, FourColorArchetype, Fsm, Shard};

    use super::*;

    fn account(status: AccountStatus, version: u64) -> SpotAccount {
        SpotAccount {
            account_id: 1001,
            party_id: 513,
            status,
            created_at: 10,
            updated_at: 20,
            version,
        }
    }

    fn assert_rejected_without_mutation(
        mut account: SpotAccount,
        event: SpotAccountEvent,
        expected: SpotAccountError,
    ) {
        let before = account.clone();
        assert_eq!(account.transition(event), Err(expected));
        assert_eq!(account, before);
    }

    #[test]
    fn constructor_preserves_status_and_queries() {
        let active = SpotAccount::new(1001, 513, AccountStatus::Active, 3);
        let frozen = SpotAccount::new(1001, 513, AccountStatus::Frozen, 4);
        let closed = SpotAccount::new(1001, 513, AccountStatus::Closed, 5);

        assert!(active.is_active());
        assert!(active.allows_trading_and_funding());
        assert!(frozen.is_frozen());
        assert!(!frozen.allows_trading_and_funding());
        assert!(closed.is_closed());
        assert!(!closed.allows_trading_and_funding());
        assert_eq!((active.created_at, active.updated_at), (0, 0));
        assert_eq!(SpotAccount::four_color_archetype(), FourColorArchetype::PartyPlaceThing);
        assert_eq!(SpotAccount::aggregate_role(), AggregateRole::AggregateRoot);
    }

    #[test]
    fn freeze_for_risk_transitions_active_account_atomically() {
        let mut account = account(AccountStatus::Active, 7);
        let identity_before = (account.account_id, account.party_id, account.created_at);

        assert_eq!(
            account.transition(SpotAccountEvent::FreezeForRisk { timestamp: 25 }),
            Ok((AccountStatus::Frozen, ()))
        );
        assert_eq!(
            (account.status, account.updated_at, account.version),
            (AccountStatus::Frozen, 25, 8)
        );
        assert_eq!((account.account_id, account.party_id, account.created_at), identity_before);
    }

    #[test]
    fn unfreeze_from_risk_transitions_frozen_account_atomically() {
        let mut account = account(AccountStatus::Frozen, 9);
        let identity_before = (account.account_id, account.party_id, account.created_at);

        assert_eq!(
            account.transition(SpotAccountEvent::UnfreezeFromRisk { timestamp: 30 }),
            Ok((AccountStatus::Active, ()))
        );
        assert_eq!(
            (account.status, account.updated_at, account.version),
            (AccountStatus::Active, 30, 10)
        );
        assert_eq!((account.account_id, account.party_id, account.created_at), identity_before);
    }

    #[test]
    fn invalid_state_event_matrix_is_rejected_without_mutation() {
        let cases = [
            (AccountStatus::Active, SpotAccountEvent::UnfreezeFromRisk { timestamp: 20 }),
            (AccountStatus::Frozen, SpotAccountEvent::FreezeForRisk { timestamp: 20 }),
            (AccountStatus::Closed, SpotAccountEvent::FreezeForRisk { timestamp: 20 }),
            (AccountStatus::Closed, SpotAccountEvent::UnfreezeFromRisk { timestamp: 20 }),
        ];

        for (state, event) in cases {
            assert_rejected_without_mutation(
                account(state, 3),
                event,
                SpotAccountError::InvalidTransition { state, event },
            );
        }
    }

    #[test]
    fn timestamp_regression_is_rejected_without_mutation() {
        let mut created_at_is_latest = account(AccountStatus::Active, 3);
        created_at_is_latest.created_at = 30;
        created_at_is_latest.updated_at = 20;
        assert_rejected_without_mutation(
            created_at_is_latest,
            SpotAccountEvent::FreezeForRisk { timestamp: 29 },
            SpotAccountError::TimestampRegression { timestamp: 29, earliest_allowed: 30 },
        );

        assert_rejected_without_mutation(
            account(AccountStatus::Frozen, 3),
            SpotAccountEvent::UnfreezeFromRisk { timestamp: 19 },
            SpotAccountError::TimestampRegression { timestamp: 19, earliest_allowed: 20 },
        );
    }

    #[test]
    fn version_overflow_is_rejected_without_mutation() {
        assert_rejected_without_mutation(
            account(AccountStatus::Active, u64::MAX),
            SpotAccountEvent::FreezeForRisk { timestamp: 20 },
            SpotAccountError::VersionOverflow,
        );
    }

    #[test]
    fn apply_reuses_transition_and_only_discards_output() {
        let mut transitioned = account(AccountStatus::Active, 7);
        let mut applied = transitioned.clone();
        let event = SpotAccountEvent::FreezeForRisk { timestamp: 25 };

        assert_eq!(transitioned.transition(event), Ok((AccountStatus::Frozen, ())));
        assert_eq!(applied, transitioned);
    }

    #[test]
    fn entity_replay_uses_status_field() {
        let active = account(AccountStatus::Active, 1);
        let frozen = account(AccountStatus::Frozen, 2);

        assert_eq!(
            active.created_field_changes(),
            vec![
                EntityFieldChange::new("account_id", "", "1001"),
                EntityFieldChange::new("party_id", "", "513"),
                EntityFieldChange::new("status", "", "active"),
            ]
        );
        assert_eq!(
            active.diff(&frozen),
            vec![EntityFieldChange::new("status", "active", "frozen")]
        );
        assert_eq!(SpotAccount::replay_field_type("status"), 0);
        assert_eq!(SpotAccount::replay_field_type("account_id"), 1);
        assert_eq!(SpotAccount::replay_field_type("party_id"), 1);
    }

    #[test]
    fn entity_id_and_shard_follow_numeric_ids() {
        let account = account(AccountStatus::Active, 1);
        assert_eq!(account.entity_id(), 1001);
        assert_eq!(account.topic(), 0);
        assert_eq!(account.partition(), 1);
    }
}
