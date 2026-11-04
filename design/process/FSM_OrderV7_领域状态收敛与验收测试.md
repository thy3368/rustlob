# 用 FSM 收敛领域状态：从 `OrderV7` 到验收测试驱动的需求发现

在交易系统里，订单状态经常从一个字段开始，最后变成一组散落的方法：撮合改状态，改单改数量，撤单改时间戳。问题是状态、允许的动作、派生结果和失败边界没有唯一入口，调用方可以绕过规则。

`SpotOrderV7` 采用更收敛的做法：订单实体持有状态，领域事件统一进入 FSM，先校验和计算，成功后一次性提交。状态转换因此成为事件驱动的领域契约。

## 先把状态和行为放回实体

`SpotOrderV7` 对外暴露订单能力，但把底层 `OrderBase` 隐藏在实体内部。调用方不应该直接改 `status`、`size` 或 `version`，而是发送三个领域事件：

```rust
enum SpotOrderV7Event {
    Match { makers: Vec<SpotOrderV7>, input: MatchSpotOrderV7Input, timestamp: u64 },
    Modify { price: Decimal, size: Decimal, timestamp: u64 },
    Cancel { timestamp: u64 },
}
```

实现 `Fsm` 后，`transition` 成为唯一状态入口：终态拒绝所有事件，其他事件先计算 next state，成功后才替换 `OrderBase`：

```rust
let (next, output) = compute_event(&self.order_base, event)?;
self.order_base = next;
Ok((self.order_base.status, output))
```

这里的“收敛”不是多加一个类型，而是让状态、转换规则、输出结果和错误原子性共用一条边界。失败时实体保持原样；版本溢出或手续费不足都不能留下半成品。`apply` 只是不关心输出，仍复用同一个 `transition`。

## `Match` 不应只返回一个布尔值

撮合的业务含义远大于“成功/失败”。`MatchSpotOrderV7Outcome` 把结果分成五类：

| Outcome | 业务事实 |
| --- | --- |
| `Resting` | 没有成交，订单继续挂单；Pending 可转为 Open |
| `PartiallyFilled` | 产生部分成交，taker 与多个 maker 的 after 状态都可核对 |
| `Filled` | taker 完成，maker 变化和全部 `trade` 一并返回 |
| `CanceledAfterPartialFill` | IOC 等场景下已经成交一部分，剩余部分取消 |
| `Rejected` | 订单被拒绝，仍返回拒绝后的订单事实 |

因此 outcome 是验收所需的业务真相：成交数量、maker 版本、手续费、trade id、时间戳和最终状态都在一个结果中，调用方不必根据 `bool` 重建 trade。

`Match` 还统一处理价格交叉、maker 数量消耗、手续费和 trade id 分配；任何一步失败都在提交前返回。实体负责单订单聚合规则，跨账户余额结算仍属于 use case。

## 验收测试是需求探测器

测试不应只证明“当前代码能跑”，而应把业务问题写成矩阵，并在失败时指出缺的是状态、事件、结果字段、错误规则，还是跨聚合编排。

| 维度 | 最小验收集合 |
| --- | --- |
| 状态 × 事件 | `Pending/Open/PartiallyFilled` 均验收 `Match/Modify/Cancel`；`Filled/Canceled/Rejected` 拒绝全部事件 |
| 成交数量 | 0、部分成交、完全成交 |
| maker 组合 | 0、1、多笔；不同价格、方向和状态 |
| 手续费 | 0、正常值、超过 reserve 导致拒绝 |
| 版本 | 普通版本与 `u64::MAX` 溢出 |
| 失败原子性 | 错误前后 taker、maker、版本、时间戳完全相等 |
| 成功一致性 | 订单版本、状态、时间戳、trade id 单调且相互一致 |
| API 语义 | `transition` 返回 outcome；`apply` 提交但丢弃 outcome |

现有测试已经证明：失败撮合和非法改单不会部分修改；三个终态拒绝全部事件；三个活跃状态允许完整事件矩阵；成功撮合会让 taker 和改变过的 maker 各推进一次版本；trade id 连续递增；版本溢出保持原子失败。

未覆盖不能写成“实现正确”。仍需确认：部分成交后的改单边界；手续费不足由订单层还是余额 use case 拒绝；多 maker 部分成交时 trade 与版本是否满足结算；撤单后的冻结资金释放语义；取消结果是否携带释放金额和结算凭证。覆盖缺口是需求议题，不自动等同于生产 bug。

## FSM、use case 与 replay 的边界

FSM 不应编排整个交易系统。按 `core / adapter / infra` 看：

```text
core
  entity: SpotOrderV7、Fsm、订单状态与不变量
  use_case: 装载多个订单、账户和余额，编排撮合与结算
adapter
  inbound: 把 HTTP/消息命令翻译为 use case 输入
  outbound: 实现 use case 定义的 port，翻译持久化或外部服务结果
infra
  数据库、消息系统、网络框架和具体 SDK
```

依赖方向是 `inbound -> use_case -> entity`、`outbound -> port <- use_case`；调用流是 `inbound -> use_case -> outbound -> infra`。实体不访问数据库，inbound 不承载撮合规则，outbound 只实现 port。

`SpotOrderV7` 收敛的是一个订单聚合的状态和行为。use case 才负责同时协调 taker、多个 maker、余额和 trade，形成多聚合的业务结果。需要稳定 before/after truth 时，再由 `StateMachineV2` 的 `StateMachineV2Diff` 计算可 replay 的 changes。这样，单实体规则、跨聚合编排和持久化重放各有唯一职责，不会把 FSM 误用成流程引擎。

## 结语：先让失败变得有意义

好的订单 FSM 要回答：事件是否允许？成功产生什么事实？失败时哪些对象不变？验收矩阵把已建模规则和未覆盖组合同时显式化。

当测试从“断言一个字段”升级为“验收 outcome 和不变量”，需求发现就不再依赖事后补丁；可信边界再由 use case 和 replay 扩展到完整流程。

## 关键实现路径

- `lib/example/entity/src/entity/spot/spot_order_v7.rs`
- `lib/common/entity/src/entity_v2/fsm.rs`
- `lib/common/entity/src/use_case/state_machine_v2.rs`
- `lib/example/entity/src/entity/spot/spot_order_primitives.rs`
