---
name: write-entity
description: Write or refactor RustLOB core entities, including entity classification, Fsm lifecycle transitions, business queries, invariants, replay metadata, and direct downstream document derivation. Use for structs and focused entity tests under core/entity crates; keep cross-aggregate orchestration in use cases.
---

# Write Entity

为 RustLOB 的 core entity 编写可复用业务规则。实体负责身份、聚合内不变量、业务查询、合法状态迁移、
回放元数据，以及稳定的直接下游单据派生；use case 负责 command admission、跨聚合编排、持久化与发布。

## 必读约束

开始设计或修改前，按任务读取以下 shared reference：

- 分类、`Entity` / `FieldDiff` 取舍及四色元数据：
  `.agents/skills/shared/entity_four_color_classification.md`
- aggregate、use case 边界及 replay/version 事实：
  `.agents/skills/shared/use_case_entity_constraints.md`
- 方法准入、`Fsm` 契约、封装、单据派生与测试要求：
  `.agents/skills/shared/entity_method_constraints.md`

这些文件是详细规则的维护点；不要在本入口复制另一套约束。需要更强的分层政策时，读取专用架构
文档或 skill。独立属性测试属于 `proptest-entity`，本 skill 只覆盖实体实现、Rustdoc 和聚焦单测。

## 工作流

1. 阅读目标实体、相邻实体，以及构造或使用它的 use case / adapter；确认哪些规则是领域事实。
2. 独立判断四色原型、mutation model、aggregate role；只有需要独立 identity、version/LSN、
   replay、audit 或生命周期语义时才实现 `Entity`，纯字段变化可只实现 `FieldDiff`。
3. 将候选方法分为 FSM transition、业务查询、不变量检查、技术/内部方法或直接下游单据派生；按
   shared method constraints 决定可见性与归属。
4. 对状态变更定义领域 `Event`，通过 `Fsm<State, Event, Output, Error>` 实现；不要新增可绕过
   FSM 的公开生命周期 mutator。
5. 为调用方真正需要的派生事实定义 `Output`。调用方明确不需要输出时才使用 `apply`。
6. 实现 `Entity` / `FieldDiff`、聚合与 replay/version 元数据时，以 trait 和 shared reference 为准；
   不把版本号误当普通字段 diff。
7. 若涉及单据链，只由上游实体纯粹派生直接下游单据；跨聚合应用、持久化和发布仍由 use case 编排。
8. 为公开业务语义、事件、输出、错误和边界行为补充中文 Rustdoc，并添加聚焦实体测试。
9. 清理 use case / adapter 中已被实体方法替代的重复业务计算，但不把 adapter/infra 细节移入 core。
10. 对触及的 crate 执行定向格式化、编译和测试。

## FSM 状态迁移

状态变更统一实现 `common_entity::Fsm`：

```rust
impl Fsm for Entity {
    type State = EntityState;
    type Event = EntityEvent;
    type Output = EntityOutput;
    type Error = EntityError;

    fn transition(
        &mut self,
        event: Self::Event,
    ) -> Result<(Self::State, Self::Output), Self::Error>;
}
```

- `Event` 表达领域意图，不是字段 patch。
- 提交前完成全部状态校验、不变量检查、checked arithmetic、版本计算和 output 推导。
- 任意错误都必须使实体与调用前完全相等；复杂迁移可先在 clone/局部 next state 上推导，成功后提交。
- 成功结果同时给出提交后的 `State` 与显式 `Output`。不要让调用方从变化后的字段重新推导成交、
  释放量、盈亏或下游事实。
- `Fsm::apply` 必须复用 `transition` 且只丢弃 output，仅用于明确不需要 output 的调用点。
- 终态必须拒绝非法后续事件并返回明确领域错误。

当前代码校准样例：

- `lib/example/entity/src/entity/account/balance_v2.rs`：余额 FSM 与原子余额/版本变更。
- `lib/example/entity/src/entity/spot/spot_trade_v3.rs`：简单终态迁移和业务查询。
- `lib/example/entity/src/entity/spot/spot_order_v7.rs`：返回成交等业务结果的订单 FSM。
- `lib/example/entity/src/entity/perp/trade/perp_position_v3.rs`：交易、资金费、杠杆和逐仓保证金等多事件 FSM。

样例用于校准当前接口，不要求机械复制其字段、可见性或聚合边界。

## 单据链与边界

保留并按需复用以下业务链：

`SpotTrade -> SettlementTransferVoucher -> BalanceLedgerEntryV2`

上游实体只能派生直接下游单据，例如 `SpotTrade -> SettlementTransferVoucher` 或
`SettlementTransferVoucher -> BalanceLedgerEntryV2`。派生方法不得加载或修改其他聚合，不得应用余额，
也不得决定持久化、发布或下游行为执行顺序。下游单据维护自身不变量，use case 编排整条跨聚合流程。

## 测试与验证

根据实体能力覆盖：

- `state x event` 合法与非法矩阵；
- 成功后的状态、output、版本和业务事实；
- 每类错误后的完整实体相等性，特别是中途校验或算术失败；
- 终态拒绝、金额/数量边界以及 Decimal/版本 overflow；
- `apply` 与 `transition` 对状态提交的一致性；
- identity、partition、LSN/version、field diff 和 replay event 元数据；
- 单据派生的字段映射与下游不变量边界。

优先使用 inline `#[cfg(test)]` 聚焦单测；需要完整业务状态枚举时使用 `proptest-entity`。运行触及
package 的格式化与测试，例如：

```bash
rustfmt path/to/entity.rs
cargo test -p crate_name
```

若 workspace 级检查受无关失败阻塞，记录准确失败并保持验证范围与改动相称。
