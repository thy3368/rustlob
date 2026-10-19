#  Rust之从0-1低时延CEX：定序器架构之基于确定性复制状态机：从有序 command 到一致的领域事实

核心命题是：**节点以相同顺序执行相同的 command，在相同初始状态、相同领域规则和确定性执行环境下，才会得到相同的 entity 状态与 event 序列。** 全局有序只解决“先做哪件事”；每一步做出相同决定，还取决于 use case 和 entity 的确定性。这里复制的是业务意图及其顺序，event 是各节点独立推导出的领域事实。

## 为什么以 command 为复制输入

command 表示“下单”等业务意图；有序日志为它分配 `lsn`，所有节点按同一顺序处理。每个节点用同一份前置状态执行同一规则，得到 `changes`，再投影为可复放的 event。这样，订单、撮合、清算或结算状态可以从相同历史重建；event 仍可持久化和分发，但它不是这个模型中节点间复制的原始决策输入。

```text
全局有序 LogEntry<Command>
    + authoritative given state
    -> use case 的校验与计算
    -> Changes（业务决定）
    -> to_replayable_events()（领域事实投影）
    -> 绑定 command 的 lsn / logical_ts
    -> persist / replay / publish
```

在本例中，core 的 `PlaceOnlySpotOrderV2UseCase` 负责校验和计算 `SpotOrderV2` 的创建变化；entity 提供订单行为和创建事件的字段投影。`StateMachineExecutor` 负责装载 authoritative state、调用规则、绑定日志元数据及按序调用 outbound；状态来源和事件落点属于 adapter/infra 边界，不参与业务决定。示例默认 outbound 的 `persist`、`replay`、`publish` 是 no-op，不能把这个示例称为已经完成持久化、共识或跨节点复制的系统。

## 领域框架中的几种对象

这些对象把“业务意图如何变成领域事实”拆成几个明确职责：

- `Command` 描述一次业务意图，同时携带 `lsn`、`logical_ts`、请求标识等日志元数据；它是复制和决策的输入，不是执行结果。
- `GivenState` 是 use case 决策所依赖的 authoritative 前置状态。它由 `StateSource` 装载，可能包含多个实体和上下文，不能用执行器本地缓存的偶然状态替代。
- `UseCase`（在本框架中由 `StateMachineOwnedV2Diff` 约束）完成命令校验、基于 `GivenState` 的业务校验，并计算本次 case 的结果。
- `Changes` 是一次业务 case 的领域真相，描述状态从 before 到 after 的变化；`ReplayableChanges` 负责把它稳定投影为事件，不描述数据库写入或消息发送等基础设施副作用。
- `Entity` 维护订单等领域对象的状态、版本和行为，并生成自己的字段变化投影。它决定实体如何演进，不负责选择存储或发布方式。
- `EntityReplayableEvent` 是 changes 的可重放领域事件，带有实体坐标、`previous_lsn`、字段变化以及稍后绑定的 command 元数据。

边界可以概括为：core 持有业务规则、实体行为以及 changes/event 投影；adapter 将外部请求转换为 command，并通过 `StateSource` / `StateSink` 提供状态加载和副作用端口；infra 实现具体存储、事件重放和发布。`StateMachineExecutor` 是编排器，不是领域规则容器，它固定执行 `check -> load -> compute -> project -> persist -> replay -> publish`：规则在 use case、状态行为在 entity，执行器只负责顺序、元数据绑定和错误阶段映射。

因此，一条业务链可以读作：`Command + GivenState -> UseCase/StateMachineOwnedV2Diff -> Changes/ReplayableChanges -> EntityReplayableEvent -> StateSink`。这条链解释了为什么复制输入是 command，而持久化、重放和发布使用的是由各节点从相同输入独立推导出的领域事实。

## 与低延时架构演进的呼应

本文与同目录的[低延时架构的演进与启示](https://mp.weixin.qq.com/s/Fk0NDOj2cq8wNVaEYnHESQ)在三个层面呼应，但这里讨论的是领域框架及其可选择的部署方式，不是已经测得的延迟收益：

- **关注点分离**：日志接收与定序、领域计算、事件落地各有职责。`StateMachineExecutor` 只编排状态加载、use case/entity 计算及 `StateSink` 的输出顺序；业务规则留在 core 的 use case 和 entity，adapter/infra 承接输入与持久化、重放、发布。
- **确定性复制**：全序的 `LogEntry<Command>` 固定输入顺序和逻辑时间。若前置状态、规则版本与执行环境也一致，各节点就能本地推导相同的 `Changes`，经 `ReplayableChanges` 投影并绑定日志元数据后得到相同的 `EntityReplayableEvent`。这对应文章所说的“以计算换一致性”，其成立条件见下文。
- **通信路径折叠**：`StateSource` / `StateSink` 保留状态加载和输出的逻辑边界。如果部署在同一进程或执行线程，adapter/infra 的实现可以用本地函数调用或内存传递缩短通信路径；这是运行时部署优化，不改变 `StateMachineOwnedV2Diff` 的领域契约，也不自动保证低延迟。

定序器架构的对应关系是：定序器只为原始 command 分配顺序与 `logical_ts`，不执行业务规则；业务状态机在定序之后消费有序日志，独立推进状态并产生输出。概念流程是 `sequencer -> ordered command -> deterministic use case/entity -> changes -> replayable events -> local/persistent/published outputs`。当前示例用 `LogEntrySequencer` 和执行器展示元数据分配与执行顺序；它没有实现独立的共识定序集群、检查点追赶、Active-Active fastest-wins 或故障恢复，也没有集成 Aeron。

## 单订单：一个意图，一个创建事实

[`place_only_spot_order_v2.rs`](place_only_spot_order_v2.rs) 构造 `PlaceOnlySpotOrderV2Cmd::Single`，其中订单 ID、价格、数量、方向等均来自 command。`LogEntrySequencer` 在进入执行器前分配 `lsn` 和 `logical_ts`。`check_command` 校验输入，`compute_state_diff` 在给定状态上完成校验与计算；其内部构造待处理订单时，把 `command.lsn()` 传给 `SpotOrderV2.version`，把 `command.logical_ts()` 传给 `created_at` 和 `updated_at`，最后得到 `PlaceOnlySpotOrderV2Changes::Single`。

`changes.to_replayable_events()` 对该订单调用 `track_create_event()`，生成一个 `SpotOrderV2` 创建事件。执行器再将 event 的 `lsn`、`logical_ts` 设为 command 的值。因此，两个节点收到相同日志条目且使用相同规则时，订单 ID、状态、`version`、字段变化及最终事件元数据一致。示例断言了一个订单、一个创建事件，以及 event 与 command 的 `lsn`、`logical_ts` 相等。此创建事件的 `previous_lsn` 为 `0`，表示没有该实体的前一版本。

## Normal TPSL：一个 command，多个有序事实

`PlaceOnlySpotOrderV2Cmd::NormalTpsl` 携带父订单和有序的 `children: Vec<_>`。规则先创建父订单，再按输入顺序创建子订单；每个子订单记录 `NormalTpslChild { parent_order_id }`，父订单记录 `NormalTpslParent`。`to_replayable_events()` 先投影父订单，再依次投影子订单。因此，包含两个子订单的同一 command 产生 **父、子 1、子 2** 三个创建事件；它们共享该 command 的 `lsn` 和 `logical_ts`，由 `(lsn, entity_type, entity_id)` 区分。执行器检查这组三元组在本次结果中不重复。

这里的事件顺序来自 `Vec` 的输入顺序和显式的父先子后投影规则，不能依赖无序集合的迭代顺序。用例测试验证了三个创建事件、父子关系及父事件位于首位。

## 因果坐标与实体版本

`lsn` 是日志条目的全局顺序号；同一个 command 可以产生多个共享该 `lsn` 的 entity event，因此 event 不能仅凭 `lsn` 唯一标识。`logical_ts` 由定序器分配：它参考定序时的物理时钟，同时保证相对于上一条日志严格递增。节点复放时使用日志中已有的值，不能各自重新取时钟。创建的 `SpotOrderV2.created_at` 和 `updated_at` 也取自 command 的 `logical_ts`。

event 的 `previous_lsn` 表示**同一实体**的前一版本坐标，不是前一个全局 command 的 `lsn`。创建事件为 `0`；更新事件需要从同一实体的旧状态推导前一版本，并遵守对应实体的版本规则。这样，全局日志顺序和单实体版本链分别回答“业务意图先后”与“这个实体从哪个版本演进”。

有一个实现边界需要注意：`track_create_event()` 初次构造 event 时会读取本地时间；本示例的 `StateMachineExecutor` 随后用 command 的 `logical_ts` 覆盖该值。因此**经此执行器返回的 event** 才满足这里的时间字段一致性断言；单独调用 `changes.to_replayable_events()` 所得的原始 event 不能据此宣称逐字节一致。

## 确定性成立的条件

1. 各节点接收完整且相同的 command 负载与日志元数据，并按相同 `lsn` 顺序执行；重试还须遵守同一幂等规则。
2. 从相同的初始快照或相同历史得到相同的 authoritative given state；缺失历史、不同装载结果或不同处理进度都可能使决定分叉。
3. use case 和 entity 规则版本兼容，分支、错误处理、字段投影和事件顺序一致；升级时须明确历史日志适用的规则版本。
4. 决策不得依赖节点本地随机数、当前时间、实时外部行情或无序容器迭代；所需外部事实应先固定为有序输入。ID、数值运算、溢出行为和序列化规则也必须一致。
5. event 的 `lsn`、`logical_ts` 必须取自同一 command；单实体 `previous_lsn` 和版本推进必须依据相同的旧状态。副作用失败后的重试与恢复还需保证既有事件不会被重复应用；当前示例的默认 outbound 只返回成功，不提供这种恢复保证。

这些条件满足时，可以逐个 `lsn` 归纳：同一前置状态与同一 command 产生同一 `changes` 和同一规范化 event，应用后得到同一后置状态；下一条 command 因而也从同一状态开始。`persist`、`replay`、`publish` 只是同一批已决定领域事实的不同落点，不应改变业务计算结果。工程上仍须处理持久化、应用和发布之间的失败恢复；领域计算的确定性本身不提供跨落点的原子性。
