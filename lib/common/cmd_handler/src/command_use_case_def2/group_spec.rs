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
    pub root: MiSpec,
    pub invariants: &'static [MiInvariantSpec],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiSpec {
    /// MI 的名称
    pub name: &'static str,
    /// MI 的身份字段
    pub identity: &'static str,
    ///  MI 如何被创建（
    pub created_by: Option<&'static str>,
    ///  MI 何时开始（
    pub starts_when: Option<&'static str>,
    ///  MI 的载荷字段列表（
    pub payload: Option<&'static [&'static str]>,
    ///  MI 为何作为根的说明（
    pub why_root: Option<&'static str>,
    ///  MI 的状态机
    pub state_machine: Option<MiStateMachineSpec>,
    ///  MI 的类型：append_only_fact, fund_side_mi 等
    pub kind: Option<&'static str>,
    ///  MI 由哪个 use case 产生
    pub produced_by: Option<&'static str>,
    ///  MI 的因果指针
    pub causal_pointer: Option<MiCausalPointerSpec>,
}

/// 根 MI 的生命周期状态机。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiStateMachineSpec {
    pub state_field: &'static str,
    pub initial_state: &'static str,
    pub states: &'static [MiStateSpec],
    pub transitions: &'static [MiStateTransitionSpec],
}

/// 根 MI 的一个业务状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiStateSpec {
    pub name: &'static str,
    pub meaning: &'static str,
    pub terminal: bool,
}

/// 子 MI 或 append-only 事实驱动根 MI 状态变化的规则。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiStateTransitionSpec {
    pub from: &'static [&'static str],
    pub to: &'static [&'static str],
    pub produced_fact: MiSpec,
    pub predicate: MiPredicateSpec,
    pub root_change: &'static str,
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
