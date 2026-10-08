# DDD 3.0：从 RustLOB Example 代码看可计算的领域架构

## 摘要

本文把 `design/支持cqrs,es,ddd,clean的领域架构，业务开发只需要关注use_case, entity设计，use_case只完成计算操作.jpg` 作为入口，但不止讲图，而是回到 `lib/example` 的真实代码：RustLOB 的 DDD 3.0 可以理解为一种“use case 原子计算 + IO 外置”的领域架构。

这里的“DDD 3.0”不是通用行业术语，而是本文对 RustLOB 当前工程实践的命名。它强调业务开发主要设计 `use_case` 和 `entity`；`command_inbound`、`event_inbound`、`event_handler`、`executor`、`outbound_adapter`、infra 负责把外部世界接进来、执行起来、落出去。

核心观点：

- `use_case` 是一个业务操作的原子计算单元。
- 原子计算不是数据库事务，而是业务决策不可再拆：一次输入、一次状态快照、一次校验、一次 changes 输出、一次事件投影。
- `command_inbound` 和 `event_inbound` 是两类入口，不是业务规则承载层。
- `event_handler` 是事件入口后的翻译与分发层，不是 use case。
- IO 被放在 executor/outbound：load、persist、replay、publish。
- 数据变化必须显性声明：`GivenState`、`Changes`、`UpdatedEntityPair`、`ReplayableChanges` 都是设计本体。
- 通过 use case 原子计算，避免同一个业务操作被拆成跨节点分布式事务。

本文只讨论 `lib/common/entity` 暴露的 MI use case 路径：`StateMachineV2Unchecked`、`StateMachineOwnedV2Diff`、`StateMachineExecutor`、`StateSource`、`StateSink`、`ReplayableChanges`。

## 1. DDD 3.0：从领域建模到原子计算核心

传统 DDD 在工程落地时容易退化成几种形态：

- service 变成业务逻辑容器。
- handler、repository、adapter 混入业务判断。
- 数据变化藏在函数内部或数据库副作用里。
- 一个业务操作被拆到多个服务或节点后，需要分布式事务兜底。

RustLOB example 选择的方向是把业务动作收敛为可计算、可审查、可回放的 `use_case`：

- `entity` 承载稳定业务规则和不变式。
- `use_case` 接收 command 与 given state，做业务校验和变化计算。
- `executor` 固定执行顺序，但不承载业务规则。
- `outbound_adapter` 负责加载状态、持久化事件、回放状态、发布消息。
- `command_inbound` 把 HTTP/CLI/exchange action 翻译成 command。
- `event_inbound` 接收外部或内部事件。
- `event_handler` 识别事件语义，选择投影或触发后续 executor。

对应到 Clean Architecture：

- `core`
  - `use_case`：业务动作，输入是 command，依赖显式 given state，输出显式 changes。
  - `entity`：实体、聚合根、业务行为、不变式。
- `adapter`
  - `inbound`：HTTP/CLI/Event/Aeron 等入口到 command/event 的翻译。
  - `outbound`：实现 use case 所需的状态源和事件副作用 port。
- `infra`
  - DB、消息队列、Aeron、HTTP runtime、序列化库、SDK、存储驱动。

三种视图要分清：

```text
role view:

core
  policies, use_case, entity

adapter
  glue / translation layer between core and infra

infra
  frameworks, SDKs, drivers, runtimes, third-party tools
```

```text
source dependency view:

inbound -> use_case -> entity
outbound -> port <- use_case
outbound -> infra
```

```text
call flow view:

command_inbound -> executor -> outbound.load_given_state
                -> use_case/entity
                -> outbound.persist/replay/publish
```

`role view` 讲职责位置，`source dependency view` 讲源码依赖方向，`call flow view` 讲运行时调用顺序。很多架构混乱来自把这三件事混在一起。

## 2. 什么是 Use Case 原子计算

一个 use case 表示一个不可再拆的业务动作。这里的“原子”不是数据库 ACID，也不是“必须在一台机器里写完所有副作用”，而是指业务决策不可再拆。

在 `lib/common/entity/src/use_case/state_machine_v2.rs` 中，`StateMachineV2Unchecked` 固定了核心语义：

- `Command`：本次业务动作输入。
- `StateGiven`：本次计算依赖的完整业务状态快照。
- `check_command`：无状态输入校验。
- `validate_state_given`：基于状态的业务校验。
- `compute_state_changed_unchecked`：计算 after truth。

`StateMachineOwnedV2Diff` 再把 after truth 转成 case 级 diff：

- `StateDiff`：本次业务动作产生的显式 changes。
- `compute_state_diff_with_context`：按固定顺序执行 validate、compute after、diff。

`ReplayableChanges` 再要求 changes 能投影成可持久化、可回放事件：

```text
Command + GivenState
  -> check_command
  -> validate_state_given
  -> compute_state_changed_unchecked
  -> StateDiff / Changes
  -> to_replayable_events
```

原子计算不包含：

- 远程调用。
- 数据库写入。
- 消息发送。
- 多节点提交协调。
- HTTP response shaping。

判断标准很简单：如果一个“业务动作”的正确性必须靠多个节点边执行边提交，它还没有被收敛成 use case 原子计算。

## 3. 先读图：Command 与 Event 两条入口

设计图里有两条入口：command 和 event。它们进入系统的方式不同，但最后都必须回到 use case/entity 的显式计算边界。

Command 路径：

```text
command_inbound -> executor -> outbound.load_given_state
                -> use_case -> entity
                -> changes -> replayable events
                -> outbound.persist/replay/publish
```

Event 路径：

```text
event_inbound -> event_handler -> executor -> use_case/entity
                              -> changes/events
                              -> outbound.persist/replay/publish
```

也可能是纯投影路径：

```text
event_inbound -> event_handler -> projection/entity helper -> outbound.publish/update view
```

关键边界：

- inbound 负责接收和翻译。
- event_handler 负责事件语义分发。
- executor 负责编排 IO 顺序。
- use_case 只负责原子计算。
- entity 负责业务行为与不变式。
- outbound 负责状态加载和副作用落地。

## 4. 计算与 IO 分离

`lib/common/entity/src/use_case/executor.rs` 里的 `StateMachineExecutor` 把运行时顺序固定为：

```text
check_command
-> load_given_state
-> compute_state_diff_with_context
-> to_replayable_events
-> persist
-> replay
-> publish
```

其中 `compute_state_diff_with_context` 内部再做：

```text
validate_state_given
-> compute_state_changed_unchecked
-> do_compute_state_diff
```

这个顺序说明一个重要事实：use case 不是去 DB 里边查边改的 service。它只接收已经加载好的 `GivenState`，然后返回业务 changes。

use case 内允许：

- `check_command`
- `validate_state_given`
- `compute_state_changed_unchecked`
- `do_compute_state_diff`
- `to_replayable_events`
- 调用 entity 的业务行为方法

use case 内不允许：

- 查 DB。
- 发消息。
- 调 SDK。
- 拼 HTTP response。
- 隐式修改外部状态。

IO 统一放到 outbound port：

- `StateSource::load_given_state`
- `StateSink::persist`
- `StateSink::replay`
- `StateSink::publish`

所以 executor 是流程编排器，不是业务规则容器；outbound 是外部机制适配器，不是业务真相来源。

## 5. Use Case 原子计算如何避免跨节点分布式事务

一个业务动作先在 use case 内完成完整决策：

1. 输入 command。
2. 由 outbound 加载 authoritative given state。
3. use case 校验 command 与 state。
4. use case 计算所有 after changes。
5. changes 投影成 replayable events。
6. executor 交给 outbound persist/replay/publish。

业务正确性不依赖多个节点边算边提交。use case 的输出是一次完整业务决策，而不是一串远程副作用。

跨系统协作通过事件衔接：

- 当前 use case 先产出本地可持久化事件。
- 后续投影、消息、下游动作由 replay/publish/event_handler 处理。
- event handler 如果要引发新的业务变化，也要重新进入另一个 use case 的原子计算边界。

这样把“同一个业务操作的一致性”收敛到单个计算边界内，把跨节点部分变成事件驱动的后续处理。

## 6. 数据变化必须显性声明

RustLOB example 里的 use case 不是只返回 `Ok(())`，也不是直接把变化写进数据库。它必须把变化显性声明出来。

几个关键类型：

- `Command` 显性声明业务输入。
- `StateGiven` 显性声明 use case 依赖的业务状态。
- `StateChanged` 显性声明 after truth。
- `StateDiff` / `Changes` 显性声明本次业务动作产生的数据变化。
- `UpdatedEntityPair<T>` 显性声明 before/after。
- `ReplayableChanges` 显性声明 changes 如何投影为事件。

review use case 时可以直接问：

- 这个 use case 读了哪些状态？
- 它改了哪些 entity？
- 哪些变化会被持久化？
- 哪些事件会被 replay/publish？
- 这个业务动作是否已经形成原子计算？
- 这个业务动作是否被错误拆成了跨节点事务？

这些问题在 `lib/example/use_case` 里可以直接从类型定义中得到答案。

## 7. 小案例：`PlaceOnlySpotOrderV2UseCase`

先看一个相对小的下单意图创建案例：`lib/example/use_case/src/use_case/trading/spot/place_only_spot_order_v2.rs`。

它的 command 是 `PlaceOnlySpotOrderV2Cmd`：

- `Single(PlaceOnlySpotOrderV2OrderCmd)`：创建单个订单。
- `NormalTpsl { parent, children }`：创建普通 TPSL 父子订单。

它的 given state 是：

```rust
pub type PlaceOnlySpotOrderV2State = ();
```

这不是偷懒，而是在表达一个业务事实：place-only 只创建 pending order，不读取盘口、余额或仓位。没有需要加载的业务状态，就显式写成空 state。

它的计算分成三段：

- `check_command`：检查账户、订单号、symbol、资产 ID、价格、数量、TIF、trigger 字段。
- `validate_state_given`：对 `normalTpsl` 做父子订单业务关系校验。
- `compute_state_changed_unchecked`：构造 `SpotOrderV2`，并为父子订单设置 `SpotOrderGroupRelation`。

它的 changes 是：

- `PlaceOnlySpotOrderV2AfterChanges`：after truth。
- `PlaceOnlySpotOrderV2Changes`：最终 diff。

它的 replayable events 来自 `ReplayableChanges`：

```text
Single -> created_order.track_create_event()
NormalTpsl -> parent create event + children create events
```

这个案例很小，但边界很清楚：即使只是创建 pending order，也不通过 DB insert 隐藏业务变化，而是先产生 `created_order` changes，再投影成 create event。

## 8. 复杂案例：`PlaceHyperliquidPerpOrderUseCase`

再看复杂一点的合约下单：`lib/example/use_case/src/use_case/trading/derivatives/hyperliquid_perp/execution/place_perp_order.rs`。

`PlaceHyperliquidPerpOrderCmd` 显性表达 Hyperliquid 下单意图：

- `party_id`
- `asset`
- `symbol`
- `is_buy`
- `size`
- `reduce_only`
- `execution`
- `cloid`

`PlaceHyperliquidPerpOrderState` 显性声明它依赖的状态：

- `trading_enabled`
- `next_order_sequence`
- `account_id`
- `margin_balance`
- `margin_asset_id`
- `market_rules`
- `position`

`PlaceHyperliquidPerpOrderChanges` 显性声明产出：

- `created_order`
- `created_balance_ledger_entry`
- `updated_margin_balances`

业务校验集中在 `validate_state_given`：

- 市场是否允许交易。
- command 账户是否匹配已加载状态。
- symbol 是否在市场规则中可交易。
- 数量是否满足最小数量。
- 保证金余额是否属于当前账户与保证金币种。
- 仓位账户、asset、symbol 是否匹配。
- 仓位状态是否自洽。
- 杠杆是否合法。
- 非 reduce-only 下是否有足够保证金。

业务计算集中在 `compute_state_changed_unchecked`：

- 根据当前仓位推导 Open/Increase/Reduce/Close 意图。
- 调用 `HyperliquidPerpOrder::place` 创建订单事实。
- 如需冻结保证金，生成冻结流水并应用到 margin balance。
- 返回订单、流水、余额 before/after。

最后 `ReplayableChanges` 把 changes 投影成事件：

- 订单 create event。
- 可选余额流水 create event。
- 保证金余额 update event。

这个 use case 的重点不是“它很复杂”，而是复杂性没有泄漏到 adapter 或数据库事务里。合约下单不是“订单服务调用余额服务再调用仓位服务”的分布式事务链，而是一个 use case 对已加载状态做原子业务决策。

## 9. Entity：领域规则的稳定承载者

`lib/example/entity/src/entity/spot/spot_order_v2.rs` 展示了 entity 应该承担什么。

`SpotOrderV2` 持有订单自身的业务行为：

- `belongs_to_account`
- `can_enter_matching`
- `has_consistent_execution_state`
- `ensure_matchable`
- `crosses_maker`
- `fill`
- `match_with_makers`
- `cancel`

文件里还明确写了原则：

```text
use case 应通过这些聚合根方法推进订单生命周期，不得绕过它们直接修改
status、filled_qty、status_reason、version。
```

这就是 entity 的价值：订单生命周期、撮合资格、成交推进、撤单释放语义，不应该散落在 inbound、executor 或 outbound 里。

entity 不知道：

- command wire format。
- HTTP request/response。
- DB 表。
- 消息队列。
- executor。

entity 只知道自己的业务状态和可执行行为。use case 可以协调多个 entity，但不应该绕过 entity 直接改生命周期字段。

## 10. Event Handler：事件入口后的业务分发层

event handler 容易被误认为 use case。RustLOB example 把它放在更外层：它处理事件语义，但不成为业务原子性边界。

看两个文件：

- `lib/example/use_case_executor/src/trading/spot/on_event/match_spot_order_created.rs`
- `lib/example/use_case_executor/src/trading/spot/on_event/spot_trade_created.rs`

`MatchSpotOrderCreatedEventHandler` 做的事是：

1. 判断 event 是否是订单 created event。
2. 从 event 字段解析 `MatchSpotOrderV3Cmd`。
3. 调用 `execute_match_spot_order_v3`。

也就是说，订单 created event 触发撮合时，新的业务变化仍然进入 `MatchSpotOrderV3UseCase`，而不是在 event handler 里直接撮合。

`SpotTradeCreatedEventHandler` 则是投影型 handler：

1. 判断 event 是否是成交 created event。
2. 解析 K 线聚合需要的最小成交事实。
3. 调用 `SpotKlineAggregator` 生成 K 线更新。

再看 `lib/example/veldra_inbound_adapter/src/on_event/spot/on_aeron_match_spot_order.rs`：Aeron adapter 负责配置、订阅、fragment 解码和运行时循环；真正的事件语义处理委托给 `MatchSpotOrderCreatedEventHandler`。

所以 event handler 的定位是：

- `event_inbound` 负责接收和解析事件载体。
- `event_handler` 负责识别事件语义，选择投影或后续 executor。
- `event_handler` 不承载核心业务计算。
- 如果事件会导致新的业务变化，仍应通过 use_case/entity 显性计算。
- event handler 是事件驱动编排点，不是业务原子性边界。

## 11. Executor 与 Outbound：IO 编排层

`lib/example/use_case_executor/src/trading/spot/place_only_spot_order_v2_executor.rs` 是 executor 层的典型形态：

```text
execute_place_only_spot_order_v2
-> execute_place_only_spot_order_v2_with_outbound
-> StateMachineExecutor.execute(...)
```

它不写业务规则，只把 use case、command、outbound 交给通用 executor。

outbound 则实现状态加载与副作用端口。比如：

- `lib/example/outbound_adapter/src/trading/place_only_default.rs`
- `lib/example/outbound_adapter/src/trading/place_default.rs`

`place_only_default.rs` 为 `PlaceOnlySpotOrderV2UseCase` 实现：

- `StateSource::load_given_state`
- `StateSink::persist`
- `StateSink::replay`
- `StateSink::publish`

测试里的 fake outbound 也很有代表性。`place_only_spot_order_v2_executor.rs` 的测试用 `FakePlaceOnlySpotOrderV2Outbound` 记录 persist/replay/publish 是否按预期被调用。这说明 IO 是可替换机制，业务原子计算才是稳定核心。

复杂一点的 `place_match_spot_order_v2_executor.rs` 也是同样结构：fake outbound 提供 maker orders、settlement balances、fee account 等 given state；use case 产出 created/updated events；executor/outbound 负责落地顺序。

## 12. Command Inbound 与 Event Inbound

Command inbound 的职责是外部命令到 core command 的翻译。

例如 `lib/example/veldra_inbound_adapter/src/command/exchange/actions/order.rs`：

- 校验 action type、签名公共字段、grouping、cloid、wire order type。
- 将 wire order 转成 `PlaceOnlySpotOrderV2OrderCmd`。
- 根据 grouping 组装 `PlaceOnlySpotOrderV2Cmd`。
- 调 `execute_place_match_spot_order_v2`。
- 把执行结果映射成 exchange reply。

这里可以做 wire-level contract validation，但不能把订单生命周期、余额冻结、撮合规则直接写在 inbound 里。那些规则属于 use case/entity。

Event inbound 的职责是外部事件载体到内部事件对象的翻译。

例如 Aeron 入口：

- 读取 env config。
- 建立 Aeron subscription。
- 轮询 fragment。
- 把 payload decode 成 `EntityReplayableEvent`。
- 交给 event handler。

二者共同约束：

- 不写核心业务规则。
- 不直接修改领域状态。
- 不绕过 use_case/entity。
- 不把 infra 的 payload 形态传播到 core。

## 13. CQRS / ES / Clean Architecture 如何合流

CQRS：

- command use case 产生 changes/events。
- query/read side 可以消费 replay 结果或 read model。
- command path 不把 HTTP reply 当成业务真相。

Event Sourcing：

- changes 通过 `to_replayable_events` 显性投影。
- outbound 负责 persist/replay/publish。
- event handler 可以消费 replayable event 做投影或触发后续 use case。

Clean Architecture：

- inbound/event_handler/executor 是外层机制。
- use_case/entity 是内层策略。
- source dependency 指向 core。
- call flow 可以从外到内再到外，但业务规则不外泄。

一个容易误解的点是：`executor -> outbound -> infra` 出现在 call flow 里，不代表 core 依赖 infra。依赖方向仍然是 outbound adapter 实现 core 所需 port。

## 14. 结论：业务开发只关注 Use Case 与 Entity

开发新业务时，优先设计：

- command/query。
- given state/read model。
- entity 行为。
- after truth。
- changes/diff。
- replayable events。

不优先设计：

- DB 表。
- HTTP DTO。
- 消息 topic。
- SDK payload。
- 跨服务事务链。

DDD 3.0 的关键不是层数更多，而是：

- use case 原子计算。
- IO 外置。
- 数据变化显性。
- 单一业务操作尽量收敛为单一 use case 决策。
- 业务边界可测试、可回放、可审查。

在 RustLOB example 里，这套实践已经落在类型和执行顺序中：`Command + GivenState -> Changes -> ReplayableEvents` 是核心；`load/persist/replay/publish` 是外层机制。业务开发只要守住这个边界，就能让复杂交易系统在高速演进时仍然可计算、可测试、可复盘。
