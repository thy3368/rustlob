# 确定性复制状态机：从有序 command 到一致的领域事实

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

在本例中，core 的 `PlaceOnlySpotOrderV2UseCase` 负责校验和计算 `SpotOrderV2` 的创建变化；entity 提供订单行为和创建事件的字段投影。`StateMachineExecutor` 负责装载状态、调用规则、绑定日志元数据及按序调用 outbound；状态来源和事件落点属于 adapter/infra 边界，不参与业务决定。

## 单订单：一个意图，一个创建事实

[`place_only_spot_order_v2.rs`](place_only_spot_order_v2.rs) 构造 `PlaceOnlySpotOrderV2Cmd::Single`，其中订单 ID、价格、数量、方向等均来自 command。`LogEntrySequencer` 在进入执行器前分配 `lsn` 和 `logical_ts`。`check_command` 校验输入，`validate_state_given` 校验给定状态，`compute_state_changed_unchecked` 以 command 的 `logical_ts` 创建待处理订单；`compute_state_diff` 得到 `PlaceOnlySpotOrderV2Changes::Single`。

`changes.to_replayable_events()` 对该订单调用 `track_create_event()`，生成一个 `SpotOrderV2` 创建事件。执行器再将 event 的 `lsn`、`logical_ts` 设为 command 的值。因此，两个节点收到相同日志条目且使用相同规则时，订单 ID、状态、字段变化及最终事件元数据一致。示例断言了一个订单、一个创建事件，以及 event 与 command 的 `lsn`、`logical_ts` 相等。此创建事件的 `previous_lsn` 为 `0`，表示没有该实体的前一版本。

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
5. event 的 `lsn`、`logical_ts` 必须取自同一 command；单实体 `previous_lsn` 和版本推进必须依据相同的旧状态。副作用失败后的重试与恢复还需保证既有事件不会被重复应用。

这些条件满足时，可以逐个 `lsn` 归纳：同一前置状态与同一 command 产生同一 `changes` 和同一规范化 event，应用后得到同一后置状态；下一条 command 因而也从同一状态开始。`persist`、`replay`、`publish` 只是同一批已决定领域事实的不同落点，不应改变业务计算结果。工程上仍须处理持久化、应用和发布之间的失败恢复；领域计算的确定性本身不提供跨落点的原子性。
