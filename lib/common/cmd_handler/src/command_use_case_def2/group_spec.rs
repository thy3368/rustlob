/// Use case group 的轻量源码规格。
///
/// 该规格只表达业务建模元数据，不参与 executor、持久化或 adapter 映射。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseCaseGroupSpec {
    pub group_name: &'static str,
    pub boundary: GroupBoundarySpec,
    pub truth_center: TruthCenterSpec,
    pub mi_chain: MiCausalChainSpec,
    pub use_cases: &'static [UseCaseInGroupSpec],
}

/// 一个 group 覆盖的业务边界。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupBoundarySpec {
    pub scope: &'static str,
    pub starts_when: &'static str,
    pub ends_when: &'static str,
    pub excludes: &'static [&'static str],
}

/// group 的业务真相中心。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TruthCenterSpec {
    pub name: &'static str,
    pub main_mi: &'static str,
    pub description: &'static str,
}

/// 一个 use case group 内端到端 MI 因果链的源码规格。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiCausalChainSpec {
    pub root: MiCausalChainRootSpec,
    pub invariants: &'static [MiInvariantSpec],
}

/// MI 因果链的根 MI。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiCausalChainRootSpec {
    pub mi: &'static str,
    pub identity: &'static str,
    pub created_by: &'static str,
    pub starts_when: &'static str,
    pub payload: &'static [&'static str],
    pub why_root: &'static str,
    pub state_machine: MiRootStateMachineSpec,
}

/// 根 MI 的生命周期状态机。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiRootStateMachineSpec {
    pub state_field: &'static str,
    pub initial_state: &'static str,
    pub states: &'static [MiRootStateSpec],
    pub transitions: &'static [MiRootStateTransitionSpec],
}

/// 根 MI 的一个业务状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiRootStateSpec {
    pub name: &'static str,
    pub meaning: &'static str,
    pub terminal: bool,
}

/// 子 MI 或 append-only 事实驱动根 MI 状态变化的规则。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiRootStateTransitionSpec {
    pub from: &'static [&'static str],
    pub to: &'static [&'static str],
    pub produced_fact: MiProducedFactSpec,
    pub predicate: MiPredicateSpec,
    pub root_change: &'static str,
}

/// 驱动根 MI 状态变化时同步生成的子 MI 或 append-only 事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiProducedFactSpec {
    pub name: &'static str,
    pub identity: &'static str,
    pub kind: &'static str,
    pub produced_by: &'static str,
    pub causal_pointer: MiCausalPointerSpec,
}

/// 因果判断成立时读取的前驱载荷与判定规则。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiPredicateSpec {
    pub reads: &'static [&'static str],
    pub rule: &'static str,
    pub when_true: &'static str,
    pub when_false: &'static str,
}

/// 后继事实必须保留的因果指针。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiCausalPointerSpec {
    pub caused_by: &'static str,
    pub due_to: &'static str,
}

/// MI 因果链展开时必须满足的业务守恒规则。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiInvariantSpec {
    pub name: &'static str,
    pub applies_to: &'static [&'static str],
    pub rule: &'static str,
}

/// group 内一个独立 use case 的源码索引。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseCaseInGroupSpec {
    pub name: &'static str,
    pub command: &'static str,
    pub given_state: &'static str,
    pub changes: &'static str,
}
