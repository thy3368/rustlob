## Purpose

`lib/common/entity/src/entity_v2/entity.rs` 是实体分类元数据的事实源。
本文说明如何使用这些 Rust 元数据判断候选领域对象的四色原型、是否应成为独立实体，
以及它在变更模型、聚合边界和财务链路中的角色。

本文不维护第二份 enum、默认值或字符串标签表。枚举变体、默认实现和 `as_str()` 标签必须以
Rust 事实源为准，尤其是：

- `FourColorArchetype`
- `EntityMutationModel`
- `AggregateRole`
- `FinancialClassification`
- `FieldDiff` 和 `Entity` trait

本文只回答以下问题：

- 对象在四色建模中是什么
- 对象是否值得拥有独立实体身份
- 对象成为实体后应使用哪些现有 `Entity` 元数据描述
- 分类如何影响 entity、use case 和 adapter 的边界

## Independent Modeling Axes

以下四个分类轴相互独立，不能从其中一个机械推导另一个：

- `four_color_archetype()`：对象在领域中的四色原型。
- `mutation_model()`：对象随业务事实演进的方式。
- `aggregate_role()`：对象在聚合一致性边界中的角色。
- `financial_classification()`：对象在财务或账务链路中的职责；仅对相关对象要求分类。

例如，`MomentInterval` 既可能是持续版本化的可变生命周期实体，也可能是创建后不再原地修改的
`AppendOnlyRecord`。同样，`Description` 是否实现 `Entity` 取决于 identity、version、replay、
audit 或生命周期需求，而不是由四色原型单独决定。聚合角色和财务分类也必须依据各自语义判断。

## Four-Color Mapping For Entity

- `FourColorArchetype::MomentInterval`
  - 表达一次业务活动、事件或时间区间。
  - 可承载可变生命周期和状态迁移，也可表达 append-only 的既成业务事实。
- `FourColorArchetype::PartyPlaceThing`
  - 表达领域中的人、组织、地点、物或其他具有业务身份的核心对象。
  - 可成为聚合根、聚合成员或被其他聚合引用的实体；不能仅凭该原型推导聚合角色。
- `FourColorArchetype::Role`
  - 默认表达 Party/Place/Thing 在特定上下文中的关系语义。
  - 只有角色本身具有独立业务 identity，以及独立约束、审计或生命周期意义时，才提升为实体。
- `FourColorArchetype::Description`
  - 表达分类、规则、能力、配置、定价或产品定义等说明性信息。
  - 通常适合作为 value object、policy 或 description；当它需要独立 identity、version、replay、
    audit 或生命周期语义时，仍可实现 `Entity`。

`FourColorArchetype::Unclassified` 只用于兼容旧实体和未治理实体，不能作为新的设计结论。

## FieldDiff Versus Entity

先判断对象是否只需要字段级变化表达：

- 纯值对象若只需复用 `EntityFieldChange`，可以仅实现 `FieldDiff`。
- 只有需要独立 identity、version/LSN、replay、audit 或生命周期语义的对象，才应实现 `Entity`。
- 实现 `FieldDiff` 不代表对象自动拥有实体身份，也不要求为它虚构聚合角色或生命周期。
- 技术临时对象、传输结构和 adapter DTO 不应仅为复用 diff 而提升为领域实体。

## Classification Procedure

1. 判断对象是否是值得独立记忆的业务事实，还是纯值、描述、关系或技术结构。
2. 检查它是否需要独立 identity、version/LSN、replay、audit 或生命周期语义。
3. 若只需字段变化表达，选择 `FieldDiff`；满足独立实体语义时才选择 `Entity`。
4. 独立判断 `four_color_archetype()`，不要由方法形态或持久化方式反推四色原型。
5. 独立判断 `mutation_model()`：当前可变状态、追加事实、时点快照或派生读模型。
6. 独立判断 `aggregate_role()`：聚合根、聚合成员，或在尚未治理时使用兼容默认值。
7. 仅当对象进入财务或账务链路时，判断 `financial_classification()`。
8. 所有枚举变体、默认值和标签回到 Rust 事实源核对，不在本文扩展元数据。

## Entity Output Contract

任何 skill 对候选实体的分类输出至少应包含：

- `business_object`
- `why_it_is_an_entity`
- `four_color_archetype`：对应 `four_color_archetype()`
- `mutation_model`：对应 `mutation_model()`
- `aggregate_role`：对应 `aggregate_role()`

若对象属于财务或账务链路，还必须包含：

- `financial_classification`：对应 `financial_classification()`

结构化字段必须直接映射到现有 `Entity` trait 方法。需要字符串标签时，使用事实源中相应枚举的
`as_str()` 语义；不要在本文或下游 skill 输出中发明标签。默认值同样以 trait 实现为准，默认值
只提供兼容行为，不代替显式建模结论。

## Boundary Implications

- `MomentInterval` 的可变生命周期通常由 entity 状态迁移表达；append-only 事实则不应为了匹配
  原型而虚构 mutator。
- `PartyPlaceThing` 常承载身份、资格、所有权、不变量和派生业务事实，但具体方法仍由业务职责决定。
- `Role` 默认停留在关系语义；提升后也只能承担其独立身份和生命周期内的规则。
- `Description` 通常不承载行为密集的状态迁移，但具备独立治理需求时可以是实体。
- 聚合内部不变量属于聚合边界；跨聚合顺序、持久化和发布由 use case 编排。
- adapter 只做边界适配，不因分类结果获得承载业务规则的理由。

## Anti-Patterns

- 把四色原型机械映射成变更模型、聚合角色或财务分类
- 把所有 `MomentInterval` 都建模为可变状态机，或都建模为 append-only 记录
- 因为对象是 `Description` 就忽略其真实的 identity、version、replay 或审计需求
- 从角色名称直接创建实体，却没有独立业务身份
- 把纯值对象、临时技术对象或 adapter DTO 提升为 `Entity`
- 在伪造的 `MomentInterval` 方法中隐藏跨聚合流程动作
- 在 Rust 事实源尚无对应方法时，先在 skill 输出中添加新的规范分类字段
- 在本文复制 enum 标签或默认值，而不是读取事实源的实现

## Trading Calibration Examples

- `SpotAccount`：`PartyPlaceThing + AggregateRoot`；变更模型使用事实源默认值时仍应理解为独立轴。
- `CexOptionInstrument`：`Description`；它因独立 identity、version 和 replay 需求而实现 `Entity`，
  不能因为名为 instrument 就一概归为 `PartyPlaceThing`。
- `SpotTrade`：`MomentInterval + AppendOnlyRecord + BusinessVoucher`；其聚合角色仍需独立判断。
- `BalanceLedgerV2`：`MomentInterval + AppendOnlyRecord + LedgerEntry`；其账本职责不由四色原型推导。
- `Maker` / `Taker`：默认是 `Role` 关系语义；只有具备独立业务身份时才提升为实体。
- `OrderType` / `RiskRule` / `FeeSchedule`：通常是 `Description`，是否实现 `Entity` 仍取决于独立治理需求。
