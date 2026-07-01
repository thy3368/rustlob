# Use Case / Entity Constraints

这是 `use_case` / `entity` 共享约束的唯一 canonical 文档。
凡是关于 `use case` 与 `entity` 的边界判断、`aggregate role`、`MI chain root`、`replay/version`
语义的 shared reference，都应只读取本文件。

本文件只保留能被以下代码直接支撑的事实：

- `lib/common/entity/src/entity.rs`
- `lib/common/entity/src/command_use_case_v6.rs`

如果需要更强的架构政策、建模偏好或工程约束，应引用独立的 architecture policy 文档或 skill，
不要把那些规则伪装成这两个 trait 的代码事实。

## Fact Sources

- `lib/common/entity/src/entity.rs`
  - `Entity` trait 是 `entity / aggregate / MI metadata / replay version` 语义事实源。
- `lib/common/entity/src/command_use_case_v6.rs`
  - `CommandUseCase6` trait 是 command-style `use case` 契约事实源。

## Use Case Facts From `CommandUseCase6`

- `use case` 的当前业务输入是 `Command`。
- `Command` 必须实现 `IssuedByParty + CommandWithGivenState`。
- `use case` 的唯一业务真相是 `Changes`。
- `Changes` 必须实现 `ReplayableChanges`。
- `use case` 的职责是：
  - `command + loaded authoritative state -> changes`
- `pre_check_command(...)` 只做不依赖状态的快速校验。
- `validate_against_state(...)` 只做基于已加载 authoritative state 的业务校验。
- `compute_before_after_changes(...)` 是确定性业务推导核心。
- `compute_before_after_changes(...)` 内应显式匹配 `Command / GivenState / Changes` 分支。
- 分支错配时必须返回明确业务错误。
- `use case` 应调用聚合根对外公开的业务方法，完成一次业务目标推导。
- `ReplayableChanges::to_replayable_events()` 从 `Changes` 投影后续可持久化、可回放、可发布的事实。

`CommandUseCase6` 注释明确说它不负责：

- DB 访问或其他状态加载
- replayable events 的发布或持久化执行
- HTTP / WebSocket reply shaping
- 权限、鉴权、审计等基础设施实现

## Entity Facts From `Entity`

- `Entity` 需要提供：
  - `entity_id()`
  - `entity_type()`
  - `entity_version()`
  - `diff(...)`
- `Entity` 默认内建 replay create/update/delete 语义：
  - `track_create_event(...)`
  - `track_update_event(...)`
  - `track_update_event_from(...)`
  - `track_delete_event(...)`
- `track_update_event_from(...)` 只表达同一实体的一次单步版本更新。
- `track_update_event_from(...)` 要求：
  - `old_state` 与当前实体必须是同一实体
  - 版本必须满足单步 `n -> n + 1`
  - `diff(...)` 不能返回空变更
- `mi_causal_sources()` 表达的是类型级前驱事实规则；具体实例仍由业务字段保存。

### Entity Metadata

- `four_color_archetype() -> FourColorArchetype`
- `mutation_model() -> EntityMutationModel`
- `aggregate_role() -> AggregateRole`
- `use_case_api_surface() -> EntityUseCaseApiSurface`
- `use_case_api_policy() -> EntityUseCaseApiPolicy`
- `is_mi_chain_root() -> bool`
- `mi_causal_sources() -> &'static [MiCausalSourceMetadata]`

### Default Values And Override Semantics

- `four_color_archetype()` 默认是 `FourColorArchetype::Unclassified`
- `mutation_model()` 默认是 `EntityMutationModel::VersionedMutable`
- `aggregate_role()` 默认是 `AggregateRole::Unclassified`
- `use_case_api_surface()` 默认是 `EntityUseCaseApiSurface::LegacyUnconstrained`
- `use_case_api_policy()` 由 `use_case_api_surface()` 映射得出
- `is_mi_chain_root()` 默认是 `false`
- `mi_causal_sources()` 默认是 `[]`

`entity.rs` 对 override 语义有直接说明：

- `VersionedMutable` 是默认值，因为 `Entity` trait 内建了 `entity_version`、`diff`、
  `track_update_event`、`track_delete_event` 这组更贴近“同一实体按版本递增演进”的 helper。
- 若实体是追加记录，应 override 为 `AppendOnlyRecord`。
- 若实体是时点截面，应 override 为 `Snapshot`。
- 若实体只是投影查询模型，应 override 为 `DerivedReadModel`。
- 若实体是聚合边界对外暴露的一致性根，应 override 为 `AggregateRoot`。
- 若实体只在某个聚合内部存在，应 override 为 `AggregateMember`。
- 当 `is_mi_chain_root() == true` 时，trait 注释要求该因果链根补充业务方法来驱动状态演进。

## Boundary Rules Directly Supported By Facts

- `use case` 的编排形状是 `Command + GivenState -> Changes`。
- `use case` 通过聚合根公开业务方法完成业务目标推导。
- 聚合根角色只通过 `AggregateRole::{AggregateRoot, AggregateMember}` 这组元数据表达建模语义。
- `is_mi_chain_root = true` 的实体，需要承担链根业务方法来驱动链内状态演进。
- replay helper 的默认运行语义天然更贴近 `VersionedMutable`。
- `AppendOnlyRecord`、`Snapshot`、`DerivedReadModel` 若要表达不同业务语义，需要显式 override
  `mutation_model()`。

## What Is No Longer Claimed

以下内容不再以 shared hard rule 形式保留，因为它们不能由上述事实源直接推出：

- `use_case` 之间绝不允许互调
- getter 最小暴露策略
- 构造器必须优先 `pub(crate)`
- 禁止公开内部集合
- `helper/query method` 是否应成为主交互面
- “跨聚合一定留在 use case” 这类超出 trait 直接表述的硬禁令
- 任何超出 `Entity` / `CommandUseCase6` 注释与签名的风格化 OO 规则

若后续仍需要这些规范，应放到独立 architecture policy 文档，而不是继续写进 shared
constraints。当前 `Entity` 对 `use case` API 收敛的架构政策落点见：

- `.agents/skills/shared/entity_use_case_api_policy.md`
