use std::fmt::Debug;

use crate::entity_v2::entity_field_change::current_timestamp;
use crate::{
    EntityChangeType, EntityError, EntityFieldChange, EntityReplayableEvent, ReplayFieldChange,
};

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

/// 实体在财务/账务链路中的语义分类。
///
/// 该分类独立于四色建模、变更模型与聚合角色，只表达实体在账务语义里的职责。
/// 它不参与 replay 事件编码、实体类型码分配，也不承诺持久化格式语义。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FinancialClassification {
    /// 尚未分类，作为旧实体和未治理实体的默认值。
    Unclassified,
    /// 业务凭证：描述业务事实本身，不直接等于入账结果。
    BusinessVoucher,
    /// 记账凭证：描述应如何入账，是账务编排依据。
    AccountingVoucher,
    /// 账本流水：描述已落账的余额或账本变化结果。
    LedgerEntry,
}

impl FinancialClassification {
    /// 返回稳定的财务分类标签字符串。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unclassified => "unclassified",
            Self::BusinessVoucher => "business_voucher",
            Self::AccountingVoucher => "accounting_voucher",
            Self::LedgerEntry => "ledger_entry",
        }
    }
}

/// 实体在聚合边界中的业务角色分类。
///
/// 该分类只表达领域建模中的聚合语义，不参与 replay 事件编码、实体类型码分配
/// 或持久化格式语义。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggregateRole {
    /// 尚未分类，作为旧实体和未治理实体的默认值。
    Unclassified,
    /// 聚合根。
    ///
    /// 本聚合对外业务入口与一致性边界由根统一承担。
    /// 典型例子：订单、账户、仓位。
    AggregateRoot,
    /// 聚合成员。
    ///
    /// 成员只供聚合根在聚合内部驱动、装配和维护不变量时使用，
    /// 不作为对外业务入口，也不应被 use case 直接依赖为主业务入口。
    /// 典型例子：owned object、聚合内明细项。
    AggregateMember,
}

impl AggregateRole {
    /// 返回稳定的聚合角色标签字符串。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unclassified => "unclassified",
            Self::AggregateRoot => "aggregate_root",
            Self::AggregateMember => "aggregate_member",
        }
    }
}

/// entity 对 use case 暴露 API 面的治理强度。
///
/// 该分类不直接限制 Rust 语法层面的 `pub fn`，而是为 architecture policy 与静态 checker
/// 提供显式 opt-in 信号。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityUseCaseApiSurface {
    /// 旧实体默认值，不对 use case API 面做强治理。
    LegacyUnconstrained,
    /// 对 use case 只暴露最小高语义业务 API。
    MinimalBusinessApi,
}

impl EntityUseCaseApiSurface {
    /// 返回稳定的治理标签字符串。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LegacyUnconstrained => "legacy_unconstrained",
            Self::MinimalBusinessApi => "minimal_business_api",
        }
    }
}

/// entity 面向 use case 暴露 API 的治理条款映射。
///
/// 该结构是工程元数据落点；真正的执行仍依赖独立 checker 与 reviewer judgement。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityUseCaseApiPolicy {
    /// 是否允许把原始内部集合直接公开给 use case。
    pub allow_raw_child_collections: bool,
    /// 是否允许把宽构造器作为 use case 主入口公开。
    pub allow_public_wide_constructor: bool,
    /// 是否要求 use case-facing 查询优先暴露高语义业务结论。
    pub require_high_semantic_queries: bool,
}

impl EntityUseCaseApiPolicy {
    /// 旧实体或未治理实体的默认政策。
    pub const fn legacy_unconstrained() -> Self {
        Self {
            allow_raw_child_collections: true,
            allow_public_wide_constructor: true,
            require_high_semantic_queries: false,
        }
    }

    /// 最小高语义业务 API 面的政策。
    pub const fn minimal_business_api() -> Self {
        Self {
            allow_raw_child_collections: false,
            allow_public_wide_constructor: false,
            require_high_semantic_queries: true,
        }
    }
}

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
    pub source_fact_type: &'static str,
    /// 当前事实与该前驱事实之间的因果关系。
    pub relation: MiCausalRelation,
    /// 同一前驱类型有多个参与角色时使用的业务角色名。
    pub source_role: &'static str,
}

/// 可复用的字段级 diff 契约。
///
/// 纯值对象可以只实现该 trait 来复用 [`EntityFieldChange`] 表达字段变化；
/// 只有需要 identity、version、replay、audit 或生命周期语义的对象才应实现 [`Entity`]。
pub trait FieldDiff {
    fn diff(&self, other: &Self) -> Vec<EntityFieldChange>;

    #[inline]
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        Vec::with_capacity(0)
    }
}

/// 实体生命周期时间约束。
///
/// 时间统一使用 Unix 纳秒。生命周期时间由调用方提供，实体只负责暴露并校验
/// 创建时间不晚于最后更新时间。
pub trait EntityLifecycle {
    /// 返回实体创建时间，单位为 Unix 纳秒。创建时从command里面取
    fn created_at(&self) -> u64;

    /// 返回实体最后更新时间，单位为 Unix 纳秒。更新时从command里面取
    fn updated_at(&self) -> u64;

    /// 有点版本号的意思，如果command的高于entity里面的，说明entity没有更新，use case里面要报错，在check state里面有用，解决数据一致问题。

    /// 返回生命周期时间是否满足 `created_at <= updated_at`。
    #[inline]
    fn has_valid_lifecycle(&self) -> bool {
        self.created_at() <= self.updated_at()
    }
}

/// Enhanced entity contract for generating compact replayable entity events.
pub trait Entity: FieldDiff + Clone + Debug + Send + Sync + 'static {
    fn entity_id(&self) -> u64;

    fn topic(&self) -> u8 {
        0
    }
    fn partition(&self) -> u8;

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

    /// 返回实体在财务/账务链路中的语义分类。
    ///
    /// 默认值是 [`FinancialClassification::Unclassified`]，用于兼容旧实体和未治理实体。
    ///
    /// [`FinancialClassification::BusinessVoucher`] 表示实体描述业务事实本身，不直接等于入账结果；
    /// [`FinancialClassification::AccountingVoucher`] 表示实体描述应如何入账，是账务编排依据；
    /// [`FinancialClassification::LedgerEntry`] 表示实体描述已落账的余额或账本变化结果。
    ///
    /// 该元数据只用于领域建模治理，不参与 replay 事件编码、实体类型码分配或持久化语义。
    #[inline]
    fn financial_classification() -> FinancialClassification
    where
        Self: Sized,
    {
        FinancialClassification::Unclassified
    }

    /// 返回实体在聚合边界中的业务角色。
    ///
    /// 默认值是 [`AggregateRole::Unclassified`]，用于兼容旧实体和未治理实体。
    ///
    /// 该元数据只用于领域模型治理，不改变 replay、实体类型码或持久化语义。
    /// 若实体是聚合边界对外暴露的一致性根，并统一承担本聚合的对外业务入口，
    /// 应 override 为 [`AggregateRole::AggregateRoot`]；
    /// 若实体只在某个聚合内部存在，应 override 为 [`AggregateRole::AggregateMember`]。
    /// `AggregateMember` 只表达聚合内部存在，不应被 use case 直接依赖为主业务入口，
    /// 也不应直接访问、装载、调用或导航到其它聚合；
    /// 聚合间协作逻辑不应下放到成员对象。
    #[inline]
    fn aggregate_role() -> AggregateRole
    where
        Self: Sized,
    {
        AggregateRole::Unclassified
    }

    /// 返回该实体面向 use case 的 API 收敛治理强度。
    ///
    /// 默认值是 [`EntityUseCaseApiSurface::LegacyUnconstrained`]，用于兼容旧实体与未治理实体。
    /// 若实体希望对 use case 只暴露最小高语义业务 API，应 override 为
    /// [`EntityUseCaseApiSurface::MinimalBusinessApi`]，再由独立 checker 执行静态治理。
    /// 其目标之一，是避免 use case 绕过聚合根，直接把聚合成员或原始成员材料当作主业务入口。
    #[inline]
    fn use_case_api_surface() -> EntityUseCaseApiSurface
    where
        Self: Sized,
    {
        EntityUseCaseApiSurface::LegacyUnconstrained
    }

    /// 返回由 `use_case_api_surface()` 推导出的 use case API 治理政策。
    ///
    /// 该元数据只表达工程治理配置，不会自动阻止额外 `pub fn` 暴露。
    /// 当 policy 收敛到最小业务 API 面时，目标之一同样是防止 use case 绕过聚合根直连成员材料。
    #[inline]
    fn use_case_api_policy() -> EntityUseCaseApiPolicy
    where
        Self: Sized,
    {
        match Self::use_case_api_surface() {
            EntityUseCaseApiSurface::LegacyUnconstrained => {
                EntityUseCaseApiPolicy::legacy_unconstrained()
            }
            EntityUseCaseApiSurface::MinimalBusinessApi => {
                EntityUseCaseApiPolicy::minimal_business_api()
            }
        }
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

    fn lsn(&self) -> u64;

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
        let mut event = EntityReplayableEvent::new_with_topic(
            self.topic(),
            self.partition(),
            current_timestamp()?,
            0,
            0,
            self.replay_entity_id()?,
            Self::entity_type(),
            EntityChangeType::Created.as_tag(),
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
        if old_state.topic() != self.topic() || old_state.partition() != self.partition() {
            return Err(EntityError::EntityIdMismatch {
                expected: format!("{}/{}", old_state.topic(), old_state.partition()),
                actual: format!("{}/{}", self.topic(), self.partition()),
            });
        }

        let old_version = old_state.lsn();
        let new_version = self.lsn();

        let changes = old_state.diff(self);
        if changes.is_empty() {
            return Err(EntityError::NoChangesDetected);
        }

        let mut event = EntityReplayableEvent::new_with_topic(
            self.topic(),
            self.partition(),
            current_timestamp()?,
            old_version,
            new_version,
            self.replay_entity_id()?,
            Self::entity_type(),
            EntityChangeType::Updated.as_tag(),
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
        let old_version = self.lsn();
        Ok(EntityReplayableEvent::new_with_topic(
            self.topic(),
            self.partition(),
            current_timestamp()?,
            old_version,
            increment_version(old_version)?,
            self.replay_entity_id()?,
            Self::entity_type(),
            EntityChangeType::Deleted.as_tag(),
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
