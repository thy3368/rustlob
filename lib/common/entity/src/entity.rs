use std::fmt::Debug;

use crate::entity_field_change::{current_timestamp, next_sequence};
use crate::{EntityError, EntityFieldChange, EntityReplayableEvent, ReplayFieldChange};

/// todo 业务方法 跨因果链放在use case里；因果链内放在链根里 这规则要通过skill保障

/// 四色建模中的实体原型分类。
///
/// 该分类只表达领域建模语义，用于标注实体在业务模型里的角色。
/// 它不参与 replay 事件编码、实体类型码分配，也不承诺持久化格式语义。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FourColorArchetype {
    /// Moment-Interval：记录一次业务活动、事件或时间区间。
    MomentInterval,
    /// Party-Place-Thing：业务中的人、地点、物或组织等核心对象。
    PartyPlaceThing,
    /// Role：某个 Party/Place/Thing 在特定上下文中的角色。
    Role,
    /// Description：用于描述、分类或定价等相对稳定的说明性信息。
    Description,
    /// 尚未分类，作为旧实体和未治理实体的默认值。
    Unclassified,
}

impl FourColorArchetype {
    /// 返回稳定的业务标签字符串。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MomentInterval => "moment_interval",
            Self::PartyPlaceThing => "party_place_thing",
            Self::Role => "role",
            Self::Description => "description",
            Self::Unclassified => "unclassified",
        }
    }
}

/// 实体的变更模型分类。
///
/// 该分类描述实体状态如何随业务事件演进，以及它是否作为业务事实来源。
/// 它独立于四色建模分类，也不参与 replay 事件编码、实体类型码分配或持久化格式语义。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityMutationModel {
    /// 可版本化修改的当前状态实体。
    ///
    /// 这种实体有稳定 identity，同一个实体会随着业务命令从版本 `n` 演进到 `n + 1`。
    /// 它通常是业务事实来源，系统关心的是“这个实体现在处于什么状态”，同时通过 replay
    /// 事件保留状态变化过程。
    ///
    /// 典型例子：订单、余额、仓位、账户。
    VersionedMutable,
    /// 追加记录。
    ///
    /// 这种实体代表一条已经发生的业务事实，创建后通常不再修改。后续修正不应该原地改写
    /// 旧记录，而应该追加新的冲正、补偿或关联记录。
    ///
    /// 典型例子：成交、资金流水、结算流水、审计记录。
    AppendOnlyRecord,
    /// 某个时点的状态截面。
    ///
    /// 这种实体通常不是业务事实本身，而是为了重建、查询、风控计算或读取加速而保存的
    /// 一份状态截面。重点不是“同一个快照被持续修改”，而是“在某个序列号或时间点生成了
    /// 一份新的状态截面”。
    ///
    /// 判断规则：如果同一个业务 ID 后续会被命令修改状态，通常应使用 [`VersionedMutable`]；
    /// 如果每条记录都代表某个时点的一份状态截面，通常应使用 [`Snapshot`]。
    ///
    /// 典型例子：盘口快照、账户风险快照、组合资产快照。
    ///
    /// [`VersionedMutable`]: Self::VersionedMutable
    /// [`Snapshot`]: Self::Snapshot
    Snapshot,
    /// 派生读模型。
    ///
    /// 这种实体由其他业务事实、事件或快照投影而来，服务查询、展示或下游消费。
    /// 它不应该作为最终业务事实来源；如果和源事实冲突，应以源事实为准并重新投影。
    ///
    /// 典型例子：用户当前挂单视图、行情展示视图、PnL 报表视图。
    DerivedReadModel,
}

impl EntityMutationModel {
    /// 返回稳定的变更模型标签字符串。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VersionedMutable => "versioned_mutable",
            Self::AppendOnlyRecord => "append_only_record",
            Self::Snapshot => "snapshot",
            Self::DerivedReadModel => "derived_read_model",
        }
    }
}

/// 实体的业务生命周期模型分类。
///
/// 该分类只回答实体是否拥有自己的业务生命周期或状态机。
/// 它独立于变更模型，不参与 replay 事件编码、实体类型码分配或持久化格式语义，
/// 也不自动限制 `track_update_event` 或 `track_delete_event` 的运行行为。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityLifecycleModel {
    /// 尚未分类，作为旧实体和未治理实体的默认值。
    Unclassified,
    /// 创建即成立、没有业务状态迁移的事实记录。
    ///
    /// 典型例子：成交、资金流水、审计记录。
    StatelessFact,
    /// 有明确业务生命周期或状态机的实体。
    ///
    /// 典型例子：订单、仓位、账户状态。
    StatefulLifecycle,
    /// 没有独立生命周期，只随所属聚合或来源事实存在。
    ///
    /// 典型例子：owned object、派生读模型。
    DependentLifecycle,
}

impl EntityLifecycleModel {
    /// 返回稳定的生命周期模型标签字符串。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unclassified => "unclassified",
            Self::StatelessFact => "stateless_fact",
            Self::StatefulLifecycle => "stateful_lifecycle",
            Self::DependentLifecycle => "dependent_lifecycle",
        }
    }
}

/// MI 建模中的稳定业务事实类型名。
///
/// 它只表达业务语义，不参与 replay 事件编码或实体类型码分配。
pub type MiFactType = &'static str;

/// MI 因果链里当前事实与前驱事实之间的业务关系。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MiCausalRelation {
    /// 当前事实由前驱事实直接触发或形成。
    CausedBy,
    /// 当前事实基于某个参与性前驱事实而成立。
    DueTo,
    /// 当前事实用于冲正或补偿前驱事实。
    Compensates,
    /// 当前事实用于结清前驱事实。
    Settles,
}

/// 当前 MI 事实允许引用的前驱事实元数据。
///
/// 该结构描述类型级因果来源；具体实例仍由实体字段保存，例如订单 ID 或成交 ID。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MiCausalSourceMetadata {
    /// 前驱事实的稳定业务类型名。
    pub source_fact_type: MiFactType,
    /// 当前事实与该前驱事实之间的因果关系。
    pub relation: MiCausalRelation,
    /// 同一前驱类型有多个参与角色时使用的业务角色名。
    pub source_role: &'static str,
}

/// Enhanced entity contract for generating compact replayable entity events.
pub trait Entity: Clone + Debug + Send + Sync + 'static {
    type Id: Debug + Clone + PartialEq + ToString;

    fn entity_id(&self) -> Self::Id;

    fn entity_type() -> u8
    where
        Self: Sized;

    /// 返回实体在四色建模中的业务原型分类。
    ///
    /// 该元数据只用于领域建模治理，不参与 replay 事件编码、实体类型码或持久化语义。
    #[inline]
    fn four_color_archetype() -> FourColorArchetype
    where
        Self: Sized,
    {
        FourColorArchetype::Unclassified
    }

    /// 返回实体状态随业务事件演进的变更模型。
    ///
    /// 默认值是 [`EntityMutationModel::VersionedMutable`]，因为当前 [`Entity`] trait
    /// 内建了 `entity_version`、`diff`、`track_update_event` 和 `track_delete_event`，
    /// 语义上更接近“同一个实体通过版本递增发生状态变化”。
    ///
    /// 该元数据只用于领域模型治理，不改变 `track_create_event`、`track_update_event`
    /// 或 `track_delete_event` 的运行行为。若实体是成交、流水、审计记录等创建后不应原地修改的
    /// 业务事实，应 override 为 [`EntityMutationModel::AppendOnlyRecord`]；若实体是一份时点截面，
    /// 应 override 为 [`EntityMutationModel::Snapshot`]；若实体只是由其他事实投影出的查询模型，
    /// 应 override 为 [`EntityMutationModel::DerivedReadModel`]。
    #[inline]
    fn mutation_model() -> EntityMutationModel
    where
        Self: Sized,
    {
        EntityMutationModel::VersionedMutable
    }

    /// 返回实体是否拥有自己的业务生命周期或状态机。
    ///
    /// 默认值是 [`EntityLifecycleModel::Unclassified`]，用于兼容旧实体和未治理实体。
    ///
    /// 该元数据只用于领域模型治理，不改变 replay、实体类型码或持久化语义，也不自动限制
    /// `track_update_event` 或 `track_delete_event`。若实体创建即代表一个已经成立的事实，
    /// 应 override 为 [`EntityLifecycleModel::StatelessFact`]；若实体有明确状态机，
    /// 应 override 为 [`EntityLifecycleModel::StatefulLifecycle`]；若实体只随所属聚合或来源事实存在，
    /// 应 override 为 [`EntityLifecycleModel::DependentLifecycle`]。
    #[inline]
    fn lifecycle_model() -> EntityLifecycleModel
    where
        Self: Sized,
    {
        EntityLifecycleModel::Unclassified
    }

    /// 返回实体对应的稳定 MI 业务事实类型名。
    ///
    /// 该元数据只用于 MI 建模治理，不参与 replay 事件编码、实体类型码或持久化语义。
    #[inline]
    fn mi_fact_type() -> Option<MiFactType>
    where
        Self: Sized,
    {
        None
    }

    /// 返回该实体类型是否是一条 MI 因果链的根事实类型。
    /// 必须：当root=true时，必须补充该因果链根的业务方法 驱动状态演进，链根是因，内链内mi是果。
    #[inline]
    fn is_mi_chain_root() -> bool
    where
        Self: Sized,
    {
        false
    }

    /// 返回当前 MI 事实允许由哪些前驱 MI 事实导致。
    ///
    /// 这里表达类型级规则；具体因果实例仍由业务字段表达。
    #[inline]
    fn mi_causal_sources() -> &'static [MiCausalSourceMetadata]
    where
        Self: Sized,
    {
        &[]
    }

    fn entity_version(&self) -> u64;

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange>;

    #[inline]
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    #[inline]
    fn replay_field_type(_field_name: &str) -> u8 {
        0
    }

    #[inline]
    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        let entity_id = self.entity_id().to_string();
        entity_id.parse::<i64>().map_err(|error| EntityError::EntityIdParseError {
            entity_id,
            reason: error.to_string(),
        })
    }

    #[inline]
    fn track_create_event(&self) -> Result<EntityReplayableEvent, EntityError>
    where
        Self: Sized,
    {
        let mut event = EntityReplayableEvent::new_created(
            current_timestamp()?,
            next_sequence(),
            self.replay_entity_id()?,
            Self::entity_type(),
        );

        for change in self.created_field_changes() {
            let field_type = Self::replay_field_type(change.field_name.as_ref());
            event
                .add_field_change(ReplayFieldChange::from_entity_field_change(&change, field_type));
        }

        Ok(event)
    }

    #[inline]
    fn track_update_event<F>(&mut self, updater: F) -> Result<EntityReplayableEvent, EntityError>
    where
        Self: Sized,
        F: FnOnce(&mut Self),
    {
        let old_state = self.clone();
        updater(self);
        self.track_update_event_from(&old_state)
    }

    #[inline]
    /// 从旧状态和当前状态生成单步实体版本更新事件。
    ///
    /// 该方法只表达同一个实体实例的一次 `version n -> n + 1` 更新，调用方必须保证
    /// `old_state` 和 `self` 是同一实体，且当前版本正好是旧版本的下一版本。
    ///
    /// 如果 case-level changes 中的 before/after pair 跨越多步版本，不能直接用该方法投影；
    /// 应从 changes 内部的有序事实记录、流水或 created records 投影出逐步 replay events。
    fn track_update_event_from(
        &self,
        old_state: &Self,
    ) -> Result<EntityReplayableEvent, EntityError>
    where
        Self: Sized,
    {
        ensure_same_entity(old_state, self)?;

        let old_version = old_state.entity_version();
        let expected_next = increment_version(old_version)?;
        let new_version = self.entity_version();
        if new_version != expected_next {
            return Err(EntityError::EntityVersionMismatch {
                expected_next,
                actual_next: new_version,
            });
        }

        let changes = old_state.diff(self);
        if changes.is_empty() {
            return Err(EntityError::NoChangesDetected);
        }

        let mut event = EntityReplayableEvent::new_updated(
            current_timestamp()?,
            next_sequence(),
            old_version,
            new_version,
            self.replay_entity_id()?,
            Self::entity_type(),
        );

        for change in changes {
            let field_type = Self::replay_field_type(change.field_name.as_ref());
            event
                .add_field_change(ReplayFieldChange::from_entity_field_change(&change, field_type));
        }

        Ok(event)
    }

    #[inline]
    fn track_delete_event(&self) -> Result<EntityReplayableEvent, EntityError>
    where
        Self: Sized,
    {
        let old_version = self.entity_version();
        Ok(EntityReplayableEvent::new_deleted(
            current_timestamp()?,
            next_sequence(),
            old_version,
            increment_version(old_version)?,
            self.replay_entity_id()?,
            Self::entity_type(),
        ))
    }
}

#[inline]
fn increment_version(version: u64) -> Result<u64, EntityError> {
    version.checked_add(1).ok_or(EntityError::VersionOverflow { version })
}

#[inline]
fn ensure_same_entity<T>(old_state: &T, new_state: &T) -> Result<(), EntityError>
where
    T: Entity,
{
    let expected = old_state.entity_id().to_string();
    let actual = new_state.entity_id().to_string();
    if expected == actual {
        Ok(())
    } else {
        Err(EntityError::EntityIdMismatch { expected, actual })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_field_change::ReplayFieldChange;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEntity {
        id: i64,
        value: String,
        version: u64,
    }

    impl Entity for TestEntity {
        type Id = i64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> u8 {
            7
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
            let mut changes = Vec::new();
            if self.value != other.value {
                changes.push(EntityFieldChange::new("value", &self.value, &other.value));
            }
            changes
        }

        fn replay_field_type(field_name: &str) -> u8 {
            match field_name {
                "value" => 1,
                _ => 0,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct StringIdEntity {
        id: String,
        version: u64,
    }

    impl Entity for StringIdEntity {
        type Id = String;

        fn entity_id(&self) -> Self::Id {
            self.id.clone()
        }

        fn entity_type() -> u8 {
            9
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
            Vec::new()
        }
    }

    #[derive(Debug, Clone)]
    struct MomentIntervalEntity {
        id: i64,
        version: u64,
    }

    impl Entity for MomentIntervalEntity {
        type Id = i64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> u8 {
            11
        }

        fn four_color_archetype() -> FourColorArchetype
        where
            Self: Sized,
        {
            FourColorArchetype::MomentInterval
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
            Vec::new()
        }
    }

    #[derive(Debug, Clone)]
    struct AppendOnlyEntity {
        id: i64,
        version: u64,
    }

    impl Entity for AppendOnlyEntity {
        type Id = i64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> u8 {
            12
        }

        fn mutation_model() -> EntityMutationModel
        where
            Self: Sized,
        {
            EntityMutationModel::AppendOnlyRecord
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
            Vec::new()
        }
    }

    #[derive(Debug, Clone)]
    struct StatefulLifecycleEntity {
        id: i64,
        version: u64,
    }

    impl Entity for StatefulLifecycleEntity {
        type Id = i64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> u8 {
            13
        }

        fn lifecycle_model() -> EntityLifecycleModel
        where
            Self: Sized,
        {
            EntityLifecycleModel::StatefulLifecycle
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
            Vec::new()
        }
    }

    #[test]
    fn entity_defaults_to_unclassified_four_color_archetype() {
        assert_eq!(TestEntity::four_color_archetype(), FourColorArchetype::Unclassified);
    }

    #[test]
    fn entity_can_override_four_color_archetype() {
        assert_eq!(
            MomentIntervalEntity::four_color_archetype(),
            FourColorArchetype::MomentInterval
        );
    }

    #[test]
    fn four_color_archetype_returns_stable_business_label() {
        assert_eq!(FourColorArchetype::MomentInterval.as_str(), "moment_interval");
        assert_eq!(FourColorArchetype::PartyPlaceThing.as_str(), "party_place_thing");
        assert_eq!(FourColorArchetype::Role.as_str(), "role");
        assert_eq!(FourColorArchetype::Description.as_str(), "description");
        assert_eq!(FourColorArchetype::Unclassified.as_str(), "unclassified");
    }

    #[test]
    fn entity_defaults_to_versioned_mutable_mutation_model() {
        assert_eq!(TestEntity::mutation_model(), EntityMutationModel::VersionedMutable);
    }

    #[test]
    fn entity_defaults_to_no_mi_metadata() {
        assert_eq!(TestEntity::mi_fact_type(), None);
        assert!(!TestEntity::is_mi_chain_root());
        assert!(TestEntity::mi_causal_sources().is_empty());
    }

    #[test]
    fn entity_can_override_mutation_model() {
        assert_eq!(AppendOnlyEntity::mutation_model(), EntityMutationModel::AppendOnlyRecord);
    }

    #[test]
    fn entity_defaults_to_unclassified_lifecycle_model() {
        assert_eq!(TestEntity::lifecycle_model(), EntityLifecycleModel::Unclassified);
    }

    #[test]
    fn entity_can_override_lifecycle_model() {
        assert_eq!(
            StatefulLifecycleEntity::lifecycle_model(),
            EntityLifecycleModel::StatefulLifecycle
        );
    }

    #[test]
    fn entity_mutation_model_returns_stable_business_label() {
        assert_eq!(EntityMutationModel::VersionedMutable.as_str(), "versioned_mutable");
        assert_eq!(EntityMutationModel::AppendOnlyRecord.as_str(), "append_only_record");
        assert_eq!(EntityMutationModel::Snapshot.as_str(), "snapshot");
        assert_eq!(EntityMutationModel::DerivedReadModel.as_str(), "derived_read_model");
    }

    #[test]
    fn entity_lifecycle_model_returns_stable_business_label() {
        assert_eq!(EntityLifecycleModel::Unclassified.as_str(), "unclassified");
        assert_eq!(EntityLifecycleModel::StatelessFact.as_str(), "stateless_fact");
        assert_eq!(EntityLifecycleModel::StatefulLifecycle.as_str(), "stateful_lifecycle");
        assert_eq!(EntityLifecycleModel::DependentLifecycle.as_str(), "dependent_lifecycle");
    }

    #[test]
    fn create_event_uses_initial_version_transition() {
        let entity = TestEntity { id: 42, value: "new".to_string(), version: 1 };

        let event = entity.track_create_event().unwrap();

        assert!(event.is_created());
        assert_eq!(event.old_version, 0);
        assert_eq!(event.new_version, 1);
        assert_eq!(event.entity_id, 42);
        assert_eq!(event.entity_type, 7);
    }

    #[test]
    fn update_event_contains_diff_field_changes() {
        let old = TestEntity { id: 1, value: "old".to_string(), version: 3 };
        let new = TestEntity { id: 1, value: "new".to_string(), version: 4 };

        let event = new.track_update_event_from(&old).unwrap();

        assert!(event.is_updated());
        assert_eq!(event.old_version, 3);
        assert_eq!(event.new_version, 4);
        assert_eq!(event.field_change_count(), 1);

        let change = &event.field_changes[0];
        assert_eq!(change.field_name_as_str().unwrap(), "value");
        assert_eq!(change.old_value_bytes(), b"old");
        assert_eq!(change.new_value_bytes(), b"new");
        assert_eq!(change.field_type, 1);
    }

    #[test]
    fn update_event_rejects_no_changes() {
        let old = TestEntity { id: 1, value: "same".to_string(), version: 3 };
        let new = TestEntity { id: 1, value: "same".to_string(), version: 4 };

        let result = new.track_update_event_from(&old);

        assert_eq!(result.unwrap_err(), EntityError::NoChangesDetected);
    }

    #[test]
    fn update_event_rejects_non_incremented_version() {
        let old = TestEntity { id: 1, value: "old".to_string(), version: 3 };
        let new = TestEntity { id: 1, value: "new".to_string(), version: 5 };

        let result = new.track_update_event_from(&old);

        assert_eq!(
            result.unwrap_err(),
            EntityError::EntityVersionMismatch { expected_next: 4, actual_next: 5 }
        );
    }

    #[test]
    fn update_event_rejects_entity_id_mismatch() {
        let old = TestEntity { id: 1, value: "old".to_string(), version: 3 };
        let new = TestEntity { id: 2, value: "new".to_string(), version: 4 };

        let result = new.track_update_event_from(&old);

        assert_eq!(
            result.unwrap_err(),
            EntityError::EntityIdMismatch { expected: "1".to_string(), actual: "2".to_string() }
        );
    }

    #[test]
    fn update_event_from_closure_tracks_after_mutation() {
        let mut entity = TestEntity { id: 1, value: "old".to_string(), version: 3 };

        let event = entity
            .track_update_event(|entity| {
                entity.value = "new".to_string();
                entity.version += 1;
            })
            .unwrap();

        assert_eq!(entity.value, "new");
        assert_eq!(event.old_version, 3);
        assert_eq!(event.new_version, 4);
        assert_eq!(event.field_change_count(), 1);
    }

    #[test]
    fn delete_event_uses_current_to_next_version() {
        let entity = TestEntity { id: 42, value: "old".to_string(), version: 8 };

        let event = entity.track_delete_event().unwrap();

        assert!(event.is_deleted());
        assert_eq!(event.old_version, 8);
        assert_eq!(event.new_version, 9);
        assert_eq!(event.entity_id, 42);
    }

    #[test]
    fn non_numeric_entity_id_returns_parse_error() {
        let entity = StringIdEntity { id: "account:BTC".to_string(), version: 1 };

        let result = entity.track_create_event();

        assert!(matches!(result, Err(EntityError::EntityIdParseError { .. })));
    }

    #[test]
    fn replay_field_change_truncates_fixed_width_data() {
        let long_field_name = "abcdefghijklmnopqrstuvwxyz0123456789";
        let long_old = vec![b'o'; 80];
        let long_new = vec![b'n'; 96];

        let change = ReplayFieldChange::new(
            ReplayFieldChange::field_name_from_str(long_field_name),
            &long_old,
            &long_new,
            2,
        );

        assert_eq!(change.field_name_as_str().unwrap().len(), 32);
        assert_eq!(change.old_value_bytes().len(), 64);
        assert_eq!(change.new_value_bytes().len(), 64);
        assert_eq!(change.field_type, 2);
    }
}
