# Entity Use Case API Policy

这是 architecture policy，不是 `lib/common/entity/src/entity.rs` 或 `CommandUseCase6` 的代码事实。

它在工程上的落点是：

- `Entity::use_case_api_surface()`
- `Entity::use_case_api_policy()`

它的执行依赖独立 checker 与 reviewer judgement，不依赖 Rust trait 本身自动禁止额外 `pub fn`。

## Purpose

- 收紧 `use case -> entity` 的可见面，避免 `use case` 直接依赖聚合内部材料。
- 不完全禁止查询；允许少量稳定、高语义、可复用的业务查询。
- 把“聚合根对外公开什么”从隐含风格偏好，提升为可标注、可检查的工程契约。

## Core / Adapter / Infra

### Core

- `use_case`
  - 负责 `command + authoritative state -> changes` 的业务编排。
- `entity`
  - 负责聚合内业务事实、不变量和稳定高语义业务 API。

### Adapter

- `inbound`
  - 只把外部输入翻译到 `use_case`。
- `outbound`
  - 只实现 `use_case` 定义的 port，不定义业务规则。

### Infra

- framework、SDK、runtime、driver、持久化与网络机制。

## Architecture Views

### Role View

```text
core
  use_case
  entity

adapter
  inbound
  outbound

infra
  frameworks, SDKs, drivers, runtimes
```

### Source Dependency View

```text
inbound -> use_case -> entity
outbound -> port <- use_case
outbound -> infra
```

### Call Flow View

```text
inbound -> use_case -> outbound -> infra
```

`entity` 不知道 `command/query`，`inbound` 不承载核心业务规则。

## Hard Rules

1. `use case` 面向 `entity` 的首选入口必须是高语义业务 API，而不是聚合内部材料。
2. 被标记为 `MinimalBusinessApi` 的实体，不应把原始子集合直接公开给 `use case`。
3. 被标记为 `MinimalBusinessApi` 的实体，不应把 `pub fn new(...)` 这类宽装配构造器作为主要 use case 入口。
4. `entity` 可以提供查询，但查询应优先返回业务结论、summary 或不变量判断，而不是 child material。
5. `use case` 可以组合多个高语义 entity API，但不应依赖 `legs()/children()/entries()` 这类聚合内部形状。
6. 若某个集合型查询确有必要，对外返回的也应是稳定 summary/view，而不是内部 child 引用或内部 ID 材料。
7. checker 负责识别明显违规形状；无法静态证明的灰区由 reviewer 按 `core.use_case -> core.entity` 边界判断。
8. `LegacyUnconstrained` 仅表示当前暂不强治理，不表示该 API 设计被推荐长期保留。

## Allowed API Shape

允许的对外形状应优先是这些高语义方法：

- `is_*`
- `can_*`
- `amount_*`
- `total_*`
- `fee_amount_*`
- `apply_*`
- `reserve_*`
- `release_*`
- `transfers_for_purpose(...) -> Vec<SettlementTransferSummary<'_>>`
- `amount_received_by_for_purpose(...) -> Option<u64>`
- `fee_amount_paid_by(...) -> Option<u64>`

这些 API 的共同点：

- 名字表达业务结论，而不是内部容器名。
- 返回值是布尔、数量、状态判断或稳定 summary。
- `use case` 依赖的是业务语义，而不是聚合内部装配细节。

## Disallowed API Shape

以下形状在 `MinimalBusinessApi` 实体上视为不合适：

- `principal_legs() -> Vec<&Leg>`
- `children() -> &[Child]`
- `entries() -> impl Iterator<Item = &Entry>`
- `all_entries() -> Vec<&Entry>`
- `raw_legs() -> &[SettlementTransferLeg]`
- `pub fn new(...)` 作为宽装配入口直接对 `use case` 公开
- `pub fn from_parts(...)`
- `pub fn assemble(...)`

这些 API 会让 `use_case` 直接绑定聚合内部材料形状，削弱 `entity` 作为 `core` 业务边界的收敛能力。

## Checker Mapping

### Checker 直接判断

- `impl Entity for X` 中显式 `use_case_api_surface() -> EntityUseCaseApiSurface::MinimalBusinessApi`
- 公开宽构造器：
  - `pub fn new(...)`
  - `pub fn from_parts(...)`
  - `pub fn assemble(...)`
- 公开原始集合材料：
  - 方法名命中 `*_legs` / `*_children` / `*_entries` / `all_*` / `raw_*` / `iter_*`
  - 或明确是 `legs` / `children` / `entries`
  - 且返回形状命中 `Vec<&...>` / `Vec<...>` / `&[...]` / `impl Iterator<Item = &...>`

### Reviewer Judgment

- 某个 `amount_*` / `total_*` 是否真的表达稳定业务结论
- 某个 summary type 是否足够稳定，还是仍在泄漏内部材料
- 某个查询是否本应属于 `use_case` 编排而不是 `entity` API

## Local Exemption

checker 可支持局部豁免注释：

```rust
/// entity-use-case-api: allow-raw-collection
pub fn transfers_for_purpose(...) -> Vec<SettlementTransferSummary<'_>> { ... }
```

约束：

- 只用于 summary/view collection 的历史兼容场景。
- 不允许为 `Vec<&Leg>`、`&[Child]`、`Iterator<Item = &Entry>` 这类原始内部材料开豁免。

## Violations

- 若 `use_case` 需要知道内部 `leg_id`、`entry_id`、child 排列顺序，说明 `core.entity` 收敛不足。
- 若 `inbound/outbound adapter` 开始依赖这些 raw child API 再反向影响 `use_case` 设计，说明边界已被外层机制污染。

## Minimal Restructuring Advice

- 先补一个高语义 summary/query API，再逐步移除 `use_case` 对 raw collection 的依赖。
- 宽构造器若仅供同模块装配，优先收回到 `pub(crate)` 或更小可见性。
- 只有显式 opt-in 到 `MinimalBusinessApi` 的实体才进入强治理；旧实体先标注，再收敛。
