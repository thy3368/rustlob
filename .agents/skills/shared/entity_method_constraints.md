# Entity Method Constraints

这是 entity 方法设计的 shared architecture policy。它不是
`lib/common/entity/src/entity.rs` 或 `UseCase` 的代码事实源；分类事实仍以
`.agents/skills/shared/entity_four_color_classification.md` 为准，`use case` / `entity`
边界事实仍以 `.agents/skills/shared/use_case_entity_constraints.md` 为准。

Use this file when deciding whether a method belongs on an entity, how to model lifecycle
transitions, and which entity methods a use case or adapter may reasonably call.

## Method Categories

Split candidate entity methods into these four buckets:

- `FSM Transition` / `FSM 状态迁移`
  - 创建后的推进、成交、关闭、取消、拒绝、消耗、释放、补偿等会改变业务生命周期或合法状态的操作。
  - 领域动作建模为 `Event`，由 `common_entity::Fsm::transition(&mut self, event)` 统一执行。
  - 返回 `Result<(State, Output), Error>`；`State` 是提交后的状态，`Output` 承载成交、释放量、下游单据或其他派生事实。
  - Aggregate-root transition 可以协调同一 aggregate 内的多个对象，但不能替代跨 aggregate 编排。
- `Business Query Method` / `业务查询方法`
  - 回答身份、归属、资产、市场、状态、资格、可用量、剩余量、业务摘要、业务要求等事实的方法。
  - 无副作用，不改变实体状态，也不伪装成状态迁移。
- `Invariant Check Method` / `不变量检查方法`
  - 检查金额守恒、执行状态、版本关系、内部字段自洽、可撤销/可终止资格等方法。
  - 只陈述 self-contained truth condition，不提交状态变更。
- `Technical/Internal Method` / `技术/内部方法`
  - `diff`、`replay_*`、trait plumbing、serde/persistence/wire helpers、私有
    `recompute_*` / `derive_*` 和中间计算。
  - 除非 trait 要求，否则保持 private 或 module-internal。

`FSM Transition` 只替代状态变更型 entity behavior；它不替代 query、invariant、构造/恢复接口，
也不替代纯下游单据派生方法。

## FSM Transition Contract

所有状态变更型 entity 行为必须通过 `common_entity::Fsm` 建模：

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

- `Event` 表达领域意图和输入，不是对字段的裸 patch。
- 先完成全部合法性检查、算术派生、版本检查和 next-state/output 计算，最后一次性提交实体字段。
- 任意错误（包括状态非法、业务不变量失败、算术溢出和版本溢出）都必须使实体保持与调用前完全相等。
- 成功返回提交后的 `State` 和派生 `Output`。`Output` 可为空元组，但若调用方需要成交、释放量、下游单据或其他事实，必须显式建模而不是让调用方重新计算。
- `Fsm::apply(event)` 仅在调用方明确不需要输出时使用；它必须复用同一 `transition` 逻辑，只丢弃输出，不得有第二套状态变更实现。
- 终态应拒绝不合法的后续事件，并返回明确的领域错误。

禁止新增可绕过 FSM 的公开 `cancel`、`consume`、`modify`、`release` 等 mutator。已有兼容方法只能临时委托
`transition`，并在 Rustdoc 中标明待移除；不得在兼容方法中复制或绕过迁移规则。

## Archetype Bias

- `Moment-Interval` 通常优先使用 `FSM Transition` 表达生命周期。
- `Party/Place/Thing` 通常偏向 `Business Query Method` 和 `Invariant Check Method`。
- Promoted `Role` entities 必须证明其生命周期，不能只是 label-only wrapper。
- `Description` objects 通常应避免承载行为密集的实体设计。
- `EntityMethodBias` 是治理信号，不是 allowlist；应结合 four-color archetype 审查方法放置。

## Admission Rules

允许成为 entity 方法的候选包括：

- `FSM Transition`：应用成交、完成匹配、用户取消、按业务原因拒绝、消耗或释放等状态推进。
- `Business Query Method`：身份、所有权、市场/产品身份、生命周期状态、可用量、剩余量、业务摘要或要求。
- `Invariant Check Method`：内部一致、可取消、可匹配、可结算、已终态或其他自洽性判断。
- 有明确业务语义且只生成直接下游单据的派生方法。
- `Technical/Internal Method`：实现上述公共接口所需的小型计算。

不要把以下内容暴露为 use-case-visible entity 方法：

- 只镜像字段的 naked getter，或让调用方重建业务规则的原始集合/字段访问。
- `remaining_qty`、`limit_price`、`crosses_order`、maker price selection 等中间计算；应提供更高层的决策、迁移、要求或摘要。
- 跨 aggregate 编排，例如移动余额、消费 reservation、决定持久化/发布顺序。
- adapter、infra、持久化、replay 解码、SQL、HTTP、CLI 或 wire-format 逻辑。
- 只服务单一 use case、没有可复用业务词汇的 process glue。

## Encapsulation

- 默认使用“私有字段 + 公共业务方法 + 最小只读 getter”。use case 不得直接修改 entity 成员值。
- 状态推进必须调用 `transition`/`apply`；不要补 setter、`*_mut()` getter 或返回内部字段的可变引用。
- getter 只用于读取外部真实依赖的 authoritative business facts，不能机械生成全字段 getter。
- `Vec<_>`、`legs`、`entries`、`children` 等集合默认不直接公开整包原始集合；优先提供业务语义访问器或 summary/query API。

## Constructors And Document Chain

- 构造器可在 adapter 需要重建历史状态时放宽输入，但必须明确“仅装配已校验/回放事实，不承担完整业务校验”。
- `new` 只有表达业务创建入口时才属于 FSM 设计的一部分；纯装配或 replay 构造默认使用 `pub(crate)` 或更窄可见性。
- 上游单据可以生成直接下游业务单据，例如 `SpotTrade -> SettlementTransferVoucher -> BalanceLedgerEntryV2`。
- 这类方法只生成直接下游单据，不执行下游状态、不跨级生成间接单据，也不编排多个 aggregate。
- 余额应用、仓位变更、记账、发布和持久化由 use case 编排；不得塞进上游单据工厂。

## Boundary And Current Calibration

`Fsm` 负责单实体或单 aggregate 的状态转换；`UseCase` 负责跨 aggregate 的 use-case 编排；
`StateDiff` 负责 before/after diff 和 replay。不要用 FSM 取代后两者。

以下实体用于校准不同的 `State` / `Event` / `Output` / `Error` 形态：

- `BalanceV2`：余额 `Open/Closed` 状态，`Freeze`、`UnfreezeAndPay`、扣减/入账、`Close` 事件；输出可为 `()`，错误覆盖余额不足、非法金额、关闭状态和算术溢出。
- `SpotTradeV3`：成交 `Unsettled/Settled` 状态，`Settle` 事件；`settlement_amounts` 是无副作用的派生查询，结算迁移通过 FSM，重复结算返回错误。
- `SpotOrderV7`：订单事件（匹配、替换/填充、修改、取消）返回包含订单后态和 `SpotTradeV3` 的输出；非法状态/输入不得部分修改 taker 或 maker。
- `PerpPositionV3`：`TradeFilled`、`FundingSettled`、`LeverageUpdated`、
  `IsolatedMarginAdjusted` 事件在 `Active` 状态推进仓位；输出分别携带平仓盈亏、资金费金额、
  杠杆更新结果或调整后逐仓保证金。跨余额/订单 aggregate 的协调仍在 use case。

## Rustdoc Requirements

不再使用旧的生命周期行为 Rustdoc tag。public FSM transition 的 Rustdoc 至少说明：

- 事件语义及其输入；
- 允许的起始状态和成功后的状态；
- `Output` 中承载的业务事实；
- 可能的 `Error` 和终态拒绝规则；
- 校验、算术派生和提交顺序，以及失败时实体保持不变。

Query、invariant、技术方法和纯单据派生方法不添加 FSM transition 标识；仍应说明副作用、校验边界和溢出行为（如适用）。

## Tests

为每个 FSM entity 添加聚焦测试：

- 覆盖 `state × event` 的合法与非法转换矩阵。
- 验证成功后的状态、版本、时间戳、输出和不变量。
- 验证失败前后实体完全相等，尤其覆盖算术溢出和版本溢出。
- 验证终态拒绝后续事件。
- 验证 `apply` 与 `transition` 使用相同转换逻辑，只丢弃输出。
- 对 query/invariant 添加正反例；对纯单据派生添加无副作用和金额/数量边界测试。
- 构造器若属于实体契约，验证所有字段均被正确保存；必要时使用小型 fixture builder 和 `proptest-entity`。

## Adapter And Use Case Cleanup

新增或迁移 FSM 后，搜索 adapter/use case 中重复的裸 struct 构造和业务计算：

```bash
rg -n "EntityName \\{|qty \\* price|reserved_quote|account_id ==" path/to/crate
```

只在依赖方向清晰时替换：core entity 不依赖 adapter；use case 调用 entity 的 FSM/query/invariant；outbound adapter
负责装配实体；use case 不调用另一个 use case。共享业务含义应复用 entity 方法或更高层编排。
