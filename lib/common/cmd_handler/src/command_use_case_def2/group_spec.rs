/// Use case group 的轻量源码规格。
///
/// 该规格只表达业务建模元数据，不参与 executor、持久化或 adapter 映射。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseCaseGroupSpec {
    pub group_name: &'static str,
    pub boundary: GroupBoundarySpec,
    pub truth_center: TruthCenterSpec,
    pub mi_chain: &'static [MiCausalEdgeSpec],
    pub terminal_facts: &'static [&'static str],
    pub use_cases: &'static [UseCaseInGroupSpec],
    pub non_use_case_items: &'static [NonUseCaseItemSpec],
    pub invariants: &'static [&'static str],
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

/// MI 因果链中的一条因果边。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiCausalEdgeSpec {
    pub predecessor: &'static str,
    pub predicate: &'static str,
    pub successor: &'static str,
    pub caused_by: &'static str,
    pub due_to: &'static str,
}

/// group 内一个独立 use case 的源码索引。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseCaseInGroupSpec {
    pub name: &'static str,
    pub command: &'static str,
    pub given_state: &'static str,
    pub changes: &'static str,
}

/// 明确排除为非独立 use case 的项目。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonUseCaseItemSpec {
    pub name: &'static str,
    pub reason: &'static str,
}
